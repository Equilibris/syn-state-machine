use crate::*;

materialize! {
    pub struct ConstantItem <Ty, Expr> {
        <- KwConst;
        id <- IdentifierOrUnder;
        <- Colon;
        ty <- Ty;
        eq <- Option<Expr> : Option<(Eq, _)> { eq.map(|v|v.1) };
        <- Semi
    }
}
