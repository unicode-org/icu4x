// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_export]
macro_rules! impl_ule_from_array {
    ($aligned:ty, $unaligned:ty, $single:path) => {
        #[doc = concat!("Convert an array of `", stringify!($aligned), "` to an array of `", stringify!($unaligned), "`.")]
        pub const fn from_array<const N: usize>(arr: [$aligned; N]) -> [Self; N] {
            if N == 0 {
                return unsafe { *(&arr as *const _ as *const [Self; N]) };
            }
            let mut result = [$single(arr[0]); N];
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
    ($aligned:ty, $unaligned:ty) => {
        impl_ule_from_array!($aligned, $unaligned, Self::from_aligned);
    };
}
