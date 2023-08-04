use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct Enumeration <Attr, Ty, Expr> {
        <- KwEnum;
        id <- Ident : Identifier;
        generic_params <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- Rep<EnumItems<Attr, Ty, Expr>> : Brace<_> { items.0 };
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct Enumeration<Attr, Ty, Expr> {
        <- KwEnum;
        id <- Ident;
        generic_params <- Option<GenericParams<Attr, Ty>>;
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    items.into_token_stream()

                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    items.to_token_stream()
                )
            )
        }
    }
}

pub type EnumItems<Attr, Ty, Expr> = InterlaceTrail<EnumItem<Attr, Ty, Expr>, Comma>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum EnumItem <Attr, Ty, Expr> [
        attrs <- Rep<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        id <- Ident : Identifier
    ] {
        Tuple(v <- EnumItemTuple<Attr, Ty>; desc <- Option<EnumItemDiscriminant<Expr>>),
        Struct(v <- EnumItemStruct<Attr, Ty>; desc <- Option<EnumItemDiscriminant<Expr>>),
        Unit(desc <- Option<EnumItemDiscriminant<Expr>>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum EnumItem<Attr, Ty, Expr> [
        attrs <- Rep<OuterAttribute<Attr>>;
        vis <- Option<Visibility>;
        id <- Ident
    ] {
        Tuple(v <- EnumItemTuple<Attr, Ty>; desc <- Option<EnumItemDiscriminant<Expr>>),
        Struct(v <- EnumItemStruct<Attr, Ty>; desc <- Option<EnumItemDiscriminant<Expr>>),
        Unit(desc <- Option<EnumItemDiscriminant<Expr>>)
    }
}

pub type EnumItemTuple<Attr, Ty> = Paren<TupleFields<Attr, Ty>>;
pub type EnumItemStruct<Attr, Ty> = Brace<StructFields<Attr, Ty>>;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct EnumItemDiscriminant <Expr> {
        <- Eq;
        expr <- Expr
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct EnumItemDiscriminant<Expr> {
        <- Eq;
        expr <- Expr
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;

    insta_match_test!(parse : it_matches_enum_item_unit,   EnumItem<Infallible, Ident, Infallible>: Block);
    insta_match_test!(parse : it_matches_enum_item_struct, EnumItem<Infallible, Ident, Infallible>: Block { hello : World });
    insta_match_test!(parse : it_matches_enum_item_tuple,  EnumItem<Infallible, Ident, Infallible>: Block(World));

    insta_match_test!(
        parse : it_matches_enum, Enumeration <Infallible, TypePath<Ident>, Infallible>:
        enum HelloWorld <F,T> where {
            Unit,
            From(F),
            To { result: T },
        }
    );
}
