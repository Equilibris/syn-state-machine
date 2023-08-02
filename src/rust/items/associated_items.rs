use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum AssociateItem <Attr, Ty, Expr, Pat> [attrs <- Rep<OuterAttribute<Attr>>] {
        Macro(v <- MacroInvocationSemi),
        TypeAlias(vis <- Option<Visibility>; v <- TypeAlias<Attr,Ty>),
        ConstantItem(vis <- Option<Visibility>; v <- ConstantItem<Attr, Expr>),
        Function(vis <- Option<Visibility>; v <- Function<Attr, Ty, Expr, Pat>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum AssociateItem<Attr, Ty, Expr, Pat> [attrs <- Rep<OuterAttribute<Attr>>] {
        Macro(v <- MacroInvocationSemi),
        TypeAlias(vis <- Option<Visibility>; v <- TypeAlias<Attr,Ty>),
        ConstantItem(vis <- Option<Visibility>; v <- ConstantItem<Attr, Expr>),
        Function(vis <- Option<Visibility>; v <- Function<Attr, Ty, Expr, Pat>)
    }
}
