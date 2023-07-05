use crate::*;

materialize! {
    pub struct Enumeration <Attr, Ty, Expr> {
        <- KwEnum;
        id <- Identifier;
        generic_params <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- Vec<EnumItems<Attr, Ty, Expr>> : Brace<_> { items.0 };
    }
}

pub type EnumItems<Attr, Ty, Expr> = InterlaceTrail<EnumItem<Attr, Ty, Expr>, Comma>;

materialize! {
    pub enum EnumItem <Attr, Ty, Expr> [
        attrs <- Vec<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        id <- Identifier
    ] {
        Tuple(v <- EnumItemTuple<Attr, Ty>; desc <- Option<EnumItemDiscriminant<Expr>>)
        Struct(v <- EnumItemStruct<Attr, Ty>; desc <- Option<EnumItemDiscriminant<Expr>>)
    }
}
pub type EnumItemTuple<Attr, Ty> = Paren<TupleFields<Attr, Ty>>;
pub type EnumItemStruct<Attr, Ty> = Brace<StructFields<Attr, Ty>>;

materialize! {
    pub struct EnumItemDiscriminant <Expr> {
        <- Eq;
        expr <- Expr
    }
}
