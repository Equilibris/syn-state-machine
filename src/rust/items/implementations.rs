use crate::*;

materialize! {
    pub struct InherentImpl <Attr, Ty, Expr, Pat> {
        <- KwImpl;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        ty <- Ty;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- (Vec<InnerAttribute<Attr>>, Vec<AssociateItem<Attr, Ty, Expr, Pat>>) : Brace<_> { items.0 }
    }
}

materialize! {
    pub struct TraitImpl <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwImpl;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        not peek <- Not;
        path <- TypePath<Ty>;
        <- KwFor;
        ty <- Ty;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- (Vec<InnerAttribute<Attr>>, Vec<AssociateItem<Attr, Ty, Expr, Pat>>) : Brace<_> { items.0 }
    }
}
