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
macro_rules! println {
    ($lit:literal $(, $arg:expr)* $(,)?) => {{
        /// For binary size benchmarks we don't want to include `std::fmt::Write` machinery,
        /// which `println!` pulls in, but we do want to actually evaluate the arguments.
        #[cfg(not(debug_assertions))]
        {
            struct Sink;
            impl std::fmt::Write for Sink {
                fn write_str(&mut self, s: &str) -> Result<(), std::fmt::Error> {
                    std::hint::black_box(s);
                    Ok(())
                }
            }

            $(
                let _infallible = writeable::Writeable::write_to(&$arg, &mut Sink);
            )*
        }
        #[cfg(debug_assertions)]
        {
            std::println!($lit, $($arg,)*);
        }
    }};
}

// If no features or all features are present, do nothing
#[cfg(any(
    not(any(feature = "benchmark_memory", feature = "rust_global_allocator")),
    all(feature = "benchmark_memory", feature = "rust_global_allocator"),
))]
#[macro_export]
macro_rules! instrument {
    () => {
        const _: () = {
            #[no_mangle]
            fn main(_argc: isize, _argv: *const *const u8) -> isize {
                self::main();
                0
            }
        };
    };
}

// Enable DHat if the "benchmark_memory" feature is present alone
#[cfg(all(feature = "benchmark_memory", not(feature = "rust_global_allocator")))]
#[macro_export]
macro_rules! instrument {
    () => {
        use $crate::dhat;
        // Use the dhat global allocator to instrument memory usage.
        #[global_allocator]
        static ALLOCATOR: dhat::Alloc = dhat::Alloc;

        const _: () = {
            #[no_mangle]
            fn main(_argc: isize, _argv: *const *const u8) -> isize {
                // The dhat instance will be alive for the life of the main function, and when dropped,
                // it will output heap usage information.
                let _profiler = $crate::dhat::Profiler::new_heap();
                self::main();
                0
            }
        };
    };
}

// Enable Dlmalloc if the "rust_global_allocator" feature is present alone
#[cfg(all(feature = "rust_global_allocator", not(feature = "benchmark_memory")))]
#[macro_export]
macro_rules! instrument {
    () => {
        use $crate::dlmalloc::GlobalDlmalloc;
        // Use Dlmalloc to remove the system allocator dependency
        #[global_allocator]
        static ALLOCATOR: GlobalDlmalloc = GlobalDlmalloc;

        const _: () = {
            #[no_mangle]
            fn main(_argc: isize, _argv: *const *const u8) -> isize {
                self::main();
                0
            }
        };
    };
}
