use crate::{rust::attributes::OuterAttribute, *};

materialize! {
    #[derive(Debug)]
    pub struct GenericParams<Attr, Ty>{
        <- Lt;
        params <- InterlaceTrail<GenericParam<Attr, Ty>, Comma>;
        <- Gt;
    }
}

materialize! {
    #[derive(Debug)]
    pub enum GenericParam<Attr, Ty> [attrs <- Vec<OuterAttribute<Attr>>;] {
        Lt(lt <-LifetimeParam;)
        Ty(ty <-TypeParam<Attr, Ty>;)
        Cp(cp <- ConstParam<Ty>;)
    }
}

materialize! {
    #[derive(Debug)]
    pub struct LifetimeParam {
        lt <- LifetimeOrLabel;
        bound <- Option<LifetimeBounds> : Option<(Colon, _)> {bound.map(|v|v.1)};
    }
}

materialize! {
    #[derive(Debug)]
    pub struct TypeParam<Attr, Ty> {
        id <- Identifier;
        bound <- Option<TypeParamBounds<Attr, Ty>>: Option<(Colon, Option<_>)> { bound.and_then(|v|v.1) };
        ty <- Option<Ty> : Option<(Eq, _)> { ty.map(|v|v.1) };
    }
}
materialize! {
    #[derive(Debug)]
    pub struct ConstParam<Ty> {
        <- KwConst;
        id <- Identifier;
        <- Colon;
        ty <- Ty;
        eq <- Option<Sum3<Infallible, Identifier, Literal>> : Option<(Eq, _)> {eq.map(|v|v.1)};
    }
}

// Where Clause

materialize! {
    #[derive(Debug)]
    pub struct WhereClause<Attr, Ty>{
        <- KwWhere;
        content <- Interlace<WhereClauseItem<Attr, Ty>, Comma>;
    }
}

materialize! {
    #[derive(Debug)]
    pub enum WhereClauseItem<Attr, Ty> {
        Lt(lt <- LifetimeWhereClauseItem;)
        Ty(ty <- TypeBoundWhereClauseItem<Attr, Ty>;)
    }
}

materialize! {
    #[derive(Debug)]
    pub struct LifetimeWhereClauseItem {
        lt <- Lifetime;
        <- Colon;
        bound <- LifetimeBounds;
    }
}

materialize! {
    #[derive(Debug)]
    pub struct TypeBoundWhereClauseItem<Attr, Ty> {
        for_lts <- Option<ForLifetimes<Attr, Ty>>;
        ty <- Ty;
        bound <- Option<TypeParamBounds<Attr, Ty>>;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(+it_matches_const_param,         ConstParam<Ident>: const HELLO: i8);
    insta_match_test!(+it_matches_const_param_bounded, ConstParam<Ident>: const HELLO: i8 = 10);

    insta_match_test!(+it_matches_type_param,         TypeParam<Infallible, Infallible>: Hello);
    insta_match_test!(+it_matches_type_param_bounded, TypeParam<Infallible, Infallible>: Hello: std::fmt::Debug);
}
