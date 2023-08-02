use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

// TODO
materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ArrayType <Ty, Expr> {
        inner <- (Ty, Expr) : Bracket<(Ty, Semi, Expr)> { (inner.0.0, inner.0.2) }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ArrayType<Ty, Expr> {
        inner <- tokens into {
            let mut stream = inner.0.into_token_stream();

            stream.extend(Semi::default().into_token_stream());
            stream.extend(inner.1.into_token_stream());

            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    stream
                )
            );
        } to {
            let mut stream = inner.0.to_token_stream();

            stream.extend(Semi::default().into_token_stream());
            inner.0.to_tokens(&mut stream);

            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    stream
                )
            );
        }
    }
}
