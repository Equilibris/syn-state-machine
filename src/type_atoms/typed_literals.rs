use crate::*;

impl<'a> Parse<Cursor<'a>> for bool {
    fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
        Ok(input.ident_matching(|id| {
            if id == "true" || id == "false" {
                Ok(())
            } else {
                Err(Error::new(id.span(), "Expected bool literal"))
            }
        })? == "true")
    }
}
impl<'a> Peek<Cursor<'a>> for bool {
    fn peek(input: &Cursor<'a>) -> Option<usize> {
        todo!()
    }
}

macro_rules! typed_lit {
    ($err:literal $ty:ty) => {
        impl<'a> Parse<Cursor<'a>> for $ty {
            fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
                let cursor = input.cursor;
                match cursor.literal() {
                    Some((lit, cursor)) => {
                        let v = Self::try_from(lit).map_err(|_| Error::new(cursor.span(), $err))?;

                        *input = cursor.into();

                        Ok(v)
                    }
                    None => Err(Error::new(cursor.span(), $err)),
                }
            }
        }

        impl<'a> Peek<Cursor<'a>> for $ty {
            fn peek(input: &Cursor<'a>) -> Option<usize> {
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
        impl<'a> PeekError<Cursor<'a>> for $ty {
            fn error(input: &Cursor<'a>) -> Error {
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

pub type NegativeIntegerLit = (FPunct<'-'>, IntegerLit);
pub type NegativeFloatLit = (FPunct<'-'>, FloatLit);

pub type SignedIntegerLit = (Option<FPunct<'-'>>, IntegerLit);
pub type SignedFloatLit = (Option<FPunct<'-'>>, FloatLit);

typed_lit!("Expected string literal" StringLit);
typed_lit!("Expected int literal" IntegerLit);
typed_lit!("Expected char literal" CharLit);
typed_lit!("Expected byte literal" ByteLit);
typed_lit!("Expected float literal" FloatLit);
typed_lit!("Expected bytestring literal" ByteStringLit);

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(it_matches_simple_string_lit, StringLit : "Hello World");
    insta_match_test!(it_matches_simple_abi, StringLit : "C");
}
