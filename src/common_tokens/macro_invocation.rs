use super::*;
use crate::*;

#[derive(Debug)]
pub struct MacroInvocation {
    pub path: SimplePath,
    pub content: TokenTree,
}
impl MappedParse for MacroInvocation {
    type Source = (SimplePath, Exclamation, TokenTree);

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            path: src.0,
            content: src.2,
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}

#[derive(Debug)]
pub struct MacroInvocationSemi {
    pub path: SimplePath,
    pub content: Tokens,
}
impl MappedParse for MacroInvocationSemi {
    type Source = (
        SimplePath,
        Sum2<(Sum2<Paren<Tokens>, Bracket<Tokens>>, Semi), Brace<Tokens>>,
    );

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(Self {
            path: src.0,
            content: match src.1 {
                Sum2::Val0(a) => match a.0 {
                    Sum2::Val0(Paren(a)) | Sum2::Val1(Bracket(a)) => a,
                },
                Sum2::Val1(Brace(a)) => a,
            },
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
