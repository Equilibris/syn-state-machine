#![feature(lint_reasons, iter_advance_by, fn_traits, control_flow_enum)]
#![expect(incomplete_features)]
#![feature(adt_const_params)]

mod internals;
#[cfg(feature = "materialize")]
mod materialize;
#[cfg(feature = "printing")]
mod print_macro;
mod rust;
mod type_atoms;

pub use internals::*;
#[cfg(feature = "materialize")]
pub use materialize::*;
#[cfg(feature = "printing")]
pub use print_macro::*;
pub use rust::*;
pub use type_atoms::*;
