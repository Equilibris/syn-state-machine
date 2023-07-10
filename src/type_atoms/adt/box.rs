use crate::*;

impl<Cursor: ParserCursor, T: Parse<Cursor>> Parse<Cursor> for Box<T> {
    fn parse(input: &mut crate::ParseBuffer<Cursor>) -> Result<Self, Cursor::Error> {
        input.parse().map(Box::new)
    }
}
impl<Cursor, T: Peek<Cursor>> Peek<Cursor> for Box<T> {
    fn peek(input: &Cursor) -> Option<usize> {
        T::peek(input)
    }
}
impl<T: FixedPeek> FixedPeek for Box<T> {
    const SKIP: usize = T::SKIP;
}
impl<Cursor: ParserCursor, T: PeekError<Cursor>> PeekError<Cursor> for Box<T> {
    fn error(input: &Cursor) -> Cursor::Error {
        T::error(input)
    }
}
