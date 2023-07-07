use crate::*;

materialize! {
    #[derive(Debug)]
    pub enum Struct<Attr, Ty>{
        StructStruct(v <- StructStruct<Attr, Ty>)
        TupleStruct(v <- TupleStruct<Attr, Ty>)
    }
}

materialize! {
    #[derive(Debug)]
    pub struct StructStruct<Attr, Ty> {
        <- KwStruct;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        fields <- Option<Brace<StructFields<Attr, Ty>>> : Sum2<_, Semi> { if let Sum2::V0(v) = fields { Some(v) } else { None } }
    }
}

materialize! {
    #[derive(Debug)]
    pub struct TupleStruct<Attr, Ty> {
        <- KwStruct;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        fields <- Paren<TupleFields<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        <- Semi
    }
}

pub type StructFields<Attr, Ty> = Interlace<StructField<Attr, Ty>, Comma>;
pub type TupleFields<Attr, Ty> = Interlace<TupleField<Attr, Ty>, Comma>;

materialize! {
    #[derive(Debug)]
    pub struct StructField <Attr, Ty> {
        attrs <- Vec<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        id <- Ident : Identifier;
        <- Colon;
        ty <- Ty
    }
}

materialize! {
    #[derive(Debug)]
    pub struct TupleField <Attr, Ty> {
        attrs <- Vec<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        ty <- Ty
    }
}
