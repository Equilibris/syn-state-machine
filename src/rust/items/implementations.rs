use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Implementation <Attr, Ty, Expr, Pat> {
        Inherent(v <- InherentImpl<Attr, Ty, Expr, Pat>),
        Trait(v <- TraitImpl<Attr, Ty, Expr, Pat>)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct InherentImpl <Attr, Ty, Expr, Pat> {
        <- KwImpl;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        ty <- Ty;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- WithInnerAttrs<Attr, Vec<AssociateItem<Attr, Ty, Expr, Pat>>> : Brace<_> { items.0 }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TraitImpl <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwImpl;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        not peek <- Not;
        path <- TypePath<Ty>;
        <- KwFor;
        ty <- Ty;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- WithInnerAttrs<Attr, Vec<AssociateItem<Attr, Ty, Expr, Pat>> > : Brace<_> { items.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(
        +it_matches_simple_inherent, Implementation <Infallible, Type<Infallible>, Infallible, Ident>:

        impl<T> Option<T> {
            pub fn is_some(&self) -> bool;
        }
    );
    insta_match_test!(
        +it_matches_simple_trait, Implementation <Infallible, TypePath<Ident>, Infallible, Ident>:

        unsafe impl<T: Copy> Copy for Option<T> {}
    );
}
