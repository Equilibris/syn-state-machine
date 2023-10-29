mod r#box;
mod except;
mod infallible;
mod interlace;
mod lrk;
mod min_length;
mod option;
mod peek_as_parse;
mod sum;
mod tuple;
mod vec;
mod p {
    use crate::internals::*;

    #[derive(Clone, Default, Debug)]
    pub struct P<T>(pub T);

    pub struct PFinalizer<T>(T);

    impl<Out, With, T: Finalizer<Out, With>> Finalizer<P<Out>, With> for PFinalizer<T> {
        fn finalize(self, value: With) -> std::ops::ControlFlow<P<Out>, P<Out>> {
            self.0.finalize(value).map_break(P).map_continue(P)
        }
    }

    impl<C: ParserCursor, W, T: Parse<C, W>> Parse<C, W> for P<T> {
        type Finalizer = PFinalizer<T::Finalizer>;

        fn parse(
            input: &mut ParseBuffer<C>,
        ) -> Result<Self::Finalizer, <C as ParserCursor>::Error> {
            Ok(PFinalizer(T::parse(input)?))
        }
    }

    impl<C, T: Peek<C>> Peek<C> for P<T> {
        fn peek(input: &C) -> Option<usize> {
            T::peek(input)
        }
    }
    impl<C: ParserCursor, T: PeekError<C>> PeekError<C> for P<T> {
        fn error(input: &C) -> <C as ParserCursor>::Error {
            T::error(input)
        }
    }
    impl<T: FixedPeek> FixedPeek for P<T> {
        const SKIP: usize = T::SKIP;
    }
}

pub use except::*;
pub use infallible::*;
pub use interlace::*;
pub use lrk::*;
pub use min_length::*;
pub use option::*;
pub use p::*;
pub use peek_as_parse::*;
pub use r#box::*;
pub use sum::*;
pub use tuple::*;
pub use vec::*;
