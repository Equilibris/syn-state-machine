use crate::*;

materialize! {
    pub enum Struct<Attr, Ty>{
        StructStruct(v <- StructStruct<Attr, Ty>)
        TupleStruct(v <- TupleStruct<Attr, Ty>)
    }
}

materialize! {
    pub struct StructStruct<Attr, Ty> {
        <- KwStruct;
        id <- Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        fields <- Option<Brace<StructFields<Attr, Ty>>> : Sum2<_, Semi> { if let Sum2::V0(v) = fields { Some(v) } else { None } }
    }
}

materialize! {
    pub struct TupleStruct<Attr, Ty> {
        <- KwStruct;
        id <- Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        fields <- Paren<TupleFields<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        <- Semi
    }
}

pub type StructFields<Attr, Ty> = Interlace<StructField<Attr, Ty>, Comma>;
pub type TupleFields<Attr, Ty> = Interlace<TupleField<Attr, Ty>, Comma>;

materialize! {
    pub struct StructField <Attr, Ty> {
        attrs <- Vec<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        id <- Identifier;
        <- Colon;
        ty <- Ty
    }
}

materialize! {
    pub struct TupleField <Attr, Ty> {
        attrs <- Vec<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        ty <- Ty
    }
}
