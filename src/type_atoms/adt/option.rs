use crate::internals::*;

impl<'a, T: Parse<'a>> Parse<'a> for Option<T> {
    fn parse(input: &mut ParseBuffer<'a>) -> Result<Self> {
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
impl<'a, T: Peek<'a>> Peek<'a> for Option<T> {
    fn peek(input: Cursor<'a>) -> Option<usize> {
        Some(T::peek(input).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(it_matches_only, Option<Ident> : <);
    insta_match_test!(it_returns_the_correct_length, Option<(Ident, Ident)> : hi <);
}
