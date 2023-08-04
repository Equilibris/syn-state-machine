use crate::internals::*;

pub type Sum0 = std::convert::Infallible;

macro_rules! sum_impl {
    (-r2 $($tys:ident)* : [] : $($gens:ident $prods:ident)*) => {
        sum_impl!($($tys)*: $($gens $prods)*);
    };
    (-r2 $($tys:ident)* : [$gen:ident $prod:ident $($inner_gens:ident $inner_prods:ident)*] : $($gens:ident $prods:ident)*) => {
        sum_impl!(-r2 $($tys)+: [$($inner_gens $inner_prods)*]: $($gens $prods)* $gen $prod);
    };

    (-rl1 : $($_:tt)*) => {
    };
    (-rl1 $($tys:ident)+ : [$($inner_gens:ident $inner_prods:ident)*] : $gen:ident $prod:ident) => {
        sum_impl!(-r2 $($tys)+: [$($inner_gens $inner_prods)*] : );
    };
    (-rl1 $($tys:ident)+ : [$($inner_gens:ident $inner_prods:ident)*] : $gen:ident $prod:ident $($gens:ident $prods:ident)* ) => {
        sum_impl!(-rl1 $($tys)+: [$($inner_gens $inner_prods)* $gen $prod] : $($gens $prods)+);
    };

    ($ty:ident $($tys:ident)* : $gen:ident $prod:ident $($gens:ident $prods:ident)* ) => {
        #[derive(Debug, Clone, Copy)]
        pub enum $ty <$gen, $($gens,)*> {
            $gen($gen),
            $($gens($gens),)*
        }

        paste::paste! {
            impl<
                W,
                [<F $gen>], $gen: Finalizer<[<F $gen>], W>,
                $(
                    [<F $gens>], $gens: Finalizer<[<F $gens>], W>,
                )*
            > Finalizer<$ty<[<F $gen>], $([<F $gens>],)*>, W> for $ty<$gen, $($gens,)*> {
                fn finalize(self, value: W) -> std::ops::ControlFlow<$ty<[<F $gen>], $([<F $gens>],)*>, $ty<[<F $gen>], $([<F $gens>],)*>> {
                    match self {
                        Self::$gen(v) => v.finalize(value).map_break($ty::$gen).map_continue($ty::$gen),
                        $(
                            Self::$gens(v) => v.finalize(value).map_break($ty::$gens).map_continue($ty::$gens),
                        )*
                    }
                }
            }
        }

        #[cfg(feature = "printing")]
        impl<
            $gen: quote::ToTokens, $($gens: quote::ToTokens,)*
        > quote::ToTokens for $ty<$gen, $($gens,)*> {
            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                match self {
                    Self::$gen(ref $prod) => $prod.to_tokens(tokens),
                    $(Self::$gens(ref $prods) => $prods.to_tokens(tokens),)*
                }
            }
            fn into_token_stream(self) -> proc_macro2::TokenStream {
                match self {
                    Self::$gen($prod) => $prod.into_token_stream(),
                    $(Self::$gens($prods) => $prods.into_token_stream(),)*
                }
            }
        }

        impl<$gen, $($gens,)*> $ty <$gen, $($gens,)*> {
            ::paste::paste! {
                pub fn [< is_ $prod >](&self) -> bool {
                    match self {
                        Self::$gen (_) => true,
                        _ => false
                    }
                }
            }
            pub fn $prod(self) -> Option<$gen> {
                match self {
                    Self::$gen (v) => Some(v),
                    _ => None
                }
            }
            $(
                ::paste::paste! {
                    pub fn [< is_ $prods >](&self) -> bool {
                        match self {
                            Self::$gens (_) => true,
                            _ => false
                        }
                    }
                }
                pub fn $prods(self) -> Option<$gens> {
                    match self {
                        Self::$gens (v) => Some(v),
                        _ => None
                    }
                }
            )*
        }

        impl<Cursor: Clone + ParserCursor, With, $gen: Parse<Cursor, With>, $($gens: Parse<Cursor, With>,)*> Parse<Cursor, With> for $ty<$gen, $($gens,)*>
        where
            Cursor::Error: CombineError<Cursor::Error>
        {
            type Finalizer = $ty<$gen::Finalizer, $($gens::Finalizer,)*>;

            fn parse(input: &mut ParseBuffer<Cursor>) -> Result<Self::Finalizer, Cursor::Error> {
                let mut temp = input.clone();

                let mut e = match $gen::parse(&mut temp) {
                    Ok(a) => {
                        *input = temp;
                        return Ok($ty::$gen(a))
                    },
                    Err(e0) => e0,
                };

                $({
                    let mut temp = input.clone();

                    match $gens::parse(&mut temp) {
                        Ok(a) => {
                            *input = temp;
                            return Ok($ty::$gens(a))
                        },
                        Err(e0) => e.combine(e0),
                    };
                })*

                Err(e)
            }
        }
        impl<Cursor, $gen: Peek<Cursor>, $($gens: Peek<Cursor>,)*> Peek<Cursor> for $ty<$gen, $($gens,)*> {
            fn peek(input: &Cursor) -> Option<usize> {
                if let Some(v) = $gen::peek(input) {
                    return Some(v);
                }
                $(
                    if let Some(v) = $gens::peek(input) {
                        return Some(v);
                    }
                )*

                None
            }
        }
        impl<Cursor: ParserCursor, $gen: PeekError<Cursor>, $($gens: PeekError<Cursor>,)*> PeekError<Cursor> for $ty<$gen, $($gens,)*>
        where
            Cursor::Error: CombineError<Cursor::Error>
        {
            fn error(input: &Cursor) -> Cursor::Error {
                let mut e = $gen::error(input);

                $(e.combine($gens::error(input));)*

                e
            }
        }
        /// Technically incorrect, One has to restrain all of the other generic values to also be
        /// equal to $gen::SKIP.
        impl<$gen: FixedPeek, $($gens,)*> FixedPeek for $ty<$gen, $($gens,)*> {
            const SKIP: usize = $gen::SKIP;
        }

        sum_impl!(-rl1 $($tys)* : [] : $gen $prod $($gens $prods)*);
    };
}

sum_impl!(Sum10 Sum9 Sum8 Sum7 Sum6 Sum5 Sum4 Sum3 Sum2: V0 v0 V1 v1 V2 v2 V3 v3 V4 v4 V5 v5 V6 v6 V7 v7 V8 v8 V9 v9);
sum_impl!(Sum15 Sum14 Sum13 Sum12 Sum11: V0 v0 V1 v1 V2 v2 V3 v3 V4 v4 V5 v5 V6 v6 V7 v7 V8 v8 V9 v9 V10 v10 V11 v11 V12 v12 V13 v13 V14 v14);

#[cfg(test)]
mod tests {
    use crate::*;

    type P = Punct;

    insta_match_test!(
    it_matches_highest_priority,
    Sum5<
        (P,P,P,P,P),
        (P,P,P,P,),
        (P,P,P,),
        (P,P,),
        (P,),
    > : ....);

    insta_match_test!(it_matches_sum_2_0, Sum2<Ident, Punct> : hello);
    insta_match_test!(it_matches_sum_2_1, Sum2<Ident, Punct> : <);
}
