use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct MethodCallExpression<Ty, Expr> {
        expr <- Expr;
        <- Dot;
        seg <- PathExprSegment<Ty>;
        params <- CallParams<Expr> : Paren<_> { params.0 }
    }
}
to_tokens! {
    impl ToTokens for struct MethodCallExpression<Expr, Ty> {
        expr <- Expr;
        <- Dot;
        seg <- PathExprSegment<Ty>;
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
    pub struct MethodCallExpressionR<Ty, Expr> {
        <- Dot;
        seg <- PathExprSegment<Ty>;
        params <- CallParams<Expr> : Paren<_> { params.0 }
    }
}
impl<Ty, Expr> Finalizer<MethodCallExpression<Ty, Expr>, Expr> for MethodCallExpressionR<Ty, Expr> {
    fn finalize(
        self,
        expr: Expr,
    ) -> std::ops::ControlFlow<MethodCallExpression<Ty, Expr>, MethodCallExpression<Ty, Expr>> {
        std::ops::ControlFlow::Break(MethodCallExpression {
            expr,
            seg: self.seg,
            params: self.params,
        })
    }
}

impl<'a, Ty: Parse<RustCursor<'a>, ()>, E: Expr + Parse<RustCursor<'a>, ()>>
    Parse<RustCursor<'a>, E> for MethodCallExpression<Ty, E>
{
    type Finalizer = MethodCallExpressionR<Ty, E>;

    fn parse(
        input: &mut ParseBuffer<RustCursor<'a>>,
    ) -> Result<Self::Finalizer, <RustCursor<'a> as ParserCursor>::Error> {
        Ok(input.parse()?)
    }
}
