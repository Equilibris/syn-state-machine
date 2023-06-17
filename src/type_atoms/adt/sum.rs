use crate::internals::*;

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

        impl<$gen: Parse, $($gens: Parse,)*> Parse for $ty<$gen, $($gens,)*> {
            fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
                let mut buf = input.clone();

                let mut e = match buf.parse() {
                    Ok(a) => return Ok((Self::$gen(a), buf)),
                    Err(e0) => e0,
                };

                $(
                    match buf.parse() {
                        Ok(a) => return Ok((Self::$gens(a), buf)),
                        Err(e0) => e.combine(e0),
                    };
                )*

                Err(e)
            }
        }
        impl<$gen: Peek, $($gens: Peek,)*> Peek for $ty<$gen, $($gens,)*> {
            fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
                let buf = input.clone();

                if let Some(v) = $gen::peek(&buf) {
                    return Some(v);
                }
                $(
                    if let Some(v) = $gens::peek(&buf) {
                        return Some(v);
                    }
                )*

                None
            }
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
