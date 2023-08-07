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
        /// Initialize the logger using `simple_logger`, or console.log/warn in WASM.
        ///
        /// Returns `false` if there was already a logger set, or if logging has not been
        /// compiled into the platform
        pub fn init_simple_logger() -> bool {
            #[cfg(all(not(target_arch = "wasm32"), not(feature = "simple_logger")))]
            return false;

            #[cfg(all(not(target_arch = "wasm32"), feature = "simple_logger"))]
            return simple_logger::init().is_ok();

            #[cfg(target_arch = "wasm32")]
            {
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

                return log::set_logger(&ConsoleLogger)
                    .map(|()| log::set_max_level(log::LevelFilter::Debug))
                    .is_ok();
            }
        }
    }
}
