use crate::*;

materialize! {
    pub struct StaticItem <Ty, Expr> {
        <- KwStatic;
        r#mut peek <- KwMut;
        id <- Identifier;
        <- Colon;
        ty <- Ty;
        eq <- Option<Expr> : Option<(Eq, _)> { eq.map(|v|v.1) };
        <- Semi
    }
}
