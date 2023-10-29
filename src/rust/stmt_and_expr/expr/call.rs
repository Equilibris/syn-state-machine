use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct CallExpression<Expr> {
        expr <- Expr;
        params <- CallParams<Expr> : Paren<_> { params.0 }
    }
}
to_tokens! {
    impl ToTokens for struct CallExpression<Expr> {
        expr <- Expr;
        params <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    params.into_token_stream()
                )
            );
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    params.to_token_stream()
                )
            );
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct CallExpressionR<Expr> {
        params <- CallParams<Expr> : Paren<_> { params.0 }
    }
}

impl<Expr> Finalizer<CallExpression<Expr>, Expr> for CallExpressionR<Expr> {
    fn finalize(
        self,
        expr: Expr,
    ) -> std::ops::ControlFlow<CallExpression<Expr>, CallExpression<Expr>> {
        std::ops::ControlFlow::Break(CallExpression {
            expr,
            params: self.params,
        })
    }
}

impl<'a, E: Expr + Parse<RustCursor<'a>, ()>> Parse<RustCursor<'a>, E> for CallExpression<E> {
    type Finalizer = CallExpressionR<E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}

pub type CallParams<Expr> = InterlaceTrail<Expr, Comma>;
