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

pub trait Parse<C: ParserCursor>: Sized {
    fn parse(input: &mut ParseBuffer<C>) -> Result<Self, C::Error>;
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
