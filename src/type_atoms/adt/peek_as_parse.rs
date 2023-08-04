use crate::*;
use std::marker::PhantomData;

pub struct PeekAsParse<T>(pub usize, PhantomData<T>);

impl<Cursor: Iterator + ParserCursor, T: Peek<Cursor> + PeekError<Cursor>> Parse<Cursor, ()>
    for PeekAsParse<T>
{
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut crate::ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
        match T::peek(&input.cursor) {
            Some(c) => {
                let _ = input.cursor.advance_by(c);
                Ok(BlackHoleFinalizer(Self(c, PhantomData)))
            }
            None => Err(T::error(&input.cursor)),
        }
    }
}
impl<Cursor, T: Peek<Cursor>> Peek<Cursor> for PeekAsParse<T> {
    fn peek(input: &Cursor) -> Option<usize> {
        T::peek(input)
    }
}
impl<T: FixedPeek> FixedPeek for PeekAsParse<T> {
    const SKIP: usize = T::SKIP;
}
impl<Cursor: ParserCursor, T: PeekError<Cursor>> PeekError<Cursor> for PeekAsParse<T> {
    fn error(input: &Cursor) -> Cursor::Error {
        T::error(input)
    }
}

#[cfg(feature = "printing")]
impl<T: quote::ToTokens + Default> quote::ToTokens for PeekAsParse<T> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.0.to_tokens(tokens)
    }

    fn to_token_stream(&self) -> TokenStream {
        self.0.to_token_stream()
    }

    fn into_token_stream(self) -> TokenStream
    where
        Self: Sized,
    {
        self.0.into_token_stream()
    }
}
