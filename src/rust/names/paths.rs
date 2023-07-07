use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct HigherOrderPath<Segment> {
        leading peek <- PathSep;
        segments <-  Interlace<Segment, PathSep> : MinLength<_> { segments.0 };
    }
}

// <simplepath>

pub type SimplePath = HigherOrderPath<SimplePathSegment>;

materialize! {
    #[derive(Debug)]
    pub enum SimplePathSegment {
        Identifier(v <-Identifier)
        Super(v <- KwSuper)
        SSelf(v <- KwLowerSelf)
        Crate(v <- KwCrate)
        MacroCrate(v <- (Dollar, KwCrate))
    }
}

// </simplepath>
// <path in expressions>

pub type PathInExpression<Ty> = HigherOrderPath<PathExprSegment<Ty>>;

materialize! {
    #[derive(Debug)]
    pub struct PathExprSegment<Ty> {
        id <- PathIdentSegment;
        generic <- Option<GenericArgs<Ty>> : Option<(PathSep, GenericArgs<Ty>)> {generic.map(|v|v.1)};
    }
}

materialize! {
    #[derive(Debug)]
    pub enum PathIdentSegment {
        Id(v <- Ident : Identifier)
        Super(v <- KwSuper)
        LowerSelf(v <- KwLowerSelf)
        UpperSelf(v <- KwUpperSelf)
        Crate(v <- KwCrate)
        MacroCrate(v <- (Dollar, KwCrate))
    }
}

materialize! {
    #[derive(Debug)]
    pub struct GenericArgs<Ty>{
        <- Lt;
        args <- InterlaceTrail<GenericArg<Ty>, Comma>;
        <- Gt;
    }
}

materialize! {
    #[derive(Debug)]
    pub enum GenericArg<Ty> {
        Lt(v <- Lifetime)
        Ty(v <- Ty)
        Const(v <- GenericArgsConst)
        Binding(v <- GenericArgsBinding<Ty>)
    }
}

pub type GenericArgsConst = Infallible;

materialize! {
    #[derive(Debug)]
    pub struct GenericArgsBinding<Ty>{
        id <- Ident : Identifier;
        <- Eq;
        ty <- Ty;
    }
}
// </path in expressions>

// <Qualified paths>

materialize! {
    #[derive(Debug)]
    pub struct QualifiedPathInExpression<Ty>{
        qualifier <- QualifiedPathType<Ty>;
        path <- Vec<(PathSep, PathExprSegment<Ty>)>;
    }
}

materialize! {
    #[derive(Debug)]
    pub struct QualifiedPathType<Ty> {
        <- Lt;
        ty <- Ty;
        as_ty <- Option<TypePath<Ty>> : Option<(KwAs, _)> { as_ty.map(|v|v.1) };
        <- Gt;
    }
}

materialize! {
    #[derive(Debug)]
    pub struct QualifiedPathInType<Ty>{
        qualifier <- QualifiedPathType<Ty>;
        path <- Vec<(PathSep, TypePathSegment<Ty>)>;
    }
}

// </Qualified paths>

pub type TypePath<Ty> = HigherOrderPath<TypePathSegment<Ty>>;

materialize! {
    #[derive(Debug)]
    pub enum TypePathSegment<Ty> [path <- PathIdentSegment] {
        Fn(fun <- TypePathFn<Ty> : (Option<PathSep>, _) { fun.1 })
        Generic(args <- GenericArgs<Ty> : (Option<PathSep>,_) { args.1 })
        Bare()
    }
}

materialize! {
    #[derive(Debug)]
    pub struct TypePathFn<Ty> {
        args <- Paren<TypePathFnInputs<Ty>>;
        returns <- Option<Ty> : Option<(RArrow, _)> { returns.map(|v|v.1) };
    }
}

type TypePathFnInputs<Ty> = InterlaceTrail<Ty, Comma>;

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
