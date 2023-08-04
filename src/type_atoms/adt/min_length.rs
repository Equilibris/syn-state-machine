use crate::{
    BlackHoleFinalizer, Interlace, InterlaceTrail, LocError, Parse, ParseBuffer, ParserCursor, Rep,
    Spanned,
};

#[allow(clippy::len_without_is_empty)]
pub trait ParsableLength {
    fn len(&self) -> usize;
}

impl<T> ParsableLength for Rep<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<A, B> ParsableLength for Interlace<A, B> {
    fn len(&self) -> usize {
        self.values.len()
    }
}
impl<A, B> ParsableLength for InterlaceTrail<A, B> {
    fn len(&self) -> usize {
        self.values.len()
    }
}
impl<T: ParsableLength> ParsableLength for MinLength<T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}
impl<T: ParsableLength> ParsableLength for Box<T> {
    fn len(&self) -> usize {
        self.as_ref().len()
    }
}

#[derive(Debug, Clone)]
pub struct MinLength<T, const LEN: usize = 1>(pub T);
impl<Cursor: ParserCursor + Spanned, const LEN: usize, T: Parse<Cursor, ()> + ParsableLength>
    Parse<Cursor, ()> for MinLength<T, LEN>
where
    Cursor::Error: for<'a> From<LocError<'a, Cursor::Loc>>,
{
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
        let c: T = input.parse()?;
        if c.len() < LEN {
            Err(LocError("Expected value", input.span()).into())
        } else {
            Ok(BlackHoleFinalizer(Self(c)))
        }
    }
}

#[cfg(feature = "printing")]
impl<T: quote::ToTokens> quote::ToTokens for MinLength<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.0.to_tokens(tokens)
    }

    fn to_token_stream(&self) -> proc_macro2::TokenStream {
        self.0.to_token_stream()
    }

    fn into_token_stream(self) -> proc_macro2::TokenStream
    where
        Self: Sized,
    {
        self.0.into_token_stream()
    }
}
