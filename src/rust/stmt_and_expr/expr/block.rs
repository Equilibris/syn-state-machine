use crate::*;

pub type BlockExpression<Attr, WoExpr, Stmt> =
    Brace<P<(Rep<InnerAttribute<Attr>>, Statements<WoExpr, Stmt>)>>;

pub type Statements<WoExpr, Stmt> = P<(Rep<Stmt>, Option<WoExpr>)>;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct AsyncBlockExpression<Attr, WoExpr, Stmt> {
        <- KwAsync;
        mv peek <- KwMove;
        block <- BlockExpression<Attr, WoExpr, Stmt>
    }
}
to_tokens! {
    impl ToTokens for struct AsyncBlockExpression<Attr, WoExpr, Stmt> {
        <- KwAsync;
        mv peek <- KwMove;
        block <- BlockExpression<Attr, WoExpr, Stmt>
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct UnsafeBlock<Attr, WoExpr, Stmt> {
        <- KwUnsafe;
        block <- BlockExpression<Attr, WoExpr, Stmt>
    }
}
to_tokens! {
    impl ToTokens for struct UnsafeBlock<Attr, WoExpr, Stmt> {
        <- KwUnsafe;
        block <- BlockExpression<Attr, WoExpr, Stmt>
    }
}
