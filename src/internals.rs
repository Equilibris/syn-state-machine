mod buffer;
mod cursor;
mod error;
mod thread;

pub use buffer::*;
pub use cursor::*;
pub use error::*;
pub(crate) use thread::*;

pub trait Spanned {
    fn span(&self) -> proc_macro2::Span;
}

pub trait Parse<C>: Sized {
    fn parse(input: &mut ParseBuffer<C>) -> Result<Self>;
}
pub trait Peek<C> {
    fn peek(input: &C) -> Option<usize>;
}
pub trait FixedPeek {
    const SKIP: usize;
}
pub trait PeekError<C> {
    fn error(input: &C) -> Error;
}
