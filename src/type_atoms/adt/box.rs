use crate::*;

impl<T: Parse> Parse for Box<T> {
    fn parse<'a>(input: &mut crate::ParseBuffer<'a>) -> crate::Result<Self> {
        input.parse().map(Box::new)
    }
}
impl<T: Peek> Peek for Box<T> {
    fn peek<'a>(input: crate::Cursor<'a>) -> Option<usize> {
        T::peek(input)
    }
}
impl<T: FixedPeek> FixedPeek for Box<T> {
    const SKIP: usize = T::SKIP;
}
impl<T: PeekError> PeekError for Box<T> {
    fn error<'a>(input: crate::Cursor<'a>) -> crate::Error {
        T::error(input)
    }
}
