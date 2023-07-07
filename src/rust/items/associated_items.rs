use crate::*;

materialize! {
    #[derive(Debug)]
    pub enum AssociateItem <Attr, Ty, Expr, Pat> [attrs <- Vec<OuterAttribute<Attr>>] {
        Macro(v <- MacroInvocationSemi)
        TypeAlias(vis <- Option<Visibility>; v <- TypeAlias<Attr,Ty>)
        ConstantItem(vis <- Option<Visibility>; v <- ConstantItem<Attr, Expr>)
        Function(vis <- Option<Visibility>; v <- Function<Attr, Ty, Expr, Pat>)
    }
}
