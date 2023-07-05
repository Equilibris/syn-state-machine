use crate::*;

materialize! {
    pub struct ImplTraitType <Attr, Ty> {
        <- KwImpl;
        bounds <- TypeParamBounds<Attr, Ty>
    }
}

materialize! {
    pub struct ImplTraitTypeOneBound <Attr, Ty> {
        <- KwImpl;
        bounds <- TraitBound<Attr, Ty>
    }
}
