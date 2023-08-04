use crate::internals::*;

#[derive(Clone, Copy)]
pub struct ParseBuffer<C> {
    pub cursor: C,
}

impl<C> From<C> for ParseBuffer<C> {
    fn from(cursor: C) -> Self {
        Self { cursor }
    }
}

impl<C: Iterator + ParserCursor> ParseBuffer<C> {
    pub fn peek<T: Peek<C>>(&mut self) -> bool {
        if let Some(x) = T::peek(&self.cursor) {
            let _ = self.cursor.advance_by(x);
            true
        } else {
            false
        }
    }
    pub fn errored_peek<T: Peek<C> + PeekError<C>>(&mut self) -> Result<(), C::Error> {
        if let Some(x) = T::peek(&self.cursor) {
            let _ = self.cursor.advance_by(x);
            Ok(())
        } else {
            Err(T::error(&self.cursor))
        }
    }
}
impl<C: ParserCursor> ParseBuffer<C> {
    pub fn parse<T: Parse<C, ()>>(&mut self) -> Result<T, C::Error> {
        Ok(match T::parse(self)?.finalize(()) {
            std::ops::ControlFlow::Continue(v) => v,
            std::ops::ControlFlow::Break(v) => v,
        })
    }
    pub fn call<T, E>(&mut self, function: impl Fn(&mut Self) -> Result<T, E>) -> Result<T, E> {
        function(self)
    }
}

impl<C: Spanned> Spanned for ParseBuffer<C> {
    type Loc = C::Loc;

    fn span(&self) -> C::Loc {
        self.cursor.span()
    }
}

impl<C> AsRef<C> for ParseBuffer<C> {
    fn as_ref(&self) -> &C {
        &self.cursor
    }
}
