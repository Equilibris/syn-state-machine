use crate::internals::*;

impl<'a, T: Parse<'a>> Parse<'a> for Vec<T> {
    fn parse(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let mut temp = input.clone();
        let mut vs = Vec::new();

        while !temp.cursor().eof() {
            match temp.parse() {
                Ok(a) => vs.push(a),
                Err(_) => {
                    *input = temp;
                    return Ok(vs);
                }
            }
        }

        *input = temp;
        Ok(vs)
    }
}
impl<'a, T: Peek<'a>> Peek<'a> for Vec<T> {
    fn peek(cursor: Cursor<'a>) -> Option<usize> {
        let mut step = 0;

        while !cursor.eof() {
            match T::peek(cursor.skip(step)) {
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

    insta_match_test!(it_matches_esoterics, Vec<(Ident, Option<(Two, Ident)>, Two)>     : r1::r2::r3::r4::r5::);
    insta_match_test!(it_matches_catch_all,                 Vec<TokenTree>              : r#hello hello struct _ 'a' { "hi" });
    insta_match_test!(it_matches_comments,                  Vec<TokenTree>              : /* comment */);
    insta_match_test!(it_matches_basic_iteration,           Vec<Ident>                  : hello world hi);
    insta_match_test!(it_specifies_correct_backstep,        Vec<(Ident, Ident)>         : hello world hi);
    insta_match_test!(it_can_work_on_individual_backtracks, Vec<(Ident, Option<Punct>)> :  hello < world hi );
}
