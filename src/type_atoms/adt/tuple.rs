use crate::internals::*;

macro_rules! tuple_impl {
    ([] [] []) => {
        #[derive(Debug, Clone, Default)]
        pub struct P0();

        impl<C: ParserCursor, W> Parse<C, W> for P0 {
            type Finalizer = BlackHoleFinalizer<Self>;

            fn parse(_: &mut ParseBuffer<C>) -> Result<Self::Finalizer, C::Error> {
                Ok(BlackHoleFinalizer(P0()))
            }
        }
        impl<C> Peek<C> for P0 {
            fn peek(_: &C) -> Option<usize> {
                Some(0)
            }
        }

        impl FixedPeek for P0 {
            const SKIP: usize = 0;
        }

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
        impl quote::ToTokens for P0 {
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
    ([$last_gen:ident $($gen:ident)*] [$lower_name:ident $($lower_names:ident)*] [$last_name:ident $($names:ident)*]) => {
        #[derive(Debug, Clone, Default)]
        pub struct $last_name<$last_gen, $($gen),*>(pub $last_gen, $(pub $gen),*);

        impl<$last_gen, $($gen),*> From<($last_gen, $($gen),*)> for $last_name<$last_gen, $($gen),*> {
            fn from(($lower_name, $($lower_names),*): ($last_gen, $($gen),*)) -> Self {
                Self($lower_name, $($lower_names),*)
            }
        }
        impl<$last_gen, $($gen),*> From<$last_name<$last_gen, $($gen),*>> for ($last_gen, $($gen),*) {
            fn from($last_name($lower_name, $($lower_names,)*): $last_name<$last_gen, $($gen),*>) -> Self {
                ($lower_name, $($lower_names,)*)
            }
        }

        #[cfg(feature = "printing")]
        impl<$last_gen: quote::ToTokens, $($gen: quote::ToTokens,)*> quote::ToTokens for $last_name <$last_gen, $($gen,)*> {
            fn into_token_stream(self) -> proc_macro2::TokenStream {
                let Self($lower_name, $($lower_names, )*) = self;

                #[allow(unused_mut)]
                let mut tokens = $lower_name.into_token_stream();

                $(tokens.extend($lower_names.into_token_stream());)*

                tokens
            }
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                let Self(ref $lower_name, $(ref $lower_names, )*) = self;

                $lower_name.to_tokens(tokens);
                $($lower_names.to_tokens(tokens);)*
            }
        }

        impl<Cursor: Clone + ParserCursor, $last_gen: Parse<Cursor, ()>, $($gen: Parse<Cursor, ()>,)*> Parse<Cursor, ()> for $last_name< $last_gen, $($gen,)*  >
        {
            type Finalizer = BlackHoleFinalizer<Self>;

            fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
                Ok(BlackHoleFinalizer(Self::from(input.parse::<( $last_gen, $($gen,)*  )>()?)))
            }
        }

        impl<Cursor: Iterator + Clone, $last_gen: Peek<Cursor>, $($gen: Peek<Cursor>),*> Peek<Cursor> for $last_name< $last_gen, $($gen,)* > {
            fn peek(cursor: &Cursor) -> Option<usize> {
                <( $last_gen, $($gen,)* ) as Peek<Cursor>>::peek(cursor)
            }
        }

        impl<$last_gen: FixedPeek, $($gen: FixedPeek),*> FixedPeek for $last_name< $last_gen, $($gen,)* > {
            const SKIP: usize = $last_gen::SKIP $(+ $gen::SKIP)*;
        }
        impl<Cursor: ParserCursor + Clone + Iterator, $last_gen: PeekError<Cursor> + FixedPeek, $($gen: PeekError<Cursor> + FixedPeek),*> PeekError<Cursor> for $last_name< $last_gen, $($gen,)* >
        where
            Cursor::Error: CombineError<Cursor::Error>
        {
            fn error(cursor: &Cursor) -> Cursor::Error {
                <( $last_gen, $($gen,)* ) as PeekError<Cursor>>::error(cursor)
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

        tuple_impl!([$($gen)*] [$($lower_names)*] [$($names)*]);
    };
}

tuple_impl!([
    Z Y X W V U T S R Q P O N M L K J I H G F E D C B A
] [
    z y x w v u t s r q p o n m l k j i h g f e d c b a
] [
    P26 P25 P24 P23 P22 P21 P20 P19 P18 P17 P16 P15 P14 P13 P12 P11 P10 P9 P8 P7 P6 P5 P4 P3 P2 P1
]);
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
    insta_match_test!(it_sums_tuple_backtracking, (Rep<(Punct, Punct, Ident, Ident)>, Punct) : >>h1 h2>>h3 h4 !);
}
