pub use std::convert::Infallible;

use crate::*;

impl<Cursor: Spanned> Parse<Cursor> for Infallible {
    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self> {
        Err(Error::new(input.span(), "Reached Infallible"))
    }
}
impl<Cursor> Peek<Cursor> for Infallible {
    fn peek(_: &Cursor) -> Option<usize> {
        None
    }
}
impl<Cursor: Spanned> PeekError<Cursor> for Infallible {
    fn error(input: &Cursor) -> Error {
        Error::new(input.span(), "Reached Infallible")
    }
}
impl FixedPeek for Infallible {
    const SKIP: usize = 0;
}
