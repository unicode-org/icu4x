// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Generates a test that checks the stack size of an item and a macro
/// that should be used with [`doc_size!`] to document it.
macro_rules! check_size {
    ($ty:ty, $id:ident, $sizes:pat) => {
        macro_rules! $id {
            () => {
                stringify!($sizes)
            };
        }
        #[test]
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

/// Documents the stack size for an item tested with [`check_size!`].
macro_rules! doc_size {
    ($id:ident) => {
        concat!("📏 This item has a stack size of ", $id!(), " bytes")
    };
}
