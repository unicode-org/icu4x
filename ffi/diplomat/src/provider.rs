// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    use icu_provider::prelude::BufferProvider;
    use icu_provider_blob::BlobDataProvider;
    use icu_provider_blob::StaticDataProvider;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct ICU4XDataProvider(pub Box<dyn BufferProvider + 'static>);

    /// A result type for `ICU4XDataProvider::create`.
    pub struct ICU4XCreateDataProviderResult {
        /// Will be `None` if `success` is `false`, do not use in that case.
        pub provider: Option<Box<ICU4XDataProvider>>,
        // May potentially add a better error type in the future
        pub success: bool,
    }

    fn convert_provider<D: BufferProvider + 'static>(x: D) -> Box<ICU4XDataProvider> {
        Box::new(ICU4XDataProvider(Box::new(x)))
    }

    impl ICU4XDataProvider {
        /// Constructs an `FsDataProvider` and returns it as an [`ICU4XDataProvider`].
        /// Requires the `provider_fs` feature.
        /// Not supported in WASM.
        #[diplomat::rust_link(icu_provider_fs::FsDataProvider, Struct)]
        #[allow(unused_variables)]
        pub fn create_fs(path: &str) -> DiplomatResult<Box<ICU4XDataProvider>, ICU4XError> {
            #[cfg(not(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            )))]
            unimplemented!();

            #[cfg(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            ))]
            icu_provider_fs::FsDataProvider::try_new(path)
                .map_err(Into::into)
                .map(convert_provider)
                .into()
        }

        /// Constructs a testdata provider and returns it as an [`ICU4XDataProvider`].
        /// Requires the `provider_test` feature.
        #[diplomat::rust_link(icu_testdata, Mod)]
        pub fn create_test() -> Box<ICU4XDataProvider> {
            #[cfg(not(feature = "provider_test"))]
            unimplemented!();

            #[cfg(feature = "provider_test")]
            if cfg!(feature = "smaller_test") {
                convert_provider(icu_testdata::get_smaller_postcard_provider())
            } else {
                convert_provider(icu_testdata::get_postcard_provider())
            }
        }

        /// Constructs a `BlobDataProvider` and returns it as an [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_provider_blob::BlobDataProvider, Struct)]
        pub fn create_from_byte_slice(
            blob: &[u8],
        ) -> DiplomatResult<Box<ICU4XDataProvider>, ICU4XError> {
            BlobDataProvider::new_from_blob(blob)
                .map_err(Into::into)
                .map(convert_provider)
                .into()
        }

        /// Constructs an empty `StaticDataProvider` and returns it as an [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_provider_blob::StaticDataProvider, Struct)]
        pub fn create_empty() -> Box<ICU4XDataProvider> {
            convert_provider(StaticDataProvider::new_empty())
        }
    }
}
