use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct FieldExpression<Expr> {
        expr <- Expr;
        <- Dot;
        id <- Ident : Identifier
    }
}
to_tokens! {
    impl ToTokens for struct FieldExpression<Expr> {
        expr <- Expr;
        <- Dot;
        id <- Ident
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct FieldExpressionR{
        <- Dot;
        id <- Ident : Identifier
    }
}
impl<Expr> Finalizer<FieldExpression<Expr>, Expr> for FieldExpressionR {
    fn finalize(
        self,
        expr: Expr,
    ) -> std::ops::ControlFlow<FieldExpression<Expr>, FieldExpression<Expr>> {
        std::ops::ControlFlow::Break(FieldExpression { expr, id: self.id })
    }
}

impl<'a, E: Expr> Parse<RustCursor<'a>, E> for FieldExpression<E> {
    type Finalizer = FieldExpressionR;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}
