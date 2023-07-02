use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct Function<Attr, Ty, Pat>{
        qualifiers <- FunctionQualifiers;
        <- KwFn;
        id <- Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        params <- Paren<FunctionParameters<Attr, Ty, Pat>>;
        ret <- Option<Ty> : Option<FunctionReturnType<_>> { ret.map(|v|v.ty) };
        where_clause <- Option<WhereClause<Attr,Ty>>;
        content <- Sum2<Semi, Brace<Vec<TokenTree>>>;
    }
}
materialize! {
    #[derive(Debug)]
    pub struct FunctionQualifiers {
        r#const peek <- KwConst;
        r#async peek <- KwAsync;
        r#unsafe peek <- KwUnsafe;
        extern_abi <- Option<Option<Abi>> : Option<(KwExtern, _)> { extern_abi.map(|v|v.1) };
    }
}

materialize! {
    #[derive(Debug)]
    pub enum SelfParam<Attr, Ty> [attrs <- Vec<OuterAttribute<Attr>>] {
        Typed(v <- TypedSelf<Ty>)
        Shorthand(v <- ShorthandSelf)
    }
}
materialize! {
    #[derive(Debug)]
    pub struct ShorthandSelf {
        reference <- Option<Option<Lifetime>> : Option<(And, _)> { reference.map(|v|v.1) };
    }
}
materialize! {
    #[derive(Debug)]
    pub struct TypedSelf<Ty> {
        mutable peek <- KwMut;
        <- KwLowerSelf;
        <- Colon;
        ty <- Ty;
    }
}
materialize! {
    #[derive(Debug)]
    pub enum FunctionParam <Attr, Ty, Pat>[attrs <- Vec<OuterAttribute<Attr>> ] {
        FunctionParamPattern(v <- FunctionParamPattern<Ty, Pat>)
        Ty(v <- Ty)
        Rest(v <- DotDotDot)
    }
}
materialize! {
    #[derive(Debug)]
    pub enum FunctionParamPattern<Ty, Pat> [ pat <- Pat; <- Colon ] {
        Ty(v <- Ty)
        Unknown(v <- DotDotDot)
    }
}

#[derive(Debug)]
pub struct FunctionParameters<Attr, Ty, Pat> {
    pub self_param: Option<SelfParam<Attr, Ty>>,
    pub params: InterlaceTrail<FunctionParam<Attr, Ty, Pat>, Comma>,
}

impl<Attr: Parse, Ty: Parse, Pat: Parse> Parse for FunctionParameters<Attr, Ty, Pat> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(
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
        )
    }
}

materialize! {
    #[derive(Debug)]
    pub struct FunctionReturnType<Ty> { <- RArrow; ty <- Ty}
}

pub type Abi = StringLit;

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(+it_matches_shorthand_self, SelfParam<Infallible, Infallible>: self);
    insta_match_test!(+it_matches_typed_self, SelfParam<Infallible, TypePath<Ident>>: mut self: Box<Self>);

    insta_match_test!(+it_matches_complex_function, Function<Infallible, Ident, Ident>: const async unsafe extern "C" fn hello<T>(self, v: i64) -> T;);
}
