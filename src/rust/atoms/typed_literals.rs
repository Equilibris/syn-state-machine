use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

impl<'a> Parse<RustCursor<'a>, ()> for bool {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        Ok(BlackHoleFinalizer(
            input.ident_matching(|id: &Ident| {
                if id == "true" || id == "false" {
                    Ok(())
                } else {
                    Err(Error::new(id.span(), "Expected bool literal"))
                }
            })? == "true",
        ))
    }
}
impl<'a> Peek<RustCursor<'a>> for bool {
    fn peek(input: &RustCursor<'a>) -> Option<usize> {
        let Some((id, _)) = input.ident() else {
            return None;
        };
        if id == "true" || id == "false" {
            Some(1)
        } else {
            None
        }
    }
}

macro_rules! typed_lit {
    ($err:literal $ty:ty) => {
        impl<'a> Parse<RustCursor<'a>, ()> for $ty {
            type Finalizer = BlackHoleFinalizer<Self>;

            fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
                let cursor = input.cursor;
                match cursor.literal() {
                    Some((lit, cursor)) => {
                        let v = Self::try_from(lit).map_err(|_| Error::new(cursor.span(), $err))?;

                        *input = cursor.into();

                        Ok(BlackHoleFinalizer(v))
                    }
                    None => Err(Error::new(cursor.span(), $err)),
                }
            }
        }

        impl<'a> Peek<RustCursor<'a>> for $ty {
            fn peek(input: &RustCursor<'a>) -> Option<usize> {
                match input.literal() {
                    Some((lit, _)) => {
                        Self::try_from(lit).ok()?;
                        Some(1)
                    }
                    None => None,
                }
            }
        }
        impl FixedPeek for $ty {
            const SKIP: usize = 1;
        }
        impl<'a> PeekError<RustCursor<'a>> for $ty {
            fn error(input: &RustCursor<'a>) -> Error {
                Error::new(input.span(), $err)
            }
        }
    };
}

pub type ByteStringLit = litrs::ByteStringLit<String>;
pub type FloatLit = litrs::FloatLit<String>;
pub type IntegerLit = litrs::IntegerLit<String>;
pub type StringLit = litrs::StringLit<String>;
pub type CharLit = litrs::CharLit<String>;
pub type ByteLit = litrs::ByteLit<String>;

macro_rules! signed_lit {
    ($name:ident $neg_name:ident $ty_name:ident) => {
        materialize! {
            on <'a> [crate::RustCursor<'a>]
                pub struct $neg_name {
                    <- FPunct<'-'>;
                    lit <- $ty_name
                }
        }
        to_tokens! {
            impl ToTokens for struct $neg_name {
                <- FPunct<'-'>;
                lit <- tokens into {
                    tokens.append(Literal::from(lit))
                } to {
                    tokens.append(Literal::from(lit.clone()))
                }
            }
        }

        materialize! {
            on <'a> [crate::RustCursor<'a>]
                pub struct $name {
                    neg peek <- FPunct<'-'>;
                    lit <- $ty_name
                }
        }
        to_tokens! {
            impl ToTokens for struct $name {
                neg peek <- FPunct<'-'>;
                lit <- tokens into {
                    tokens.append(Literal::from(lit))
                } to {
                    tokens.append(Literal::from(lit.clone()))
                }
            }
        }
    };
}
signed_lit!(SignedIntegerLit NegativeIntegerLit IntegerLit);
signed_lit!(SignedFloatLit NegativeFloatLit FloatLit);

typed_lit!("Expected string literal" StringLit);
typed_lit!("Expected int literal" IntegerLit);
typed_lit!("Expected char literal" CharLit);
typed_lit!("Expected byte literal" ByteLit);
typed_lit!("Expected float literal" FloatLit);
typed_lit!("Expected bytestring literal" ByteStringLit);

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(parse peek : it_matches_simple_string_lit, StringLit : "Hello World");
    insta_match_test!(parse peek : it_matches_simple_abi, StringLit : "C");
}
