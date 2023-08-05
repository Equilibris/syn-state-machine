use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Struct<Attr, Ty>{
        StructStruct(v <- StructStruct<Attr, Ty>),
        TupleStruct(v <- TupleStruct<Attr, Ty>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum Struct<Attr, Ty> {
        StructStruct(v <- StructStruct<Attr, Ty>),
        TupleStruct(v <- TupleStruct<Attr, Ty>)
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct StructStruct<Attr, Ty> {
        <- KwStruct;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        fields <- Option<Brace<StructFields<Attr, Ty>>> : Sum2<_, Semi> { if let Sum2::V0(v) = fields { Some(v) } else { None } }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct StructStruct<Attr, Ty> {
        <- KwStruct;
        id <- Ident;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        fields <- tokens into {
            if let Some(fields) = fields {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        fields.into_token_stream()
                    )
                )
            } else {
                tokens.extend(Semi::default().into_token_stream());
            }
        } to {
            if let Some(fields) = fields {
                tokens.append(
                    proc_macro2::Group::new(
                        proc_macro2::Delimiter::Brace,
                        fields.to_token_stream()
                    )
                )
            } else {
                tokens.extend(Semi::default().into_token_stream());
            }
        }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
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
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TupleStruct<Attr, Ty> {
        <- KwStruct;
        id <- Ident;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        fields <- Paren<TupleFields<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        <- Semi
    }
}

pub type StructFields<Attr, Ty> = InterlaceTrail<StructField<Attr, Ty>, Comma>;
pub type TupleFields<Attr, Ty> = InterlaceTrail<TupleField<Attr, Ty>, Comma>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct StructField <Attr, Ty> {
        attrs <- Rep<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        id <- Ident : Identifier;
        <- Colon;
        ty <- Ty
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct StructField<Attr, Ty> {
        attrs <- Rep<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        id <- Ident;
        <- Colon;
        ty <- Ty
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TupleField <Attr, Ty> {
        attrs <- Rep<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        ty <- Ty
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TupleField<Attr, Ty> {
        attrs <- Rep<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        ty <- Ty
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    // TODO:
    insta_match_test! {
        parse : it_matches_struct_struct, Struct<P<Infallible>, Ident> :
        struct Hello {
            pub hi: There,
            pub(crate) hello: World
        }
    }
    insta_match_test! {
        parse print : it_matches_tuple_struct, Struct<P<Infallible>, Ident> :
        struct Hello (Hello, pub World);
    }
    insta_match_test! {
        parse print : it_matches_unit, Struct<P<Infallible>, Ident> :
        struct Hello;
    }
}
