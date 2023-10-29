use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum ClosureExpression<Attr, Ty, TyNB, PatNT, Expr, BExpr> [
        mv peek <- KwMove;
        <- Or;
        params <- ClosureParameters<Attr, Ty, PatNT>;
    ] {
        Inferred(expr <- Expr),
        Typed(<- RArrow; ty <- TyNB; expr <- BExpr)
    }
}
to_tokens! {
    impl ToTokens for enum ClosureExpression<Attr, Ty, TyNB, PatNT, Expr, BExpr> [
        mv peek <- KwMove;
        <- Or;
        params <- ClosureParameters<Attr, Ty, PatNT>;
    ] {
        Inferred(expr <- Expr),
        Typed(<- RArrow; ty <- TyNB; expr <- BExpr)
    }
}

pub type ClosureParameters<Attr, Ty, PatNT> = InterlaceTrail<ClosureParam<Attr, Ty, PatNT>, Comma>;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct ClosureParam<Attr, Ty, PatNT> {
        attrs <- Rep<OuterAttribute<Attr>>;
        pat <- PatNT;
        ty <- Option<Ty> : Option<(Colon, _)> { ty.map(|v| v.1) }
    }
}
to_tokens! {
    impl ToTokens for struct ClosureParam<Attr, Ty, PatNT> {
        attrs <- Rep<OuterAttribute<Attr>>;
        pat <- PatNT;
        ty <- tokens into {
            if let Some(ty) = ty {
                tokens.extend(Colon::default().into_token_stream());
                tokens.extend(ty.into_token_stream())
            }
        } to {
            if let Some(ty) = ty {
                tokens.extend(Colon::default().into_token_stream());
                ty.to_tokens(tokens)
            }
        }
    }
}
