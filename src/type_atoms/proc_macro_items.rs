use crate::internals::*;
use proc_macro2::{extra::DelimSpan, Spacing, Span};

pub use proc_macro2::{Ident, Literal, Punct, TokenTree};

macro_rules! pm2_impl {
    ($st:ident $m:ident) => {
        impl Parse for $st {
            fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
                let input = input.cursor();

                match input.$m() {
                    Some((id, cur)) => Ok((id.clone(), cur.into())),
                    None => Err(Error::new(
                        input.span(),
                        concat!("Expected ", stringify!($m)),
                    )),
                }
            }
        }
        impl Peek for $st {
            fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
                match input.cursor().$m() {
                    Some(_) => Some(1),
                    None => None,
                }
            }
        }
    };
}

pm2_impl!(Ident ident);
pm2_impl!(Punct punct);
pm2_impl!(Literal literal);
pm2_impl!(TokenTree token_tree);

#[derive(Clone, Debug)]
pub struct FIdent<const VAL: &'static str>(pub Span);
impl<const VAL: &'static str> Parse for FIdent<VAL> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let cur = input.cursor();

        match cur.ident() {
            Some((v, c)) if v == VAL => Ok((Self(v.span()), c.into())),
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: &'static str> Peek for FIdent<VAL> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        match input.cursor().ident() {
            Some((v, _)) if v == VAL => Some(1),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FPunct<const VAL: char>(pub Span);
impl<const VAL: char> Parse for FPunct<VAL> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let cur = input.cursor();

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL => Ok((Self(v.span()), c.into())),
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: char> Peek for FPunct<VAL> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        match input.cursor().punct() {
            Some((v, _)) if v.as_char() == VAL => Some(1),
            _ => None,
        }
    }
}
#[derive(Clone, Debug)]
pub struct FJointPunct<const VAL: char>(pub Span);
impl<const VAL: char> Parse for FJointPunct<VAL> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let cur = input.cursor();

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => {
                Ok((Self(v.span()), c.into()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: char> Peek for FJointPunct<VAL> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        match input.cursor().punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => Some(1),
            _ => None,
        }
    }
}
#[derive(Clone, Debug)]
pub struct FAlonePunct<const VAL: char>(pub Span);
impl<const VAL: char> Parse for FAlonePunct<VAL> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let cur = input.cursor();

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => {
                Ok((Self(v.span()), c.into()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: char> Peek for FAlonePunct<VAL> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        match input.cursor().punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => Some(1),
            _ => None,
        }
    }
}

macro_rules! grouped {
    ($ty:ident $del:ident $emsg:literal) => {
        pub struct $ty<T>(pub T, pub DelimSpan);
        impl<T: Parse> Parse for $ty<T> {
            fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
                let cur = input.cursor();

                match cur.group(proc_macro2::Delimiter::$del) {
                    Some((inner, span, after)) => {
                        let mut pb = ParseBuffer::from(inner);
                        let v = pb.parse()?;

                        let cur = pb.cursor();

                        if cur.eof() {
                            Ok((Self(v, span), after.into()))
                        } else {
                            Err(Error::new(
                                cur.span().join(cur.skip_to_end().prev().span()).unwrap(),
                                "Expected nothing",
                            ))
                        }
                    }
                    None => Err(Error::new(cur.span(), concat!("Expected '", $emsg, "'"))),
                }
            }
        }
        impl<T: Peek> Peek for $ty<T> {
            fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
                let cur = input.cursor();
                cur.group(proc_macro2::Delimiter::$del)
                    .and_then(|(inner, _, aft)| {
                        T::peek(&ParseBuffer::from(inner)).map(|v| (inner.skip(v).eof(), aft))
                    })
                    .filter(|(a, _)| *a)
                    .map(|(_, b)| b.current - cur.current)
            }
        }
    };
}

grouped!(Paren   Parenthesis "( ... )");
grouped!(Brace   Brace       "{ ... }");
grouped!(Bracket Bracket     "[ ... ]");

#[cfg(test)]
mod tests_id {
    use crate::*;

    insta_match_test!(it_matches_id, Ident: id);
    insta_match_test!(it_matches_fixed, FIdent<"id"> : id);
    insta_match_test!(it_fails_on_incorrect, FIdent<"id"> : ident);
}

#[cfg(test)]
mod tests_punct {
    use crate::*;

    insta_match_test!(it_matches_only, Punct : < );
    insta_match_test!(it_matches_fixed, FPunct<'<'> : < );
    insta_match_test!(it_matches_dollar, FPunct<'$'> : $ );

    // insta_match_test!(it_matches_joint, (FJointPunct<'\''>, Ident) : 'hello );
    insta_match_test!(it_matches_both, (FJointPunct<'<'>, FAlonePunct<'='>) : <= );

    insta_match_test!(it_matches_dollar_crate, (FPunct<'$'>,FIdent<"crate">) : $crate);
}
