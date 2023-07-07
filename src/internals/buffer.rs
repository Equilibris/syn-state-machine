use proc_macro2::Span;

use crate::{internals::Result, Error};

#[derive(Clone, Copy)]
pub struct ParseBuffer<C> {
    pub cursor: C,
}

impl<C> From<C> for ParseBuffer<C> {
    fn from(cursor: C) -> Self {
        Self { cursor }
    }
}

impl<C: Skip> ParseBuffer<C> {
    pub fn peek<T: Peek<C>>(&mut self) -> bool {
        if let Some(x) = T::peek(&self.cursor) {
            self.cursor.skip(x);
            true
        } else {
            false
        }
    }
    pub fn errored_peek<T: Peek<C> + PeekError<C>>(&mut self) -> Result<()> {
        if let Some(x) = T::peek(&self.cursor) {
            self.cursor.skip(x);
            Ok(())
        } else {
            Err(T::error(&self.cursor))
        }
    }
}
impl<C> ParseBuffer<C> {
    pub fn parse<T: Parse<C>>(&mut self) -> Result<T> {
        T::parse(self)
    }
    pub fn call<T, E>(&mut self, function: impl Fn(&mut Self) -> Result<T>) -> Result<T> {
        function(self)
    }
}

impl<C: Spanned> Spanned for ParseBuffer<C> {
    fn span(&self) -> Span {
        self.cursor.span()
    }
}

pub trait Skip {
    fn eof(&self) -> bool;
    fn skip(&mut self, count: usize);
}

pub trait Spanned {
    fn span(&self) -> Span;
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
