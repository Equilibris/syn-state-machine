use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    pub struct Union <Attr, Ty> {
        <- KwUnion;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        fields <- StructFields<Attr, Ty> : Brace<_> { fields.0 }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct Union<Attr, Ty> {
        <- KwUnion;
        id <- Ident;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        fields <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    fields.into_token_stream()
                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    fields.to_token_stream()
                )
            )
        }
    }
}
