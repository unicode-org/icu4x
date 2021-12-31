// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    #[cfg(all(
        feature = "provider_fs",
        not(any(target_arch = "wasm32", target_os = "none"))
    ))]
    use alloc::string::ToString;

    use icu_provider::prelude::BufferProvider;
    use icu_provider_blob::BlobDataProvider;
    #[cfg(all(
        feature = "provider_fs",
        not(any(target_arch = "wasm32", target_os = "none"))
    ))]
    use icu_provider_fs::FsDataProvider;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html) for more information.
    pub struct ICU4XDataProvider(pub Box<dyn BufferProvider + 'static>);

    /// A result type for `ICU4XDataProvider::create`.
    pub struct ICU4XCreateDataProviderResult {
        /// Will be `None` if `success` is `false`, do not use in that case.
        pub provider: Option<Box<ICU4XDataProvider>>,
        // May potentially add a better error type in the future
        pub success: bool,
    }

    impl ICU4XDataProvider {
        /// Constructs an `FsDataProvider` and returns it as an [`ICU4XDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_fs/struct.FsDataProvider.html) for more details.
        #[allow(unused_variables)]
        pub fn create_fs(path: &str) -> ICU4XCreateDataProviderResult {
            #[cfg(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            ))]
            match FsDataProvider::try_new(path.to_string()) {
                Ok(fs) => ICU4XCreateDataProviderResult {
                    provider: Some(Box::new(ICU4XDataProvider(Box::new(fs)))),
                    success: true,
                },
                Err(_) => ICU4XCreateDataProviderResult {
                    provider: None,
                    success: false,
                },
            }

            #[cfg(not(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            )))]
            unimplemented!();
        }

        /// Constructs an `StaticDataProvider` and returns it as an [`ICU4XDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html) for more details.
        pub fn create_static() -> ICU4XCreateDataProviderResult {
            #[cfg(not(feature = "provider_static"))]
            unimplemented!();

            #[cfg(feature = "provider_static")]
            {
                #[cfg(feature = "smaller_static")]
                let provider = icu_testdata::get_smaller_static_provider();
                #[cfg(not(feature = "smaller_static"))]
                let provider = icu_testdata::get_static_provider();
                ICU4XCreateDataProviderResult {
                    provider: Some(Box::new(ICU4XDataProvider(Box::new(provider)))),
                    success: true,
                }
            }
        }

        /// Constructs a `BlobDataProvider` and returns it as an [`ICU4XDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.BlobDataProvider.html) for more details.
        pub fn create_from_byte_slice(blob: &[u8]) -> ICU4XCreateDataProviderResult {
            use alloc::rc::Rc;
            match BlobDataProvider::new_from_rc_blob(Rc::from(blob)) {
                Ok(provider) => ICU4XCreateDataProviderResult {
                    provider: Some(Box::new(ICU4XDataProvider(Box::new(provider)))),
                    success: true,
                },
                Err(_) => ICU4XCreateDataProviderResult {
                    provider: None,
                    success: false,
                },
            }
        }

        /// Constructs an empty `StaticDataProvider` and returns it as an [`ICU4XDataProvider`].
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html) for more details.
        pub fn create_empty() -> ICU4XCreateDataProviderResult {
            let provider = StaticDataProvider::new_empty();
            ICU4XCreateDataProviderResult {
                provider: Some(Box::new(ICU4XDataProvider(Box::new(provider)))),
                success: true,
            }
        }
    }
}
