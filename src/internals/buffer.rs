use proc_macro2::{Ident, Span};

use crate::{internals::Result, Error};

use super::cursor::Cursor;

#[derive(Clone, Copy)]
pub struct ParseBuffer<'a>(Cursor<'a>);

impl<'a> From<Cursor<'a>> for ParseBuffer<'a> {
    fn from(value: Cursor<'a>) -> Self {
        Self(value)
    }
}

impl<'a> ParseBuffer<'a> {
    pub fn span(&self) -> Span {
        self.0.span()
    }
    pub fn cursor(&self) -> Cursor<'a> {
        self.0
    }

    pub fn ident_matching<Pred: FnOnce(&'a Ident) -> Result<()>>(
        &mut self,
        pred: Pred,
    ) -> Result<&'a Ident> {
        match self.cursor().ident() {
            Some((val, cur)) => {
                if let Err(e) = pred.call_once((val,)) {
                    Err(e)
                } else {
                    self.0 = cur;
                    Ok(val)
                }
            }
            None => Err(Error::new(self.span(), "Expected Ident")),
        }
    }

    pub fn peek<T: Peek>(&mut self) -> bool {
        if let Some(x) = T::peek(self.0) {
            self.0 = self.0.skip(x);
            true
        } else {
            false
        }
    }
    pub fn errored_peek<T: Peek + PeekError>(&mut self) -> Result<()> {
        if let Some(x) = T::peek(self.0) {
            self.0 = self.0.skip(x);
            Ok(())
        } else {
            Err(T::error(self.0))
        }
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(self)
    }
    pub fn call<T, E>(&mut self, function: impl Fn(&mut Self) -> Result<T>) -> Result<T> {
        function(self)
    }
}

pub trait Parse: Sized {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self>;
}
pub trait Peek {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize>;
}
pub trait FixedPeek {
    const SKIP: usize;
}
pub trait PeekError {
    fn error<'a>(input: Cursor<'a>) -> Error;
}
