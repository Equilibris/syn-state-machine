use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Type <Attr> {
        ImplTrait(v <- ImplTraitType<Attr, Box<Self>>),
        TraitObject(v <- TraitObjectType<Attr, Box<Self>>),
        NoBounds(v <- TypeNoBounds<Attr, Box<Self>>)
    }
}
// TODO: correct ordering
materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum TypeNoBounds <Attr, Ty> {
        ParenthesizedType(v <- ParenthesizedType<Ty>),
        ImplTraitTypeOneBound(v <- ImplTraitTypeOneBound<Attr, Ty>),
        TraitObjectTypeOneBound(v <- TraitObjectTypeOneBound<Attr, Ty>),
        TypePath(v <- TypePath<Ty>),
        TupleType(v <- TupleType<Ty>),
        NeverType(v <- NeverType),
        RawPointerType(v <- RawPointerType<Ty>),
        ReferenceType(v <- ReferenceType<Ty>),
        ArrayType(v <- ArrayType<Attr, Ty>),
        SliceType(v <- SliceType<Ty>),
        InferredType(v <- InferredType),
        QualifiedPathInType(v <- QualifiedPathInType<Ty>),
        BareFunctionType(v <- BareFunctionType<Attr, Ty, Box<Self>>),
        MacroInvocation(v <- MacroInvocation)
    }
}

pub type ParenthesizedType<T> = Paren<T>;
