use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct MacroInvocation {
        path <- SimplePath;
        <- Not;
        tt <- DelimTokenTree;
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct MacroInvocation {
        path <- SimplePath;
        <- Not;
        tt <- DelimTokenTree
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct DelimTokenTree {
        stream <- TokenStream : Sum3<Paren<_>,Brace<_>,Bracket<_>> {
            match stream {
                Sum3::V0(a) => a.0,
                Sum3::V1(a) => a.0,
                Sum3::V2(a) => a.0
            }
        }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct DelimTokenTree {
        stream <- tokens into {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    stream
                )
            );
        } to {
            tokens.append(
                proc_macro2::Group::new(
                    proc_macro2::Delimiter::Brace,
                    stream.clone()
                )
            );
        }
    }
}

type MacroInvocationContent = Sum3<
    (Paren<TokenStream>, PeekAsParse<Semi>),
    (Bracket<TokenStream>, PeekAsParse<Semi>),
    Brace<TokenStream>,
>;
materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct MacroInvocationSemi {
        path <- SimplePath;
        <- Not;
        stream <- TokenStream : MacroInvocationContent {
            match stream {
                Sum3::V0(a) => a.0.0,
                Sum3::V1(a) => a.0.0,
                Sum3::V2(a) => a.0
            }
        }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct MacroInvocationSemi {
        path <- SimplePath;
        <- Not;
        stream <- tokens into {
            tokens.append(proc_macro2::Group::new(proc_macro2::Delimiter::Brace, stream));
        } to {
            tokens.append(proc_macro2::Group::new(proc_macro2::Delimiter::Brace, stream.clone()));
        }
    }
}
