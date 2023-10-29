use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum Statement<Attr, Ty, Expr, BExpr, WoExpr, WExpr, Pat, PatNT> {
        Semi(<- Semi),
        Item(item <- Item<Attr, Ty, Expr, Pat>),
        LetStatement(stmt <- LetStatement<Attr, Ty, Expr, BExpr, PatNT>),
        ExpressionStatement(stmt <- ExpressionStatement<WoExpr, WExpr>)
    }
}
to_tokens! {
    impl ToTokens for enum Statement<Attr, Ty, Expr, BExpr, WoExpr, WExpr, Pat, PatNT> {
        Semi(<- Semi),
        Item(item <- Item<Attr, Ty, Expr, Pat>),
        LetStatement(stmt <- LetStatement<Attr, Ty, Expr, BExpr, PatNT>),
        ExpressionStatement(stmt <- ExpressionStatement<WoExpr, WExpr>)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    #[derive(Debug)]
    pub struct LetStatement<Attr, Ty, Expr, BExpr, PatNT> {
        attrs <- Rep<OuterAttribute<Attr>>;
        <- KwLet;
        pat <- PatNT;
        ty <- Option<Ty> : Option<(Colon, _)> { ty.map(|v|v.1) };
        eq <- Option<(Expr, Option<BExpr>)> : Option<(Eq, _, Option<(KwElse, _)>)> { eq.map(|v| (v.1, v.2.map(|v|v.1))) };
        <- Semi
    }
}
to_tokens! {
    impl ToTokens for struct LetStatement<Attr, Ty, Expr, BExpr, PatNT> {
        attrs <- Rep<OuterAttribute<Attr>>;
        <- KwLet;
        pat <- PatNT;
        ty <- tokens into {
            if let Some(ty) = ty {
                tokens.extend(Colon::default().into_token_stream());
                tokens.extend(ty.into_token_stream());
            }
        } to {
            if let Some(ty) = ty {
                tokens.extend(Colon::default().into_token_stream());
                ty.to_tokens(tokens);
            }
        };
        eq <- tokens into {
            if let Some((expr, else_block)) = eq {
                tokens.extend(Eq::default().into_token_stream());
                tokens.extend(expr.into_token_stream())
                if let Some(eb) = else_block {
                    tokens.extend(KwElse::default().into_token_stream());
                    tokens.extend(eb.into_token_stream())
                }
            }
        } to {
            if let Some((expr, else_block)) = eq {
                tokens.extend(Eq::default().into_token_stream());
                expr.to_tokens(tokens);
                if let Some(eb) = else_block {
                    tokens.extend(KwElse::default().into_token_stream());
                    eb.to_tokens(tokens)
                }
            }
        };
        <- Semi
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum ExpressionStatement<WoExpr, WExpr> {
        WoExpr(v <- WoExpr; <- Semi),
        WExpr(v <- WExpr; trailing peek <- Semi)
    }
}
to_tokens! {
    impl ToTokens for enum ExpressionStatement<Wo, WExpr> {
        WoExpr(v <- WoExpr; <- Semi),
        WExpr(v <- WExpr; trailing peek <- Semi)
    }
}
