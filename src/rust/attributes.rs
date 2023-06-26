use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct InnerAttribute<T>{
        <- Pound;
        <- Not;
        content <- T : Bracket<T> {content.0};
    }
}
materialize! {
    #[derive(Debug)]
    pub struct OuterAttribute<T>{
        <- Pound;
        content <- T : Bracket<T> {content.0};
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    insta_match_test!(+it_matches_simple_function, OuterAttribute<(Ident, Paren<Ident>)>: #[hello(world)]);
}
