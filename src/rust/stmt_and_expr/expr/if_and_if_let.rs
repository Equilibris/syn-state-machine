use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct IfExpression<Expr, BExpr, Scrutinee, Pat> {
        <- KwIf;
        cond <- Expr;
        block <- BExpr;

        r#else <- Box<Option<Sum3<BExpr, Self, IfLetExpression<Expr, BExpr, Scrutinee, Pat>>>>
    }
}
to_tokens! {
    impl ToTokens for struct IfExpression<Expr, BExpr, Scrutinee, Pat> {
        <- KwIf;
        cond <- Expr;
        block <- BExpr;

        r#else <- tokens into {
            tokens.extend(match r#else.into_inner() {
                Some(Sum3::V0(v)) => v.into_token_stream(),
                Some(Sum3::V1(v)) => v.into_token_stream(),
                Some(Sum3::V2(v)) => v.into_token_stream(),
                None => ()
            })
        } to {
            match r#else.as_ref() {
                Some(Sum3::V0(v)) => v.to_tokens(tokens),
                Some(Sum3::V1(v)) => v.to_tokens(tokens),
                Some(Sum3::V2(v)) => v.to_tokens(tokens),
                None => ()
            }
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct IfLetExpression<Expr, BExpr, Scrutinee, Pat> {
        <- KwIf;
        <- KwLet;

        pat <- Pat;

        <- Eq;

        scrutinee <- Scrutinee;

        block <- BExpr;

        r#else <- Box<Option<Sum3<BExpr, Self, IfExpression<Expr, BExpr, Scrutinee, Pat>>>>
    }
}
to_tokens! {
    impl ToTokens for struct IfLetExpression<Expr, BExpr, Scrutinee, Pat> {
        <- KwIf;
        <- KwLet;

        pat <- Pat;

        <- Eq;

        scrutinee <- Scrutinee;

        block <- BExpr;

        r#else <- tokens into {
            tokens.extend(match r#else.into_inner() {
                Some(Sum3::V0(v)) => v.into_token_stream(),
                Some(Sum3::V1(v)) => v.into_token_stream(),
                Some(Sum3::V2(v)) => v.into_token_stream(),
                None => ()
            })
        } to {
            match r#else.as_ref() {
                Some(Sum3::V0(v)) => v.to_tokens(tokens),
                Some(Sum3::V1(v)) => v.to_tokens(tokens),
                Some(Sum3::V2(v)) => v.to_tokens(tokens),
                None => ()
            }
        }
    }
}
