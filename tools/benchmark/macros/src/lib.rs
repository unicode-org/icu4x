// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]

/// Re-export tools so that macros can access it without needing a dependency.
#[cfg(feature = "benchmark_memory")]
pub use dhat;
#[cfg(all(
    feature = "rust_global_allocator",
    any(target_os = "linux", target_os = "macos", target_arch = "wasm32")
))]
pub use dlmalloc;

#[macro_export]
macro_rules! println_noop {
    ($($arg:tt)*) => {{}};
}

// If no features or all features are present, do nothing
#[cfg(any(
    not(any(feature = "benchmark_memory", feature = "rust_global_allocator")),
    all(feature = "benchmark_memory", feature = "rust_global_allocator"),
))]
#[macro_export]
macro_rules! bench {
    (fn main() $main:block) => {
        #[cfg(not(debug_assertions))]
        use $crate::println_noop as println;
        #[cfg_attr(not(debug_assertions), allow(unused_variables))]
        #[no_mangle]
        fn main(_argc: isize, _argv: *const *const u8) -> isize {
            let () = $main;
            0
        }
    };
}

// Enable DHat if the "benchmark_memory" feature is present alone
#[cfg(all(feature = "benchmark_memory", not(feature = "rust_global_allocator")))]
#[macro_export]
macro_rules! bench {
    (fn main() $main:block) => {
        use $crate::dhat;
        #[cfg(not(debug_assertions))]
        use $crate::println_noop as println;
        // Use the dhat global allocator to instrument memory usage.
        #[global_allocator]
        static ALLOCATOR: dhat::Alloc = dhat::Alloc;

        #[cfg_attr(not(debug_assertions), allow(unused_variables))]
        #[no_mangle]
        fn main(_argc: isize, _argv: *const *const u8) -> isize {
            // The dhat instance will be alive for the life of the main function, and when dropped,
            // it will output heap usage information.
            let _profiler = dhat::Profiler::new_heap();
            let () = $main;
            0
        }
    };
}

// Enable Dlmalloc if the "rust_global_allocator" feature is present alone
#[cfg(all(feature = "rust_global_allocator", not(feature = "benchmark_memory")))]
#[macro_export]
macro_rules! bench {
    (fn main() $main:block) => {
        use $crate::dlmalloc::GlobalDlmalloc;
        #[cfg(not(debug_assertions))]
        use $crate::println_noop as println;
        // Use Dlmalloc to remove the system allocator dependency
        #[global_allocator]
        static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;

        #[cfg_attr(not(debug_assertions), allow(unused_variables))]
        #[no_mangle]
        fn main(_argc: isize, _argv: *const *const u8) -> isize {
            let () = $main;
            0
        }
    };
}
