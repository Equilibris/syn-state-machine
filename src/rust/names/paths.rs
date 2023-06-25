use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct HigherOrderPath<Segment> {
        leading peek <- PathSep;
        segments <- MinLength<Interlace<Segment, PathSep>>;
    }
}

// <simplepath>

pub type SimplePath = HigherOrderPath<SimplePathSegment>;

#[derive(Debug)]
pub enum SimplePathSegment {
    Identifier(Identifier),
    Super(KwSuper),
    SSelf(KwLowerSelf),
    Crate(KwCrate),
    MacroCrate((Dollar, KwCrate)),
}

impl Parse for SimplePathSegment {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(match input.parse::<Sum5<_, _, _, _, _>>()? {
            Sum5::V0(a) => Self::Identifier(a),
            Sum5::V1(a) => Self::Super(a),
            Sum5::V2(a) => Self::SSelf(a),
            Sum5::V3(a) => Self::Crate(a),
            Sum5::V4(a) => Self::MacroCrate(a),
        })
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

#[derive(Debug)]
pub enum PathIdentSegment {
    Id(Identifier),
    Super(KwSuper),
    LowerSelf(KwLowerSelf),
    UpperSelf(KwUpperSelf),
    Crate(KwCrate),
    MacroCrate((Dollar, KwCrate)),
}

impl Parse for PathIdentSegment {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(match input.parse::<Sum6<_, _, _, _, _, _>>()? {
            Sum6::V0(a) => Self::Id(a),
            Sum6::V1(a) => Self::Super(a),
            Sum6::V2(a) => Self::LowerSelf(a),
            Sum6::V3(a) => Self::UpperSelf(a),
            Sum6::V4(a) => Self::Crate(a),
            Sum6::V5(a) => Self::MacroCrate(a),
        })
    }
}

#[derive(Debug)]
pub struct GenericArgs<Ty>(pub InterlaceTrail<GenericArg<Ty>, Comma>);

impl<Ty: Parse> Parse for GenericArgs<Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        input.errored_peek::<Lt>()?;

        let v: InterlaceTrail<GenericArg<Ty>, FPunct<','>> = input.parse()?;

        println!("{:#?}", v.values.len());

        input.errored_peek::<Gt>()?;

        Ok(Self(v))
    }
}

#[derive(Debug)]
pub enum GenericArg<Ty> {
    Lt(Lifetime),
    Ty(Ty),
    Const(GenericArgsConst),
    Binding(GenericArgsBinding<Ty>),
}

impl<Ty: Parse> Parse for GenericArg<Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(match input.parse::<Sum4<_, _, _, _>>()? {
            Sum4::V0(a) => Self::Lt(a),
            Sum4::V1(a) => Self::Ty(a),
            Sum4::V2(a) => Self::Const(a),
            Sum4::V3(a) => Self::Binding(a),
        })
    }
}

pub type GenericArgsConst = Infallible;

#[derive(Debug)]
pub struct GenericArgsBinding<Ty>(pub Identifier, pub Ty);
impl<Ty: Parse> Parse for GenericArgsBinding<Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let i = input.parse()?;
        input.errored_peek::<Eq>()?;

        Ok(Self(i, input.parse()?))
    }
}
// </path in expressions>

// <Qualified paths>

#[derive(Debug)]
pub struct QualifiedPathInExpression<Ty>(
    pub QualifiedPathType<Ty>,
    Vec<(PathSep, PathExprSegment<Ty>)>,
);

impl<Ty: Parse> Parse for QualifiedPathInExpression<Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(Self(input.parse()?, input.parse()?))
    }
}

#[derive(Debug)]
pub struct QualifiedPathType<Ty> {
    pub ty: Ty,
    pub as_ty: Option<TypePath<Ty>>,
}

impl<Ty: Parse> Parse for QualifiedPathType<Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        input.errored_peek::<Lt>()?;

        let ty = input.parse()?;
        let as_ty = input.parse::<Option<(KwAs, _)>>()?.map(|v| v.1);
        input.errored_peek::<Gt>()?;

        Ok(Self { ty, as_ty })
    }
}

#[derive(Debug)]
pub struct QualifiedPathInType<Ty>(
    pub QualifiedPathType<Ty>,
    Vec<(PathSep, TypePathSegment<Ty>)>,
);

impl<Ty: Parse> Parse for QualifiedPathInType<Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(Self(input.parse()?, input.parse()?))
    }
}

// </Qualified paths>

pub type TypePath<Ty> = HigherOrderPath<TypePathSegment<Ty>>;

#[derive(Debug)]
pub enum TypePathSegment<Ty> {
    Bare(PathIdentSegment),
    Generic(PathIdentSegment, GenericArgs<Ty>),
    Fn(PathIdentSegment, TypePathFn<Ty>),
}

impl<Ty: Parse> Parse for TypePathSegment<Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        let i = input.parse()?;

        Ok(
            match input.parse::<Option<(Option<PathSep>, Sum2<_, _>)>>()? {
                Some((_, Sum2::V0(a))) => Self::Fn(i, a),
                Some((_, Sum2::V1(a))) => Self::Generic(i, a),
                None => Self::Bare(i),
            },
        )
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
