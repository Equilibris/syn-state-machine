use crate::internals::*;
use proc_macro2::{extra::DelimSpan, Spacing, Span};

pub use proc_macro2::{Ident, Literal, Punct, TokenStream, TokenTree};

impl<'a> Parse<Cursor<'a>> for TokenStream {
    fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
        let cursor = input.cursor;
        let stream = cursor.token_stream();
        *input = cursor.skip_to_end().into();

        Ok(stream)
    }
}

macro_rules! pm2_impl {
    ($st:ident $m:ident) => {
        impl<'a> Parse<Cursor<'a>> for $st {
            fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
                let cur = input.cursor;

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
        impl<'a> Peek<Cursor<'a>> for $st {
            fn peek(input: &Cursor<'a>) -> Option<usize> {
                match input.$m() {
                    Some(_) => Some(1),
                    None => None,
                }
            }
        }

        impl FixedPeek for $st {
            const SKIP: usize = 1;
        }
        impl<'a> PeekError<Cursor<'a>> for $st {
            fn error(input: &Cursor<'a>) -> Error {
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
impl<'a, const VAL: &'static str> Parse<Cursor<'a>> for FIdent<VAL> {
    fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
        let cur = input.cursor;

        match cur.ident() {
            Some((v, c)) if v == VAL => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: &'static str> Peek<Cursor<'a>> for FIdent<VAL> {
    fn peek(input: &Cursor<'a>) -> Option<usize> {
        match input.ident() {
            Some((v, _)) if v == VAL => Some(1),
            _ => None,
        }
    }
}

impl<const VAL: &'static str> FixedPeek for FIdent<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: &'static str> PeekError<Cursor<'a>> for FIdent<VAL> {
    fn error(input: &Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[derive(Clone, Debug)]
pub struct FPunct<const VAL: char>(pub Span);
impl<'a, const VAL: char> Parse<Cursor<'a>> for FPunct<VAL> {
    fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
        let cur = input.cursor;

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: char> Peek<Cursor<'a>> for FPunct<VAL> {
    fn peek(input: &Cursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FPunct<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: char> PeekError<Cursor<'a>> for FPunct<VAL> {
    fn error(input: &Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[derive(Clone, Debug)]
pub struct FJointPunct<const VAL: char>(pub Span);
impl<'a, const VAL: char> Parse<Cursor<'a>> for FJointPunct<VAL> {
    fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
        let cur = input.cursor;

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: char> Peek<Cursor<'a>> for FJointPunct<VAL> {
    fn peek(input: &Cursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FJointPunct<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: char> PeekError<Cursor<'a>> for FJointPunct<VAL> {
    fn error(input: &Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[derive(Clone, Debug)]
pub struct FAlonePunct<const VAL: char>(pub Span);
impl<'a, const VAL: char> Parse<Cursor<'a>> for FAlonePunct<VAL> {
    fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
        let cur = input.cursor;

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => {
                *input = c.into();
                Ok(Self(v.span()))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: char> Peek<Cursor<'a>> for FAlonePunct<VAL> {
    fn peek(input: &Cursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FAlonePunct<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: char> PeekError<Cursor<'a>> for FAlonePunct<VAL> {
    fn error(input: &Cursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

macro_rules! grouped {
    ($ty:ident $del:ident $emsg:literal) => {
        #[derive(Debug, Clone)]
        pub struct $ty<T>(pub T, pub DelimSpan);
        impl<'a, T: Parse<Cursor<'a>>> Parse<Cursor<'a>> for $ty<T> {
            fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
                let cur = input.cursor;

                match cur.group(proc_macro2::Delimiter::$del) {
                    Some((inner, span, after)) => {
                        let mut pb = ParseBuffer::from(inner);
                        let v = pb.parse()?;

                        let cur = pb.cursor;

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
        impl<'a, T: Peek<Cursor<'a>>> Peek<Cursor<'a>> for $ty<T> {
            fn peek(input: &Cursor<'a>) -> Option<usize> {
                input
                    .group(proc_macro2::Delimiter::$del)
                    .and_then(|(inner, _, aft)| T::peek(&inner).map(|v| (inner.skip(v).eof(), aft)))
                    .filter(|(a, _)| *a)
                    .map(|(_, b)| b.current - input.current)
            }
        }
        impl<T: FixedPeek> FixedPeek for $ty<T> {
            const SKIP: usize = T::SKIP + 2;
        }
        impl<'a, T: PeekError<Cursor<'a>>> PeekError<Cursor<'a>> for $ty<T> {
            fn error(cursor: &Cursor<'a>) -> Error {
                match cursor.group(proc_macro2::Delimiter::$del) {
                    Some((inner, _, _)) => T::error(&inner),
                    None => Error::new(cursor.span(), concat!("Expected '", $emsg, "'")),
                }
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

    insta_match_test!(it_fails_on_empty_paren, Paren<()> : );
    insta_match_test!(it_matches_simple_grouped_paren, Paren<()> : ());
    insta_match_test!(it_matches_contentful_grouped_paren, Paren<Ident> : (hello));

    insta_match_test!(it_fails_on_empty_brace, Brace<()> : );
    insta_match_test!(it_matches_simple_grouped_brace, Brace<()> : {});
    insta_match_test!(it_matches_contentful_grouped_brace, Brace<Ident> : {hello});

    insta_match_test!(it_fails_on_empty_bracket, Bracket<()> : );
    insta_match_test!(it_matches_simple_grouped_bracket, Bracket<()> : [  ]);
    insta_match_test!(it_matches_contentful_grouped_bracket, Bracket<Ident> : [ hello ]);
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
