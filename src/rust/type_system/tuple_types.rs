use crate::*;

pub type TupleType<Ty> = Paren<InterlaceTrail<Ty, Comma>>;
