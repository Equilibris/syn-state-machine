use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct TypeAlias<Attr, Ty> {
        <- KwType;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        bounds <- Option<TypeParamBounds<Attr, Ty>> : Option<(Colon, _)> { bounds.map(|v|v.1) };
        where_clause <- Option<WhereClause<Attr, Ty>>;
        eq <- Option<(Ty, Option<WhereClause<Attr, Ty>>)> : Option<(Eq, _)> { eq.map(|v|v.1) };
        <- Semi
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    insta_match_test!(+it_matches_simple,    TypeAlias<Infallible, Type<Infallible>>: type Point;);
    insta_match_test!(+it_matches_simple_eq, TypeAlias<Infallible, Type<Infallible>>: type Point = (u8, u8););
    insta_match_test!(+it_matches_complex,   TypeAlias<Infallible, Type<Infallible>>: type Point<T> where T: std::ops::Add<T> = (T, T););
    insta_match_test!(+it_matches_complex_eq,TypeAlias<Infallible, Type<Infallible>>: type Point<T> where T: std::ops::Add<T> = (T, T););
}
