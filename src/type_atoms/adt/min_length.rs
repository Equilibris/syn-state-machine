use crate::{Error, Interlace, InterlaceTrail, Parse, ParseBuffer, Peek, Result};

#[derive(Debug, Clone)]
pub struct MinLength<T, const LEN: usize = 1>(pub T);

impl<const LEN: usize, A: Parse, B: Peek> Parse for MinLength<Interlace<A, B>, LEN> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let c: Interlace<A, B> = input.parse()?;
        if c.values.len() < LEN {
            Err(Error::new(input.span(), "Expected value"))
        } else {
            Ok(Self(c))
        }
    }
}
impl<const LEN: usize, A: Parse, B: Peek> Parse for MinLength<InterlaceTrail<A, B>, LEN> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let c: InterlaceTrail<A, B> = input.parse()?;
        if c.values.len() < LEN {
            Err(Error::new(input.span(), "Expected value"))
        } else {
            Ok(Self(c))
        }
    }
}
impl<const LEN: usize, A: Parse> Parse for MinLength<Vec<A>, LEN> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let c: Vec<A> = input.parse()?;
        if c.len() < LEN {
            Err(Error::new(input.span(), "Expected value"))
        } else {
            Ok(Self(c))
        }
    }
}
