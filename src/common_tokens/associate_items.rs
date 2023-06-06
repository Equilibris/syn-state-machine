use super::*;
use crate::*;
use std::fmt::Debug;

pub type AssociateItems<T, Ty> = Vec<AssociateItem<T, Ty>>;

pub enum AssociateItem<T: Parsable, Ty: Parsable> {
    MacroInvocation(Attrs<T>, MacroInvocationSemi),
    TypeAlias(Attrs<T>, Option<Visibility>, TypeAlias<T, Ty>),
    ConstantItem(Attrs<T>, Option<Visibility>, ConstantItem<Ty>),
    Function(Attrs<T>, Option<Visibility>, Function<T, Ty>),
}
impl<T: Parsable, Ty: Parsable> MappedParse for AssociateItem<T, Ty> {
    type Source = WithAttrs<
        T,
        Sum2<
            MacroInvocationSemi,
            (
                Option<Visibility>,
                Sum3<MBox<TypeAlias<T, Ty>>, MBox<ConstantItem<Ty>>, MBox<Function<T, Ty>>>,
            ),
        >,
    >;

    type Output = Self;
    type Error = SmErr<Self::Source>;

    fn map(
        src: SmOut<Self::Source>,
    ) -> Result<<Self as MappedParse>::Output, <Self as MappedParse>::Error> {
        Ok(match src.1 {
            Sum2::Val0(a) => Self::MacroInvocation(src.0, a),
            Sum2::Val1((vis, Sum3::Val0(a))) => Self::TypeAlias(src.0, vis, a),
            Sum2::Val1((vis, Sum3::Val1(a))) => Self::ConstantItem(src.0, vis, a),
            Sum2::Val1((vis, Sum3::Val2(a))) => Self::Function(src.0, vis, a),
        })
    }

    fn map_err(src: SmErr<Self::Source>) -> <Self as MappedParse>::Error {
        src
    }
}
impl<T: Parsable, Ty: Parsable> Debug for AssociateItem<T, Ty>
where
    SmOut<T>: Debug,
    SmOut<Ty>: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacroInvocation(arg0, arg1) => f
                .debug_tuple("MacroInvocation")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::TypeAlias(arg0, arg1, arg2) => f
                .debug_tuple("TypeAlias")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::ConstantItem(arg0, arg1, arg2) => f
                .debug_tuple("ConstantItem")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
            Self::Function(arg0, arg1, arg2) => f
                .debug_tuple("Function")
                .field(arg0)
                .field(arg1)
                .field(arg2)
                .finish(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::insta_match_test;
}
