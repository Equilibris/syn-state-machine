use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct AwaitExpression <Expr> {
        expr <- Expr;
        <- Dot;
        <- KwAwait;
    }
}
to_tokens! {
    impl ToTokens for struct AwaitExpression <Expr> {
        expr <- Expr;
        <- Dot;
        <- KwAwait;
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct AwaitExpressionR { <- Dot; <- KwAwait; }
}

impl<E: Expr> Finalizer<AwaitExpression<E>, E> for AwaitExpressionR {
    fn finalize(self, expr: E) -> std::ops::ControlFlow<AwaitExpression<E>, AwaitExpression<E>> {
        std::ops::ControlFlow::Break(AwaitExpression { expr })
    }
}

impl<'a, E: Expr> Parse<RustCursor<'a>, E> for AwaitExpression<E> {
    type Finalizer = AwaitExpressionR;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}
