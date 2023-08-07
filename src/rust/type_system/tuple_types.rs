use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

// pub type TupleType<Ty> = Paren<InterlaceTrail<Ty, Comma>>;
materialize! {
    on <'a> [RustCursor<'a>]
    #[derive(Debug)]
    pub struct TupleType<Ty> {
        elements <-
            InterlaceTrail<Ty, Comma> :
            Paren<Option<(MinLength<Rep<(_, PeekAsParse<Comma>)>>, Option<_>)>> {
            if let Some(elements) = elements.0 {
                InterlaceTrail::new(
                    elements
                    .0
                    .0
                    .0
                    .into_iter()
                    .map(|v|v.0)
                    .chain(elements.1)
                    .collect()
                    )
            } else {
                InterlaceTrail::default()
            }
        }
    }
}
to_tokens! {
    impl ToTokens for struct TupleType<Ty> {
        elements <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    elements.into_token_stream()
                )
            );
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Parenthesis,
                    elements.to_token_stream()
                )
            );
        }
    }
}
