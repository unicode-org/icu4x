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

    use icu_provider::serde::SerdeDeDataProvider;
    #[cfg(all(
        feature = "provider_fs",
        not(any(target_arch = "wasm32", target_os = "none"))
    ))]
    use icu_provider_fs::FsDataProvider;
    use icu_provider_blob::StaticDataProvider;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider/prelude/trait.DataProvider.html) for more information.
    pub struct ICU4XDataProvider(pub Box<dyn SerdeDeDataProvider + 'static>);

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
                Ok(fs) => {
                    let erased = Box::new(fs);
                    ICU4XCreateDataProviderResult {
                        provider: Some(Box::new(ICU4XDataProvider(erased))),
                        success: true,
                    }
                }
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
                let erased = Box::new(provider);
                ICU4XCreateDataProviderResult {
                    provider: Some(Box::new(ICU4XDataProvider(erased))),
                    success: true,
                }
            }
        }
    }

    #[diplomat::opaque]
    /// An ICU4X data provider backed by static data. This is a specialization of
    /// [`ICU4XDataProvider`] intended to reduce code size.
    ///
    /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html) for more information.
    pub struct ICU4XStaticDataProvider(pub Box<StaticDataProvider>);

    /// A result type for `ICU4XStaticDataProvider::create`.
    pub struct ICU4XCreateStaticDataProviderResult {
        /// Will be `None` if `success` is `false`, do not use in that case.
        pub provider: Option<Box<ICU4XStaticDataProvider>>,
        // May potentially add a better error type in the future
        pub success: bool,
    }

    impl ICU4XStaticDataProvider {
        /// Constructs an `StaticDataProvider` and returns it as an [`ICU4XStaticDataProvider`].
        ///
        /// See [the Rust docs](https://unicode-org.github.io/icu4x-docs/doc/icu_provider_blob/struct.StaticDataProvider.html) for more details.
        pub fn create() -> ICU4XCreateStaticDataProviderResult {
            #[cfg(not(feature = "provider_static"))]
            unimplemented!();

            #[cfg(feature = "provider_static")]
            {
                #[cfg(feature = "smaller_static")]
                let provider = icu_testdata::get_smaller_static_provider();
                #[cfg(not(feature = "smaller_static"))]
                let provider = icu_testdata::get_static_provider();
                let erased = Box::new(provider);
                ICU4XCreateStaticDataProviderResult {
                    provider: Some(Box::new(ICU4XStaticDataProvider(erased))),
                    success: true,
                }
            }
        }
    }
}
