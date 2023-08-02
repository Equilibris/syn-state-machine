use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct HigherOrderPath<Segment> {
        leading peek <- PathSep;
        segments <-  Interlace<Segment, PathSep> : MinLength<_> { segments.0 };
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct HigherOrderPath<Segment> {
        leading peek <- PathSep;
        segments <- Interlace<Segment, PathSep>
    }
}

// <simplepath>

pub type SimplePath = HigherOrderPath<SimplePathSegment>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum SimplePathSegment {
        Identifier(v <-Identifier),
        Super(v <- KwSuper),
        SSelf(v <- KwLowerSelf),
        Crate(v <- KwCrate),
        MacroCrate(v <- P2<Dollar, KwCrate>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum SimplePathSegment {
        Identifier(v <- Identifier),
        Super(v <- KwSuper),
        SSelf(v <- KwLowerSelf),
        Crate(v <- KwCrate),
        MacroCrate(v <- P2<Dollar, KwCrate>)
    }
}

// </simplepath>
// <path in expressions>

pub type PathInExpression<Ty> = HigherOrderPath<PathExprSegment<Ty>>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct PathExprSegment<Ty> {
        id <- PathIdentSegment;
        generic <- Option<GenericArgs<Ty>> : Option<(PathSep, GenericArgs<Ty>)> {generic.map(|v|v.1)};
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct PathExprSegment<Ty> {
        id <- PathIdentSegment;
        generic <- tokens into {
            if let Some(v) = generic {
                tokens.append_all(PathSep::default());
                tokens.append_all(v.into_token_stream());
            }
        } to {
            if let Some(ref v) = generic {
                tokens.extend(PathSep::default().into_token_stream());
                v.to_tokens(tokens)
            }
        }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum PathIdentSegment {
        Id(v <- Ident : Identifier),
        Super(v <- KwSuper),
        LowerSelf(v <- KwLowerSelf),
        UpperSelf(v <- KwUpperSelf),
        Crate(v <- KwCrate),
        MacroCrate(v <- P2< Dollar, KwCrate >)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum PathIdentSegment {
        Id(v <- Ident),
        Super(v <- KwSuper),
        LowerSelf(v <- KwLowerSelf),
        UpperSelf(v <- KwUpperSelf),
        Crate(v <- KwCrate),
        MacroCrate(v <- P2< Dollar, KwCrate >)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct GenericArgs<Ty>{
        <- Lt;
        args <- InterlaceTrail<GenericArg<Ty>, Comma>;
        <- Gt
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct GenericArgs<Ty> {
        <- Lt;
        args <- InterlaceTrail<GenericArg<Ty>, Comma>;
        <- Gt
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum GenericArg<Ty> {
        Lt(v <- Lifetime),
        Ty(v <- Ty),
        Const(v <- GenericArgsConst),
        Binding(v <- GenericArgsBinding<Ty>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum GenericArg<Ty> {
        Lt(v <- Lifetime),
        Ty(v <- Ty),
        Const(_v <- _tts into { todo!() } to { todo!() }),
        Binding(v <- GenericArgsBinding<Ty>)
    }
}

pub type GenericArgsConst = Infallible;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct GenericArgsBinding<Ty>{
        id <- Ident : Identifier;
        <- Eq;
        ty <- Ty;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct GenericArgsBinding<Ty> {
        id <- Ident;
        <- Eq;
        ty <- Ty;
    }
}
// </path in expressions>

// <Qualified paths>

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct QualifiedPathInExpression<Ty>{
        qualifier <- QualifiedPathType<Ty>;
        path <- Rep<P2<PathSep, PathExprSegment<Ty>>>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct QualifiedPathInExpression<Ty> {
        qualifier <- QualifiedPathType<Ty>;
        path <- Rep<P2<PathSep, PathExprSegment<Ty>>>;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct QualifiedPathType<Ty> {
        <- Lt;
        ty <- Ty;
        as_ty <- Option<TypePath<Ty>> : Option<(KwAs, _)> { as_ty.map(|v|v.1) };
        <- Gt;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct QualifiedPathType<Ty> {
        <- Lt;
        ty <- Ty;
        as_ty <- tokens into {
            if let Some(as_ty) = as_ty {
                tokens.extend(KwAs::default().into_token_stream());
                tokens.extend(as_ty.into_token_stream())
            }
        } to {
            if let Some(as_ty) = as_ty {
                tokens.extend(KwAs::default().into_token_stream());
                as_ty.to_tokens(tokens)
            }
        };
        <- Gt;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct QualifiedPathInType<Ty>{
        qualifier <- QualifiedPathType<Ty>;
        path <- Rep<P2<PathSep, TypePathSegment<Ty>>>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct QualifiedPathInType<Ty> {
        qualifier <- QualifiedPathType<Ty>;
        path <- Rep<P2<PathSep, TypePathSegment<Ty>>>;
    }
}

// </Qualified paths>

pub type TypePath<Ty> = HigherOrderPath<TypePathSegment<Ty>>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum TypePathSegment<Ty> [path <- PathIdentSegment] {
        Fn(fun <- TypePathFn<Ty> : (Option<PathSep>, _) { fun.1 }),
        Generic(args <- GenericArgs<Ty> : (Option<PathSep>,_) { args.1 }),
        Bare()
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum TypePathSegment<Ty> [path <- PathIdentSegment] {
        Fn(fun <- tokens into {
            tokens.extend(PathSep::default().into_token_stream());
            tokens.extend(fun.into_token_stream())
        } to {
            tokens.extend(PathSep::default().into_token_stream());
            fun.to_tokens(tokens)
        }),
        Generic(args <- tokens into {
            tokens.extend(PathSep::default().into_token_stream());
            tokens.extend(args.into_token_stream())
        } to {
            tokens.extend(PathSep::default().into_token_stream());
            args.to_tokens(tokens)
        }),
        Bare()
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TypePathFn<Ty> {
        args <- Paren<TypePathFnInputs<Ty>>;
        returns <- Option<Ty> : Option<(RArrow, _)> { returns.map(|v|v.1) };
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TypePathFn<Ty> {
        args <- Paren<TypePathFnInputs<Ty>>;
        returns <- tokens into {
            if let Some(returns) = returns {
                tokens.extend(RArrow::default().into_token_stream());
                tokens.extend(returns.into_token_stream())
            }
        } to {
            if let Some(returns) = returns {
                tokens.extend(RArrow::default().into_token_stream());
                returns.to_tokens(tokens)
            }
        }
    }
}

type TypePathFnInputs<Ty> = InterlaceTrail<Ty, Comma>;

#[cfg(test)]
#[cfg(feature = "printing")]
mod tests {
    use crate::{SimplePath, SimplePathSegment};

    struct _Test
    where
        SimplePath: quote::ToTokens,
        SimplePathSegment: quote::ToTokens;
}

#[cfg(test)]
mod type_tests {
    use crate::*;

    insta_match_test!(+it_matches_hello, TypePath<Ident>: hello);
    insta_match_test!(+it_matches_tri_path, TypePath<Ident>: hello::world::hi);
    insta_match_test!(+it_matches_bi_path, TypePath<Ident>: hello::world);
    insta_match_test!(+it_matches_long_generic, TypePath<Ident>: hello::<Hi>);
    insta_match_test!(+it_matches_short_generic, TypePath<Ident>: hello<Hi>);
    insta_match_test!(+it_matches_fun, TypePath<Infallible>: Fn());
    insta_match_test!(+it_matches_fun_ret, TypePath<Paren<()>>: Fn() -> ());

    #[test]
    fn it_matches_multigeneric_type_path() {
        println!(
            "{:#?}",
            parse::<TypePath<TypePath<Ident>>>(quote::quote!(hello<hello::Hi, 10, 'a>)).unwrap()
        );
    }
}
#[cfg(test)]
mod qualified_tests {
    use super::*;

    insta_match_test!(+it_matches_simple_paths, QualifiedPathInType<Ident> : <hello as Default>::Default);
}

#[cfg(test)]
mod generic_tests {
    use super::*;
    use std::convert::Infallible;

    insta_match_test!(+it_matches_empty_generic_args, GenericArgs<Infallible>: <>);
    insta_match_test!(+it_matches_lifetime_args, GenericArgs<Infallible>: <'a>);
    insta_match_test!(+it_matches_typed_args, GenericArgs<Ident>: <T>);
    insta_match_test!(+it_matches_pathed_args, GenericArgs<TypePath<Infallible>>: <hello::world>);
    insta_match_test!(+it_matches_multi_args, GenericArgs<TypePath<Infallible>>: <'a, T, hello::world>);
}
