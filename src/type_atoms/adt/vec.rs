use crate::internals::*;
impl<T: Parse> Parse for Vec<T> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let mut input = input.clone();
        let mut vs = Vec::new();

        while !input.cursor().eof() {
            match input.parse() {
                Ok(a) => vs.push(a),
                Err(_) => return Ok((vs, input)),
            }
        }

        Ok((vs, input))
    }
}
impl<T: Peek> Peek for Vec<T> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        let cursor = input.cursor();
        let mut step = 0;

        while !cursor.eof() {
            match T::peek(&cursor.skip(step).into()) {
                Some(a) => step += a,
                None => return Some(step),
            }
        }

        Some(step)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    type Two = (FPunct<':'>, FPunct<':'>);

    insta_match_test!(it_matches_esoterics, Vec<(Ident, Option<(Two, Ident)>, Two)> : r1::r2::r3::r4::r5::);
    insta_match_test!(it_matches_catch_all, Vec<TokenTree> : r#hello hello struct _ 'a' { "hi" });
    insta_match_test!(it_matches_comments, Vec<TokenTree> : /* comment */);
    insta_match_test!(it_matches_basic_iteration, Vec<Ident> : hello world hi);
    insta_match_test!(it_specifies_correct_backstep, Vec<(Ident,Ident)> : hello world hi);
    insta_match_test!(it_can_work_on_individual_backtracks, Vec<(Ident, Option<Punct>)> :  hello < world hi );
}
