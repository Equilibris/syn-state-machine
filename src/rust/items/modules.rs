use proc_macro2::Ident;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

use crate::{rust::attributes::InnerAttribute, *};

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Module<InnerAttr, Item> [
        r#unsafe peek <- KwUnsafe;
        <- KwMod;
        id <- Ident : Identifier { id.0 };
    ] {
        Sourced(<- Semi;),
        Inline(
            content <- P2<Rep<InnerAttribute<InnerAttr>>, Rep<Item>> : Bracket<_> { content.0 };
        )
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum Module<InnerAttr, Item> [
        r#unsafe peek <- KwUnsafe;
        <- KwMod;
        id <- Ident
    ] {
        Sourced(<- Semi;),
        Inline(
            content <- tokens into {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        content.into_token_stream()
                    )
                )
            } to {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        content.into_token_stream()
                    )
                )
            }
        )
    }
}
