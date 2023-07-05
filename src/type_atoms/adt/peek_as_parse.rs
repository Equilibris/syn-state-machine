use crate::*;
use std::marker::PhantomData;

pub struct PeekAsParse<T>(pub usize, PhantomData<T>);

impl<'a, T: Peek<'a> + PeekError<'a>> Parse<'a> for PeekAsParse<T> {
    fn parse(input: &mut crate::ParseBuffer<'a>) -> crate::Result<Self> {
        match T::peek(input.cursor()) {
            Some(c) => {
                *input = input.cursor().skip(c).into();
                Ok(Self(c, PhantomData))
            }
            None => Err(T::error(input.cursor())),
        }
    }
}
impl<'a, T: Peek<'a>> Peek<'a> for PeekAsParse<T> {
    fn peek(input: crate::Cursor<'a>) -> Option<usize> {
        T::peek(input)
    }
}
impl<T: FixedPeek> FixedPeek for PeekAsParse<T> {
    const SKIP: usize = T::SKIP;
}
impl<'a, T: PeekError<'a>> PeekError<'a> for PeekAsParse<T> {
    fn error(input: crate::Cursor<'a>) -> crate::Error {
        T::error(input)
    }
}
