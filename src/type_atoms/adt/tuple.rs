use crate::internals::*;

macro_rules! tuple_impl {
    () => {
        impl Parse for () {
            fn parse<'a>(_: &mut ParseBuffer<'a>) -> Result<Self> {
                Ok(())
            }
        }
        impl Peek for () {
            fn peek<'a>(_: Cursor<'a>) -> Option<usize> {
                Some(0)
            }
        }

        impl FixedPeek for () {
            const SKIP: usize = 0;
        }
        // Do not implement
        // impl PeekError for () {
        //     fn error<'a>(input: &ParseBuffer<'a>) -> Error {
        //         panic!("Atemted to fail infallable peek")
        //         // Error::new(input.span(), "Should never be reached")
        //     }
        // }
    };
    ($last_gen:ident $($gen:ident)*) => {
        impl<$last_gen: Parse, $($gen: Parse,)*> Parse for ($last_gen, $($gen,)* ) {
            fn parse<'a>(input: &mut ParseBuffer<'a>) -> Result<Self> {
                let mut temp = input.clone();

                let vs = (temp.parse()?, $(temp.parse::<$gen>()?,)*);

                *input = temp;

                Ok(vs)
            }
        }

        impl<$last_gen:Peek, $($gen: Peek),*> Peek for ($last_gen, $($gen,)*) {
            #[allow(unused_mut)]
            fn peek<'a>(input: Cursor<'a>) -> Option<usize> {
                let mut v = $last_gen::peek(input)?;

                $({
                    v += $gen::peek(input.skip(v))?;
                })*

                Some(v)
            }
        }

        impl<$last_gen: FixedPeek, $($gen: FixedPeek),*> FixedPeek for ($last_gen, $($gen,)*) {
            const SKIP: usize = $last_gen::SKIP $(+ $gen::SKIP)*;
        }
        // Not perfect. In reality the last generic does not have to impl FixedPeek but this is
        // close enougth
        impl<$last_gen: PeekError + FixedPeek, $($gen: PeekError + FixedPeek),*> PeekError for ($last_gen, $($gen,)*) {
            #[allow(unused_mut, unused)]
            fn error<'a>(input: Cursor<'a>) -> Error {
                let mut e = $last_gen::error(input);
                let mut skip = $last_gen::SKIP;

                $({
                    e.combine($gen::error(input.skip(skip)));
                    skip += $gen::SKIP;
                })*

                e
            }
        }

        tuple_impl!($($gen)*);
    };
}

tuple_impl!(Z Y X W V U T S R Q P O N M L K J I H G F E D C B A);
// tuple_impl!(AZ AY AX AW AV AU AT AS AR AQ AP AO AN AM AL AK AJ AI AH AG AF AE AD AC AB AA Z Y X W V U T S R Q P O N M L K J I H G F E D C B A);
#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(it_matches_2_tuple, (Ident, FIdent<"world">) : hello world);
    insta_match_test!(it_steps_back_for_options, (Option<Ident>, Option<Punct>) : <);
    insta_match_test!(it_only_steps_back_on_fail_for_options, (Option<Ident>, Option<Punct>) : hi);
    insta_match_test!(it_steps_back_for_multi_tuples, (Option<Ident>, Option<Punct>, Option<Ident>, Option<Punct>) : hi <>);
    insta_match_test!(it_sums_tuple_backtracking, (Vec<(Punct, Punct, Ident, Ident)>, Punct) : >>h1 h2>>h3 h4 !);
}
