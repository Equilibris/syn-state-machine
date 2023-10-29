use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

pub type ArrayExpression<Expr> = Bracket<ArrayElements<Expr>>;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum ArrayElements<Expr> {
        Repeat(operand <- Expr; <- Semi; len <- Expr),
        Uniform(els <- InterlaceTrail<Expr, Comma>)
    }
}

to_tokens! {
    impl ToTokens for enum ArrayElements<Expr> {
        Repeat(operand <- Expr; <- Semi; len <- Expr),
        Uniform(els <- InterlaceTrail<Expr, Comma>)
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct IndexExpression<Expr> {
        obj <- Expr;
        idx <- Expr : Bracket<_> { idx.0 }
    }
}
to_tokens! {
    impl ToTokens for struct IndexExpression<Expr> {
        obj <- Expr;
        idx <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Bracket,
                    idx.into_token_stream()
                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Bracket,
                    idx.to_token_stream()
                )
            )
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct IndexExpressionR<Expr> {
        idx <- Expr : Bracket<_> { idx.0 }
    }
}

impl<Expr> Finalizer<IndexExpression<Expr>, Expr> for IndexExpressionR<Expr> {
    fn finalize(
        self,
        obj: Expr,
    ) -> std::ops::ControlFlow<IndexExpression<Expr>, IndexExpression<Expr>> {
        std::ops::ControlFlow::Break(IndexExpression { obj, idx: self.idx })
    }
}

// TODO: Specialize this to the expression type
impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E> for IndexExpression<E> {
    type Finalizer = IndexExpressionR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}
