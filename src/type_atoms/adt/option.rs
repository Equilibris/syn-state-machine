use crate::internals::*;

pub struct OptionFinalizer<T>(Option<T>);
impl<Out, With, T: Finalizer<Out, With>> Finalizer<Option<Out>, With> for OptionFinalizer<T> {
    fn finalize(self, value: With) -> std::ops::ControlFlow<Option<Out>, Option<Out>> {
        match self.0 {
            Some(v) => v.finalize(value).map_break(Some).map_continue(Some),
            None => std::ops::ControlFlow::Break(None),
        }
    }
}

impl<Cursor: Clone + ParserCursor, With, T: Parse<Cursor, With>> Parse<Cursor, With> for Option<T> {
    type Finalizer = OptionFinalizer<T::Finalizer>;

    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
        let mut temp = input.clone();

        Ok(match T::parse(&mut temp) {
            Ok(ok) => {
                *input = temp;
                OptionFinalizer(Some(ok))
            }
            Err(_) => OptionFinalizer(None),
        })
    }
}
impl<Cursor, T: Peek<Cursor>> Peek<Cursor> for Option<T> {
    fn peek(input: &Cursor) -> Option<usize> {
        let v = T::peek(input);

        Some(v.unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(peek parse : it_matches_only, Option<Ident> : <);
    insta_match_test!(peek parse : it_returns_the_correct_length, Option<(Ident, Ident)> : hi <);
}
