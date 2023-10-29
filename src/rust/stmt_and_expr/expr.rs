mod array;
mod await_expressions;
mod block;
mod call;
mod closure;
mod field;
mod grouped;
mod if_and_if_let;
mod lit;
mod loops;
mod match_expression;
mod method;
mod ops;
mod path;
mod range;
mod return_expressions;
mod struct_expr;
mod tuple;
mod underscore_expressions;

pub use array::*;
pub use await_expressions::*;
pub use block::*;
pub use call::*;
pub use closure::*;
pub use field::*;
pub use grouped::*;
pub use if_and_if_let::*;
pub use lit::*;
pub use loops::*;
pub use match_expression::*;
pub use method::*;
pub use ops::*;
pub use path::*;
pub use range::*;
pub use return_expressions::*;
pub use struct_expr::*;
pub use tuple::*;
pub use underscore_expressions::*;

use crate::*;

pub trait Expr {
    fn correct_order_of_operations(&mut self);
}
