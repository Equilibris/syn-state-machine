use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TraitObjectType <Attr, Ty> {
        <- KwDyn;
        bounds <- TypeParamBounds<Attr, Ty>
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TraitObjectTypeOneBound <Attr, Ty> {
        <- KwDyn;
        bounds <- TraitBound<Attr, Ty>
    }
}
