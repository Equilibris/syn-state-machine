use crate::*;

materialize! {
    pub struct BareFunctionType <Attr, Ty, TyNB>{
        for_lifetimes <- Option<ForLifetimes<Attr, Ty>>;
        qualifiers <- FunctionTypeQualifiers;
        <- KwFn;
        params <- FunctionParametersMaybeNamedVariadic<Attr, TyNB> : Paren<_> { params.0 };
        ret <- Option<BareFunctionReturnType<TyNB>>;
    }
}
materialize! {
    pub struct FunctionTypeQualifiers {
        r#unsafe peek <- KwUnsafe;
        r#extern <- Option<Option<Abi>> : Option<(KwExtern, Option<Abi>)> { extern.map(|v| v.1) }
    }
}
pub type BareFunctionReturnType<TyNB> = FunctionReturnType<TyNB>;

materialize! {
    pub enum FunctionParametersMaybeNamedVariadic <Attr, Ty> {
        NAdic(v <- MaybeNamedFunctionParametersVariadic<Attr, Ty>)
        Variadic(v <- MaybeNamedFunctionParameters<Attr, Ty>)
    }
}

materialize! {
    pub struct MaybeNamedFunctionParametersVariadic<Attr, Ty> {
        params <- Interlace<MaybeNamedParam<Attr,Ty>, Comma>;
        <- Comma;
        attrs <- Vec<OuterAttribute<Attr>>;
        <- DotDotDot
    }
}

pub type MaybeNamedFunctionParameters<Attr, Ty> = InterlaceTrail<MaybeNamedParam<Attr, Ty>, Comma>;

materialize! {
    pub struct MaybeNamedParam <Attr, Ty> {
        attrs <- Vec<OuterAttribute<Attr>>;
        id <- Option<IdentifierOrUnder> : Option<(_, Colon)> { id.map(|v|v.0) };
        ty <- Ty
    }
}
