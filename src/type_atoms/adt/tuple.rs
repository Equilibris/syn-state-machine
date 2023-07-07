use crate::internals::*;

macro_rules! tuple_impl {
    () => {
        impl<C> Parse<C> for () {
            fn parse(_: &mut ParseBuffer<C>) -> Result<Self> {
                Ok(())
            }
        }
        impl<C> Peek<C> for () {
            fn peek(_: &C) -> Option<usize> {
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
        impl<Cursor: Clone, $last_gen: Parse<Cursor>, $($gen: Parse<Cursor>,)*> Parse<Cursor> for ($last_gen, $($gen,)* ) {
            fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self> {
                let mut temp = input.clone();

                let vs = (temp.parse()?, $(temp.parse::<$gen>()?,)*);

                *input = temp;

                Ok(vs)
            }
        }

        impl<Cursor: Skip + Clone, $last_gen: Peek<Cursor>, $($gen: Peek<Cursor>),*> Peek<Cursor> for ($last_gen, $($gen,)*) {

            fn peek(cursor: &Cursor) -> Option<usize> {
                #[allow(unused_mut)]
                let mut v = $last_gen::peek(cursor)?;
                let mut cursor = cursor.clone();

                cursor.skip(v);

                $({
                    let delta = $gen::peek(&cursor)?;
                    cursor.skip(delta);
                    v += delta;
                })*

                dbg!(v);
                Some(v)
            }
        }

        impl<$last_gen: FixedPeek, $($gen: FixedPeek),*> FixedPeek for ($last_gen, $($gen,)*) {
            const SKIP: usize = $last_gen::SKIP $(+ $gen::SKIP)*;
        }
        // Not perfect. In reality the last generic does not have to impl FixedPeek but this is
        // close enougth
        impl<Cursor: Clone + Skip, $last_gen: PeekError<Cursor> + FixedPeek, $($gen: PeekError<Cursor> + FixedPeek),*> PeekError<Cursor> for ($last_gen, $($gen,)*) {
            fn error(cursor: &Cursor) -> Error {
                #[allow(unused_mut)]
                let mut e = $last_gen::error(&cursor);
                let mut cursor = cursor.clone();
                cursor.skip($last_gen::SKIP);

                $({
                    e.combine($gen::error(&cursor));
                    cursor.skip($gen::SKIP);
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

    insta_match_test!(it_fails_2_tuple, (Ident, FIdent<"world">) : hello *);
    insta_match_test!(it_fails_2_types, (Ident, Ident) : hello *);
    insta_match_test!(it_matches_2_tuple, (Ident, FIdent<"world">) : hello world);
    insta_match_test!(it_steps_back_for_options, (Option<Ident>, Option<Punct>) : <);
    insta_match_test!(it_only_steps_back_on_fail_for_options, (Option<Ident>, Option<Punct>) : hi);
    insta_match_test!(it_steps_back_for_multi_tuples, (Option<Ident>, Option<Punct>, Option<Ident>, Option<Punct>) : hi <>);
    insta_match_test!(it_sums_tuple_backtracking, (Vec<(Punct, Punct, Ident, Ident)>, Punct) : >>h1 h2>>h3 h4 !);
}
