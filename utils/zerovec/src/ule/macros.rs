// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Given `Self` (`$aligned`), `Self::ULE` (`$unaligned`), and a conversion function (`$single` or
/// `Self::from_aligned`), implement `from_array` for arrays of `$aligned` to `$unaligned`.
///
/// The `$default` argument is due to current compiler limitations.
/// Pass any (cheap to construct) value.
#[macro_export]
macro_rules! impl_ule_from_array {
    ($aligned:ty, $unaligned:ty, $default:expr, $single:path, $fn_name:ident) => {
        #[doc = concat!("Convert an array of `", stringify!($aligned), "` to an array of `", stringify!($unaligned), "`.")]
        pub const fn $fn_name<const N: usize>(arr: [$aligned; N]) -> [Self; N] {
            let mut result = [$default; N];
            let mut i = 0;
            // Won't panic because i < N and arr has length N
            #[allow(clippy::indexing_slicing)]
            while i < N {
                result[i] = $single(arr[i]);
                i += 1;
            }
            result
        }
    };
    ($aligned:ty, $unaligned:ty, $default:expr) => {
        $crate::impl_ule_from_array!($aligned, $unaligned, $default, Self::from_aligned, from_array);
    };
}

#[macro_export]
macro_rules! impl_const_as_ule_array {
    ($aligned:ty, $unaligned:ty, $default:expr, $single:path) => {
        #[doc = concat!("Convert an array of `", stringify!($aligned), "` to an array of `", stringify!($unaligned), "`.")]
        #[allow(dead_code)]
        pub const fn aligned_to_unaligned_array<const N: usize>(arr: [$aligned; N]) -> [$unaligned; N] {
            let mut result = [$default; N];
            let mut i = 0;
            // Won't panic because i < N and arr has length N
            #[allow(clippy::indexing_slicing)]
            while i < N {
                result[i] = $single(arr[i]);
                i += 1;
            }
            result
        }
    };
    ($aligned:ty, $unaligned:ty, $default:expr) => {
        $crate::impl_const_as_ule_array!($aligned, $unaligned, $default, Self::aligned_to_unaligned);
    };
}

#[macro_export]
macro_rules! const_ule_conversion_fn {
    (u8) => {
        core::convert::identity
    };
    (i8) => {
        core::convert::identity
    };
    (u16) => {
        <u16 as $crate::ule::AsULE>::ULE::from_unsigned
    };
    (i16) => {
        <i16 as $crate::ule::AsULE>::ULE::from_signed
    };
    (u32) => {
        <u32 as $crate::ule::AsULE>::ULE::from_unsigned
    };
    (i32) => {
        <i32 as $crate::ule::AsULE>::ULE::from_signed
    };
    (u64) => {
        <u64 as $crate::ule::AsULE>::ULE::from_unsigned
    };
    (i64) => {
        <i64 as $crate::ule::AsULE>::ULE::from_signed
    };
    (u128) => {
        <u128 as $crate::ule::AsULE>::ULE::from_unsigned
    };
    (i128) => {
        <i128 as $crate::ule::AsULE>::ULE::from_signed
    };
    (UnvalidatedChar) => {
        <UnvalidatedChar as $crate::ule::AsULE>::ULE::from_unvalidated_char
    };
    ($aligned:ty) => {
        <$aligned as $crate::ule::AsULE>::ULE::from_aligned
    };
}
