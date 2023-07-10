pub use std::convert::Infallible;

use crate::*;

impl<Cursor: Spanned + ParserCursor> Parse<Cursor> for Infallible
where
    Cursor::Error: for<'a> From<LocError<'a, Cursor::Loc>>,
{
    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self, Cursor::Error> {
        Err(Self::error(input.as_ref()))
    }
}
impl<Cursor> Peek<Cursor> for Infallible {
    fn peek(_: &Cursor) -> Option<usize> {
        None
    }
}
impl<Cursor: ParserCursor + Spanned> PeekError<Cursor> for Infallible
where
    Cursor::Error: for<'a> From<LocError<'a, Cursor::Loc>>,
{
    fn error(input: &Cursor) -> Cursor::Error {
        LocError("Reached Infallible", input.span()).into()
    }
}
impl FixedPeek for Infallible {
    const SKIP: usize = 0;
}
