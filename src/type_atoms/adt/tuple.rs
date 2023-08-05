use crate::internals::*;

type PrintValue<T> = crate::type_atoms::adt::p::P<T>;

macro_rules! tuple_impl {
    ([] []) => {
        impl<C: ParserCursor, W> Parse<C, W> for () {
            type Finalizer = BlackHoleFinalizer<Self>;

            fn parse(_: &mut ParseBuffer<C>) -> Result<Self::Finalizer, C::Error> {
                Ok(BlackHoleFinalizer(()))
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

        #[cfg(feature = "printing")]
        impl quote::ToTokens for PrintValue<()> {
            fn to_tokens(&self, _: &mut proc_macro2::TokenStream) {}
        }
        // Do not implement
        // impl PeekError for () {
        //     fn error<'a>(input: &ParseBuffer<'a>) -> Error {
        //         panic!("Atemted to fail infallable peek")
        //         // Error::new(input.span(), "Should never be reached")
        //     }
        // }
    };
    ([$last_gen:ident $($gen:ident)*] [$lower_name:ident $($lower_names:ident)*]) => {
        #[cfg(feature = "printing")]
        impl<$last_gen: quote::ToTokens, $($gen: quote::ToTokens,)*> quote::ToTokens for PrintValue<($last_gen, $($gen,)*)> {
            fn into_token_stream(self) -> proc_macro2::TokenStream {
                let Self(($lower_name, $($lower_names, )*)) = self;

                #[allow(unused_mut)]
                let mut tokens = $lower_name.into_token_stream();

                $(tokens.extend($lower_names.into_token_stream());)*

                tokens
            }
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                let Self((ref $lower_name, $(ref $lower_names, )*)) = self;

                $lower_name.to_tokens(tokens);
                $($lower_names.to_tokens(tokens);)*
            }
        }

        impl<Cursor: Clone + ParserCursor, $last_gen: Parse<Cursor, ()>, $($gen: Parse<Cursor, ()>,)*> Parse<Cursor, ()> for ($last_gen, $($gen,)* )
        {
            type Finalizer = BlackHoleFinalizer<Self>;

            fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
                let mut temp = input.clone();

                let vs = (temp.parse()?, $(temp.parse::<$gen>()?,)*);

                *input = temp;

                Ok(BlackHoleFinalizer(vs))
            }
        }

        impl<Cursor: Iterator + Clone, $last_gen: Peek<Cursor>, $($gen: Peek<Cursor>),*> Peek<Cursor> for ($last_gen, $($gen,)*) {
            fn peek(cursor: &Cursor) -> Option<usize> {
                #[allow(unused_mut)]
                let mut v = $last_gen::peek(cursor)?;
                let mut cursor = cursor.clone();

                let _ = cursor.advance_by(v);

                $({
                    let delta = $gen::peek(&cursor)?;
                    let _ = cursor.advance_by(delta);
                    v += delta;
                })*

                Some(v)
            }
        }

        impl<$last_gen: FixedPeek, $($gen: FixedPeek),*> FixedPeek for ($last_gen, $($gen,)*) {
            const SKIP: usize = $last_gen::SKIP $(+ $gen::SKIP)*;
        }
        // Not perfect. In reality the last generic does not have to impl FixedPeek but this is
        // close enougth
        impl<Cursor: ParserCursor + Clone + Iterator, $last_gen: PeekError<Cursor> + FixedPeek, $($gen: PeekError<Cursor> + FixedPeek),*> PeekError<Cursor> for ($last_gen, $($gen,)*)
        where
            Cursor::Error: CombineError<Cursor::Error>
        {
            fn error(cursor: &Cursor) -> Cursor::Error {
                #[allow(unused_mut)]
                let mut e = $last_gen::error(&cursor);
                let mut cursor = cursor.clone();
                let _ = cursor.advance_by($last_gen::SKIP);

                $({
                    e.combine($gen::error(&cursor));
                    let _  =cursor.advance_by($gen::SKIP);
                })*

                e
            }
        }

        tuple_impl!([$($gen)*] [$($lower_names)*]);
    };
}

tuple_impl!([
    Z Y X W V U T S R Q P O N M L K J I H G F E D C B A
] [
    z y x w v u t s r q p o n m l k j i h g f e d c b a
]);
// tuple_impl!(AZ AY AX AW AV AU AT AS AR AQ AP AO AN AM AL AK AJ AI AH AG AF AE AD AC AB AA Z Y X W V U T S R Q P O N M L K J I H G F E D C B A);
#[cfg(test)]
mod tests {
    use crate::*;

    insta_match_test!(peek parse : it_fails_2_tuple, P<(Ident, FIdent<"world">)> : hello *);
    insta_match_test!(peek parse : it_fails_2_types, P<(Ident, Ident)> : hello *);
    insta_match_test!(peek parse print : it_steps_back_for_options, P<(P<Option<Ident>>, P<Option<Punct>>)> : <);
    insta_match_test!(peek parse print : it_only_steps_back_on_fail_for_options, P<(P<Option<Ident>>, P<Option<Punct>>)> : hi);
    insta_match_test!(peek parse print : it_steps_back_for_multi_tuples, P<(P<Option<Ident>>, P<Option<Punct>>, P<Option<Ident>>, P<Option<Punct>>)> : hi <>);
    insta_match_test!(peek parse print : it_matches_2_tuple, P<(Ident, FIdent<"world">)> : hello world);
    insta_match_test!(peek parse print : it_sums_tuple_backtracking, P<(Rep<P<(Punct, Punct, Ident, Ident)>>, Punct)> : >>h1 h2>>h3 h4 !);
}
