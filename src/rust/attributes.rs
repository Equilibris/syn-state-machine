use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct InnerAttribute<T>{
        <- Pound;
        <- Not;
        content <- T : Bracket<T> {content.0};
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct InnerAttribute<T> {
        <- Pound;
        <- Not;
        content <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Bracket,
                    content.into_token_stream()
                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Bracket,
                    content.to_token_stream()
                )
            )
        }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct OuterAttribute<T>{
        <- Pound;
        content <- T : Bracket<T> {content.0};
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct OuterAttribute<T> {
        <- Pound;
        content <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Bracket,
                    content.into_token_stream()
                )
            )
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Bracket,
                    content.to_token_stream()
                )
            )
        }
    }
}

pub type WithOuterAttrs<Attr, Ty> = P<(Rep<OuterAttribute<Attr>>, Ty)>;
pub type WithInnerAttrs<Attr, Ty> = P<(Rep<InnerAttribute<Attr>>, Ty)>;

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(parse print : it_matches_simple_function, OuterAttribute<P<(Ident, Paren<Ident>)>>: #[hello(world)]);
}
