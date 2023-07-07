use crate::internals::*;

impl<Cursor: Clone, T: Parse<Cursor>> Parse<Cursor> for Option<T> {
    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self> {
        let mut temp = input.clone();

        Ok(match temp.parse() {
            Ok(a) => {
                *input = temp;
                Some(a)
            }
            Err(_) => None,
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

    insta_match_test!(it_matches_only, Option<Ident> : <);
    insta_match_test!(it_returns_the_correct_length, Option<(Ident, Ident)> : hi <);
}
