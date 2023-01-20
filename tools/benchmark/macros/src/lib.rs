// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
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

// If no features or all features are present, define empty macros
#[cfg(any(
    not(any(feature = "benchmark_memory", feature = "rust_global_allocator")),
    all(feature = "benchmark_memory", feature = "rust_global_allocator"),
))]

mod default {
    #[macro_export]
    macro_rules! static_setup {
        () => {};
    }
    #[macro_export]
    macro_rules! main_setup {
        () => {};
    }
}

// Enable DHat if the "benchmark_memory" feature is present alone
#[cfg(all(feature = "benchmark_memory", not(feature = "rust_global_allocator")))]
pub mod benchmark_memory {
    /// Re-export tools so that macros can access it without needing a dependency.
    pub use dhat;
    #[macro_export]
    macro_rules! static_setup {
        () => {
            use $crate::benchmark_memory::dhat;
            // Use the dhat global allocator to instrument memory usage.
            #[global_allocator]
            static ALLOCATOR: dhat::Alloc = dhat::Alloc;
        };
    }
    #[macro_export]
    macro_rules! main_setup {
        () => {
            // The dhat instance will be alive for the life of the main function, and when dropped,
            // it will output heap usage information.
            let _profiler = dhat::Profiler::new_heap();
            eprintln!("Feature");
        };
    }
}

// Enable Dlmalloc if the "rust_global_allocator" feature is present alone
#[cfg(all(feature = "rust_global_allocator", not(feature = "benchmark_memory")))]
pub mod rust_global_allocator {
    /// Re-export tools so that macros can access it without needing a dependency.
    pub use dlmalloc;
    #[macro_export]
    macro_rules! static_setup {
        () => {
            use $crate::rust_global_allocator::dlmalloc::GlobalDlmalloc;
            // Use Dlmalloc to remove the system allocator dependency
            #[global_allocator]
            static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;
        };
    }
    #[macro_export]
    macro_rules! main_setup {
        () => {};
    }
}
