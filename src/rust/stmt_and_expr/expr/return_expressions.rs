pub use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct ReturnExpression<Expr> {
        <- KwIf;
        expr <- Option<Expr>
    }
}
to_tokens! {
    impl ToTokens for struct ReturnExpression<Expr> {
        <- KwIf;
        expr <- tokens into {
            if let Some(expr) = expr {
                tokens.extend(expr.into_token_stream())
            }
        } to {
            if let Some(ref expr) = expr {
                expr.to_tokens(tokens)
            }
        }
    }
}
