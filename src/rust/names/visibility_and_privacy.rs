use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Visibility [ <- KwPub ] {
        Crate(<- Paren<KwCrate>),
        LSelf(<- Paren<KwLowerSelf>),
        Super(<- Paren<KwSuper>),

        In(v <- SimplePath : Paren<(KwIn, _)> {v.0.1} ),
        Pub()
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    #[cfg(feature = "printing")]
    impl ToTokens for enum Visibility {
        Crate(<- Paren<KwCrate>),
        LSelf(<- Paren<KwLowerSelf>),
        Super(<- Paren<KwSuper>),

        In(v <- tokens into {
            let mut inner = KwIn::default().into_token_stream();
            inner.extend(v.into_token_stream());
            tokens.append(proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, inner));
        } to {
            let mut inner = KwIn::default().into_token_stream();
            v.to_tokens(&mut inner);
            tokens.append(proc_macro2::Group::new(proc_macro2::Delimiter::Parenthesis, inner));
        }),
        Pub()
    }
}
