use proc_macro2::Ident;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

use crate::{rust::attributes::InnerAttribute, *};

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Module<Attr, Item> [
        r#unsafe peek <- KwUnsafe;
        <- KwMod;
        id <- Ident : Identifier { id.0 };
    ] {
        Sourced(<- Semi;),
        Inline(
            content <- P<(Rep<InnerAttribute<Attr>>, Rep<Item>)> : Brace<_> { content.0 };
        )
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum Module<Attr, Item> [
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

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test! { parse print : it_matches_sourced, Module<P<Infallible>, P<Infallible>> : mod hello; }
    insta_match_test! { parse print : it_matches_inline,  Module<P<Infallible>, Ident> : mod hello { hello world } }
}
