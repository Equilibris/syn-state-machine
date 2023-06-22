pub use std::convert::Infallible;

use crate::*;

impl Parse for Infallible {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Err(Error::new(input.span(), "Reached Infallible"))
    }
}
impl Peek for Infallible {
    fn peek<'a>(_: Cursor<'a>) -> Option<usize> {
        None
    }
}
impl PeekError for Infallible {
    fn error<'a>(input: Cursor<'a>) -> Error {
        Error::new(input.span(), "Reached Infallible")
    }
}
impl FixedPeek for Infallible {
    const SKIP: usize = 0;
}
