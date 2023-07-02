mod extern_crate;
mod functions;
mod generic_parameters;
mod modules;
mod use_declarations;
mod alias {
    use crate::*;

    materialize! {
        pub struct TypeAlias<Attr, Ty> {
            id <- Identifier;
            generic_parameters <- Option<GenericParams<Attr, Ty>>;
            bounds <- Option<TypeParamBounds<Attr, Ty>> : Option<(Colon, _)> { bounds.map(|v|v.1) };
            eq <- Option<(Eq, Ty, Option<WhereClause<Attr, Ty>>)>;
            <- Semi
        }
    }

    // #[cfg(test)]
    // mod tests {
    //     use super::*;
    //     use std::convert::Infallible;

    //     // insta_match_test!(*it_matches_simple, TypeAlias<Infallible, Ident>: type Point = (u8, u8););
    //     // insta_match_test!(*it_matches_complex, TypeAlias<Infallible, Type<Infallible>>: type Point<T> where T: std::ops::Add = (T, T););
    // }
}

pub use extern_crate::*;
pub use generic_parameters::*;
pub use modules::*;
pub use use_declarations::*;
