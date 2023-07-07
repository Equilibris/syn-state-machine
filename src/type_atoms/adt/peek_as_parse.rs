use crate::*;
use std::marker::PhantomData;

pub struct PeekAsParse<T>(pub usize, PhantomData<T>);

impl<Cursor: Skip, T: Peek<Cursor> + PeekError<Cursor>> Parse<Cursor> for PeekAsParse<T> {
    fn parse(input: &mut crate::ParseBuffer<Cursor>) -> crate::Result<Self> {
        match T::peek(&input.cursor) {
            Some(c) => {
                input.cursor.skip(c);
                Ok(Self(c, PhantomData))
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
impl<Cursor, T: PeekError<Cursor>> PeekError<Cursor> for PeekAsParse<T> {
    fn error(input: &Cursor) -> crate::Error {
        T::error(input)
    }
}
