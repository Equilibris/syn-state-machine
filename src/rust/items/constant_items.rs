use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct ConstantItem <Ty, Expr> {
        <- KwConst;
        id <- Ident : IdentifierOrUnder;
        <- Colon;
        ty <- Ty;
        eq <- Option<Expr> : Option<(Eq, _)> { eq.map(|v|v.1) };
        <- Semi
    }
}
