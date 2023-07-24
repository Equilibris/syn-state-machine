use crate::*;

// TODO
materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ArrayType <Ty, Expr> {
        inner <- (Ty, Expr) : Bracket<(Ty, Semi, Expr)> { (inner.0.0, inner.0.2) }
    }
}
