use crate::*;

#[derive(Debug)]
pub struct InnerAttribute<T>(T);

impl<T: Parse> Parse for InnerAttribute<T> {
    fn parse<'a>(input: &mut crate::ParseBuffer<'a>) -> crate::Result<Self> {
        input.errored_peek::<Pound>()?;
        input.errored_peek::<Not>()?;

        Ok(Self(input.parse::<Bracket<T>>()?.0))
    }
}

#[derive(Debug)]
pub struct OuterAttribute<T>(T);

impl<T: Parse> Parse for OuterAttribute<T> {
    fn parse<'a>(input: &mut crate::ParseBuffer<'a>) -> crate::Result<Self> {
        input.errored_peek::<Pound>()?;

        Ok(Self(input.parse::<Bracket<T>>()?.0))
    }
}
