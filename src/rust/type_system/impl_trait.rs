use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ImplTraitType <Attr, Ty> {
        <- KwImpl;
        bounds <- TypeParamBounds<Attr, Ty>
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ImplTraitType<Attr, Ty> {
        <- KwImpl;
        bounds <- TypeParamBounds<Attr, Ty>
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ImplTraitTypeOneBound <Attr, Ty> {
        <- KwImpl;
        bounds <- TraitBound<Attr, Ty>
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ImplTraitTypeOneBound<Attr, Ty> {
        <- KwImpl;
        bounds <- TraitBound<Attr, Ty>
    }
}
