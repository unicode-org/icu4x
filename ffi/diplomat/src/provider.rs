// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)] // feature-specific
use alloc::boxed::Box;
use icu_provider::prelude::*;
#[allow(unused_imports)] // feature-specific
use icu_provider::RcWrapBounds;
#[allow(unused_imports)] // feature-specific
use yoke::{trait_hack::YokeTraitHack, Yokeable};
#[allow(unused_imports)] // feature-specific
use zerofrom::ZeroFrom;

pub enum ICU4XDataProviderInner {
    #[cfg(feature = "any_provider")]
    Any(Box<dyn AnyProvider + 'static>),
    #[cfg(feature = "buffer_provider")]
    Buffer(Box<dyn BufferProvider + 'static>),
}

#[diplomat::bridge]
pub mod ffi {
    use crate::errors::ffi::ICU4XError;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct ICU4XDataProvider(pub super::ICU4XDataProviderInner);

    /// A result type for `ICU4XDataProvider::create`.
    pub struct ICU4XCreateDataProviderResult {
        /// Will be `None` if `success` is `false`, do not use in that case.
        pub provider: Option<Box<ICU4XDataProvider>>,
        // May potentially add a better error type in the future
        pub success: bool,
    }

    #[cfg(feature = "any_provider")]
    fn convert_any_provider<D: icu_provider::AnyProvider + 'static>(
        x: D,
    ) -> Box<ICU4XDataProvider> {
        Box::new(ICU4XDataProvider(
            super::ICU4XDataProviderInner::from_any_provider(x),
        ))
    }

    #[cfg(feature = "buffer_provider")]
    fn convert_buffer_provider<D: icu_provider::BufferProvider + 'static>(
        x: D,
    ) -> Box<ICU4XDataProvider> {
        Box::new(ICU4XDataProvider(
            super::ICU4XDataProviderInner::from_buffer_provider(x),
        ))
    }

    impl ICU4XDataProvider {
        /// Constructs an `FsDataProvider` and returns it as an [`ICU4XDataProvider`].
        /// Requires the `provider_fs` feature.
        /// Not supported in WASM.
        #[diplomat::rust_link(icu_provider_fs::FsDataProvider, Struct)]
        #[allow(unused_variables)] // conditional on features
        pub fn create_fs(path: &str) -> DiplomatResult<Box<ICU4XDataProvider>, ICU4XError> {
            #[cfg(not(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            )))]
            panic!("Requires feature 'provider_fs' (not supported on wasm32)");

            #[cfg(all(
                feature = "provider_fs",
                not(any(target_arch = "wasm32", target_os = "none"))
            ))]
            icu_provider_fs::FsDataProvider::try_new(path)
                .map_err(Into::into)
                .map(convert_buffer_provider)
                .into()
        }

        /// Constructs a testdata provider and returns it as an [`ICU4XDataProvider`].
        /// Requires the `provider_test` feature.
        #[diplomat::rust_link(icu_testdata, Mod)]
        pub fn create_test() -> Box<ICU4XDataProvider> {
            #[cfg(not(feature = "provider_test"))]
            panic!("Requires feature 'provider_test'");

            #[cfg(all(
                feature = "provider_test",
                not(any(feature = "any_provider", feature = "buffer_provider"))
            ))]
            panic!("Requires feature 'any_provider' or 'buffer_provider'");

            #[cfg(all(
                feature = "provider_test",
                feature = "any_provider",
                not(feature = "buffer_provider")
            ))]
            return convert_any_provider(icu_testdata::any());

            #[cfg(all(
                feature = "provider_test",
                feature = "buffer_provider",
                not(feature = "any_provider")
            ))]
            return if cfg!(feature = "smaller_test") {
                convert_buffer_provider(icu_testdata::small_buffer())
            } else {
                convert_buffer_provider(icu_testdata::buffer())
            };

            #[cfg(all(
                feature = "provider_test",
                feature = "any_provider",
                feature = "buffer_provider"
            ))]
            return if cfg!(feature = "smaller_test") {
                convert_buffer_provider(icu_testdata::small_buffer())
            } else {
                convert_any_provider(icu_testdata::any())
            };
        }

        /// Constructs a `BlobDataProvider` and returns it as an [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_provider_blob::BlobDataProvider, Struct)]
        #[allow(unused_variables)] // conditional on features
        pub fn create_from_byte_slice(
            blob: &[u8],
        ) -> DiplomatResult<Box<ICU4XDataProvider>, ICU4XError> {
            #[cfg(not(feature = "buffer_provider"))]
            panic!("Requires feature 'buffer_provider'");

            #[cfg(feature = "buffer_provider")]
            icu_provider_blob::BlobDataProvider::try_new_from_blob(blob)
                .map_err(Into::into)
                .map(convert_buffer_provider)
                .into()
        }

        /// Constructs an empty `StaticDataProvider` and returns it as an [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_provider_blob::StaticDataProvider, Struct)]
        pub fn create_empty() -> Box<ICU4XDataProvider> {
            #[cfg(not(any(feature = "any_provider", feature = "buffer_provider")))]
            panic!("Requires feature 'any_provider' or 'buffer_provider'");

            #[cfg(all(feature = "buffer_provider", not(feature = "any_provider")))]
            return convert_buffer_provider(icu_provider_adapters::empty::EmptyDataProvider::new());

            #[cfg(feature = "any_provider")]
            return convert_any_provider(icu_provider_adapters::empty::EmptyDataProvider::new());
        }
    }
}

