use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct BareFunctionType <Attr, Ty, TyNB>{
        for_lifetimes <- Option<ForLifetimes<Attr, Ty>>;
        qualifiers <- FunctionTypeQualifiers;
        <- KwFn;
        params <- FunctionParametersMaybeNamedVariadic<Attr, TyNB> : Paren<_> { params.0 };
        ret <- Option<BareFunctionReturnType<TyNB>>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct BareFunctionType<Attr, Ty, TyNB> {
        for_lifetimes <- Option<ForLifetimes<Attr, Ty>>;
        qualifiers <- FunctionTypeQualifiers;
        <- KwFn;
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
        };
        ret <- Option<BareFunctionReturnType<TyNB>>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct FunctionTypeQualifiers {
        r#unsafe peek <- KwUnsafe;
        r#extern <- Option<Option<Abi>> : Option<(KwExtern, Option<Abi>)> { r#extern.map(|v| v.1) }
    }
}
to_tokens! {
    impl ToTokens for struct FunctionTypeQualifiers {
        r#unsafe peek <- KwUnsafe;
        r#extern <- tokens into {
            if r#extern.is_some() {
                tokens.extend(KwExtern::default().into_token_stream());
            }
            if let Some(Some(abi)) = r#extern {
                tokens.append(proc_macro2::Literal::from(abi))
            }
        } to {
            if r#extern.is_some() {
                tokens.extend(KwExtern::default().into_token_stream());
            }
            if let Some(Some(abi)) = r#extern {
                tokens.append(proc_macro2::Literal::from(abi.clone()))
            }
        };
    }
}
pub type BareFunctionReturnType<TyNB> = FunctionReturnType<TyNB>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum FunctionParametersMaybeNamedVariadic <Attr, Ty> {
        NAdic(v <- MaybeNamedFunctionParametersVariadic<Attr, Ty>),
        Variadic(v <- MaybeNamedFunctionParameters<Attr, Ty>)
    }
}
to_tokens! {
    impl ToTokens for enum FunctionParametersMaybeNamedVariadic<Attr, Ty> {
        NAdic(v <- MaybeNamedFunctionParametersVariadic<Attr, Ty>),
        Variadic(v <- MaybeNamedFunctionParameters<Attr, Ty>)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct MaybeNamedFunctionParametersVariadic<Attr, Ty> {
        params <- Interlace<MaybeNamedParam<Attr,Ty>, Comma>;
        <- Comma;
        attrs <- Rep<OuterAttribute<Attr>>;
        <- DotDotDot
    }
}
to_tokens! {
    impl ToTokens for struct MaybeNamedFunctionParametersVariadic<Attr, Ty> {
        params <- Interlace<MaybeNamedParam<Attr,Ty>, Comma>;
        <- Comma;
        attrs <- Rep<OuterAttribute<Attr>>;
        <- DotDotDot
    }
}

pub type MaybeNamedFunctionParameters<Attr, Ty> = InterlaceTrail<MaybeNamedParam<Attr, Ty>, Comma>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct MaybeNamedParam <Attr, Ty> {
        attrs <- Rep<OuterAttribute<Attr>>;
        id <- Option<IdentifierOrUnder> : Option<(_, Colon)> { id.map(|v|v.0) };
        ty <- Ty
    }
}
to_tokens! {
    impl ToTokens for struct MaybeNamedParam<Attr, Ty> {
        attrs <- Rep<OuterAttribute<Attr>>;
        id <- tokens into {
            if let Some(id) = id {}
        } to {
            if let Some(id) = id {
                id.0.to_tokens(tokens);
                tokens.extend(Colon::default().into_token_stream());
            }
        };
        ty <- Ty
    }
}
