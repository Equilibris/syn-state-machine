use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct StaticItem <Ty, Expr> {
        <- KwStatic;
        r#mut peek <- KwMut;
        id <- Ident : Identifier;
        <- Colon;
        ty <- Ty;
        eq <- Option<Expr> : Option<(Eq, _)> { eq.map(|v|v.1) };
        <- Semi
    }
}
