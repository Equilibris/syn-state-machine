use proc_macro2::Ident;

use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ExternCrate {
        <- KwExtern;
        <- KwCrate;
        crate_ref <- CrateRef;
        as_clause <- Option<AsClause>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ExternCrate {
        <- KwExtern;
        <- KwCrate;
        crate_ref <- CrateRef;
        as_clause <- Option<AsClause>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct AsClause {
        <- KwAs;
        id <- Ident : IdentifierOrUnder;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct AsClause {
        <- KwAs;
        id <- Ident;
    }
}

#[derive(Debug)]
pub struct CrateRef {
    pub id: Ident,
}
impl<'a> Parse<RustCursor<'a>, ()> for CrateRef {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        Ok(BlackHoleFinalizer(Self {
            id: input
                .ident_matching(|id| {
                    if id == "self" {
                        Ok(())
                    } else {
                        get_error_from_ident(id)
                    }
                })?
                .clone(),
        }))
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct CrateRef {
        id <-
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(parse : it_matches_extern,    ExternCrate : extern crate hi;);
    insta_match_test!(parse : it_matches_extern_as, ExternCrate : extern crate hi as _;);
}
