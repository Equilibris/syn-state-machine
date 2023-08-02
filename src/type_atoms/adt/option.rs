use crate::internals::*;

// pub struct POption<T>(pub Option<T>);
// impl<T> From<Option<T>> for POption<T> {
//     fn from(value: Option<T>) -> Self {
//         Self(value)
//     }
// }
// impl<T> From<POption<T>> for Option<T> {
//     fn from(value: POption<T>) -> Self {
//         value.0
//     }
// }

// #[cfg(feature = "printing")]
// impl<T: quote::ToTokens> quote::ToTokens for POption<T> {
//     fn into_token_stream(self) -> proc_macro2::TokenStream
//     where
//         Self: Sized,
//     {
//         if let Some(v) = self.0 {
//             v.into_token_stream()
//         } else {
//             proc_macro2::TokenStream::new()
//         }
//     }
//     fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
//         if let Some(ref v) = self.0 {
//             v.to_tokens(tokens)
//         }
//     }
// }

impl<Cursor: Clone + ParserCursor, T: Parse<Cursor>> Parse<Cursor> for Option<T> {
    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self, Cursor::Error> {
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