#[cfg(not(any(feature = "any_provider", feature = "buffer_provider")))]
impl<M> DataProvider<M> for ICU4XDataProviderInner
where
    M: KeyedDataMarker + 'static,
{
    fn load(&self, _req: DataRequest) -> Result<DataResponse<M>, DataError> {
        panic!("Requires feature 'any_provider' or 'buffer_provider'");
    }
}

#[cfg(all(feature = "buffer_provider", not(feature = "any_provider")))]
impl<M> DataProvider<M> for ICU4XDataProviderInner
where
    M: KeyedDataMarker + 'static,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::Deserialize<'de>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Buffer(buffer_provider) => {
                buffer_provider.as_deserializing().load(req)
            }
        }
    }
}

#[cfg(all(feature = "any_provider", not(feature = "buffer_provider")))]
impl<M> DataProvider<M> for ICU4XDataProviderInner
where
    M: KeyedDataMarker + 'static,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    M::Yokeable: ZeroFrom<'static, M::Yokeable>,
    M::Yokeable: RcWrapBounds,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Any(any_provider) => any_provider.as_downcasting().load(req),
        }
    }
}

#[cfg(all(feature = "buffer_provider", feature = "any_provider"))]
impl<M> DataProvider<M> for ICU4XDataProviderInner
where
    M: KeyedDataMarker + 'static,
    for<'a> YokeTraitHack<<M::Yokeable as Yokeable<'a>>::Output>: Clone,
    M::Yokeable: ZeroFrom<'static, M::Yokeable>,
    M::Yokeable: RcWrapBounds,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::Deserialize<'de>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Any(any_provider) => any_provider.as_downcasting().load(req),
            ICU4XDataProviderInner::Buffer(buffer_provider) => {
                buffer_provider.as_deserializing().load(req)
            }
        }
    }
}

impl ICU4XDataProviderInner {
    #[cfg(feature = "any_provider")]
    fn from_any_provider(any_provider: impl AnyProvider + 'static) -> Self {
        Self::Any(Box::new(any_provider))
    }
    #[cfg(feature = "buffer_provider")]
    fn from_buffer_provider(buffer_provider: impl BufferProvider + 'static) -> Self {
        Self::Buffer(Box::new(buffer_provider))
    }
}
