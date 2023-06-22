use crate::*;

pub type TypeParamBounds<Attr, Ty> = MinLength<InterlaceTrail<TypeParamBound<Attr, Ty>, Plus>>;

pub enum TypeParamBound<Attr, Ty> {
    Lt(Lifetime),
    Tr(TraitBound<Attr, Ty>),
}

impl<Attr: Parse, Ty: Parse> Parse for TypeParamBound<Attr, Ty> {
    fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
        Ok(match input.parse::<Sum2<_, _>>()? {
            Sum2::V0(a) => Self::Lt(a),
            Sum2::V1(a) => Self::Tr(a),
        })
    }
}

materialize! {
    pub struct TraitBound<Attr, Ty> {
        q peek <- Question;
        for_lts <- Option<ForLifetimes<Attr, Ty>>;
        path <- TypePath<Ty>;
    }
}

pub type LifetimeBounds = MinLength<Interlace<Lifetime, Plus>>;

pub type Lifetime = LifetimeToken;

materialize! {
    pub struct ForLifetimes<Attr, Ty> {
        <- KwFor;
        args <- GenericParams<Attr, Ty>;
    }
}
