use crate::internals::*;
#[cfg(feature = "printing")]
use crate::to_tokens;

#[derive(Debug, Clone)]
pub struct LRk<T> {
    pub value: T,
}

#[cfg(feature = "printing")]
to_tokens! {
    impl ToTokens for struct LRk<T> {
        value <- T
    }
}

impl<C: ParserCursor, T: Parse<C, ()> + Parse<C, T>> Parse<C, ()> for LRk<T> {
    type Finalizer = BlackHoleFinalizer<Self>;

    fn parse(input: &mut ParseBuffer<C>) -> Result<Self::Finalizer, <C as ParserCursor>::Error> {
        let mut value = <T as Parse<C, ()>>::parse(input)?.finalize(());

        use std::ops::ControlFlow::*;

        Ok(BlackHoleFinalizer(loop {
            match value {
                Continue(v) => value = <T as Parse<C, T>>::parse(input)?.finalize(v),
                Break(value) => break Self { value },
            }
        }))
    }
}
