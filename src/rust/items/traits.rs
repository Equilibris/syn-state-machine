use crate::*;

materialize! {
    pub struct Trait <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwTrait;
        id <- Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        bounds <- Option<TypeParamBound<Attr, Ty>> : Option<(Colon, Option<_>)> { bounds.and_then(|v| v.1) };
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- (Vec<InnerAttribute<Attr>>, Vec<AssociateItem<Attr, Ty, Expr, Pat>>) : Brace<_> { items.0 }
    }
}
