use proc_macro2::Ident;

use crate::{rust::attributes::InnerAttribute, *};

materialize! {
    #[derive(Debug)]
    pub enum Module<InnerAttr, Item> [
        r#unsafe peek <- KwUnsafe;
        <- KwMod;
        id <- Ident : Identifier { id.0 };
    ] {
        Sourced(<- Semi;)
        Inline(
            content <- (Vec<InnerAttribute<InnerAttr>>, Vec<Item>) : Bracket<_> { content.0 };
        )
    }
}
