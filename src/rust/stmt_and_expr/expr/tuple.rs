use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct TupleExpression<Expr> {
        elements <- TupleElements<Expr> : Option<_> { elements.unwrap_or_default() }
    }
}
to_tokens! {
    impl ToTokens for struct TupleExpression<Expr> {
        elements <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    elements.into_token_stream()
                )
            );
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    elements.to_token_stream()
                )
            );
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct TupleElements<Expr> {
        elements <-
            InterlaceTrail<Expr, Comma> :
            (MinLength<Rep<(_, PeekAsParse<Comma>)>>, Option<_>) {
            let vs = elements.0.0.0
                .into_iter()
                .map(|v| v.0)
                .chain(elements.1)
                .collect();

            InterlaceTrail::new(vs)
        }
    }
}
impl<Expr> std::default::Default for TupleElements<Expr> {
    fn default() -> Self {
        Self {
            elements: Default::default(),
        }
    }
}
to_tokens! {
    impl ToTokens for struct TupleElements<Expr> {
        elements <- InterlaceTrail<Expr, Comma>
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct TupleIndexingExpression<Expr> {
        expr <- Expr;
        <- Dot;
        idx <- TupleIndex
    }
}
to_tokens! {
    impl ToTokens for struct TupleIndexingExpression<Expr> {
        expr <- Expr;
        <- Dot;
        idx <- tokens into {
            tokens.append(Literal::from(idx))
        } to {
            tokens.append(Literal::from(idx.clone()))
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct TupleIndexingExpressionR {
        <- Dot;
        idx <- TupleIndex
    }
}

impl<Expr> Finalizer<TupleIndexingExpression<Expr>, Expr> for TupleIndexingExpressionR {
    fn finalize(
        self,
        expr: Expr,
    ) -> std::ops::ControlFlow<TupleIndexingExpression<Expr>, TupleIndexingExpression<Expr>> {
        std::ops::ControlFlow::Break(TupleIndexingExpression {
            expr,
            idx: self.idx,
        })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr> Parse<RustCursor<'a>, E> for TupleIndexingExpression<E> {
    type Finalizer = TupleIndexingExpressionR;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}
