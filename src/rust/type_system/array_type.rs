use crate::*;

// TODO
materialize! {
    pub struct ArrayType <Ty, Expr> {
        inner <- (Ty, Expr) : Bracket<(Ty, Semi, Expr)> { (inner.0.0, inner.0.2) }
    }
}
