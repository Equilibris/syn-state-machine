use crate::*;

materialize! {
    on <'a> [RustCursor<'a>]
    pub enum PathExpression<Ty> {
        PathInExpression(v <- PathInExpression<Ty>),
        QualifiedPathInExpression(v <- QualifiedPathInExpression<Ty>)
    }
}
to_tokens! {
    impl ToTokens for enum PathExpression<Ty> {
        PathInExpression(v <- PathInExpression<Ty>),
        QualifiedPathInExpression(v <- QualifiedPathInExpression<Ty>)
    }
}
