#![feature(lint_reasons, iter_advance_by, fn_traits)]
#![expect(incomplete_features)]
#![feature(adt_const_params)]

mod internals;
mod materialize;
mod rust;
mod type_atoms;

pub use internals::*;
pub use materialize::*;
pub use rust::*;
pub use type_atoms::*;

pub fn parse_borrowed<'a, T: Parse<Cursor<'a>>>(buf: &'a TokenBuffer) -> Result<T> {
    let mut stream = ParseBuffer::from(buf.begin());

    stream.parse()
}
pub fn parse<T: for<'a> Parse<Cursor<'a>>>(ts: TokenStream) -> Result<T> {
    let buf = TokenBuffer::from(ts);

    parse_borrowed(&buf)
}

#[cfg(test)]
pub fn test_peek<T: for<'a> Peek<Cursor<'a>>>(ts: TokenStream) -> Option<usize> {
    let buf = TokenBuffer::from(ts);

    T::peek(&buf.begin())
}

#[cfg(test)]
#[macro_export]
macro_rules! insta_match_test {
    (+parse $ty:ty : $($t:tt)*) => {
        $crate::parse::<$ty>(::quote::quote!{$($t)*})
    };
    (*peek $ty:ty : $($t:tt)*) => {
        $crate::test_peek::<$ty>(::quote::quote!{$($t)*})
    };
    (+$test_name:ident, $ty:ty : $($t:tt)*) => {
        #[test]
        fn $test_name() {
            ::insta::assert_debug_snapshot!(
                insta_match_test!(+parse $ty : $($t)*)
            );
        }
    };
    (*$test_name:ident, $ty:ty : $($t:tt)*) => {
        #[test]
        fn $test_name() {
            ::insta::assert_debug_snapshot!(
                insta_match_test!(**$ty : $($t)*)
            );
        }
    };
    ($test_name:ident, $ty:ty : $($t:tt)*) => {
        #[test]
        fn $test_name() {
            ::insta::assert_debug_snapshot!((
                insta_match_test!(+parse $ty : $($t)*),
                insta_match_test!(*peek  $ty : $($t)*),
            ));
        }
    };
}
