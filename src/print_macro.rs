#[macro_export]
macro_rules! to_tokens {
    (
        enum $id:ident
        $method:ident $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  []
    ) => {
        match $self { $($output)* }
    };
    (
        enum $id:ident
        into_token_stream $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident () $(, $($next:tt)*)?]
    ) => {
        to_tokens! {
            enum $id
            into_token_stream $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: []
            variant_code:  []

            output: [
                $($output)*
                Self::$variant_id ($($common_names, )* $($variant_names, )*) => {
                    $($common_code)*
                    $($variant_code)*

                    let _ = $tokens;
                },
            ]

            before: []
            munch:  [$($($next)*)?]
        }
    };
    (
        enum $id:ident
        to_tokens $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident () $(, $($next:tt)*)?]
    ) => {
        to_tokens! {
            enum $id
            to_tokens $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: []
            variant_code:  []

            output: [
                $($output)*
                Self::$variant_id ($(ref $common_names, )* $(ref $variant_names, )*) => {
                    $($common_code)*
                    $($variant_code)*

                    let _ = $tokens;
                },
            ]

            before: []
            munch:  [$($($next)*)?]
        }
    };
    (
        enum $id:ident
        into_token_stream $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $tts:ident into {$($conversion:tt)*} to {$($_:tt)*} $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            to_tokens $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)* $val]
            variant_code:  [
                $($variant_code)*
                let $tts = $tokens;
                let $tokens;
                { $($conversion)* }
                $tokens = $tts;
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        to_tokens $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $tts:ident into {$($_:tt)*} to {$($conversion:tt)*} $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            to_tokens $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)* $val]
            variant_code:  [
                $($variant_code)*
                let $tts = $tokens;
                let $tokens;
                { $($conversion)* }
                $tokens = $tts;
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        to_tokens $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $source_ty:ty : $ty:ty $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            to_tokens $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)* $val]
            variant_code:  [
                $($variant_code)*
                $tokens.extend(<$source_ty as Into<$ty>>::into($val.clone()).into_token_stream());
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        into_token_stream $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $source_ty:ty : $ty:ty $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            into_token_stream $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)* $val]
            variant_code:  [
                $($variant_code)*
                $tokens.extend(<$source_ty as Into<$ty>>::into($val).into_token_stream());
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        $method:ident $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident peek <- $ty:ty $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            $method $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)* $val]
            variant_code:  [
                $($variant_code)*
                if $val.clone() {
                    let temp: $ty = Default::default();
                    $tokens.extend(<$ty as quote::ToTokens>::into_token_stream(temp));
                }
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        to_tokens $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $($ty:ty)? $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            to_tokens $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)* $val]
            variant_code:  [
                $($variant_code)*
                $val.to_tokens($tokens);
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        into_token_stream $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident ($val:ident <- $($ty:ty)? $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            into_token_stream $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)* $val]
            variant_code:  [
                $($variant_code)*
                $tokens.extend($val.into_token_stream());
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        $method:ident $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: []
        munch:  [$variant_id:ident (<- $ty:ty $(; $($source_code:tt)*)?) $($next:tt)*]
    ) => {
        to_tokens! {
            enum $id
            $method $self $tokens

            common_names: [$($common_names)*]
            common_code:  [$($common_code)*]

            variant_names: [$($variant_names)*]
            variant_code:  [
                $($variant_code)*
                let temp: $ty = Default::default();
                $tokens.extend(<$ty as quote::ToTokens>::into_token_stream(temp));
            ]

            output: [$($output)*]

            before: []
            munch:  [$variant_id ($($($source_code)*)?) $($next)*]
        }
    };
    (
        enum $id:ident
        to_tokens $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [$val:ident <- $tts:ident into { $($_:tt)* } to { $($conversion:tt)* } $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            into_token_stream $self $tokens

            common_names: [$($common_names)* $val]
            common_code:  [
                $($common_code)*
                let $tts = $tokens;
                let $tokens;
                { $($conversion)* }
                $tokens = $tts;
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        enum $id:ident
        into_token_stream $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [$val:ident <- $tts:ident into { $($conversion:tt)* } to { $($_:tt)* } $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            into_token_stream $self $tokens

            common_names: [$($common_names)* $val]
            common_code:  [
                $($common_code)*
                let $tts = $tokens;
                let $tokens;
                { $($conversion)* }
                let $tokens = $tts
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        enum $id:ident
        into_token_stream $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [$val:ident <- $source_ty:ty : $ty:ty $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            into_token_stream $self $tokens

            common_names: [$($common_names)* $val]
            common_code:  [
                $($common_code)*
                $tokens.extend(<$source_ty as Into<$ty>>::into($val).into_token_stream());
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        enum $id:ident
        to_tokens $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [$val:ident <- $source_ty:ty : $ty:ty $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            to_tokens $self $tokens

            common_names: [$($common_names)* $val]
            common_code:  [
                $($common_code)*
                $tokens.extend(<$source_ty as Into<$ty>>::into($val.clone()).into_token_stream());
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        enum $id:ident
        $method:tt $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [$val:ident peek <- $ty:ty $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            $method $self $tokens

            common_names: [$($common_names)* $val]
            common_code:  [
                $($common_code)*
                if $val.clone() {
                    let temp: $ty = Default::default();
                    $tokens.extend(<$ty as quote::ToTokens>::into_token_stream(temp));
                }
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        enum $id:ident
        into_token_stream $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [$val:ident <- $($ty:ty)? $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            into_token_stream $self $tokens

            common_names: [$($common_names)* $val]
            common_code:  [
                $($common_code)*
                $tokens.extend($val.into_token_stream());
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        enum $id:ident
        to_tokens $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [$val:ident <- $($ty:ty)? $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            to_tokens $self $tokens

            common_names: [$($common_names)* $val]
            common_code:  [
                $($common_code)*
                $val.to_tokens($tokens);
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        enum $id:ident
        $method:ident $self:ident $tokens:ident

        common_names: [$($common_names:ident)*]
        common_code:  [$($common_code:tt)*]

        variant_names: [$($variant_names:ident)*]
        variant_code:  [$($variant_code:tt)*]

        output: [$($output:tt)*]

        before: [<- $ty:ty $(; $($before:tt)*)?]
        munch:  [$($source_code:tt)*]
    ) => {
        to_tokens! {
            enum $id
            $method $self $tokens

            common_names: [$($common_names)*]
            common_code:  [
                $($common_code)*
                let temp: $ty = Default::default();
                $tokens.extend(<$ty as quote::ToTokens>::into_token_stream(temp));
            ]

            variant_names: [$($variant_names)*]
            variant_code:  [$($variant_code)*]

            output: [$($output)*]

            before: [$($($before)*)?]
            munch:  [$($source_code)*]
        }
    };
    (
        $(#[$($macros:tt)*])*
        impl ToTokens for enum $id:ident $(<$($gens:ident),*>)?
        $([$($before:tt)*])?
        { $($source_code:tt)* }
    ) => {
        $(#[$($macros)*])*
        impl$(<$($gens: quote::ToTokens,)*>)? quote::ToTokens for $id$(<$($gens),*>)? {
            fn into_token_stream(self) -> proc_macro2::TokenStream {
                let mut tts = proc_macro2::TokenStream::new();
                let tokens = &mut tts;

                to_tokens! {
                    enum $id
                    into_token_stream self tokens

                    common_names: []
                    common_code:  []

                    variant_names: []
                    variant_code:  []

                    output: []

                    before: [$($($before)*)?]
                    munch:  [$($source_code)*]
                }

                tts
            }

            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                to_tokens! {
                    enum $id
                    to_tokens self tokens

                    common_names: []
                    common_code:  []

                    variant_names: []
                    variant_code:  []

                    output: []

                    before: [$($($before)*)?]
                    munch:  [$($source_code)*]
                }
            }
        }
    };
    (
        struct
        $method:ident $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  []
    ) => {
        $($output)*
    };
    (
        struct
        into_token_stream $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [$val:ident <- $tts:ident into { $($conversion:tt)* } to { $($_:tt)* } $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            to_tokens $self $tokens

            output: [
                $($output)*
                let $tts = $tokens;
                let $tokens;
                { let $val = $self.$val; $($conversion)* }
                $tokens = $tts;
            ]
            munch:  [$($($next)*)?]
        }
    };
    (
        struct
        to_tokens $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [$val:ident <- $tts:ident into { $($_:tt)* } to { $($conversion:tt)* } $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            to_tokens $self $tokens

            output: [
                $($output)*
                let $tts = $tokens;
                let $tokens;
                { let $val = &$self.$val; $($conversion)* }

                $tokens = $tts;
            ]
            munch:  [$($($next)*)?]
        }
    };
    (
        struct
        to_tokens $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [$val:ident <- $source_ty:ty : $ty:ty $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            to_tokens $self $tokens

            output: [
                $($output)*
                $tokens.extend(<$source_ty as Into<$ty>>::into($self.$val.clone()).into_token_stream());
            ]
            munch:  [$($($next)*)?]
        }
    };
    (
        struct
        into_token_stream $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [$val:ident <- $source_ty:ty : $ty:ty $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            to_tokens $self $tokens

            output: [
                $($output)*
                $tokens.extend(<$source_ty as Into<$ty>>::into($self.$val).into_token_stream());
            ]
            munch:  [$($($next)*)?]
        }
    };
    (
        struct
        $method:ident $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [<- $ty:ty $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            $method $self $tokens

            output: [
                $($output)*
                let temp: $ty = Default::default();
                $tokens.extend(<$ty as quote::ToTokens>::into_token_stream(temp));
            ]
            munch:  [$($($next)*)?]
        }
    };
    (
        struct
        $method:ident $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [$val:ident peek <- $ty:ty $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            $method $self $tokens

            output: [
                $($output)*
                if $self.$val.clone() {
                    let temp: $ty = Default::default();
                    $tokens.extend(<$ty as quote::ToTokens>::into_token_stream(temp));
                }
            ]
            munch:  [$($($next)*)?]
        }
    };
    (
        struct
        into_token_stream $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [$val:ident <- $($ty:ty)? $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            to_tokens $self $tokens

            output: [$($output)* $tokens.extend($self.$val.into_token_stream());]
            munch:  [$($($next)*)?]
        }
    };
    (
        struct
        to_tokens $self:ident $tokens:ident

        output: [$($output:tt)*]
        munch:  [$val:ident <- $($ty:ty)? $(; $($next:tt)*)?]
    ) => {
        to_tokens! {
            struct
            to_tokens $self $tokens

            output: [$($output)* $self.$val.to_tokens($tokens);]
            munch:  [$($($next)*)?]
        }
    };
    (
        $(#[$($macros:tt)*])*
        impl ToTokens for struct $id:ident $(<$($gens:ident),*>)? { $($source_code:tt)* }
        ) => {
        $(#[$($macros)*])*
        impl$(<$($gens: quote::ToTokens,)*>)? quote::ToTokens for $id$(<$($gens),*>)? {
            fn into_token_stream(self) -> proc_macro2::TokenStream {
                let mut tts = proc_macro2::TokenStream::new();
                let tokens = &mut tts;

                to_tokens! {
                    struct
                    into_token_stream self tokens

                    output: []
                    munch:  [$($source_code)*]
                }
                let _ = tokens;

                tts
            }

            fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
                // use quote::TokenStreamExt;
                to_tokens! {
                    struct
                    to_tokens self tokens

                    output: []
                    munch:  [$($source_code)*]
                }
                let _ = tokens;
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::*;

    materialize! {
        on <'a> [crate::RustCursor<'a>]
        struct SimpleStruct {
            <- FIdent<"hello">;
            hi0 <- Ident;
            hi1 peek <- Ident;
            hi2 <- Ident : Ident
        }
    }
    to_tokens! {
        impl ToTokens for struct SimpleStruct {
            <- FIdent<"hello">;
            hi0 <- Ident;
            hi1 peek <- FIdent<"hi">;
            hi2 <- Ident : Ident
        }
    }

    materialize! {
        on <'a> [crate::RustCursor<'a>]
        enum SimpleEnum [<- FIdent<"hello">] {
            World(
                hi0 <- Ident;
                hi1 peek <- FIdent<"hi">;
                hi2 <- Ident : Ident
            ),
            Hi(),
        }
    }
    to_tokens! {
        impl ToTokens for enum SimpleEnum [<- FIdent<"hello">] {
            World(
                hi0 <- Ident;
                hi1 peek <- FIdent<"hi">;
                hi2 <- Ident : Ident
            ),
            Hi(),
        }
    }
}
