use std::marker::PhantomData;

use crate::{Cursor, Parse, ParseBuffer, Peek, Result};

pub struct Interlace<A, B> {
    pub values: Vec<A>,
    phantom: PhantomData<B>,
}

impl<A, B> Default for Interlace<A, B> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            phantom: Default::default(),
        }
    }
}

impl<A: std::fmt::Debug, B> std::fmt::Debug for Interlace<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interlace")
            .field("values", &self.values)
            .finish()
    }
}

impl<A, B> Interlace<A, B> {
    fn new(values: Vec<A>) -> Self {
        Self {
            values,
            phantom: Default::default(),
        }
    }
}

impl<A: Parse, B: Peek> Parse for Interlace<A, B> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let mut temp = input.clone();
        let mut values = Vec::new();

        match temp.parse() {
            Ok(value) => {
                *input = temp;
                values.push(value);
            }
            _ => return Ok(Self::new(values)),
        }

        while !input.cursor().eof() {
            let mut tmp = input.clone();

            if tmp.peek::<B>() {
                match tmp.parse() {
                    Ok(value) => {
                        *input = tmp;
                        values.push(value);
                    }
                    _ => return Ok(Self::new(values)),
                }
            } else {
                return Ok(Self::new(values));
            }
        }

        Ok(Self::new(values))
    }
}

impl<A: Peek, B: Peek> Peek for Interlace<A, B> {
    fn peek<'a>(cursor: Cursor<'a>) -> Option<usize> {
        let mut offset = 0;

        match A::peek(cursor) {
            Some(value) => offset += value,
            _ => return Some(offset),
        }

        while !cursor.skip(offset).eof() {
            if let Some(a) = B::peek(cursor.skip(offset)) {
                if let Some(b) = A::peek(cursor.skip(offset + a)) {
                    offset += a + b
                } else {
                    return Some(offset);
                }
            } else {
                return Some(offset);
            }
        }

        Some(offset)
    }
}

pub struct InterlaceTrail<A, B> {
    pub values: Vec<A>,
    phantom: PhantomData<B>,
}

impl<A, B> Default for InterlaceTrail<A, B> {
    fn default() -> Self {
        Self {
            values: Default::default(),
            phantom: Default::default(),
        }
    }
}

impl<A: std::fmt::Debug, B> std::fmt::Debug for InterlaceTrail<A, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Interlace")
            .field("values", &self.values)
            .finish()
    }
}

impl<A, B> InterlaceTrail<A, B> {
    fn new(values: Vec<A>) -> Self {
        Self {
            values,
            phantom: Default::default(),
        }
    }
}

impl<A: Parse, B: Peek> Parse for InterlaceTrail<A, B> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let mut temp = input.clone();

        let mut values = Vec::new();

        match temp.parse() {
            Ok(value) => {
                *input = temp;
                values.push(value);
            }
            _ => return Ok(Self::new(values)),
        }

        while !input.cursor().eof() {
            if input.peek::<B>() {
                match input.parse() {
                    Ok(value) => {
                        values.push(value);
                    }
                    _ => return Ok(Self::new(values)),
                }
            } else {
                return Ok(Self::new(values));
            }
        }

        Ok(Self::new(values))
    }
}

impl<A: Peek, B: Peek> Peek for InterlaceTrail<A, B> {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
        let mut offset = 0;

        match A::peek(input) {
            Some(value) => offset += value,
            _ => return Some(offset),
        }

        while !input.skip(offset).eof() {
            if let Some(a) = B::peek(input.skip(offset)) {
                offset += a;
            } else {
                return Some(offset);
            }
            if let Some(b) = A::peek(input.skip(offset)) {
                offset += b
            } else {
                return Some(offset);
            }
        }

        Some(offset)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    type Two = (FPunct<':'>, FPunct<':'>);

    insta_match_test!(it_matches_esoterics, Interlace<(Ident, Option<(Two, Ident)>), Two> : r1::r2::r3::r4::r5);
    insta_match_test!(it_matches_empty, Interlace<Ident, FPunct<','>> : );
    insta_match_test!(it_matches_tokens_after_interlace, 
                      (Interlace<Ident, (FJointPunct<':'>, FPunct<':'>)>, (FPunct<'>'>, FPunct<';'>),) 
                      :  hello > ;);
    insta_match_test!(it_matches_comma_seperation, Interlace<Ident, FPunct<','>> :  hello, world, hi, there,);
    insta_match_test!(it_matches_comma_seperation_with_backstep, Interlace<(Ident, Option<Ident>), FPunct<','>> :  hello, world, hi, there);
    insta_match_test!(it_matches_with_arbitrarilly_sized_interlacing, Interlace<(Ident, Option<Ident>), Vec<FPunct<','>>> : hello hi world,,, hi, there  hello, world, hi, there);

    insta_match_test!(it_matches_with_arbitrarilly, Interlace<(Ident, Vec<Ident>), Vec<FPunct<','>>> :  hello hi world,,, hi, there );

    insta_match_test!(it_matches_trailing, InterlaceTrail<Ident, FPunct<','>>: hi, hello);
    insta_match_test!(it_matches_trailing_with, InterlaceTrail<Ident, FPunct<','>>: hi, hello,);
}
