use crate::*;
#[cfg(feature = "printing")]
use quote::TokenStreamExt;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ExternBlock <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwExtern;
        abi <- Option<Abi>;
        items <- WithInnerAttrs<Attr, Rep<ExternalItem<Attr, Ty, Expr, Pat>> > : Brace<_> { items.0 }
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct ExternBlock<Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwExtern;
        abi <- tokens into {
            if let Some(abi) = abi {
                tokens.append(proc_macro2::Literal::from(abi))
            }
        } to {
            if let Some(abi) = abi {
                tokens.append(proc_macro2::Literal::from(abi.clone()))
            }
        };
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
                    items.into_token_stream()
                )
            )
        }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum ExternalItem<Attr, Ty, Expr, Pat> [attrs <- Rep<OuterAttribute<Attr>> ] {
        Macro(v <- MacroInvocationSemi),
        Static(vis <- Option<Visibility>; v <- StaticItem<Ty, Expr>),
        Function(vis <- Option<Visibility>; v <- Function<Attr, Ty, Expr, Pat>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum ExternalItem<Attr, Ty, Expr, Pat> [attrs <- Rep<OuterAttribute<Attr>> ] {
        Macro(v <- MacroInvocationSemi),
        Static(vis <- Option<Visibility>; v <- StaticItem<Ty, Expr>),
        Function(vis <- Option<Visibility>; v <- Function<Attr, Ty, Expr, Pat>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::Infallible;

    insta_match_test!(+it_matches_item, ExternalItem<Infallible, Type<Infallible>, Infallible, Ident>:
        fn with_name(format: *const u8);
    );

    insta_match_test!(+it_matches_simple_extern_block, ExternBlock<Infallible, Type<Infallible>, Infallible, Ident>:
    extern "C" {
        fn with_name(format: *const u8);
    } );
}
