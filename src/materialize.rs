#[macro_export]
macro_rules! materialize {
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: []
        munch:  []
    ) => {
        $(#[$($macro_input)*])*
        $vis enum $id $(<$($gen),*>)*{
            $($out_def)*
        }

        impl<
            $($($domain_lts,)*
            $($domain_gens,)*)?
            $($($gen: $crate::Parse<$domain, ()>,)*)?
        > $crate::Parse<$domain, ()> for $id $(<$($gen,)*>)? {
            type Finalizer = $crate::BlackHoleFinalizer<Self>;

            fn parse(
                $input: &mut $crate::ParseBuffer<$domain>
            ) -> Result<
                Self::Finalizer, <$domain as $crate::ParserCursor>::Error
            > {
                $($common_parse_code)*

                use $crate::$sum_name::*;
                Ok($crate::BlackHoleFinalizer(match $input.parse::<$crate::$sum_name<$($out_type)*>> ()? { $($out_match)* }))
            }
        }
    };
    ([+Sum0 V0] $($next:tt)*) => { materialize!{ [Sum1 V1] $($next)* } };
    ([+Sum1 V1] $($next:tt)*) => { materialize!{ [Sum2 V2] $($next)* } };
    ([+Sum2 V2] $($next:tt)*) => { materialize!{ [Sum3 V3] $($next)* } };
    ([+Sum3 V3] $($next:tt)*) => { materialize!{ [Sum4 V4] $($next)* } };
    ([+Sum4 V4] $($next:tt)*) => { materialize!{ [Sum5 V5] $($next)* } };
    ([+Sum5 V5] $($next:tt)*) => { materialize!{ [Sum6 V6] $($next)* } };
    ([+Sum6 V6] $($next:tt)*) => { materialize!{ [Sum7 V7] $($next)* } };
    ([+Sum7 V7] $($next:tt)*) => { materialize!{ [Sum8 V8] $($next)* } };
    ([+Sum8 V8] $($next:tt)*) => { materialize!{ [Sum9 V9] $($next)* } };
    ([+Sum9 V9] $($next:tt)*) => { materialize!{ [Sum10 V10] $($next)* } };
    ([+Sum10 V10] $($next:tt)*) => { materialize!{ [Sum11 V11] $($next)* } };
    ([+Sum11 V11] $($next:tt)*) => { materialize!{ [Sum12 V12] $($next)* } };
    ([+Sum12 V12] $($next:tt)*) => { materialize!{ [Sum13 V13] $($next)* } };
    ([+Sum13 V13] $($next:tt)*) => { materialize!{ [Sum14 V14] $($next)* } };
    ([+Sum14 V14] $($next:tt)*) => { materialize!{ [Sum15 V15] $($next)* } };
    ([+Sum15 V15] $($next:tt)*) => { materialize!{ [Sum16 V16] $($next)* } };
    ([+Sum16 V16] $($next:tt)*) => { materialize!{ [Sum17 V17] $($next)* } };
    ([+Sum17 V17] $($next:tt)*) => { materialize!{ [Sum18 V18] $($next)* } };
    ([+Sum18 V18] $($next:tt)*) => { materialize!{ [Sum19 V19] $($next)* } };
    ([+Sum19 V19] $($next:tt)*) => { materialize!{ [Sum20 V20] $($next)* } };
    ([+Sum20 V20] $($next:tt)*) => { WENT TOO DEEP };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: []
        munch:  [$variant_id:ident () $(, $($after:tt)*)?]
    ) => {
        materialize! {
            [+$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)*]
            common_parse_output: [$($common_parse_output)*]
            common_parse_code:   [$($common_parse_code)*]

            variant_definition:   []
            variant_parse_output: []
            variant_parse_code:   []
            variant_parse_names:  []

            out_type:  [$($out_type)* ($($variant_parse_code)*),]
            out_match: [
                $($out_match)* $variant_name(($($variant_parse_names)*)) =>
                    $id::$variant_id($($common_parse_output)* $($variant_parse_output)*),
            ]
            out_def:[
                $($out_def)* $variant_id ($($common_definition)* $($variant_definition)*),
            ]

            before: []
            munch:  [$($($after)*)?]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: []
        munch:  [$variant_id:ident (<- $ty:ty $(; $($next_var:tt)*)?) $($after:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)*]
            common_parse_output: [$($common_parse_output)*]
            common_parse_code:   [$($common_parse_code)*]

            variant_definition:   [$($variant_definition)*]
            variant_parse_output: [$($variant_parse_output)*]
            variant_parse_code:   [$($variant_parse_code)* $crate::PeekAsParse<$ty>,]
            variant_parse_names:  [$($variant_parse_names)* _, ]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: []
            munch:  [$variant_id ($($($next_var)*)?) $($after)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident peek <- $ty:ty $(; $($next_var:tt)*)?) $($after:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)*]
            common_parse_output: [$($common_parse_output)*]
            common_parse_code:   [$($common_parse_code)*]

            variant_definition:   [$($variant_definition)* bool,]
            variant_parse_output: [$($variant_parse_output)* $val.is_some(),]
            variant_parse_code:   [$($variant_parse_code)* Option<$ty>, ]
            variant_parse_names:  [$($variant_parse_names)* $val,]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: []
            munch:  [$variant_id ($($($next_var)*)?) $($after)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $ty:ty : $from_ty:ty { $($conversion:tt)* } $(; $($next_var:tt)*)?) $($after:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)*]
            common_parse_output: [$($common_parse_output)*]
            common_parse_code:   [$($common_parse_code)*]

            variant_definition:   [$($variant_definition)* $ty,]
            variant_parse_output: [$($variant_parse_output)* {$($conversion)*},]
            variant_parse_code:   [$($variant_parse_code)* $from_ty, ]
            variant_parse_names:  [$($variant_parse_names)* $val, ]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: []
            munch:  [$variant_id ($($($next_var)*)?) $($after)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $ty:ty : $from_ty:ty $(; $($next_var:tt)*)?) $($after:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)*]
            common_parse_output: [$($common_parse_output)*]
            common_parse_code:   [$($common_parse_code)*]

            variant_definition:   [$($variant_definition)*   $ty,]
            variant_parse_output: [$($variant_parse_output)* <$from_ty as Into<$ty>>::into($val),]
            variant_parse_code:   [$($variant_parse_code)*   $from_ty, ]
            variant_parse_names:  [$($variant_parse_names)*  $val, ]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: []
            munch:  [$variant_id ($($($next_var)*)?) $($after)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $ty:ty $(; $($next_var:tt)*)?) $($after:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)*]
            common_parse_output: [$($common_parse_output)*]
            common_parse_code:   [$($common_parse_code)*]

            variant_definition:   [$($variant_definition)*   $ty,  ]
            variant_parse_output: [$($variant_parse_output)* $val, ]
            variant_parse_code:   [$($variant_parse_code)*   $ty,  ]
            variant_parse_names:  [$($variant_parse_names)*  $val, ]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: []
            munch:  [$variant_id ($($($next_var)*)?) $($after)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: [$val:ident <- $ty:ty : $from:ty {$($conversion:tt)*} $(; $($next:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)* $ty,]
            common_parse_output: [$($common_parse_output)* $val,]
            common_parse_code:   [$($common_parse_code)* let $val: $ty = {let $val: $from = $input.parse()?; $($conversion)* };]

            variant_definition:   [$($variant_definition)*]
            variant_parse_output: [$($variant_parse_output)*]
            variant_parse_code:   [$($variant_parse_code)*]
            variant_parse_names:  [$($variant_parse_names)*]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: [$($($next)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: [$val:ident <- $ty:ty : $from_ty:ty $(; $($next:tt)*)?]
        munch: [$($source_code:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)* $ty,]
            common_parse_output: [$($common_parse_output)* $val,]
            common_parse_code:   [$($common_parse_code)* let $val: $ty = <$from_ty as Into<$ty>>::into($input.parse::<$from_ty>()?);]

            variant_definition:   [$($variant_definition)*]
            variant_parse_output: [$($variant_parse_output)*]
            variant_parse_code:   [$($variant_parse_code)*]
            variant_parse_names:  [$($variant_parse_names)*]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: [$($($next)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: [<- $ty:ty $(; $($next:tt)*)?]
        munch: [$($source_code:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)*]
            common_parse_output: [$($common_parse_output)* ]
            common_parse_code:   [$($common_parse_code)* $input.errored_peek::<$ty>()?;]

            variant_definition:   [$($variant_definition)*]
            variant_parse_output: [$($variant_parse_output)*]
            variant_parse_code:   [$($variant_parse_code)*]
            variant_parse_names:  [$($variant_parse_names)*]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: [$($($next)*)?]
            munch: [$($source_code)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: [$val:ident peek <- $ty:ty $(; $($next:tt)*)?]
        munch: [$($source_code:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)* bool,]
            common_parse_output: [$($common_parse_output)* $val,]
            common_parse_code:   [$($common_parse_code)* let $val = $input.peek::<$ty>();]

            variant_definition:   [$($variant_definition)*]
            variant_parse_output: [$($variant_parse_output)*]
            variant_parse_code:   [$($variant_parse_code)*]
            variant_parse_names:  [$($variant_parse_names)*]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: [$($($next)*)?]
            munch: [$($source_code)*]
        }
    };
    (
        [$sum_name:ident $variant_name:ident]
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])*
        $vis:vis enum $id:ident$(<$($gen:ident),*>)?

        $input:ident

        common_definition:   [$($common_definition:tt)*]
        common_parse_output: [$($common_parse_output:tt)*]
        common_parse_code:   [$($common_parse_code:tt)*]

        variant_definition:   [$($variant_definition:tt)*]
        variant_parse_output: [$($variant_parse_output:tt)*]
        variant_parse_code:   [$($variant_parse_code:tt)*]
        variant_parse_names:  [$($variant_parse_names:tt)*]

        out_type:  [$($out_type:tt)* ]
        out_match: [$($out_match:tt)*]
        out_def:   [$($out_def:tt)*]

        before: [$val:ident <- $ty:ty $(; $($next:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        materialize! {
            [$sum_name $variant_name]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            $input

            common_definition:   [$($common_definition)* $ty,]
            common_parse_output: [$($common_parse_output)* $val,]
            common_parse_code:   [$($common_parse_code)* let $val: $ty = $input.parse()?;]

            variant_definition:   [$($variant_definition)*]
            variant_parse_output: [$($variant_parse_output)*]
            variant_parse_code:   [$($variant_parse_code)*]
            variant_parse_names:  [$($variant_parse_names)*]

            out_type:  [$($out_type)* ]
            out_match: [$($out_match)*]
            out_def: [$($out_def)*]

            before: [$($($next)*)?]
            munch:  [$($source_code)*]
        }
    };

    (on $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)? [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis enum $id:ident$(<$($gen:ident),*>)?
        $([$($before:tt)*])?
        {$($source_code:tt)*}
    ) => {
        materialize! {
            [Sum0 V0]
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            enum $id$(<$($gen),*>)?

            input

            common_definition:   []
            common_parse_output: []
            common_parse_code:   []

            variant_definition:   []
            variant_parse_output: []
            variant_parse_code:   []
            variant_parse_names:  []

            out_type:  []
            out_match: []
            out_def: []

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };

    (
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis
        struct $id:ident$(<$($gen:ident),*>)?

        $input:ident

        definition: [$($defs:tt)*]
        parse_output: [$($parse_out:tt)*]
        parse_code: [$($parse_code:tt)*]
        munch: []
    ) => {
        $(#[$($macro_input)+])*
        $vis struct $id $(<$($gen,)*>)? { $($defs)* }

        impl<
            $($($domain_lts,)*
            $($domain_gens,)*)?
            $($($gen: $crate::Parse<$domain, ()>,)*)?
        > $crate::Parse<$domain, ()> for $id $(<$($gen,)*>)? {
            type Finalizer = $crate::BlackHoleFinalizer<Self>;

            fn parse(
                $input: &mut $crate::ParseBuffer<$domain>
            ) -> Result<
                Self::Finalizer, <$domain as $crate::ParserCursor>::Error
            > {
                $($parse_code)*

                Ok($crate::BlackHoleFinalizer(Self { $($parse_out)* }))
            }
        }
    };
    (
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis
        struct $id:ident$(<$($gen:ident),*>)?

        $input:ident

        definition: [$($defs:tt)*]
        parse_output: [$($parse_out:tt)*]
        parse_code: [$($parse_code:tt)*]
        munch: [$val:ident peek <- $ty:ty $(; $($source_code:tt)*)?]
    ) => {
        materialize! {
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            struct $id$(<$($gen),*>)?

            $input

            definition:   [$($defs)*       $vis $val: bool, ]
            parse_output: [$($parse_out)*  $val, ]
            parse_code:   [$($parse_code)* let $val = $input.peek::<$ty>();]
            munch: [$($($source_code)*)?]
        }
    };
    (
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis
        struct $id:ident$(<$($gen:ident),*>)?

        $input:ident

        definition: [$($defs:tt)*]
        parse_output: [$($parse_out:tt)*]
        parse_code: [$($parse_code:tt)*]
        munch: [$val:ident <- $ty:ty : $from_ty:ty $(; $($source_code:tt)*)?]
    ) => {
        materialize! {
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            struct $id$(<$($gen),*>)?

            $input

            definition:   [$($defs)*       $vis $val: $ty, ]
            parse_output: [$($parse_out)*  $val, ]
            parse_code:   [$($parse_code)* let $val: $ty = <$from_ty as Into<$ty>>::into($input.parse::<$from_ty>()?); ]
            munch: [$($($source_code)*)?]
        }
    };
    (
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis
        struct $id:ident$(<$($gen:ident),*>)?

        $input:ident

        definition: [$($defs:tt)*]
        parse_output: [$($parse_out:tt)*]
        parse_code: [$($parse_code:tt)*]
        munch: [$val:ident <- $ty:ty : $from_ty:ty {$($conversion:tt)*} $(; $($source_code:tt)*)?]
    ) => {
        materialize! {
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            struct $id$(<$($gen),*>)?

            $input

            definition:   [$($defs)*       $vis $val: $ty, ]
            parse_output: [$($parse_out)*  $val, ]
            parse_code:   [$($parse_code)* let $val: $ty = { let $val: $from_ty = $input.parse()?; $($conversion)*}; ]
            munch: [$($($source_code)*)?]
        }
    };
    (
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis
        struct $id:ident$(<$($gen:ident),*>)?

        $input:ident

        definition: [$($defs:tt)*]
        parse_output: [$($parse_out:tt)*]
        parse_code: [$($parse_code:tt)*]
        munch: [$val:ident <- $ty:ty $(; $($source_code:tt)*)?]
    ) => {
        materialize! {
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            struct $id$(<$($gen),*>)?

            $input

            definition:   [$($defs)*       $vis $val: $ty, ]
            parse_output: [$($parse_out)*  $val, ]
            parse_code:   [$($parse_code)* let $val: $ty = $input.parse()?; ]
            munch: [$($($source_code)*)?]
        }
    };
    (
        $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)?
        [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis
        struct $id:ident$(<$($gen:ident),*>)?

        $input:ident

        definition: [$($defs:tt)*]
        parse_output: [$($parse_out:tt)*]
        parse_code: [$($parse_code:tt)*]
        munch: [<- $ty:ty $(; $($source_code:tt)*)?]
    ) => {
        materialize! {
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            struct $id$(<$($gen),*>)?

            $input

            definition:   [$($defs)* ]
            parse_output: [$($parse_out)* ]
            parse_code:   [$($parse_code)* $input.errored_peek::<$ty>()?; ]
            munch: [$($($source_code)*)?]
        }
    };

    (on $(<$($domain_lts:lifetime)* $($domain_gens:ident)*>)? [$domain:ty]
        $(#[$($macro_input:tt)+])* $vis:vis struct $id:ident$(<$($gen:ident),*>)?
        {$($source_code:tt)*}
    ) => {
        materialize! {
            $(<$($domain_lts)* $($domain_gens)*>)?
            [$domain]
            $(#[$($macro_input)+])* $vis
            struct $id$(<$($gen),*>)?

            input

            definition: []
            parse_output: []
            parse_code: []
            munch: [$($source_code)*]
        }
    };
}

#[cfg(test)]
mod tests {
    use proc_macro2::Ident;

    materialize! {
        on <'a> [crate::RustCursor<'a>]
        #[derive(Debug)]
        pub enum Hello [
            hi0 <- Ident;
            <- Ident;
            hi1 <- Ident : Ident { hi1 };
            hi10 <- Ident : Ident;
            hi2 peek <- Ident
        ] {
            Hi(
                hi3 <- Ident;
                <- Ident;
                hi4 <- Ident : Ident { hi4 };
                hi5 <- Ident;
            ),
            World()
        }
    }
    materialize! {
        on <'a> [crate::RustCursor<'a>]
        pub struct Hi {
            hi0 <- Ident;
            <- Ident;
            hi1 <- Ident : Ident { hi1 };
            hi2 peek <- Ident
        }
    }
    materialize! {
        on <'a> [crate::RustCursor<'a>]
        pub struct Hi2<Other> {
            hi0 <- Other;
            <- Ident;
            hi1 <- Ident : Ident { hi1 };
            hi2 peek <- Ident
        }
    }
}
