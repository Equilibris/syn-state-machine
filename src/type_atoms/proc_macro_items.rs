use crate::internals::*;
use proc_macro2::{extra::DelimSpan, Spacing, Span};

pub use proc_macro2::{Ident, Literal, Punct, TokenTree};

macro_rules! pm2_impl {
    ($st:ident $m:ident) => {
        impl Parse for $st {
            fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
                let cur = input.cursor();

                match cur.$m() {
                    Some((id, cur)) => {
                        *input = cur.into();
                        Ok(id.clone())
                    }
                    None => Err(Error::new(
                        input.span(),
                        concat!("Expected ", stringify!($m)),
                    )),
                }
            }
        }
        impl Peek for $st {
            fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
                match input.$m() {
                    Some(_) => Some(1),
                    None => None,
                }
            }
        }

        impl FixedPeek for $st {
            const SKIP: usize = 1;
        }
        impl PeekError for $st {
            fn error<'a>(input: Cursor<'a>) -> Error {
                Error::new(input.span(), concat!("Expected ", stringify!($m)))
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
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let cur = input.cursor();

        match cur.ident() {
            Some((v, c)) if v == VAL => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: &'static str> Peek for FIdent<VAL> {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
        match input.ident() {
            Some((v, _)) if v == VAL => Some(1),
            _ => None,
        }
    }
}

impl<const VAL: &'static str> FixedPeek for FIdent<VAL> {
    const SKIP: usize = 1;
}
impl<const VAL: &'static str> PeekError for FIdent<VAL> {
    fn error<'a>(input: Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[derive(Clone, Debug)]
pub struct FPunct<const VAL: char>(pub Span);
impl<const VAL: char> Parse for FPunct<VAL> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let cur = input.cursor();

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: char> Peek for FPunct<VAL> {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FPunct<VAL> {
    const SKIP: usize = 1;
}
impl<const VAL: char> PeekError for FPunct<VAL> {
    fn error<'a>(input: Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[derive(Clone, Debug)]
pub struct FJointPunct<const VAL: char>(pub Span);
impl<const VAL: char> Parse for FJointPunct<VAL> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let cur = input.cursor();

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: char> Peek for FJointPunct<VAL> {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FJointPunct<VAL> {
    const SKIP: usize = 1;
}
impl<const VAL: char> PeekError for FJointPunct<VAL> {
    fn error<'a>(input: Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[derive(Clone, Debug)]
pub struct FAlonePunct<const VAL: char>(pub Span);
impl<const VAL: char> Parse for FAlonePunct<VAL> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let cur = input.cursor();

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<const VAL: char> Peek for FAlonePunct<VAL> {
    fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FAlonePunct<VAL> {
    const SKIP: usize = 1;
}
impl<const VAL: char> PeekError for FAlonePunct<VAL> {
    fn error<'a>(input: Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

macro_rules! grouped {
    ($ty:ident $del:ident $emsg:literal) => {
        #[derive(Debug, Clone)]
        pub struct $ty<T>(pub T, pub DelimSpan);
        impl<T: Parse> Parse for $ty<T> {
            fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
                let cur = input.cursor();

                match cur.group(proc_macro2::Delimiter::$del) {
                    Some((inner, span, after)) => {
                        let mut pb = ParseBuffer::from(inner);
                        let v = pb.parse()?;

                        let cur = pb.cursor();

                        if cur.eof() {
                            *input = after.into();
                            Ok(Self(v, span))
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
            fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
                input
                    .group(proc_macro2::Delimiter::$del)
                    .and_then(|(inner, _, aft)| T::peek(inner).map(|v| (inner.skip(v).eof(), aft)))
                    .filter(|(a, _)| *a)
                    .map(|(_, b)| b.current - input.current)
            }
        }
    };
}

grouped!(Paren   Parenthesis "( ... )");
grouped!(Brace   Brace       "{ ... }");
grouped!(Bracket Bracket     "[ ... ]");

#[cfg(test)]
mod tests_groups {
    use crate::*;

    insta_match_test!(it_matches_simple_grouped, Paren<()> : ());
    insta_match_test!(it_matches_contentful_grouped, Paren<Ident> : (hello));
}

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
