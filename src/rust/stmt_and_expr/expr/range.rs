use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum RangeExpression<Expr> {
        RangeExpr(v <- RangeExpr<Expr>),
        RangeFromExpr(v <- RangeFromExpr<Expr>),
        RangeToExpr(v <- RangeToExpr<Expr>),
        RangeFullExpr(v <- RangeFullExpr),
        RangeInclusiveExpr(v <- RangeInclusiveExpr<Expr>),
        RangeToInclusiveExpr(v <- RangeToInclusiveExpr<Expr>),
    }
}
to_tokens! {
    impl ToTokens for enum RangeExpression<Expr> {
        RangeExpr(v <- RangeExpr<Expr>),
        RangeFromExpr(v <- RangeFromExpr<Expr>),
        RangeToExpr(v <- RangeToExpr<Expr>),
        RangeFullExpr(v <- RangeFullExpr),
        RangeInclusiveExpr(v <- RangeInclusiveExpr<Expr>),
        RangeToInclusiveExpr(v <- RangeToInclusiveExpr<Expr>),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeExpr<Expr> {
        lhs <- Expr;
        <- DotDot;
        rhs <- Expr;
    }
}
to_tokens! {
    impl ToTokens for struct RangeExpr<Expr> {
        lhs <- Expr;
        <- DotDot;
        rhs <- Expr;
    }
}
materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeExprR<Expr> {
        <- DotDot;
        rhs <- Expr;
    }
}

impl<E: Expr> Finalizer<RangeExpr<E>, E> for RangeExprR<E> {
    fn finalize(self, lhs: E) -> std::ops::ControlFlow<RangeExpr<E>, RangeExpr<E>> {
        std::ops::ControlFlow::Break(RangeExpr { lhs, rhs: self.rhs })
    }
}
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E> for RangeExpr<E> {
    type Finalizer = RangeExprR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeFromExpr<Expr> {
        lhs <- Expr;
        <- DotDot;
    }
}
to_tokens! {
    impl ToTokens for struct RangeFromExpr<Expr> {
        lhs <- Expr;
        <- DotDot;
    }
}
materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeFromExprR {
        <- DotDot;
    }
}
impl<E: Expr> Finalizer<RangeFromExpr<E>, E> for RangeFromExprR {
    fn finalize(self, lhs: E) -> std::ops::ControlFlow<RangeFromExpr<E>, RangeFromExpr<E>> {
        std::ops::ControlFlow::Break(RangeFromExpr { lhs })
    }
}
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E> for RangeFromExpr<E> {
    type Finalizer = RangeFromExprR;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeToExpr<Expr> {
        <- DotDot;
        lhs <- Expr;
    }
}
to_tokens! {
    impl ToTokens for struct RangeToExpr<Expr> {
        <- DotDot;
        lhs <- Expr;
    }
}

pub type RangeFullExpr = DotDot;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeInclusiveExpr<Expr> {
        lhs <- Expr;
        <- DotDotEq;
        rhs <- Expr;
    }
}
to_tokens! {
    impl ToTokens for struct RangeInclusiveExpr<Expr> {
        lhs <- Expr;
        <- DotDotEq;
        rhs <- Expr;
    }
}
materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeInclusiveExprR<Expr> {
        <- DotDotEq;
        rhs <- Expr;
    }
}
impl<E: Expr> Finalizer<RangeInclusiveExpr<E>, E> for RangeInclusiveExprR<E> {
    fn finalize(
        self,
        lhs: E,
    ) -> std::ops::ControlFlow<RangeInclusiveExpr<E>, RangeInclusiveExpr<E>> {
        std::ops::ControlFlow::Break(RangeInclusiveExpr { lhs, rhs: self.rhs })
    }
}

impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E> for RangeInclusiveExpr<E> {
    type Finalizer = RangeInclusiveExprR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct RangeToInclusiveExpr<Expr> {
        <- DotDotEq;
        lhs <- Expr;
    }
}
to_tokens! {
    impl ToTokens for struct RangeToInclusiveExpr<Expr> {
        <- DotDotEq;
        lhs <- Expr;
    }
}
