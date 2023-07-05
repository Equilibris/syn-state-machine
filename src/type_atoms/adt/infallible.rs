pub use std::convert::Infallible;

use crate::*;

impl<'a> Parse<'a> for Infallible {
    fn parse(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Err(Error::new(input.span(), "Reached Infallible"))
    }
}
impl<'a> Peek<'a> for Infallible {
    fn peek(_: Cursor<'a>) -> Option<usize> {
        None
    }
}
impl<'a> PeekError<'a> for Infallible {
    fn error(input: Cursor<'a>) -> Error {
        Error::new(input.span(), "Reached Infallible")
    }
}
impl FixedPeek for Infallible {
    const SKIP: usize = 0;
}
