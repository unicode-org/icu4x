// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Generates a test that checks the stack size of an item and a macro
/// that should be used with `#[doc]` to document it.
///
/// The size should correspond to the Rust version in rust-toolchain.toml.
///
/// The test is ignored by default but runs in CI. To run the test locally,
/// either set `RUSTFLAGS=--cfg=icu4x_check_sizes="true"` or run `cargo test`
/// with `-- --include-ignored`.
macro_rules! check_size {
    ($ty:ty, $id:ident, $sizes:pat) => {
        macro_rules! $id {
            () => {
                concat!("📏 This item has a stack size of ", stringify!($sizes), " bytes")
            };
        }
        #[test]
        #[cfg_attr(not(icu4x_check_sizes = "true"), ignore)]
        fn $id() {
            let size = core::mem::size_of::<$ty>();
            assert!(
                matches!(size, $sizes),
                "size_of {} = {}",
                stringify!($ty),
                size
            );
        }
    };
}
