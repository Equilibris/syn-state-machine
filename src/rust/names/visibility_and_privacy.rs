use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Visibility [ <- KwPub ] {
        Crate(<- Paren<KwCrate>),
        LSelf(<- Paren<KwLowerSelf>),
        Super(<- Paren<KwSuper>),

        In(v <- SimplePath : Paren<(KwIn, _)> {v.0.1} ),
        Pub()
    }
}
