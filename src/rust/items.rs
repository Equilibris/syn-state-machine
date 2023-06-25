mod modules {
    use proc_macro2::Ident;

    use crate::{rust::attributes::InnerAttribute, *};

    #[derive(Debug)]
    pub enum Module<InnerAttr, Item> {
        Sourced(bool, Ident),
        Inline(bool, Ident, Vec<InnerAttribute<InnerAttr>>, Vec<Item>),
    }

    impl<InnerAttr: Parse, Item: Parse> Parse for Module<InnerAttr, Item> {
        fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
            let r#unsafe = input.peek::<KwUnsafe>();

            input.errored_peek::<KwMod>()?;

            let id = input.parse::<Identifier>()?;

            Ok(
                match input
                    .parse::<Sum2<Semi, Bracket<(Vec<InnerAttribute<InnerAttr>>, Vec<Item>)>>>()?
                {
                    Sum2::V0(_) => Self::Sourced(r#unsafe, id.0),
                    Sum2::V1(c) => Self::Inline(r#unsafe, id.0, c.0 .0, c.0 .1),
                },
            )
        }
    }
}
mod extern_crate {
    use proc_macro2::Ident;

    use crate::*;

    materialize! {
        #[derive(Debug)]
        pub struct ExternCrate {
            <- KwExtern;
            <- KwCrate;
            crate_ref <- CrateRef;
            as_clause <- AsClause;
        }
    }
    materialize! {
        #[derive(Debug)]
        pub struct AsClause {
            <- KwAs;
            id <- IdentifierOrUnder;
        }
    }

    #[derive(Debug)]
    pub struct CrateRef(pub Ident);
    impl Parse for CrateRef {
        fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
            Ok(Self(
                input
                    .ident_matching(|id| {
                        if id == "self" {
                            Ok(())
                        } else {
                            get_error_from_ident(id)
                        }
                    })?
                    .clone(),
            ))
        }
    }
}
mod generic_parameters {
    use crate::{rust::attributes::OuterAttribute, *};

    materialize! {
        #[derive(Debug)]
        pub struct GenericParams<Attr, Ty>{
            <- Lt;
            params <- InterlaceTrail<GenericParam<Attr, Ty>, Comma>;
            <- Gt;
        }
    }

    #[derive(Debug)]
    pub enum GenericParam<Attr, Ty> {
        Lt(Vec<OuterAttribute<Attr>>, LifetimeParam),
        Ty(Vec<OuterAttribute<Attr>>, TypeParam<Attr, Ty>),
        Cp(Vec<OuterAttribute<Attr>>, ConstParam<Ty>),
    }

    impl<Attr: Parse, Ty: Parse> Parse for GenericParam<Attr, Ty> {
        fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
            let attrs = input.parse()?;

            Ok(match input.parse::<Sum3<_, _, _>>()? {
                Sum3::V0(a) => Self::Lt(attrs, a),
                Sum3::V1(a) => Self::Ty(attrs, a),
                Sum3::V2(a) => Self::Cp(attrs, a),
            })
        }
    }

    materialize! {
        #[derive(Debug)]
        pub struct LifetimeParam {
            lt <- LifetimeOrLabel;
            bound <- Option<LifetimeBounds> : Option<(Colon, _)> {bound.map(|v|v.1)};
        }
    }

    materialize! {
        #[derive(Debug)]
        pub struct TypeParam<Attr, Ty> {
            id <- Identifier;
            bound <- Option<TypeParamBounds<Attr, Ty>>: Option<(Colon, Option<_>)> { bound.and_then(|v|v.1) };
            ty <- Option<Ty> : Option<(Eq, _)> { ty.map(|v|v.1) };
        }
    }
    materialize! {
        #[derive(Debug)]
        pub struct ConstParam<Ty> {
            <- KwConst;
            id <- Identifier;
            <- Colon;
            ty <- Ty;
            eq <- Option<Sum3<Infallible, Identifier, Literal>> : Option<(Eq, _)> {eq.map(|v|v.1)};
        }
    }

    // Where Clause

    materialize! {
        #[derive(Debug)]
        pub struct WhereClause<Attr, Ty>{
            <- KwWhere;
            content <- Interlace<WhereClauseItem<Attr, Ty>, Comma>;
        }
    }

    #[derive(Debug)]
    pub enum WhereClauseItem<Attr, Ty> {
        Lt(LifetimeWhereClauseItem),
        Ty(TypeBoundWhereClauseItem<Attr, Ty>),
    }

    impl<Attr: Parse, Ty: Parse> Parse for WhereClauseItem<Attr, Ty> {
        fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
            Ok(match input.parse::<Sum2<_, _>>()? {
                Sum2::V0(a) => Self::Lt(a),
                Sum2::V1(a) => Self::Ty(a),
            })
        }
    }

    materialize! {
        #[derive(Debug)]
        pub struct LifetimeWhereClauseItem {
            lt <- Lifetime;
            <- Colon;
            bound <- LifetimeBounds;
        }
    }

    materialize! {
        #[derive(Debug)]
        pub struct TypeBoundWhereClauseItem<Attr, Ty> {
            for_lts <- Option<ForLifetimes<Attr, Ty>>;
            ty <- Ty;
            bound <- Option<TypeParamBounds<Attr, Ty>>;
        }
    }
}

pub use extern_crate::*;
pub use generic_parameters::*;
pub use modules::*;
