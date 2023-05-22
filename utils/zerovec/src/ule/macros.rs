// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_export]
macro_rules! impl_ule_from_array {
    ($source:ty, $dest:ty, $single:path, $zero:expr) => {
        /// Convert an array of `$source` to an array of `$dest`.
        pub const fn from_array<const N: usize>(arr: [$source; N]) -> [Self; N] {
            let mut result = [$zero; N];
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
}
