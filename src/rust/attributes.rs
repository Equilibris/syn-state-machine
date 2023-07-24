use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct InnerAttribute<T>{
        <- Pound;
        <- Not;
        content <- T : Bracket<T> {content.0};
    }
}
materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct OuterAttribute<T>{
        <- Pound;
        content <- T : Bracket<T> {content.0};
    }
}

pub type WithOuterAttrs<Attr, Ty> = (Vec<OuterAttribute<Attr>>, Ty);
pub type WithInnerAttrs<Attr, Ty> = (Vec<InnerAttribute<Attr>>, Ty);

#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(+it_matches_simple_function, OuterAttribute<(Ident, Paren<Ident>)>: #[hello(world)]);
}
