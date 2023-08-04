use crate::*;

#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Implementation <Attr, Ty, Expr, Pat> {
        Inherent(v <- InherentImpl<Attr, Ty, Expr, Pat>),
        Trait(v <- TraitImpl<Attr, Ty, Expr, Pat>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum Implementation<Attr, Ty, Expr, Pat> {
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
        items <- WithInnerAttrs<Attr, Rep<AssociateItem<Attr, Ty, Expr, Pat>>> : Brace<_> { items.0 }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct InherentImpl<Attr, Ty, Expr, Pat> {
        <- KwImpl;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        ty <- Ty;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    items.into_token_stream()
                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    items.to_token_stream()
                )
            )
        }
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
        items <- WithInnerAttrs<Attr, Rep<AssociateItem<Attr, Ty, Expr, Pat>> > : Brace<_> { items.0 }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TraitImpl<Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwImpl;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        not peek <- Not;
        path <- TypePath<Ty>;
        <- KwFor;
        ty <- Ty;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    items.into_token_stream()
                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    items.to_token_stream()
                )
            )
        }

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(
        parse : it_matches_simple_inherent, Implementation <Infallible, Type<Infallible>, Infallible, Ident>:

        impl<T> Option<T> {
            pub fn is_some(&self) -> bool;
        }
    );
    insta_match_test!(
        parse : it_matches_simple_trait, Implementation <Infallible, TypePath<Ident>, Infallible, Ident>:

        unsafe impl<T: Copy> Copy for Option<T> {}
    );
}
