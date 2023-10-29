use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct LoopLabel {
        lt <- LifetimeOrLabel;
        <- Colon
    }
}
to_tokens! {
    impl ToTokens for struct LoopLabel {
        lt <- LifetimeOrLabel;
        <- Colon
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum LoopExpression<Attr, BExpr, WoExpr, Scrutinee, Pat, Stmt> [ lt <- Option<LoopLabel> ] {
        InfiniteLoopExpression(v <- InfiniteLoopExpression<BExpr>),
        PredicateLoopExpression(v <- PredicateLoopExpression<BExpr, Scrutinee>),
        PredicatePatternLoopExpression(v <- PredicatePatternLoopExpression<BExpr, Scrutinee, Pat>),
        IteratorLoopExpression(v <- IteratorLoopExpression<BExpr, Scrutinee, Pat>),
        LabelBlockExpression(v <- LabelBlockExpression<Attr, WoExpr, Stmt>)
    }
}
to_tokens! {
    impl ToTokens for enum LoopExpression<Attr, BExpr, WoExpr, Scrutinee, Pat, Stmt> [ lt <- Option<LoopLabel> ] {
        InfiniteLoopExpression(v <- InfiniteLoopExpression<BExpr>),
        PredicateLoopExpression(v <- PredicateLoopExpression),
        PredicatePatternLoopExpression(v <- PredicatePatternLoopExpression),
        IteratorLoopExpression(v <- IteratorLoopExpression),
        LabelBlockExpression(v <- LabelBlockExpression)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct InfiniteLoopExpression<BExpr> {
        <- KwLoop;
        expr <- BExpr
    }
}
to_tokens! {
    impl ToTokens for struct InfiniteLoopExpression<BExpr> {
        <- KwLoop;
        expr <- BExpr
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct PredicateLoopExpression<BExpr, Scrutinee> {
        <- KwWhile;
        expr <- Scrutinee;
        block <- BExpr;
    }
}
to_tokens! {
    impl ToTokens for struct PredicateLoopExpression<BExpr, Scrutinee> {
        <- KwWhile;
        expr <- Scrutinee;
        block <- BExpr;
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct PredicatePatternLoopExpression<BExpr, Scrutinee, Pat> {
        <- KwWhile;
        <- KwLet;
        pat <- Pat;
        <- Eq;
        scrutinee <- Scrutinee;
        block <- BExpr
    }
}
to_tokens! {
    impl ToTokens for struct PredicatePatternLoopExpression<BExpr, Scrutinee, Pat> {
        <- KwWhile;
        <- KwLet;
        pat <- Pat;
        <- Eq;
        scrutinee <- Scrutinee;
        block <- BExpr
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct IteratorLoopExpression<BExpr, Scrutinee, Pat> {
        <- KwFor;
        pat <- Pat;
        <- KwIn;
        expr <- Scrutinee;
        block <- BExpr
    }
}
to_tokens! {
    impl ToTokens for struct IteratorLoopExpression<BExpr, Scrutinee, Pat> {
        <- KwFor;
        pat <- Pat;
        <- KwIn;
        block <- BExpr
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct BreakExpression<Expr> {
        <- KwBreak;
        lt <- Option<LifetimeOrLabel>;
        expr <- Expr
    }
}
to_tokens! {
    impl ToTokens for struct BreakExpression<Expr> {
        <- KwBreak;
        lt <- tokens into {
            if let Some(lt) = lt {
                tokens.extend(lt.into_token_stream())
            }
        } to {
            if let Some(lt) = lt {
                lt.to_tokens(tokens)
            }
        };
        expr <- Expr
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct ContinueExpression{
        <- KwContinue;
        lt <- Option<LifetimeOrLabel>;
    }
}
to_tokens! {
    impl ToTokens for struct ContinueExpression{
        <- KwBreak;
        lt <- tokens into {
            if let Some(lt) = lt {
                tokens.extend(lt.into_token_stream())
            }
        } to {
            if let Some(lt) = lt {
                lt.to_tokens(tokens)
            }
        };
    }
}

pub type LabelBlockExpression<Attr, WoExpr, Stmt> = BlockExpression<Attr, WoExpr, Stmt>;
