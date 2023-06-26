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
