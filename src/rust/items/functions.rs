use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct Function<Attr, Ty, Expr, Pat>{
        qualifiers <- FunctionQualifiers;
        <- KwFn;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        params <- Paren<FunctionParameters<Attr, Ty, Pat>>;
        ret <- Option<Ty> : Option<FunctionReturnType<_>> { ret.map(|v|v.ty) };
        where_clause <- Option<WhereClause<Attr,Ty>>;
        content <- Sum2<Semi, Expr>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct Function<Attr, Ty, Expr, Pat> {
        qualifiers <- FunctionQualifiers;
        <- KwFn;
        id <- Ident;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        params <- Paren<FunctionParameters<Attr, Ty, Pat>>;
        ret <- tokens into {
            if let Some(ret) = ret {
                tokens.extend(RArrow::default().into_token_stream());
                tokens.extend(ret.into_token_stream())
            }
        } to {
            if let Some(ret) = ret {
                tokens.extend(RArrow::default().into_token_stream());
                ret.to_tokens(tokens)
            }
        };
        where_clause <- Option<WhereClause<Attr,Ty>>;
        content <- Sum2<Semi, Expr>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct FunctionQualifiers {
        r#const peek <- KwConst;
        r#async peek <- KwAsync;
        r#unsafe peek <- KwUnsafe;
        extern_abi <- Option<Option<Abi>> : Option<(KwExtern, _)> { extern_abi.map(|v|v.1) };
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct FunctionQualifiers {
        r#const peek <- KwConst;
        r#async peek <- KwAsync;
        r#unsafe peek <- KwUnsafe;
        extern_abi <- tokens into {
            if let Some(extern_abi) = extern_abi {
                tokens.extend(KwExtern::default().into_token_stream());

                if let Some(extern_abi) = extern_abi {
                    tokens.append(Literal::from(extern_abi))
                }
            }
        } to {
            if let Some(extern_abi) = extern_abi {
                tokens.extend(KwExtern::default().into_token_stream());

                if let Some(extern_abi) = extern_abi {
                    tokens.append(Literal::from(extern_abi.clone()))
                }
            }
        }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum SelfParam<Attr, Ty> [attrs <- Rep<OuterAttribute<Attr>>] {
        Typed(v <- TypedSelf<Ty>),
        Shorthand(v <- ShorthandSelf)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum SelfParam<Attr, Ty> [attrs <- Rep<OuterAttribute<Attr>>] {
        Typed(v <- TypedSelf<Ty>),
        Shorthand(v <- ShorthandSelf)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ShorthandSelf {
        reference <- Option<Option<Lifetime>> : Option<(And, _)> { reference.map(|v|v.1) };
        <- KwLowerSelf
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ShorthandSelf {
        reference <- tokens into {
            if let Some(reference) = reference {
                tokens.extend(And::default().into_token_stream());

                if let Some(lt) = reference {
                    tokens.extend(lt.into_token_stream())
                }
            }
        } to {
            if let Some(reference) = reference {
                tokens.extend(And::default().into_token_stream());

                if let Some(lt) = reference {
                    lt.to_tokens(tokens)
                }
            }
        };
        <- KwLowerSelf
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TypedSelf<Ty> {
        mutable peek <- KwMut;
        <- KwLowerSelf;
        <- Colon;
        ty <- Ty;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TypedSelf<Ty> {
        mutable peek <- KwMut;
        <- KwLowerSelf;
        <- Colon;
        ty <- Ty
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum FunctionParam <Attr, Ty, Pat> [ attrs <- Rep<OuterAttribute<Attr>> ] {
        FunctionParamPattern(v <- FunctionParamPattern<Ty, Pat>),
        Ty(v <- Ty),
        Rest(v <- DotDotDot)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum FunctionParam<Attr, Ty, Pat> [ attrs <- Rep<OuterAttribute<Attr>> ] {
        FunctionParamPattern(v <- FunctionParamPattern<Ty, Pat>),
        Ty(v <- Ty),
        Rest(v <- DotDotDot)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum FunctionParamPattern<Ty, Pat> [ pat <- Pat; <- Colon ] {
        Ty(v <- Ty),
        Unknown(v <- DotDotDot)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum FunctionParamPattern<Ty, Pat> [ pat <- Pat; <- Colon ] {
        Ty(v <- Ty),
        Unknown(v <- DotDotDot)
    }
}

#[derive(Debug)]
pub struct FunctionParameters<Attr, Ty, Pat> {
    pub self_param: Option<SelfParam<Attr, Ty>>,
    pub params: InterlaceTrail<FunctionParam<Attr, Ty, Pat>, Comma>,
}

impl<
        'a,
        Attr: Parse<RustCursor<'a>, ()>,
        Ty: Parse<RustCursor<'a>, ()>,
        Pat: Parse<RustCursor<'a>, ()>,
    > Parse<RustCursor<'a>, ()> for FunctionParameters<Attr, Ty, Pat>
{
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<RustCursor<'a>>) -> Result<Self::Finalizer, Error> {
        Ok(BlackHoleFinalizer(
            match input.parse::<Sum2<(Option<(_, Comma)>, _), (_, Option<Comma>)>>()? {
                Sum2::V0((self_param, params)) => Self {
                    self_param: self_param.map(|v| v.0),
                    params,
                },
                Sum2::V1((self_param, _)) => Self {
                    self_param: Some(self_param),
                    params: Default::default(),
                },
            },
        ))
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct FunctionParameters<Attr, Ty, Pat> {
        self_param <- tokens into {
            if let Some(self_param) = self_param {
                tokens.extend(self_param.into_token_stream());
                tokens.extend(Comma::default().into_token_stream());
            }
        } to {
            if let Some(self_param) = self_param {
                self_param.to_tokens(tokens);
                tokens.extend(Comma::default().into_token_stream());
            }
        };
        params <- InterlaceTrail<FunctionParam<Attr, Ty, Pat>, Comma>
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct FunctionReturnType<Ty> { <- RArrow; ty <- Ty}
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct FunctionReturnType<Ty> {
        <- RArrow;
        ty <- Ty
    }
}

pub type Abi = StringLit;

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(parse print : it_matches_shorthand_self, SelfParam<P<Infallible>, P<Infallible>>: self);
    insta_match_test!(parse print : it_matches_typed_self,     SelfParam<P<Infallible>, TypePath<Ident>>: mut self: Box<Self>);

    insta_match_test!(parse print : it_matches_complex_function, Function<P<Infallible>, Ident, P<Infallible>, Ident>: const async unsafe extern "C" fn hello<T>(self, v: i64) -> T;);
}
