use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct TypeAlias<Attr, Ty> {
        <- KwType;
        id <- Ident : Identifier;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        bounds <- Option<TypeParamBounds<Attr, Ty>> : Option<(Colon, _)> { bounds.map(|v|v.1) };
        where_clause <- Option<WhereClause<Attr, Ty>>;
        eq <- Option<(Ty, Option<WhereClause<Attr, Ty>>)> : Option<(Eq, _)> { eq.map(|v|v.1) };
        <- Semi
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct TypeAlias <Attr,Ty> {
        <- KwType;
        id <- Ident;
        generic_parameters <- Option<GenericParams<Attr, Ty>>;
        bounds <- tokens into {
            if let Some(bounds) = bounds {
                tokens.extend(KwIn::default().into_token_stream());
                tokens.extend(bounds.into_token_stream());
            }
        } to {
            if let Some(bounds) = bounds {
                tokens.extend(KwIn::default().into_token_stream());
                bounds.to_tokens(tokens)
            }
        };
        where_clause <- Option<WhereClause<Attr, Ty>>;
        eq <- tokens into {
            if let Some((ty, bound)) = eq {
                tokens.extend(Eq::default().into_token_stream());
                tokens.extend(ty.into_token_stream());

                if let Some(bound) = bound {
                    tokens.extend(bound.into_token_stream())
                }
            }
        } to {
            if let Some((ty, bound)) = eq {
                tokens.extend(Eq::default().into_token_stream());
                ty.to_tokens(tokens);

                if let Some(bound) = bound {
                    bound.to_tokens(tokens)
                }
            }
        };
        <- Semi
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    insta_match_test!(parse : it_matches_simple,    TypeAlias<Infallible, Type<Infallible>>: type Point;);
    insta_match_test!(parse : it_matches_simple_eq, TypeAlias<Infallible, Type<Infallible>>: type Point = (u8, u8););
    insta_match_test!(parse : it_matches_complex,   TypeAlias<Infallible, Type<Infallible>>: type Point<T> where T: std::ops::Add<T> = (T, T););
    insta_match_test!(parse : it_matches_complex_eq,TypeAlias<Infallible, Type<Infallible>>: type Point<T> where T: std::ops::Add<T> = (T, T););
}
