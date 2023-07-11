#[cfg(feature = "rust-atoms")]
mod atoms;
#[cfg(feature = "rust-atoms")]
pub use atoms::*;

#[cfg(feature = "rust")]
mod attributes;
#[cfg(feature = "rust")]
mod items;
#[cfg(feature = "rust")]
mod lexical_structure;
#[cfg(feature = "rust")]
mod macros;
#[cfg(feature = "rust")]
mod names;
#[cfg(feature = "rust")]
mod patterns;
#[cfg(feature = "rust")]
mod type_system;

#[cfg(feature = "rust")]
pub use attributes::*;
#[cfg(feature = "rust")]
pub use items::*;
#[cfg(feature = "rust")]
pub use lexical_structure::*;
#[cfg(feature = "rust")]
pub use macros::*;
#[cfg(feature = "rust")]
pub use names::*;
#[cfg(feature = "rust")]
pub use patterns::*;
#[cfg(feature = "rust")]
pub use type_system::*;

#[cfg(feature = "rust-atoms")]
use crate::{Parse, ParseBuffer};

#[cfg(feature = "rust-atoms")]
pub fn parse_borrowed<'a, T: Parse<RustCursor<'a>>>(buf: &'a TokenBuffer) -> Result<T, Error> {
    let mut stream = ParseBuffer::from(buf.begin());

    stream.parse()
}
#[cfg(feature = "rust-atoms")]
pub fn parse<T: for<'a> Parse<RustCursor<'a>>>(ts: TokenStream) -> std::result::Result<T, Error> {
    let buf = TokenBuffer::from(ts);

    parse_borrowed(&buf)
}

#[cfg(test)]
#[cfg(feature = "rust-atoms")]
pub fn test_peek<T: for<'a> crate::Peek<RustCursor<'a>>>(ts: TokenStream) -> Option<usize> {
    let buf = TokenBuffer::from(ts);

    T::peek(&buf.begin())
}

#[cfg(test)]
#[cfg(feature = "rust-atoms")]
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
