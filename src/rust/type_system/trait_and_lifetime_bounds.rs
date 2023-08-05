use crate::*;

pub type TypeParamBounds<Attr, Ty> = MinLength<InterlaceTrail<TypeParamBound<Attr, Ty>, Plus>>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum TypeParamBound<Attr, Ty> {
        Lt(v <- Lifetime),
        Tr(v <- TraitBound<Attr, Ty>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum TypeParamBound<Attr, Ty> {
        Lt(v <- Lifetime),
        Tr(v <- TraitBound<Attr, Ty>)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TraitBound<Attr, Ty> {
        q peek  <- Question;
        for_lts <- Option<ForLifetimes<Attr, Ty>>;
        path    <- TypePath<Ty>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TraitBound<Attr, Ty> {
        q peek  <- Question;
        for_lts <- Option<ForLifetimes<Attr, Ty>>;
        path    <- TypePath<Ty>;
    }
}

pub type LifetimeBounds = MinLength<Interlace<Lifetime, Plus>>;
pub type Lifetime = LifetimeToken;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ForLifetimes<Attr, Ty> {
        <- KwFor;
        args <- GenericParams<Attr, Ty>;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ForLifetimes<Attr, Ty> {
        <- KwFor;
        args <- GenericParams<Attr, Ty>;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(parse print : it_matches_lifetime, Lifetime : 'a);
    insta_match_test!(parse print : it_matches_lifetimes_bounds, LifetimeBounds : 'a + 'b);
    insta_match_test!(parse print : it_matches_bound_path, TraitBound<P<Infallible>, SimplePath>: std::fmt::Debug);
    insta_match_test!(parse print : it_matches_for_paths,  TraitBound<P<Infallible>, SimplePath>: for<'a> std::fmt::Debug);
    insta_match_test!(
        parse print : it_matches_path_type_param_bound,
        TypeParamBound<P<Infallible>, SimplePath>: std::fmt::Debug
    );
    insta_match_test!(
        parse print : it_matches_for_paths_type_param_bound,
        TypeParamBound<P<Infallible>, SimplePath>: for<'a> std::fmt::Debug
    );
    insta_match_test!(
        parse print : it_matches_lifetime_type_param_bound,
        TypeParamBound<P<Infallible>, SimplePath>: 'a
    );
    insta_match_test!(
        parse print : it_matches_for_lifetimes,
        ForLifetimes<P<Infallible>, P<Infallible>>: for<'a, 'b>
    );
}
