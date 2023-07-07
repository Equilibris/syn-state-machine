use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct ImplTraitType <Attr, Ty> {
        <- KwImpl;
        bounds <- TypeParamBounds<Attr, Ty>
    }
}

materialize! {
    #[derive(Debug)]
    pub struct ImplTraitTypeOneBound <Attr, Ty> {
        <- KwImpl;
        bounds <- TraitBound<Attr, Ty>
    }
}
