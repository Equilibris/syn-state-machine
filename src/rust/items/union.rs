use crate::*;

materialize! {
    pub struct Union <Attr, Ty> {
        <- KwUnion;
        id <- Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        fields <- StructFields<Attr, Ty> : Brace<_> { fields.0 }
    }
}
