use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum StructExpression<Ty, Expr> {
        StructExprStruct(v <- StructExprStruct<Ty, Expr>),
        StructExprTuple(v <- StructExprTuple<Ty, Expr>),
        StructExprUnit(v <- StructExprUnit<Ty>),
    }
}
to_tokens! {
    impl ToTokens for enum StructExpression<Ty, Expr> {
        StructExprStruct(v <- StructExprStruct<Ty, Expr>),
        StructExprTuple(v <- StructExprTuple<Ty, Expr>),
        StructExprUnit(v <- StructExprUnit<Ty>),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct StructExprStruct<Ty, Expr> {
        path <- PathInExpression<Ty>;

        fields <- StructExprFields<Expr> : Brace<Sum2<_, _>> {
            match fields.0 {
                Sum2::V0(v) => v,
                Sum2::V1(base) => StructExprFields { fields: Default::default(), base: Some(base) }
            }
        }
    }
}
to_tokens! {
    impl ToTokens for struct StructExprStruct<Ty, Expr> {
        path <- PathInExpression<Ty>;

        fields <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    fields.into_token_stream()
                )
            );
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    fields.to_token_stream()
                )
            );
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct StructExprFields<Expr> {
        fields <- Interlace<Expr, Comma> : MinLength<_> { fields.0 };
        base <- Option<StructBase<Expr>> : Sum3<(Comma, _, Option<Comma>), Comma, ()> {
            match base {
                Sum3::V0((_, v,_)) => Some(v),
                _ => None
            }
        }
    }
}
to_tokens! {
    impl ToTokens for struct StructExprFields<Expr> {
        fields <- Interlace<Expr, Comma>;
        base <- tokens into {
            if let Some(base) = base {
                tokens.extend(Comma::default().into_token_stream());
                tokens.extend(base.into_token_stream())
            }
        } to {
            if let Some(base) = base {
                tokens.extend(Comma::default().into_token_stream());
                base.to_tokens(tokens)
            }
        }
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum StructExprField<Expr> {
        Shorthand(id <- Ident : Identifier),
        ExplicitTuple(idx <- TupleIndex; <- Colon; expr <- Expr),
        ExplicitStruct(idx <- Ident : Identifier; <- Colon; expr <- Expr),
    }
}
to_tokens! {
    impl ToTokens for enum StructExprField<Expr> {
        Shorthand(id <- Ident),
        ExplicitTuple(
            idx <- tokens into {
                tokens.append(Literal::from(idx.clone()))
            } to {
                tokens.append(Literal::from(idx.clone()))
            };
            <- Colon;
            expr <- Expr
        ),
        ExplicitStruct(idx <- Ident; <- Colon; expr <- Expr),
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct StructBase<Expr> {
        <- DotDot;
        expr <- Expr
    }
}
to_tokens! {
    impl ToTokens for struct StructBase<Expr> {
        <- DotDot;
        expr <- Expr
    }
}

materialize! {
    on <'a> [RustCursor<'a>]
    pub struct StructExprTuple<Ty, Expr> {
        path <- PathInExpression<Ty>;
        elements  <- InterlaceTrail<Expr, Comma> : Paren<_> { elements.0 }
    }
}
to_tokens! {
    impl ToTokens for struct StructExprTuple<Ty, Expr> {
        path <- PathInExpression<Ty>;
        elements <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    elements.into_token_stream()
                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    elements.to_token_stream()
                )
            )
        }
    }
}

pub type StructExprUnit<Ty> = PathInExpression<Ty>;
