use crate::*;

materialize! {
    pub struct TraitObjectType <Attr, Ty> {
        <- KwDyn;
        bounds <- TypeParamBounds<Attr, Ty>
    }
}

materialize! {
    pub struct TraitObjectTypeOneBound <Attr, Ty> {
        <- KwDyn;
        bounds <- TraitBound<Attr, Ty>
    }
}
