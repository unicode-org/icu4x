// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Generates a test that checks the stack size of an item and a macro
/// that should be used with `#[doc]` to document it.
///
/// ```ignore
/// size_test!(MyType, my_type_size, 32);
///
/// // Add this annotation to the type's docs:
/// #[doc = my_type_size!()]
/// ```
///
/// The size should correspond to the Rust version in rust-toolchain.toml.
///
/// If the size on latest nightly differs from rust-toolchain.toml, use the
/// named arguments version of this macro to specify both sizes:
///
/// ```ignore
/// size_test!(MyType, my_type_size, pinned = 32, nightly = 24);
/// ```
///
/// The test is ignored by default but runs in CI. To run the test locally,
/// run `cargo test -- --include-ignored`
macro_rules! size_test {
    ($ty:ty, $id:ident, pinned = $pinned:literal, nightly = $nightly:literal) => {
        macro_rules! $id {
            () => {
                concat!(
                    "📏 This item has a stack size of <b>",
                    stringify!($pinned),
                    " bytes</b> in the default ICU4X Rust stable toolchain and <b>",
                    stringify!($nightly),
                    " bytes</b> on nightly at release date."
                )
            };
        }
        #[test]
        #[cfg_attr(not(icu4x_run_size_tests), ignore)]
        fn $id() {
            let size = core::mem::size_of::<$ty>();
            let success = if cfg!(not(icu4x_run_size_tests)) {
                // Manual invocation: match either size
                matches!(size, $pinned | $nightly)
            } else if option_env!("ICU4X_BUILDING_WITH_FORCED_NIGHTLY").is_some() {
                // CI invocation: match nightly size
                size == $nightly
            } else {
                // CI invocation: match pinned stable size
                size == $pinned
            };
            assert!(
                success,
                "size_of {} = {}.\n** To reproduce this failure, run `cargo test -- --ignored` **",
                stringify!($ty),
                size,
            );
        }
    };
    ($ty:ty, $id:ident, $size:literal) => {
        macro_rules! $id {
            () => {
                concat!(
                    "📏 This item has a stack size of <b>",
                    stringify!($size),
                    " bytes</b> in the default ICU4X Rust stable toolchain."
                )
            };
        }
        #[test]
        #[cfg_attr(not(icu4x_run_size_tests), ignore)]
        fn $id() {
            let size = core::mem::size_of::<$ty>();
            let expected = $size;
            assert_eq!(
                size,
                expected,
                "size_of {} = {}.\n** To reproduce this failure, run `cargo test -- --ignored` **",
                stringify!($ty),
                size,
            );
        }
    };
}
