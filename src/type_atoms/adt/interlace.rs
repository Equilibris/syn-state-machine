use std::marker::PhantomData;

#[cfg(feature = "printing")]
use quote::{ToTokens, TokenStreamExt};

use crate::{BlackHoleFinalizer, Parse, ParseBuffer, ParserCursor, Peek, Rep};

pub struct Interlace<A, B> {
    pub values: Vec<A>,
    phantom: PhantomData<B>,
}
impl<A, B> From<Vec<A>> for Interlace<A, B> {
    fn from(values: Vec<A>) -> Self {
        Self {
            values,
            phantom: PhantomData,
        }
    }
}
impl<A, B> From<Interlace<A, B>> for Vec<A> {
    fn from(val: Interlace<A, B>) -> Self {
        val.values
    }
}
impl<A, B> From<Rep<A>> for Interlace<A, B> {
    fn from(values: Rep<A>) -> Self {
        Self {
            values: values.0,
            phantom: PhantomData,
        }
    }
}
impl<A, B> From<Interlace<A, B>> for Rep<A> {
    fn from(val: Interlace<A, B>) -> Self {
        Rep(val.values)
    }
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

impl<Cursor: ParserCursor + Clone + Iterator, A: Parse<Cursor, ()>, B: Peek<Cursor>>
    Parse<Cursor, ()> for Interlace<A, B>
{
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
        let mut temp = input.clone();
        let mut values = Vec::new();

        match temp.parse() {
            Ok(value) => {
                *input = temp;
                values.push(value);
            }
            _ => return Ok(BlackHoleFinalizer(Self::new(values))),
        }

        while input.cursor.size_hint().0 > 0 {
            let mut tmp = input.clone();

            if tmp.peek::<B>() {
                match tmp.parse() {
                    Ok(value) => {
                        *input = tmp;
                        values.push(value);
                    }
                    _ => return Ok(BlackHoleFinalizer(Self::new(values))),
                }
            } else {
                return Ok(BlackHoleFinalizer(Self::new(values)));
            }
        }

        Ok(BlackHoleFinalizer(Self::new(values)))
    }
}

impl<Cursor: Iterator + Clone, A: Peek<Cursor>, B: Peek<Cursor>> Peek<Cursor> for Interlace<A, B> {
    fn peek(cursor: &Cursor) -> Option<usize> {
        let mut offset = 0;
        let mut cursor = cursor.clone();

        match A::peek(&cursor) {
            Some(value) => {
                offset += value;
                let _ = cursor.advance_by(value);
            }
            _ => return Some(0),
        }

        while cursor.size_hint().0 > 0 {
            let mut temp = cursor.clone();
            if let Some(a) = B::peek(&temp) {
                let _ = temp.advance_by(a);
                if let Some(b) = A::peek(&temp) {
                    let _ = temp.advance_by(b);
                    cursor = temp;
                    offset += a + b;
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

impl<Cursor: ParserCursor + Clone + Iterator, A: Parse<Cursor, ()>, B: Peek<Cursor>>
    Parse<Cursor, ()> for InterlaceTrail<A, B>
{
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
        let mut temp = input.clone();

        let mut values = Vec::new();

        match temp.parse() {
            Ok(value) => {
                *input = temp;
                values.push(value);
            }
            _ => return Ok(BlackHoleFinalizer(Self::new(values))),
        }

        while input.cursor.size_hint().0 > 0 {
            if input.peek::<B>() {
                match input.parse() {
                    Ok(value) => {
                        values.push(value);
                    }
                    _ => return Ok(BlackHoleFinalizer(Self::new(values))),
                }
            } else {
                return Ok(BlackHoleFinalizer(Self::new(values)));
            }
        }

        Ok(BlackHoleFinalizer(Self::new(values)))
    }
}

impl<Cursor: Clone + Iterator, A: Peek<Cursor>, B: Peek<Cursor>> Peek<Cursor>
    for InterlaceTrail<A, B>
{
    fn peek(input: &Cursor) -> Option<usize> {
        let mut offset = 0;
        let mut cursor = input.clone();

        match A::peek(input) {
            Some(value) => offset += value,
            _ => return Some(0),
        }
        let _ = cursor.advance_by(offset);

        while input.size_hint().0 > 0 {
            if let Some(a) = B::peek(&cursor) {
                let _ = cursor.advance_by(a);
                offset += a;
            } else {
                return Some(offset);
            }
            if let Some(b) = A::peek(&cursor) {
                let _ = cursor.advance_by(b);
                offset += b
            } else {
                return Some(offset);
            }
        }

        Some(offset)
    }
}

#[cfg(feature = "printing")]
impl<A: ToTokens, B: ToTokens + Default> ToTokens for Interlace<A, B> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut iterator = self.values.iter();

        if let Some(v) = iterator.next() {
            v.to_tokens(tokens);
        }
        for i in iterator {
            tokens.append_all(B::default().into_token_stream());
            i.to_tokens(tokens);
        }
    }
}

#[cfg(feature = "printing")]
impl<A: ToTokens, B: ToTokens + Default> ToTokens for InterlaceTrail<A, B> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut iterator = self.values.iter();

        if let Some(v) = iterator.next() {
            v.to_tokens(tokens);
        }
        for i in iterator {
            tokens.append_all(B::default().into_token_stream());
            i.to_tokens(tokens);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    type Two = P<(FPunct<':'>, FPunct<':'>)>;

    insta_match_test!(peek parse print : it_matches_esoterics, Interlace<P<(Ident, P<Option<P<(Two, Ident)>>>)>, Two> : r1::r2::r3::r4::r5);
    insta_match_test!(peek parse print : it_matches_empty, Interlace<Ident, FPunct<','>> : );
    insta_match_test!(peek parse print : it_matches_tokens_after_interlace, 
                      P<(Interlace<Ident, P<(FJointPunct<':'>, FPunct<':'>)>>, P<(FPunct<'>'>, FPunct<';'>)>)>
                      :  hello > ;);
    insta_match_test!(peek parse print : it_matches_comma_seperation, Interlace<Ident, FPunct<','>> :  hello, world, hi, there,);
    insta_match_test!(peek parse print : it_matches_comma_seperation_with_backstep, Interlace<P<(Ident, P<Option<Ident>>)>, FPunct<','>> :  hello, world, hi, there);

    // This fails printing, this makes a lot of sense based of the default value to Rep<T>,
    // following this it should not be tested with printing
    insta_match_test!(peek parse : it_matches_with_arbitrarilly_sized_interlacing, Interlace<P<(Ident, P<Option<Ident>>)>, Rep<FPunct<','>>> : hello hi world,,, hi, there  hello, world, hi, there);
    insta_match_test!(peek parse : it_matches_with_arbitrarilly, Interlace<P<(Ident, Rep<Ident>)>, Rep<FPunct<','>>> :  hello hi world,,, hi, there );

    insta_match_test!(peek parse print : it_matches_trailing, InterlaceTrail<Ident, FPunct<','>>: hi, hello);
    insta_match_test!(peek parse print : it_matches_trailing_with, InterlaceTrail<Ident, FPunct<','>>: hi, hello,);
}
