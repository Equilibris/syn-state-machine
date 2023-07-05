use crate::*;

impl<'a, T: Parse<'a>> Parse<'a> for Box<T> {
    fn parse(input: &mut crate::ParseBuffer<'a>) -> crate::Result<Self> {
        input.parse().map(Box::new)
    }
}
impl<'a, T: Peek<'a>> Peek<'a> for Box<T> {
    fn peek(input: crate::Cursor<'a>) -> Option<usize> {
        T::peek(input)
    }
}
impl<T: FixedPeek> FixedPeek for Box<T> {
    const SKIP: usize = T::SKIP;
}
impl<'a, T: PeekError<'a>> PeekError<'a> for Box<T> {
    fn error(input: crate::Cursor<'a>) -> crate::Error {
        T::error(input)
    }
}
