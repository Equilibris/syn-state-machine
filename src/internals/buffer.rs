use crate::internals::Result;

use super::cursor::Cursor;

#[derive(Clone, Copy)]
pub struct ParseBuffer<'a>(Cursor<'a>);

impl<'a> From<Cursor<'a>> for ParseBuffer<'a> {
    fn from(value: Cursor<'a>) -> Self {
        Self(value)
    }
}

impl<'a> ParseBuffer<'a> {
    fn handle_result<T>(&mut self, res: Result<(T, ParseBuffer<'a>)>) -> Result<T> {
        match res {
            Ok((output, inner)) => {
                *self = inner;
                Ok(output)
            }
            Err(e) => Err(e),
        }
    }

    pub fn cursor(&self) -> Cursor<'a> {
        self.0
    }

    pub fn peek<T: Peek>(&mut self) -> bool {
        if let Some(x) = T::peek(self) {
            self.0 = self.0.skip(x);
            true
        } else {
            false
        }
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        self.handle_result(T::parse(self))
    }
    pub fn call<T, E>(
        &mut self,
        function: impl Fn(&Self) -> Result<(T, ParseBuffer<'a>)>,
    ) -> Result<T> {
        self.handle_result(function(self))
    }
}

pub trait Parse: Sized {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)>;
}
pub trait Peek {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize>;
}
