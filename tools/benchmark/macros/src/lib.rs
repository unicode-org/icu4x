// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Re-export tools so that macros can access it without needing a dependency.
#[cfg(feature = "benchmark_memory")]
pub use dhat;
#[cfg(feature = "rust_global_allocator")]
pub use dlmalloc;

#[cfg(not(any(feature = "benchmark_memory", feature = "rust_global_allocator")))]
#[macro_export]
macro_rules! static_setup {
    () => {};
}

#[cfg(all(feature = "benchmark_memory", feature = "rust_global_allocator"))]
#[macro_export]
macro_rules! static_setup {
    () => {};
}

#[cfg(all(feature = "benchmark_memory", not(feature = "rust_global_allocator")))]
#[macro_export]
macro_rules! static_setup {
    () => {
        use icu_benchmark_macros::dhat::{Dhat, DhatAlloc};

        // Use the DhatAlloc global allocator to instrument memory usage.
        #[global_allocator]
        static ALLOCATOR: DhatAlloc = DhatAlloc;
    };
}

#[cfg(all(feature = "rust_global_allocator", not(feature = "benchmark_memory")))]
#[macro_export]
macro_rules! static_setup {
    () => {
        use icu_benchmark_macros::dlmalloc::GlobalDlmalloc;

        // Use Dlmalloc to remove the system allocator dependency
        #[global_allocator]
        static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;
    };
}

#[cfg(not(feature = "benchmark_memory"))]
#[macro_export]
macro_rules! main_setup {
    () => {};
}

#[cfg(feature = "benchmark_memory")]
#[macro_export]
macro_rules! main_setup {
    () => {
        // The dhat instance will be alive for the life of the main function, and when dropped,
        // it will output heap usage information.
        let _dhat = Dhat::start_heap_profiling();
        eprintln!("Feature");
    };
}
