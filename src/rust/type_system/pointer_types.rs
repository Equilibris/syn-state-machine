use crate::*;

materialize! {
    #[derive(Debug)]
    pub struct ReferenceType<Ty> {
        <- And;
        lt <- Option<Lifetime>;
        r#mut peek <- KwMut;
        ty <- Ty;
    }
}

materialize! {
    #[derive(Debug)]
    pub enum RawPointerType <Ty> [ <- Star ] {
        Mut(<- KwMut; v <- Ty)
        Const(<- KwConst; v <- Ty)
    }
}
