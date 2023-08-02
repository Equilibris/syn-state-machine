use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct GenericParams<Attr, Ty>{
        <- Lt;
        params <- InterlaceTrail<GenericParam<Attr, Ty>, Comma>;
        <- Gt;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct GenericParams<Attr, Ty> {
        <- Lt;
        params <- InterlaceTrail<GenericParam<Attr, Ty>, Comma>;
        <- Gt;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum GenericParam<Attr, Ty> [attrs <- Rep<OuterAttribute<Attr>>;] {
        Lt(lt <-LifetimeParam;),
        Ty(ty <-TypeParam<Attr, Ty>;),
        Cp(cp <- ConstParam<Ty>;)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum GenericParam<Attr, Ty> [attrs <- Rep<OuterAttribute<Attr>>;] {
        Lt(lt <-LifetimeParam;),
        Ty(ty <-TypeParam<Attr, Ty>;),
        Cp(cp <- ConstParam<Ty>;)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct LifetimeParam {
        lt <- LifetimeOrLabel;
        bound <- Option<LifetimeBounds> : Option<(Colon, _)> {bound.map(|v|v.1)};
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct LifetimeParam {
        lt <- LifetimeOrLabel;
        bound <- tokens into {
            if let Some(bound) = bound {
                tokens.extend(Colon::default().into_token_stream());
                tokens.extend(bound.into_token_stream())
            }
        } to {
            if let Some(ref bound) = bound {
                tokens.extend(Colon::default().into_token_stream());
                bound.to_tokens(tokens)
            }
        }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TypeParam<Attr, Ty> {
        id <- Ident : Identifier;
        bound <- Option<TypeParamBounds<Attr, Ty>>: Option<(Colon, Option<_>)> { bound.and_then(|v|v.1) };
        ty <- Option<Ty> : Option<(Eq, _)> { ty.map(|v|v.1) };
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TypeParam<Attr, Ty> {
        id <- Ident;
        bound <- tokens into {
            if let Some(bound) = bound {
                tokens.extend(Colon::default().into_token_stream());
                tokens.extend(bound.into_token_stream())
            }
        } to {
            if let Some(ref bound) = bound {
                tokens.extend(Colon::default().into_token_stream());
                bound.to_tokens(tokens)
            }
        };
        ty <- tokens into {
            if let Some(ty) = ty {
                tokens.extend(Eq::default().into_token_stream());
                tokens.extend(ty.into_token_stream())
            }
        } to {
            if let Some(ty) = ty {
                tokens.extend(Eq::default().into_token_stream());
                ty.to_tokens(tokens)
            }
        }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ConstParam<Ty> {
        <- KwConst;
        id <- Ident : Identifier;
        <- Colon;
        ty <- Ty;
        eq <- Option<Sum2<Identifier, Literal>> : Option<(Eq, _)> {eq.map(|v|v.1)};
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ConstParam<Ty> {
        <- KwConst;
        id <- Ident;
        <- Colon;
        ty <- Ty;
        eq <- tokens into {
            if let Some(eq) = eq {
                tokens.extend(Eq::default().into_token_stream());
                tokens.extend(eq.into_token_stream())
            }
        } to {
            if let Some(eq) = eq {
                tokens.extend(Eq::default().into_token_stream());
                eq.to_tokens(tokens)
            }
        }
    }
}

// Where Clause

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct WhereClause<Attr, Ty>{
        <- KwWhere;
        content <- Interlace<WhereClauseItem<Attr, Ty>, Comma>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct WhereClause<Attr, Ty> {
        <- KwWhere;
        content <- Interlace<WhereClauseItem<Attr, Ty>, Comma>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum WhereClauseItem<Attr, Ty> {
        Ty(ty <- TypeBoundWhereClauseItem<Attr, Ty>),
        Lt(lt <- LifetimeWhereClauseItem)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum WhereClauseItem<Attr, Ty> {
        Ty(ty <- TypeBoundWhereClauseItem<Attr, Ty>),
        Lt(lt <- LifetimeWhereClauseItem)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct LifetimeWhereClauseItem {
        lt <- Lifetime;
        <- Colon;
        bound <- LifetimeBounds;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct LifetimeWhereClauseItem {
        lt <- Lifetime;
        <- Colon;
        bound <- LifetimeBounds;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TypeBoundWhereClauseItem<Attr, Ty> {
        for_lts <- Option<ForLifetimes<Attr, Ty>>;
        ty <- Ty;
        <- Colon;
        bound <- Option<TypeParamBounds<Attr, Ty>>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TypeBoundWhereClauseItem<Attr, Ty> {
        for_lts <- Option<ForLifetimes<Attr, Ty>>;
        ty <- Ty;
        <- Colon;
        bound <- Option<TypeParamBounds<Attr, Ty>>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    insta_match_test!(+it_matches_simple_where, WhereClause<Infallible, Ident> : where F: Into<T>);
    insta_match_test!(+it_matches_simple_where_with_path, WhereClause<Infallible, Ident> : where F: std::ops::Add);

    insta_match_test!(+it_matches_const_param,         ConstParam<Ident>: const HELLO: i8);
    insta_match_test!(+it_matches_const_param_bounded, ConstParam<Ident>: const HELLO: i8 = 10);

    insta_match_test!(+it_matches_type_param,         TypeParam<Infallible, Infallible>: Hello);
    insta_match_test!(+it_matches_type_param_bounded, TypeParam<Infallible, Infallible>: Hello: std::fmt::Debug);
}
