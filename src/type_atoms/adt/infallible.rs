pub use std::convert::Infallible;

use crate::*;

impl<Out, With> Finalizer<Out, With> for std::convert::Infallible {
    fn finalize(self, _: With) -> std::ops::ControlFlow<Out, Out> {
        unreachable!()
    }
}

impl<Cursor: Spanned + ParserCursor, With> Parse<Cursor, With> for Infallible
where
    Cursor::Error: for<'a> From<LocError<'a, Cursor::Loc>>,
{
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
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

#[cfg(feature = "printing")]
impl ::quote::ToTokens for P<Infallible> {
    fn to_tokens(&self, _: &mut TokenStream) {
        unreachable!()
    }
}
