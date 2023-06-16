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

impl<T: Parse> Parse for Option<T> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let mut input = input.clone();

        Ok((
            match input.parse() {
                Ok(a) => Some(a),
                Err(_) => None,
            },
            input,
        ))
    }
}
impl<T: Peek> Peek for Option<T> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        Some(T::peek(input).unwrap_or_default())
    }
}

impl<T: Parse> Parse for Vec<T> {
    fn parse<'a>(input: &ParseBuffer<'a>) -> Result<(Self, ParseBuffer<'a>)> {
        let mut input = input.clone();
        let mut vs = Vec::new();

        while !input.cursor().eof() {
            match input.parse() {
                Ok(a) => vs.push(a),
                Err(_) => return Ok((vs, input)),
            }
        }

        Ok((vs, input))
    }
}
impl<T: Peek> Peek for Vec<T> {
    fn peek<'a>(input: &ParseBuffer<'a>) -> Option<usize> {
        let cursor = input.cursor();
        let mut step = 0;

        while !cursor.eof() {
            match T::peek(&cursor.skip(step).into()) {
                Some(a) => step += a,
                None => return Some(step),
            }
        }

        Some(step)
    }
}

#[cfg(test)]
mod tests_tuple {
    use crate::*;

    insta_match_test!(it_matches_2_tuple, (Ident, FIdent<"world">) : hello world);
    insta_match_test!(it_steps_back_for_options, (Option<Ident>, Option<Punct>) : <);
    insta_match_test!(it_only_steps_back_on_fail_for_options, (Option<Ident>, Option<Punct>) : hi);
    insta_match_test!(it_steps_back_for_multi_tuples, (Option<Ident>, Option<Punct>, Option<Ident>, Option<Punct>) : hi <>);
    insta_match_test!(it_sums_tuple_backtracking, (Vec<(Punct, Punct, Ident, Ident)>, Punct) : >>h1 h2>>h3 h4 !);
}
#[cfg(test)]
mod tests_sum {
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
