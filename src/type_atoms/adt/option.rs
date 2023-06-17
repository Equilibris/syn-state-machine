use crate::internals::*;

impl<T: Parse> Parse for Option<T> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let mut input = input.clone();

        Ok((
            match input.parse() {
                Ok(a) => Some(a),
                Err(_) => None,
            },
            input,
        ))
    }
}
impl<T: Peek> Peek for Option<T> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        Some(T::peek(input).unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(it_matches_only, Option<Ident> : <);
    insta_match_test!(it_returns_the_correct_length, Option<(Ident, Ident)> : hi <);
}
