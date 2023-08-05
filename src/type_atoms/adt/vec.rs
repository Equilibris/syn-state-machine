use crate::internals::*;

#[derive(Debug, Default)]
pub struct Rep<T>(pub Vec<T>);

impl<T> From<Vec<T>> for Rep<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}
impl<T> From<Rep<T>> for Vec<T> {
    fn from(value: Rep<T>) -> Self {
        value.0
    }
}

#[cfg(feature = "printing")]
impl<T: quote::ToTokens> quote::ToTokens for Rep<T> {
    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        let mut stream = proc_macro2::TokenStream::new();

        for i in self.0 {
            stream.extend(i.into_token_stream())
        }
        stream
    }
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        for i in &self.0 {
            i.to_tokens(tokens)
        }
    }
}

impl<C: Iterator + Clone + ParserCursor, T: Parse<C, ()>> Parse<C, ()> for Rep<T> {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<C>) -> Result<Self::Finalizer, C::Error> {
        let mut temp = input.clone();
        let mut vs = Vec::new();

        while temp.cursor.size_hint().0 > 0 {
            match temp.parse() {
                Ok(a) => vs.push(a),
                Err(_) => {
                    *input = temp;
                    return Ok(BlackHoleFinalizer(Rep(vs)));
                }
            }
        }

        *input = temp;
        Ok(BlackHoleFinalizer(Rep(vs)))
    }
}
impl<C: Iterator + Clone, T: Peek<C>> Peek<C> for Rep<T> {
    fn peek(cursor: &C) -> Option<usize> {
        let mut step = 0;
        let mut cursor = cursor.clone();

        while cursor.size_hint().0 > 0 {
            match T::peek(&cursor) {
                Some(a) => {
                    step += a;
                    let _ = cursor.advance_by(a);
                }
                None => return Some(step),
            }
        }

        Some(step)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    type Two = P<(FPunct<':'>, FPunct<':'>)>;

    insta_match_test!(peek parse print : it_matches_esoterics, Rep<P<(Ident, P<Option<P<(Two, Ident)>>>, Two)>> : r1::r2::r3::r4::r5::);
    insta_match_test!(peek parse print : it_matches_catch_all,                 Rep<TokenTree>              : r#hello hello struct _ 'a' { "hi" });
    insta_match_test!(peek parse print : it_matches_comments,                  Rep<TokenTree>              : /* comment */);
    insta_match_test!(peek parse print : it_matches_basic_iteration,           Rep<Ident>                  : hello world hi);
    insta_match_test!(peek parse print : it_specifies_correct_backstep,        Rep<P<(Ident, Ident)>>         : hello world hi);
    insta_match_test!(peek parse print : it_can_work_on_individual_backtracks, Rep<P<(Ident, P<Option<Punct>>)>> :  hello < world hi );
}
