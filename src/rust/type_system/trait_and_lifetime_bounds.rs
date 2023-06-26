use crate::*;

pub type TypeParamBounds<Attr, Ty> = MinLength<InterlaceTrail<TypeParamBound<Attr, Ty>, Plus>>;

#[derive(Debug)]
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
    #[derive(Debug)]
    pub struct TraitBound<Attr, Ty> {
        q peek <- Question;
        for_lts <- Option<ForLifetimes<Attr, Ty>>;
        path <- TypePath<Ty>;
    }
}

pub type LifetimeBounds = MinLength<Interlace<Lifetime, Plus>>;
pub type Lifetime = LifetimeToken;

materialize! {
    #[derive(Debug)]
    pub struct ForLifetimes<Attr, Ty> {
        <- KwFor;
        args <- GenericParams<Attr, Ty>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(+it_matches_lifetime, Lifetime : 'a);
    insta_match_test!(+it_matches_lifetimes_bounds, LifetimeBounds : 'a + 'b);
    insta_match_test!(+it_matches_bound_path, TraitBound<Infallible, SimplePath>: std::fmt::Debug);
    insta_match_test!(+it_matches_for_paths,  TraitBound<Infallible, SimplePath>: for<'a> std::fmt::Debug);
    insta_match_test!(
        +it_matches_path_type_param_bound,
        TypeParamBound<Infallible, SimplePath>: std::fmt::Debug
    );
    insta_match_test!(
        +it_matches_for_paths_type_param_bound,
        TypeParamBound<Infallible, SimplePath>: for<'a> std::fmt::Debug
    );
    insta_match_test!(
        +it_matches_lifetime_type_param_bound,
        TypeParamBound<Infallible, SimplePath>: 'a
    );
    insta_match_test!(
        +it_matches_for_lifetimes,
        ForLifetimes<Infallible, Infallible>: for<'a, 'b>
    );
}
