#![feature(lint_reasons)]
#![expect(incomplete_features)]
#![feature(adt_const_params, fn_traits)]

mod internals;
mod rust;
mod type_atoms;

pub use internals::*;
pub use rust::*;
pub use type_atoms::*;

use proc_macro2::TokenStream;

pub fn parse_borrowed<'a, T: Parse>(buf: &'a TokenBuffer) -> Result<T> {
    let mut stream = ParseBuffer::from(buf.begin());

    stream.parse()
}
pub fn parse<T: Parse>(ts: TokenStream) -> Result<T> {
    let buf = TokenBuffer::from(ts);

    parse_borrowed(&buf)
}

#[cfg(test)]
pub fn test_peek<T: Peek>(ts: TokenStream) -> Option<usize> {
    let buf = TokenBuffer::from(ts);

    T::peek(buf.begin())
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
