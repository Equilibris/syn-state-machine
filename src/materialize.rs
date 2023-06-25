#[macro_export]
macro_rules! materialize {
    // <Struct Building>
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*]) => {
        $(#[$($macro_input)+])*
        $vis struct $id $(<$($gen),*>)? {$($prev)*}
    };
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*] $var:ident peek <- $ty:ty $(: $from_ty:ty {$($convert:tt)*})?; $($next:tt)*) => {
        materialize!(!struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)* $vis $var: bool,] $($next)*);
    };
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*] $var:ident <- $ty:ty $(: $from_ty:ty {$($convert:tt)*})?; $($next:tt)*) => {
        materialize!(!struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)* $vis $var: $ty,] $($next)*);
    };
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*] <- $ty:ty; $($next:tt)*) => {
        materialize!(!struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)*] $($next)*);
    };
    // </Struct Building>
    // <Parser Building>
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*]) => {
        impl$(<$($gen: $crate::Parse),*>)? $crate::Parse for $id$(<$($gen),*>)? {
            fn parse<'a>($input: &mut $crate::ParseBuffer<'a>) -> $crate::Result<Self> {
                $($prev_main)*

                Ok(Self {
                    $($prev_self)*
                })
            }
        }
    };
    (+struct $id:ident $input:ident$(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*] $var:ident <- $ty:ty : $from_ty:ty {$($convert:tt)*}; $($next:tt)*) => {
        materialize!(+struct $id $input $(<$($gen),*>)?[$($prev_self)* $var,][$($prev_main)* let $var = {let $var = $input.parse::<$from_ty>()?; {$($convert)*}};] $($next)*);
    };
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*] $var:ident peek <- $ty:ty; $($next:tt)*) => {
        materialize!(+struct $id $input $(<$($gen),*>)?[$($prev_self)* $var,][$($prev_main)* let $var = $input.peek::<$ty>();] $($next)*);
    };
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*] $var:ident <- $ty:ty; $($next:tt)*) => {
        materialize!(+struct $id $input $(<$($gen),*>)?[$($prev_self)* $var,][$($prev_main)* let $var = $input.parse::<$ty>()?;] $($next)*);
    };
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*] <- $ty:ty; $($next:tt)*) => {
        materialize!(+struct $id $input $(<$($gen),*>)?[$($prev_self)*][$($prev_main)* $input.errored_peek::<$ty>()?;] $($next)*);
    };
    // </Parser Building>
    // Entry
    ($(#[$($macro_input:tt)+])* $vis:vis struct $id:ident$(<$($gen:ident),*>)? { $($($var:ident $($sym:ident)?)? <- $ty:ty $(: $from_ty:ty {$($convert:tt)*})?;)* }) => {
        materialize!(!struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[] $($($var $($sym)?)? <- $ty $(: $from_ty {$($convert:tt)*})?;)*);
        materialize!(+struct $id input $(<$($gen),*>)?[][] $($($var $($sym)?)? <- $ty $(: $from_ty {$($convert)*})?;)*);
    };
}

#[cfg(test)]
mod tests {}
