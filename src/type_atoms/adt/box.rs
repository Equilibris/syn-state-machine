use crate::*;

pub struct BoxFinalizer<T>(T);

impl<Out, With, T: Finalizer<Out, With>> Finalizer<Box<Out>, With> for BoxFinalizer<T> {
    fn finalize(self, value: With) -> std::ops::ControlFlow<Box<Out>, Box<Out>> {
        self.0
            .finalize(value)
            .map_break(Box::new)
            .map_continue(Box::new)
    }
}

impl<Cursor: ParserCursor, With, T: Parse<Cursor, With>> Parse<Cursor, With> for Box<T> {
    type Finalizer = BoxFinalizer<T::Finalizer>;

    fn parse(input: &mut crate::ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
        Ok(BoxFinalizer(T::parse(input)?))
    }
}
impl<Cursor, T: Peek<Cursor>> Peek<Cursor> for Box<T> {
    fn peek(input: &Cursor) -> Option<usize> {
        T::peek(input)
    }
}
impl<T: FixedPeek> FixedPeek for Box<T> {
    const SKIP: usize = T::SKIP;
}
impl<Cursor: ParserCursor, T: PeekError<Cursor>> PeekError<Cursor> for Box<T> {
    fn error(input: &Cursor) -> Cursor::Error {
        T::error(input)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test! { parse peek print : it_matches_boxed_ident, Box<Ident> : hello }
}
