use crate::{Error, Interlace, InterlaceTrail, Parse, ParseBuffer, Result, Spanned};

pub trait ParsableLength {
    fn len(&self) -> usize;
}

impl<T> ParsableLength for Vec<T> {
    fn len(&self) -> usize {
        self.len()
    }
}
impl<A, B> ParsableLength for Interlace<A, B> {
    fn len(&self) -> usize {
        self.values.len()
    }
}
impl<A, B> ParsableLength for InterlaceTrail<A, B> {
    fn len(&self) -> usize {
        self.values.len()
    }
}
impl<T: ParsableLength> ParsableLength for MinLength<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<T: ParsableLength> ParsableLength for Box<T> {
    fn len(&self) -> usize {
        self.len()
    }
}

#[derive(Debug, Clone)]
pub struct MinLength<T, const LEN: usize = 1>(pub T);
impl<Cursor: Spanned, const LEN: usize, T: Parse<Cursor> + ParsableLength> Parse<Cursor>
    for MinLength<T, LEN>
{
    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self> {
        let c: T = input.parse()?;
        if c.len() < LEN {
            Err(Error::new(input.span(), "Expected value"))
        } else {
            Ok(Self(c))
        }
    }
}
