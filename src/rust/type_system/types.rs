use crate::*;

materialize! {
    on <'a> [crate::RustCursor<'a>]
    #[derive(Debug)]
    pub enum Type<Attr> {
        ImplTrait(v <- ImplTraitType<Attr, Box<Self>>),
        TraitObject(v <- TraitObjectType<Attr, Box<Self>>),
        NoBounds(v <- TypeNoBounds<Attr, Box<Self>>)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum Type<Attr> {
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
        BareFunctionType(v <- BareFunctionType<Attr, Ty, Box<Self>>),
        TypePath(v <- TypePath<Ty>),
        TupleType(v <- TupleType<Ty>),
        NeverType(v <- NeverType),
        RawPointerType(v <- RawPointerType<Ty>),
        ReferenceType(v <- ReferenceType<Ty>),
        ArrayType(v <- ArrayType<Attr, Ty>),
        SliceType(v <- SliceType<Ty>),
        InferredType(v <- InferredType),
        QualifiedPathInType(v <- QualifiedPathInType<Ty>),
        MacroInvocation(v <- MacroInvocation)
    }
}
#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for enum TypeNoBounds<Attr, Ty> {
        ParenthesizedType(v <- ParenthesizedType<Ty>),
        ImplTraitTypeOneBound(v <- ImplTraitTypeOneBound<Attr, Ty>),
        TraitObjectTypeOneBound(v <- TraitObjectTypeOneBound<Attr, Ty>),
        BareFunctionType(v <- BareFunctionType<Attr, Ty, Box<Self>>),
        TypePath(v <- TypePath<Ty>),
        TupleType(v <- TupleType<Ty>),
        NeverType(v <- NeverType),
        RawPointerType(v <- RawPointerType<Ty>),
        ReferenceType(v <- ReferenceType<Ty>),
        ArrayType(v <- ArrayType<Attr, Ty>),
        SliceType(v <- SliceType<Ty>),
        InferredType(v <- InferredType),
        QualifiedPathInType(v <- QualifiedPathInType<Ty>),
        MacroInvocation(v <- MacroInvocation)
    }
}

pub type ParenthesizedType<T> = Paren<T>;

#[cfg(test)]
mod tests {
    use crate::*;

    // insta_match_test!(parse print : it_matches_array,           Type<P<Infallible>>: [i32; 5]);
    insta_match_test!(parse print : it_matches_primitive,       Type<P<Infallible>>: u32);
    insta_match_test!(parse print : it_matches_reference,       Type<P<Infallible>>: &str);
    insta_match_test!(parse print : it_matches_tuple,           Type<P<Infallible>>: (i32, f64));
    insta_match_test!(parse print : it_matches_slice,           Type<P<Infallible>>: &[u8]);
    insta_match_test!(parse print : it_matches_function,        Type<P<Infallible>>: fn(i32) -> i32);
    insta_match_test!(parse print : it_matches_option,          Type<P<Infallible>>: Option<i32>);
    insta_match_test!(parse print : it_matches_result,          Type<P<Infallible>>: Result<i32, String>);
    insta_match_test!(parse print : it_matches_generic_struct,  Type<P<Infallible>>: Vec<i32>);
    insta_match_test!(parse print : it_matches_generic_trait,   Type<P<Infallible>>: Box<dyn MyTrait>);
    insta_match_test!(parse print : it_matches_closure,         Type<P<Infallible>>: &dyn Fn(i32) -> i32);
    insta_match_test!(parse print : it_matches_nested_generics, Type<P<Infallible>>: Result<Option<i32>, String>);
    insta_match_test!(parse print : it_matches_generic_enum,    Type<P<Infallible>>: Option<Result<i32, String>>);
    // insta_match_test!(parse print : it_matches_array_of_tuples, Type<P<Infallible>>: [(i32, f64); 3]);
    insta_match_test!(parse print : it_matches_generic_trait_2, Type<P<Infallible>>: Box<dyn MyTrait + Send>);
    insta_match_test!(parse print : it_matches_unit_type,       Type<P<Infallible>>: ());
    insta_match_test!(parse print : it_matches_fn_with_lt,      Type<P<Infallible>>: for<'a> fn(&'a str) -> &'a str);
    insta_match_test!(parse print : it_matches_tuple_with_lt,   Type<P<Infallible>>: (&'a str, &'a str));
    insta_match_test!(parse print : it_matches_generic_tuple,   Type<P<Infallible>>: (i32, Option<String>, f64));
}
