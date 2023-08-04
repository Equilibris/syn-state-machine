mod buffer;

pub use buffer::*;

pub trait ParserCursor {
    type Error;
}

pub struct LocError<'a, Loc>(pub &'a str, pub Loc);

pub trait Spanned {
    type Loc;

    fn span(&self) -> Self::Loc;
}

pub trait Finalizer<Out, With> {
    fn finalize(self, value: With) -> std::ops::ControlFlow<Out, Out>;
}

pub struct BlackHoleFinalizer<T>(pub T);

impl<Out, With> Finalizer<Out, With> for BlackHoleFinalizer<Out> {
    fn finalize(self, _: With) -> std::ops::ControlFlow<Out, Out> {
        std::ops::ControlFlow::Break(self.0)
    }
}

pub trait Parse<C: ParserCursor, With>: Sized {
    type Finalizer: Finalizer<Self, With>;

    fn parse(input: &mut ParseBuffer<C>) -> Result<Self::Finalizer, C::Error>;
}

pub trait Peek<C> {
    fn peek(input: &C) -> Option<usize>;
}
pub trait FixedPeek {
    const SKIP: usize;
}
pub trait PeekError<C: ParserCursor> {
    fn error(input: &C) -> C::Error;
}
pub trait CombineError<Other> {
    fn combine(&mut self, other: Other);
}
