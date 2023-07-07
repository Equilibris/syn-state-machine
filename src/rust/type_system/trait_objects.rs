use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct TraitObjectType <Attr, Ty> {
        <- KwDyn;
        bounds <- TypeParamBounds<Attr, Ty>
    }
}

materialize! {
    #[derive(Debug)]
    pub struct TraitObjectTypeOneBound <Attr, Ty> {
        <- KwDyn;
        bounds <- TraitBound<Attr, Ty>
    }
}
