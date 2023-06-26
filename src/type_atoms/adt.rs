mod infallible;
mod interlace;
mod min_length;
mod option;
mod sum;
mod tuple;
mod vec;
mod r#box {
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
}
mod peek_as_parse {
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
}

pub use infallible::*;
pub use interlace::*;
pub use min_length::*;
pub use option::*;
pub use peek_as_parse::*;
pub use r#box::*;
pub use sum::*;
pub use tuple::*;
pub use vec::*;
