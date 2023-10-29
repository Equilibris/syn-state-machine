pub use crate::internals::*;

pub struct Except<Value, Not> {
    pub value: Value,
    phantom: std::marker::PhantomData<Not>,
}

impl<Value, Not> Except<Value, Not> {
    pub fn new(value: Value) -> Self {
        Self {
            value,
            phantom: std::marker::PhantomData,
        }
    }
}

impl<C: ParserCursor, Value: Peek<C>, Not: Peek<C>> Peek<C> for Except<Value, Not> {
    fn peek(input: &C) -> Option<usize> {
        let value = Value::peek(input);
        let not = Not::peek(input);

        match (value, not) {
            (Some(v), None) => Some(v),
            _ => None,
        }
    }
}

pub struct ExceptFin<Fin>(Fin);

impl<Out, With, Fin: Finalizer<Out, With>, Not> Finalizer<Except<Out, Not>, With>
    for ExceptFin<Fin>
{
    fn finalize(self, value: With) -> std::ops::ControlFlow<Except<Out, Not>, Except<Out, Not>> {
        self.0
            .finalize(value)
            .map_break(Except::new)
            .map_continue(Except::new)
    }
}

impl<C: ParserCursor + Spanned, With, Value: Parse<C, With>, Not: Parse<C, With>> Parse<C, With>
    for Except<Value, Not>
where
    C: Clone,
    C::Error: for<'a> From<LocError<'a, C::Loc>>,
{
    type Finalizer = ExceptFin<<Value as Parse<C, With>>::Finalizer>;

    fn parse(input: &mut ParseBuffer<C>) -> Result<Self::Finalizer, <C as ParserCursor>::Error> {
        let mut checkpoint = input.clone();

        let value = Value::parse(&mut checkpoint)?;

        let mut negation_checkpoint = input.clone();
        let not = Not::parse(&mut negation_checkpoint);

        if not.is_err() {
            Ok(ExceptFin(value))
        } else {
            let left = input.span();

            Err(LocError("Got a value that has been excepted", left).into())
        }
    }
}

impl<Value: quote::ToTokens, Not> quote::ToTokens for Except<Value, Not> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.value.to_tokens(tokens)
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.value.to_token_stream()
    }

    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        self.value.into_token_stream()
    }
}
