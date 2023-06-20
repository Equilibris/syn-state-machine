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

    pub struct ExternCrate(pub CrateRef, pub Option<AsClause>);
    impl Parse for ExternCrate {
        fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
            input.errored_peek::<KwExtern>()?;
            input.errored_peek::<KwCrate>()?;

            Ok(Self(input.parse()?, input.parse()?))
        }
    }

    #[derive(Debug)]
    pub struct AsClause(pub IdentifierOrUnder);
    impl Parse for AsClause {
        fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
            input.errored_peek::<KwAs>()?;

            Ok(Self(input.parse()?))
        }
    }

    #[derive(Debug)]
    pub struct CrateRef(pub Ident);
    impl Parse for CrateRef {
        fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
            input.errored_peek::<KwAs>()?;

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
mod names {
    mod paths {
        use crate::*;

        pub enum SimplePathSegment {
            Identifier(Identifier),
            Super(KwSuper),
            SSelf(KwLowerSelf),
            Crate(KwCrate),
            MacroCrate((Dollar, KwCrate)),
        }
    }
    mod visibility_and_privacy {}

    pub use paths::*;
    pub use visibility_and_privacy::*;
}

pub use extern_crate::*;
pub use modules::*;
pub use names::*;
