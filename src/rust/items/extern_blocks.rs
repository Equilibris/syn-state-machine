use crate::*;

materialize! {
    pub struct ExternBlock <Attr, Ty, Expr, Pat> {
        r#unsafe peek <- KwUnsafe;
        <- KwExtern;
        abi <- Option<Abi>;
        items <- (Vec<InnerAttribute<Attr>>, Vec<ExternalItem<Attr, Ty, Expr, Pat>>) : Brace<_> { items.0 }
    }
}

materialize! {
    pub enum ExternalItem<Attr, Ty, Expr, Pat> [attrs <- Vec<OuterAttribute<Attr>> ] {
        Macro(v <- MacroInvocationSemi)
        Static(vis <- Option<Visibility>; v <- StaticItem<Ty, Expr>)
        Function(vis <- Option<Visibility>; v <- Function<Attr, Ty, Expr, Pat>)
    }
}
