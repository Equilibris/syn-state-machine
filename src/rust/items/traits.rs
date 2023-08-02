use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct Trait <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwTrait;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        bounds <- Option<TypeParamBound<Attr, Ty>> : Option<(Colon, Option<_>)> { bounds.and_then(|v| v.1) };
        where_clause <- Option<WhereClause<Attr, Ty>>;
        items <- WithInnerAttrs<Attr, Rep<AssociateItem<Attr, Ty, Expr, Pat>> > : Brace<_> { items.0 }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct Trait<Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwTrait;
        id <- Ident;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        bounds <- tokens into {
            if let Some(bounds) = bounds {
                tokens.extend(Colon::default().into_token_stream());
                tokens.extend(bounds.into_token_stream());
            }
        } to {
            if let Some(bounds) = bounds {
                tokens.extend(Colon::default().into_token_stream());
                bounds.to_tokens(tokens)
            }
        };
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

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(
        +it_matches_trait, Trait <Infallible, Infallible, Type<Infallible>, Infallible>:
        unsafe trait HelloWorld<T> : From<T> T: Sized {
            type Hello: World;
        }
    );
}
