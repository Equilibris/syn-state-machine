use proc_macro2::Ident;

use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct ExternCrate {
        <- KwExtern;
        <- KwCrate;
        crate_ref <- CrateRef;
        as_clause <- Option<AsClause>;
    }
}
materialize! {
    #[derive(Debug)]
    pub struct AsClause {
        <- KwAs;
        id <- Ident : IdentifierOrUnder;
    }
}

#[derive(Debug)]
pub struct CrateRef(pub Ident);
impl<'a> Parse<Cursor<'a>> for CrateRef {
    fn parse(input: &mut ParseBuffer<Cursor<'a>>) -> Result<Self> {
        Ok(Self(
            input
                .ident_matching(|id| {
                    if id == "self" {
                        Ok(())
                    } else {
                        get_error_from_ident(id)
                    }
                })?
                .clone(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(+it_matches_extern, ExternCrate : extern crate hi;);
    insta_match_test!(+it_matches_extern_as, ExternCrate : extern crate hi as _;);
}
