use crate::internals::*;

macro_rules! tuple_impl {
    () => {
        impl Parse for () {
            fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
                Ok(((), input.clone()))
            }
        }
        impl Peek for () {
            fn peek<'a>(_: &ParseBuffer<'a>) -> Option<usize> {
                Some(0)
            }
        }
    };
    ($last_gen:ident $($gen:ident)*) => {
        impl<$last_gen: Parse, $($gen: Parse,)*> Parse for ($last_gen, $($gen,)* ) {
            fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
                let mut pb = input.clone();
                Ok(((pb.parse()?, $(pb.parse::<$gen>()?,)*), pb))
            }
        }

        impl<$last_gen:Peek, $($gen: Peek),*> Peek for ($last_gen, $($gen,)*) {
            #[allow(unused_mut)]
            fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
                let cur = input.cursor();

                let mut v = $last_gen::peek(&ParseBuffer::from(cur))?;

                $({
                    v += $gen::peek(&ParseBuffer::from(cur.skip(v)))?;
                })*

                Some(v)
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
