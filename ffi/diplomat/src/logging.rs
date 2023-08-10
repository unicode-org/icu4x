// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[diplomat::opaque]
    /// An object allowing control over the logging used
    pub struct ICU4XLogger;

    impl ICU4XLogger {
        /// Initialize the logger using `simple_logger`
        ///
        /// Requires the `simple_logger` Cargo feature.
        ///
        /// Returns `false` if there was already a logger set.
        #[cfg(all(not(target_arch = "wasm32"), feature = "simple_logger"))]
        pub fn init_simple_logger() -> bool {
            simple_logger::init().is_ok()
        }

        /// Initialize the logger to use the WASM console.
        ///
        /// Only available on `wasm32` targets.
        ///
        /// Returns `false` if there was already a logger set.
        #[cfg(target_arch = "wasm32")]
        pub fn init_console_logger() -> bool {
            // Define a custom `log::Log` that uses the `diplomat_runtime` bindings.
            // TODO: Maybe this logger should be defined in `diplomat_runtime` and use
            // console.{error, warn, log, debug, info} instead of just {warn, log}.
            struct ConsoleLogger;
            impl log::Log for ConsoleLogger {
                fn enabled(&self, _: &log::Metadata) -> bool {
                    true
                }

                fn log(&self, record: &log::Record) {
                    let msg = alloc::format!("[{}] {}", record.level(), record.args());
                    if record.level() <= log::Level::Warn {
                        diplomat_runtime::console_warn(&msg);
                    } else {
                        diplomat_runtime::console_log(&msg);
                    }
                }

                fn flush(&self) {}
            }

            log::set_logger(&ConsoleLogger)
                .map(|()| log::set_max_level(log::LevelFilter::Debug))
                .is_ok()
        }
    }
}
