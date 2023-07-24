use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub struct ReferenceType<Ty> {
        <- And;
        lt <- Option<Lifetime>;
        r#mut peek <- KwMut;
        ty <- Ty;
    }
}

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum RawPointerType <Ty> [ <- Star ] {
        Mut(<- KwMut; v <- Ty),
        Const(<- KwConst; v <- Ty)
    }
}
