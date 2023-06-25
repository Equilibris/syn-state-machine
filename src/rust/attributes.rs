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
