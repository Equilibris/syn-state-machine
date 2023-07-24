use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct Trait <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwTrait;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        bounds <- Option<TypeParamBound<Attr, Ty>> : Option<(Colon, Option<_>)> { bounds.and_then(|v| v.1) };
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- WithInnerAttrs<Attr, Vec<AssociateItem<Attr, Ty, Expr, Pat>> > : Brace<_> { items.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(
        +it_matches_trait, Trait <Infallible, Infallible, Type<Infallible>, Infallible>:
        unsafe trait HelloWorld<T> : From<T> T: Sized {
            type Hello: World;
        }
    );
}
