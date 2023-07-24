use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ExternBlock <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwExtern;
        abi <- Option<Abi>;
        items <- WithInnerAttrs< Attr, Vec<ExternalItem<Attr, Ty, Expr, Pat>> > : Brace<_> { items.0 }
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum ExternalItem<Attr, Ty, Expr, Pat> [attrs <- Vec<OuterAttribute<Attr>> ] {
        Macro(v <- MacroInvocationSemi),
        Static(vis <- Option<Visibility>; v <- StaticItem<Ty, Expr>),
        Function(vis <- Option<Visibility>; v <- Function<Attr, Ty, Expr, Pat>)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;
    use std::convert::Infallible;

    insta_match_test!(+it_matches_item, ExternalItem<Infallible, Type<Infallible>, Infallible, Ident>:
        fn with_name(format: *const u8);
    );

    insta_match_test!(+it_matches_simple_extern_block, ExternBlock<Infallible, Type<Infallible>, Infallible, Ident>:
    extern "C" {
        fn with_name(format: *const u8);
    } );
}
