// Please somehow refactor into something sane
#[macro_export]
macro_rules! materialize {
    // <enum>
    // <enum binding>
    // construct enum
    (!!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?[$($prev:tt)*][$($content:tt)*][]) => {
        $(#[$($macro_input)+])*
        $vis enum $id $(<$($gen),*>)? {$($content)*}
    };
    // <variant>
    // continue to next variant
    (
        !!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?
        [$($prev:tt)*][$($content:tt)*]
        [$($next_variants:tt)*]

        $variant:ident

        [$($prior:tt)*]
        []
    ) => {
        materialize!(!!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)*][$($content)* $variant ($($prev)* $($prior)*),][$($next_variants)*] );
    };
    // match peek in variant
    (
        !!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?
        [$($prev:tt)*][$($content:tt)*]
        [$($next_variants:tt)*]

        $variant:ident

        [$($prior:tt)*]
        [$_:ident peek <- $ty:ty$(; $($next_variant_source_code:tt)*)?]
    ) => {
        materialize!(
            !!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            [$($prev)*][$($content)*]
            [$($next_variants)*]

            $variant

            [$($prior)* bool,]
            [$($($next_variant_source_code)*)?]
        );
    };
    // match mapped parse in variant
    (
        !!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?
        [$($prev:tt)*][$($content:tt)*]
        [$($next_variants:tt)*]

        $variant:ident

        [$($prior:tt)*]
        [$_:ident <- $ty:ty : $from_ty:ty $({ $($conversion:tt)* })?$(; $($next_variant_source_code:tt)*)?]
    ) => {
        materialize!(
            !!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            [$($prev)*][$($content)*]
            [$($next_variants)*]

            $variant

            [$($prior)* $ty,]
            [$($($next_variant_source_code)*)?]
        );
    };
    // match parse in variant
    (
        !!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?
        [$($prev:tt)*][$($content:tt)*]
        [$($next_variants:tt)*]

        $variant:ident

        [$($prior:tt)*]
        [$_:ident <- $ty:ty$(; $($next_variant_source_code:tt)*)?]
    ) => {
        materialize!(
            !!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            [$($prev)*][$($content)*]
            [$($next_variants)*]

            $variant

            [$($prior)* $ty,]
            [$($($next_variant_source_code)*)?]
        );
    };
    // match erring peek in variant
    (
        !!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?
        [$($prev:tt)*][$($content:tt)*]
        [$($next_variants:tt)*]

        $variant:ident

        [$($prior:tt)*]
        [<- $ty:ty$(; $($next_variant_source_code:tt)*)?]
    ) => {
        materialize!(
            !!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            [$($prev)*][$($content)*]
            [$($next_variants)*]

            $variant
            [$($prior)*]
            [$($($next_variant_source_code)*)?]
        );
    };
    // Select variant from variant listing and start parsing it
    (
        !!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?
        [$($prev:tt)*][$($content:tt)*]
        [$variant:ident ($($curr_source_code:tt)*) $($next_variants:tt)*]
    ) => {
        materialize!(!!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)*][$($content)*][$($next_variants)*] $variant [][$($curr_source_code)*] );
    };
    // </variant>
    // <shared>
    // conclude matching of shared data
    (!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?[$($prev:tt)*][][$($variant_source_code:tt)*]) => {
        materialize!(!!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)*][][$($variant_source_code)*]);
    };
    // match shared erring peek
    (!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?[$($prev:tt)*][<- $ty:ty$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)*][$($($src_next)*)?][$($variant_source_code)*]);
    };
    // match shared mapped parse
    (!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?[$($prev:tt)*][$_:ident<- $ty:ty : $from_ty:ty $({$($conversion:tt)*})?$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)* $ty,][$($($src_next)*)?][$($variant_source_code)*]);
    };
    // match shared parse
    (!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?[$($prev:tt)*][$_:ident<- $ty:ty$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)* $ty,][$($($src_next)*)?][$($variant_source_code)*]);
    };
    // match shared peek
    (!enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?[$($prev:tt)*][$_:ident peek <- $ty:ty$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(!enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[$($prev)* bool,][$($($src_next)*)?][$($variant_source_code)*]);
    };
    // </shared>
    // </enum binding>
    // <enum parsing>

    // Conclude Variant matching
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][]
     [$($prev_type:tt)*][$($prev_match:tt)*]
    ) => {
        impl<'a, $($($gen: Parse<$crate::Cursor<'a>>),*)?> $crate::Parse<$crate::Cursor<'a>> for $id$(<$($gen),*>)? {
            fn parse($input: &mut $crate::ParseBuffer<$crate::Cursor<'a>>) -> $crate::Result<Self>{
                $($prev_parse)*

                use $crate::$sum_name::*;

                Ok(match $input.parse::<$crate::$sum_name<$($prev_type)*>>()? {
                    $($prev_match)*
                })
            }
        }
    };

    // Conclude single variant, continue to next
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][$($next_variant_source_code:tt)*]
     [$($prev_type:tt)*][$($prev_match:tt)*]
     $variant:ident []
     [$($prev_match_type:tt)*]:[$($prev_match_pat:tt)*]:[$($prev_output_map:tt)*]) => {
        materialize!(
            ++$sum_name $current_variant_name enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($next_variant_source_code)*]
            [$($prev_type)* ($($prev_match_type)*),][$($prev_match)* $current_variant_name (($($prev_match_pat)*)) => Self::$variant ($($prev_names)* $($prev_output_map)*),]
        );
    };
    // Handle variant erring peek
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][$($next_variant_source_code:tt)*]
     [$($prev_type:tt)*][$($prev_match:tt)*]
     $variant:ident [<- $ty:ty$(; $($variant_source_code:tt)*)?]
     [$($prev_match_type:tt)*]:[$($prev_match_pat:tt)*]:[$($prev_output_map:tt)*]) => {
        materialize!(
            ++$sum_name $current_variant_name enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($next_variant_source_code)*]
            [$($prev_type)*][$($prev_match)*]
            $variant [$($($variant_source_code)*)?]
            [$($prev_match_type)* $crate::PeekAsParse<$ty>,]:[$($prev_match_pat)* _,]:[$($prev_output_map)*]
        );
    };
    // Handle variant peek
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][$($next_variant_source_code:tt)*]
     [$($prev_type:tt)*][$($prev_match:tt)*]
     $variant:ident [$variant_id:ident peek <- $ty:ty $(; $($variant_source_code:tt)*)?]
     [$($prev_match_type:tt)*]:[$($prev_match_pat:tt)*]:[$($prev_output_map:tt)*]) => {
        materialize!(
            ++$sum_name $current_variant_name enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($next_variant_source_code)*]
            [$($prev_type)*][$($prev_match)*]
            $variant [$($($variant_source_code)*)?]
            [$($prev_match_type)* Option<$crate::PeekAsParse<$ty>>,]:[$($prev_match_pat)* $variant_id,]:[$($prev_output_map)* $variant_id.is_some(),]
        );
    };
    // Handle variant mapped parse into
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][$($next_variant_source_code:tt)*]
     [$($prev_type:tt)*][$($prev_match:tt)*]
     $variant:ident [$variant_id:ident <- $ty:ty : $from_ty:ty $(; $($variant_source_code:tt)*)?]
     [$($prev_match_type:tt)*]:[$($prev_match_pat:tt)*]:[$($prev_output_map:tt)*]) => {
        materialize!(
            ++$sum_name $current_variant_name enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($next_variant_source_code)*]
            [$($prev_type)*][$($prev_match)*]
            $variant [$($($variant_source_code)*)?]
            [$($prev_match_type)* $from_ty,]:[$($prev_match_pat)* $variant_id,]:[$($prev_output_map)* $variant_id.into(),]
        );
    };
    // Handle variant mapped parse
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][$($next_variant_source_code:tt)*]
     [$($prev_type:tt)*][$($prev_match:tt)*]
     $variant:ident [$variant_id:ident <- $ty:ty : $from_ty:ty {$($conversion:tt)*}$(; $($variant_source_code:tt)*)?]
     [$($prev_match_type:tt)*]:[$($prev_match_pat:tt)*]:[$($prev_output_map:tt)*]) => {
        materialize!(
            ++$sum_name $current_variant_name enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($next_variant_source_code)*]
            [$($prev_type)*][$($prev_match)*]
            $variant [$($($variant_source_code)*)?]
            [$($prev_match_type)* $from_ty,]:[$($prev_match_pat)* $variant_id,]:[$($prev_output_map)* {$($conversion)*},]
        );
    };
    // Handle variant parse
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][$($next_variant_source_code:tt)*]
     [$($prev_type:tt)*][$($prev_match:tt)*]
     $variant:ident [$variant_id:ident <- $ty:ty$(; $($variant_source_code:tt)*)?]
     [$($prev_match_type:tt)*]:[$($prev_match_pat:tt)*]:[$($prev_output_map:tt)*]) => {
        materialize!(
            ++$sum_name $current_variant_name enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($next_variant_source_code)*]
            [$($prev_type)*][$($prev_match)*]
            $variant [$($($variant_source_code)*)?]
            [$($prev_match_type)* $ty,]:[$($prev_match_pat)* $variant_id,]:[$($prev_output_map)* $variant_id,]
        );
    };
    // Entry for individual variant
    (++$sum_name:ident $current_variant_name:ident enum
     $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident
     [$($prev_parse:tt)*]:[$($prev_names:tt)*][$variant:ident ($($variant_source_code:tt)*) $($variant_next:tt)*]
     [$($prev_type:tt)*][$($prev_match:tt)*]
    ) => {
        materialize!(
            +++$sum_name $current_variant_name enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($variant_next)*]
            [$($prev_type)*][$($prev_match)*]
            $variant [$($variant_source_code)*]
            []:[]:[]
        );
    };

    // Basic run length counting, succ
    (+++Sum0 NO $($next:tt)*)=> { materialize!(++Sum1 V0 $($next)*); };
    (+++Sum1 V0 $($next:tt)*)=> { materialize!(++Sum2 V1 $($next)*); };
    (+++Sum2 V1 $($next:tt)*)=> { materialize!(++Sum3 V2 $($next)*); };
    (+++Sum3 V2 $($next:tt)*)=> { materialize!(++Sum4 V3 $($next)*); };
    (+++Sum4 V3 $($next:tt)*)=> { materialize!(++Sum5 V4 $($next)*); };
    (+++Sum5 V4 $($next:tt)*)=> { materialize!(++Sum6 V5 $($next)*); };
    (+++Sum6 V5 $($next:tt)*)=> { materialize!(++Sum7 V6 $($next)*); };
    (+++Sum7 V6 $($next:tt)*)=> { materialize!(++Sum8 V7 $($next)*); };
    (+++Sum8 V7 $($next:tt)*)=> { materialize!(++Sum9 V8 $($next)*); };
    (+++Sum9 V8 $($next:tt)*)=> { materialize!(++Sum10 V9 $($next)*); };
    (+++Sum10 V9 $($next:tt)*)=> { materialize!(++Sum11 V10 $($next)*); };
    (+++Sum11 V10 $($next:tt)*)=> { materialize!(++Sum12 V11 $($next)*); };
    (+++Sum12 V11 $($next:tt)*)=> { materialize!(++Sum13 V12 $($next)*); };
    (+++Sum13 V12 $($next:tt)*)=> { materialize!(++Sum14 V13 $($next)*); };
    (+++Sum14 V13 $($next:tt)*)=> { materialize!(++Sum15 V14 $($next)*); };
    (+++Sum15 V14 $($next:tt)*)=> { materialize!(++Sum16 V15 $($next)*); };

    // <shared>
    // Conclude shared parsing, continue to variants
    (+enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)?
     $input:ident[$($prev_parse:tt)*]:[$($prev_names:tt)*][][$($variant_source_code:tt)*]) => {
        materialize!(
            ++Sum0 NO enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)? $input
            [$($prev_parse)*]:[$($prev_names)*][$($variant_source_code)*]
            [][]
        );
    };
    // Handle parsing of shared erring peek
    (+enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident[$($prev_parse:tt)*]:[$($prev_names:tt)*][<- $ty:ty$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(
            +enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            $input[$($prev_parse)* $input.errored_peek::<$ty>()?;]:[$($prev_names)*]
            [$($($src_next)*)?][$($variant_source_code)*]
        );
    };
    // Handle parsing of shared mapped parse into
    (+enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident[$($prev_parse:tt)*]:[$($prev_names:tt)*][$p_name:ident<- $ty:ty : $from_ty:ty $(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(
            +enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            $input[$($prev_parse)* let $p_name: $ty = $input.parse::<$from_ty>()?.into();]:[$($prev_names)* $p_name,]
            [$($($src_next)*)?][$($variant_source_code)*]
        );
    };
    // Handle parsing of shared mapped parse
    (+enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident[$($prev_parse:tt)*]:[$($prev_names:tt)*][$p_name:ident<- $ty:ty : $from_ty:ty {$($conversion:tt)*}$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(
            +enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            $input[$($prev_parse)* let $p_name: $ty = {let $p_name: $from_ty = $input.parse()?; $($conversion)*};]:[$($prev_names)* $p_name,]
            [$($($src_next)*)?][$($variant_source_code)*]
        );
    };
    // Handle parsing of shared parse
    (+enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident[$($prev_parse:tt)*]:[$($prev_names:tt)*][$p_name:ident<- $ty:ty$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(
            +enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            $input[$($prev_parse)* let $p_name:$ty = $input.parse()?;]:[$($prev_names)* $p_name,]
            [$($($src_next)*)?][$($variant_source_code)*]
        );
    };
    // Handle parsing of shared peek
    (+enum $(#[$($macro_input:tt)+])* $vis:vis $id:ident $(<$($gen:ident),*>)? $input:ident[$($prev_parse:tt)*]:[$($prev_names:tt)*][$p_name:ident peek <- $ty:ty$(; $($src_next:tt)*)?][$($variant_source_code:tt)*]) => {
        materialize!(
            +enum $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            $input[$($prev_parse)* let $p_name = $input.peek::<$ty>();]:[$($prev_names)* $p_name,]
            [$($($src_next)*)?][$($variant_source_code)*]
        );
    };
    // </shared>
    // </enum parsing>
    // Enum entry
    ($(#[$($macro_input:tt)+])*
     $vis:vis enum $id:ident$(<$($gen:ident),*>)? $([$($source_code:tt)*])? {$($variant:ident($($variant_source_code:tt)*))*}) => {
        materialize!(
            !enum $(#[$($macro_input)+])*
            $vis $id
            $(<$($gen),*>)?[][$($($source_code)*)?]
            [$($variant($($variant_source_code)*))*]
        );

        materialize!(
            +enum $(#[$($macro_input)+])*
            $vis $id
            $(<$($gen),*>)? input []:[][$($($source_code)*)?]
            [$($variant($($variant_source_code)*))*]
        );
    };

    // </enum>
    // <Struct>
    // <Struct Building>
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*][]) => {
        $(#[$($macro_input)+])*
        $vis struct $id $(<$($gen),*>)? {$($prev)*}
    };
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*][$var:ident peek <- $ty:ty $(: $from_ty:ty {$($convert:tt)*})?$(; $($next:tt)*)?]) => {
        materialize!(
            !struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            [$($prev)* $vis $var: bool,]
            [$($($next)*)?]
        );
    };
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*][$var:ident <- $ty:ty $(: $from_ty:ty $({$($convert:tt)*})?)?$(; $($next:tt)*)?]) => {
        materialize!(
            !struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            [$($prev)* $vis $var: $ty,]
            [$($($next)*)?]
        );
    };
    (!struct $(#[$($macro_input:tt)+])* $vis:vis $id:ident$(<$($gen:ident),*>)?[$($prev:tt)*][<- $ty:ty$(; $($next:tt)*)?]) => {
        materialize!(
            !struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?
            [$($prev)*]
            [$($($next)*)?]
        );
    };
    // </Struct Building>
    // <Parser Building>
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*][]) => {
        impl<'a, $($($gen: $crate::Parse<$crate::Cursor<'a>>),*)?> $crate::Parse<$crate::Cursor<'a>> for $id$(<$($gen),*>)? {
            fn parse($input: &mut $crate::ParseBuffer<$crate::Cursor<'a>>) -> $crate::Result<Self> {
                $($prev_main)*

                Ok(Self {
                    $($prev_self)*
                })
            }
        }
    };
    // match mapped parse convert
    (+struct $id:ident $input:ident$(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*][$var:ident <- $ty:ty : $from_ty:ty $(; $($next:tt)*)?]) => {
        materialize!(
            +struct $id $input $(<$($gen),*>)?
            [$($prev_self)* $var,][$($prev_main)* let $var: $ty = $input.parse::<$from_ty>()?.into();]
            [$($($next)*)?]
        );
    };
    // match mapped parse
    (+struct $id:ident $input:ident$(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*][$var:ident <- $ty:ty : $from_ty:ty {$($convert:tt)*}$(; $($next:tt)*)?]) => {
        materialize!(
            +struct $id $input $(<$($gen),*>)?
            [$($prev_self)* $var,][$($prev_main)* let $var = {let $var = $input.parse::<$from_ty>()?; {$($convert)*}};]
            [$($($next)*)?]
        );
    };
    // match peek
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*][$var:ident peek <- $ty:ty$(; $($next:tt)*)?]) => {
        materialize!(
            +struct $id $input $(<$($gen),*>)?
            [$($prev_self)* $var,][$($prev_main)* let $var = $input.peek::<$ty>();]
            [$($($next)*)?]
        );
    };
    // match parse
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*][$var:ident <- $ty:ty$(; $($next:tt)*)?]) => {
        materialize!(
            +struct $id $input $(<$($gen),*>)?
            [$($prev_self)* $var,][$($prev_main)* let $var = $input.parse::<$ty>()?;]
            [$($($next)*)?]
        );
    };
    // match erring peek
    (+struct $id:ident $input:ident $(<$($gen:ident),*>)?[$($prev_self:tt)*][$($prev_main:tt)*][<- $ty:ty$(; $($next:tt)*)?]) => {
        materialize!(
            +struct $id $input $(<$($gen),*>)?
            [$($prev_self)*][$($prev_main)* $input.errored_peek::<$ty>()?;]
            [$($($next)*)?]
        );
    };
    // </Parser Building>
    // Entry
    ($(#[$($macro_input:tt)+])* $vis:vis struct $id:ident$(<$($gen:ident),*>)? {$($source_code:tt)*}) => {
        materialize!(!struct $(#[$($macro_input)+])* $vis $id $(<$($gen),*>)?[][$($source_code)*]);
        materialize!(+struct $id input $(<$($gen),*>)?[][][$($source_code)*]);
    };
    // </Struct>
}

#[cfg(test)]
mod tests {
    use proc_macro2::Ident;

    materialize! {
        #[derive(Debug)]
        pub enum Hello [
            hi0 <- Ident;
            <- Ident;
            hi1 <- Ident : Ident { hi1 };
            hi2 peek <- Ident
        ] {
            Hi(
                hi3 <- Ident;
                <- Ident;
                hi4 <- Ident : Ident { hi4 };
                hi5 <- Ident;
                hi6 peek <- Ident;
            )
            World()
        }
    }

    materialize! {
        pub struct Hi {
            hi0 <- Ident;
            <- Ident;
            hi1 <- Ident : Ident { hi1 };
            hi2 peek <- Ident
        }
    }
}
