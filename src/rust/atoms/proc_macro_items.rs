use super::*;
use crate::internals::*;
use proc_macro2::{extra::DelimSpan, Spacing, Span};

pub use proc_macro2::{Ident, Literal, Punct, TokenStream, TokenTree};
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

impl<'a> Parse<RustCursor<'a>, ()> for TokenStream {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        let cursor = input.cursor;
        let stream = cursor.token_stream();
        *input = cursor.skip_to_end().into();

        Ok(BlackHoleFinalizer(stream))
    }
}

macro_rules! pm2_impl {
    ($st:ident $m:ident) => {
        impl<'a> Parse<RustCursor<'a>, ()> for $st {
            type Finalizer = BlackHoleFinalizer<Self>;

            fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
                let cur = input.cursor;

                match cur.$m() {
                    Some((id, cur)) => {
                        *input = cur.into();
                        Ok(BlackHoleFinalizer(id.clone()))
                    }
                    None => Err(Error::new(
                        input.span(),
                        concat!("Expected ", stringify!($m)),
                    )),
                }
            }
        }
        impl<'a> Peek<RustCursor<'a>> for $st {
            fn peek(input: &RustCursor<'a>) -> Option<usize> {
                match input.$m() {
                    Some(_) => Some(1),
                    None => None,
                }
            }
        }

        impl FixedPeek for $st {
            const SKIP: usize = 1;
        }
        impl<'a> PeekError<RustCursor<'a>> for $st {
            fn error(input: &RustCursor<'a>) -> Error {
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
impl<'a, const VAL: &'static str> Parse<RustCursor<'a>, ()> for FIdent<VAL> {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        let cur = input.cursor;

        match cur.ident() {
            Some((v, c)) if v == VAL => {
                *input = c.into();
                Ok(BlackHoleFinalizer(Self(v.span())))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: &'static str> Peek<RustCursor<'a>> for FIdent<VAL> {
    fn peek(input: &RustCursor<'a>) -> Option<usize> {
        match input.ident() {
            Some((v, _)) if v == VAL => Some(1),
            _ => None,
        }
    }
}

impl<const VAL: &'static str> FixedPeek for FIdent<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: &'static str> PeekError<RustCursor<'a>> for FIdent<VAL> {
    fn error(input: &RustCursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[cfg(feature = "printing")]
impl<const VAL: &'static str> quote::ToTokens for FIdent<VAL> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Ident::new(VAL, Span::call_site()))
    }
}
impl<const VAL: &'static str> Default for FIdent<VAL> {
    fn default() -> Self {
        Self(Span::call_site())
    }
}

#[derive(Clone, Debug)]
pub struct FPunct<const VAL: char>(pub Span);
impl<'a, const VAL: char> Parse<RustCursor<'a>, ()> for FPunct<VAL> {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        let cur = input.cursor;

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL => {
                *input = c.into();
                Ok(BlackHoleFinalizer(Self(v.span())))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: char> Peek<RustCursor<'a>> for FPunct<VAL> {
    fn peek(input: &RustCursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FPunct<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: char> PeekError<RustCursor<'a>> for FPunct<VAL> {
    fn error(input: &RustCursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[cfg(feature = "printing")]
impl<const VAL: char> quote::ToTokens for FPunct<VAL> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new(VAL, Spacing::Alone))
    }
}
impl<const VAL: char> Default for FPunct<VAL> {
    fn default() -> Self {
        Self(Span::call_site())
    }
}

#[derive(Clone, Debug)]
pub struct FJointPunct<const VAL: char>(pub Span);
impl<'a, const VAL: char> Parse<RustCursor<'a>, ()> for FJointPunct<VAL> {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        let cur = input.cursor;

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => {
                *input = c.into();
                Ok(BlackHoleFinalizer(Self(v.span())))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: char> Peek<RustCursor<'a>> for FJointPunct<VAL> {
    fn peek(input: &RustCursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Joint => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FJointPunct<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: char> PeekError<RustCursor<'a>> for FJointPunct<VAL> {
    fn error(input: &RustCursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[cfg(feature = "printing")]
impl<const VAL: char> quote::ToTokens for FJointPunct<VAL> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new(VAL, Spacing::Joint))
    }
}
impl<const VAL: char> Default for FJointPunct<VAL> {
    fn default() -> Self {
        Self(Span::call_site())
    }
}

#[derive(Clone, Debug)]
pub struct FAlonePunct<const VAL: char>(pub Span);
impl<'a, const VAL: char> Parse<RustCursor<'a>, ()> for FAlonePunct<VAL> {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        let cur = input.cursor;

        match cur.punct() {
            Some((v, c)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => {
                *input = c.into();
                Ok(BlackHoleFinalizer(Self(v.span())))
            }
            _ => Err(Error::new(cur.span(), format!("Expected '{}'", VAL))),
        }
    }
}
impl<'a, const VAL: char> Peek<RustCursor<'a>> for FAlonePunct<VAL> {
    fn peek(input: &RustCursor<'a>) -> Option<usize> {
        match input.punct() {
            Some((v, _)) if v.as_char() == VAL && v.spacing() == Spacing::Alone => Some(1),
            _ => None,
        }
    }
}
impl<const VAL: char> FixedPeek for FAlonePunct<VAL> {
    const SKIP: usize = 1;
}
impl<'a, const VAL: char> PeekError<RustCursor<'a>> for FAlonePunct<VAL> {
    fn error(input: &RustCursor<'a>) -> Error {
        Error::new(input.span(), format!("Expected '{}'", VAL))
    }
}

#[cfg(feature = "printing")]
impl<const VAL: char> quote::ToTokens for FAlonePunct<VAL> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Punct::new(VAL, Spacing::Alone))
    }
}
impl<const VAL: char> Default for FAlonePunct<VAL> {
    fn default() -> Self {
        Self(Span::call_site())
    }
}

macro_rules! grouped {
    ($ty:ident $del:ident $emsg:literal) => {
        #[derive(Debug, Clone)]
        pub struct $ty<T>(pub T, pub DelimSpan);
        impl<T: Default> Default for $ty<T> {
            fn default() -> Self {
                Self(
                    T::default(),
                    proc_macro2::Group::new(proc_macro2::Delimiter::$del, TokenStream::new())
                        .delim_span(),
                )
            }
        }
        impl<'a, T: Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, ()> for $ty<T> {
            type Finalizer = BlackHoleFinalizer<Self>;

            fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
                let cur = input.cursor;

                match cur.group(proc_macro2::Delimiter::$del) {
                    Some((inner, span, after)) => {
                        let mut pb = ParseBuffer::from(inner);
                        let v = pb.parse()?;

                        let cur = pb.cursor;

                        if cur.eof() {
                            *input = after.into();
                            Ok(BlackHoleFinalizer(Self(v, span)))
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
        impl<'a, T: Peek<RustCursor<'a>>> Peek<RustCursor<'a>> for $ty<T> {
            fn peek(input: &RustCursor<'a>) -> Option<usize> {
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
        impl<'a, T: PeekError<RustCursor<'a>>> PeekError<RustCursor<'a>> for $ty<T> {
            fn error(cursor: &RustCursor<'a>) -> Error {
                match cursor.group(proc_macro2::Delimiter::$del) {
                    Some((inner, _, _)) => T::error(&inner),
                    None => Error::new(cursor.span(), concat!("Expected '", $emsg, "'")),
                }
            }
        }
        #[cfg(feature = "printing")]
        impl<T: quote::ToTokens> quote::ToTokens for $ty<T> {
            fn into_token_stream(self) -> TokenStream {
                let mut ts = TokenStream::new();

                ts.append(proc_macro2::Group::new(
                    proc_macro2::Delimiter::$del,
                    self.0.into_token_stream(),
                ));

                ts
            }

            fn to_tokens(&self, tokens: &mut TokenStream) {
                tokens.append(proc_macro2::Group::new(
                    proc_macro2::Delimiter::$del,
                    self.0.to_token_stream(),
                ))
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

    insta_match_test!(peek parse : it_fails_on_empty_paren, Paren<P<()>> : );
    insta_match_test!(peek parse print : it_matches_simple_grouped_paren, Paren<P<()>> : ());
    insta_match_test!(peek parse print : it_matches_contentful_grouped_paren, Paren<Ident> : (hello));

    insta_match_test!(peek parse : it_fails_on_empty_brace, Brace<P<()>> : );
    insta_match_test!(peek parse print : it_matches_simple_grouped_brace, Brace<P<()>> : {});
    insta_match_test!(peek parse print : it_matches_contentful_grouped_brace, Brace<Ident> : {hello});

    insta_match_test!(peek parse : it_fails_on_empty_bracket, Bracket<P<()>> : );
    insta_match_test!(peek parse print : it_matches_simple_grouped_bracket, Bracket<P<()>> : [  ]);
    insta_match_test!(peek parse print : it_matches_contentful_grouped_bracket, Bracket<Ident> : [ hello ]);
}

#[cfg(test)]
mod tests_id {
    use crate::*;

    insta_match_test!(peek parse : it_fails_on_incorrect, FIdent<"id"> : ident);
    insta_match_test!(peek parse print : it_matches_id, Ident: id);
    insta_match_test!(peek parse print : it_matches_fixed, FIdent<"id"> : id);
}

#[cfg(test)]
mod tests_punct {
    use crate::*;

    insta_match_test!(peek parse print : it_matches_only, Punct : < );
    insta_match_test!(peek parse print : it_matches_fixed, FPunct<'<'> : < );
    insta_match_test!(peek parse print : it_matches_dollar, FPunct<'$'> : $ );

    // insta_match_test!(peek parse : it_matches_joint, (FJointPunct<'\''>, Ident) : 'hello );
    insta_match_test!(peek parse print : it_matches_both, P<(FJointPunct<'<'>, FAlonePunct<'='>)> : <= );

    insta_match_test!(peek parse print : it_matches_dollar_crate, P<(FPunct<'$'>, FIdent<"crate">)> : $crate);
}
