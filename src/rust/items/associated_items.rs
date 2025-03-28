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

#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(
        parse print : it_matches_type_alias, AssociateItem<P<Infallible>, Type<P<Infallible>>, P<Infallible>, P<Infallible>> :
        type Point<T> where T: std::ops::Add<T> = (T, T);
    );
    insta_match_test!(
        parse print : it_matches_function, AssociateItem<P<Infallible>, Ident, P<Infallible>, Ident> :
        const async unsafe extern "C" fn hello<T>(self, v: i64) -> T;
    );
}
