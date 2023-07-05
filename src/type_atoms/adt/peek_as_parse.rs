use crate::*;
use std::marker::PhantomData;

pub struct PeekAsParse<T>(pub usize, PhantomData<T>);

impl<T: Peek + PeekError> Parse for PeekAsParse<T> {
    fn parse<'a>(input: &mut crate::ParseBuffer<'a>) -> crate::Result<Self> {
        match T::peek(input.cursor()) {
            Some(c) => {
                *input = input.cursor().skip(c).into();
                Ok(Self(c, PhantomData))
            }
            None => Err(T::error(input.cursor())),
        }
    }
}
impl<T: Peek> Peek for PeekAsParse<T> {
    fn peek<'a>(input: crate::Cursor<'a>) -> Option<usize> {
        T::peek(input)
    }
}
impl<T: FixedPeek> FixedPeek for PeekAsParse<T> {
    const SKIP: usize = T::SKIP;
}
impl<T: PeekError> PeekError for PeekAsParse<T> {
    fn error<'a>(input: crate::Cursor<'a>) -> crate::Error {
        T::error(input)
    }
}
