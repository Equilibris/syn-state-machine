use crate::*;
use std::marker::PhantomData;

pub struct PeekAsParse<T>(pub usize, PhantomData<T>);

impl<Cursor: Iterator + ParserCursor, T: Peek<Cursor> + PeekError<Cursor>> Parse<Cursor>
    for PeekAsParse<T>
{
    fn parse(input: &mut crate::ParseBuffer<Cursor>) -> Result<Self, Cursor::Error> {
        match T::peek(&input.cursor) {
            Some(c) => {
                let _ = input.cursor.advance_by(c);
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
impl<Cursor: ParserCursor, T: PeekError<Cursor>> PeekError<Cursor> for PeekAsParse<T> {
    fn error(input: &Cursor) -> Cursor::Error {
        T::error(input)
    }
}
