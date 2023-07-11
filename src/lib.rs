#![feature(lint_reasons, iter_advance_by, fn_traits)]
#![expect(incomplete_features)]
#![feature(adt_const_params)]

mod internals;
#[cfg(feature = "materialize")]
mod materialize;
mod rust;
mod type_atoms;

pub use internals::*;
#[cfg(feature = "materialize")]
pub use materialize::*;
pub use rust::*;
pub use type_atoms::*;
