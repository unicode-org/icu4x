// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(unused_imports)] // feature-specific
use alloc::boxed::Box;
use icu_provider::prelude::*;
#[allow(unused_imports)] // feature-specific
use icu_provider::MaybeSendSync;
use icu_provider_adapters::empty::EmptyDataProvider;
#[allow(unused_imports)] // feature-specific
use yoke::{trait_hack::YokeTraitHack, Yokeable};
#[allow(unused_imports)] // feature-specific
use zerofrom::ZeroFrom;

pub enum ICU4XDataProviderInner {
    Empty,
    #[cfg(feature = "any_provider")]
    Any(Box<dyn AnyProvider + 'static>),
    #[cfg(feature = "buffer_provider")]
    Buffer(Box<dyn BufferProvider + 'static>),
}

impl Default for ICU4XDataProviderInner {
    fn default() -> Self {
        Self::Empty
    }
}

#[diplomat::bridge]
pub mod ffi {
    use super::ICU4XDataProviderInner;
    use crate::errors::ffi::ICU4XError;
    use crate::fallbacker::ffi::ICU4XLocaleFallbacker;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    #[allow(unused_imports)] // feature-gated
    use icu_provider_adapters::fallback::LocaleFallbackProvider;
    #[allow(unused_imports)] // feature-gated
    use icu_provider_adapters::fork::predicates::MissingLocalePredicate;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct ICU4XDataProvider(pub ICU4XDataProviderInner);

    /// A result type for `ICU4XDataProvider::create`.
    pub struct ICU4XCreateDataProviderResult {
        /// Will be `None` if `success` is `false`, do not use in that case.
        pub provider: Option<Box<ICU4XDataProvider>>,
        // May potentially add a better error type in the future
        pub success: bool,
    }

    #[cfg(feature = "any_provider")]
    #[allow(dead_code)] // feature-specific
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
            {
                // #2520
                // In the future we can start using OsString APIs to support non-utf8 paths
                if let Err(e) = core::str::from_utf8(path.as_bytes()) {
                    crate::errors::log_conversion(&e, ICU4XError::DataIoError);
                    return Err(ICU4XError::DataIoError).into();
                }
                icu_provider_fs::FsDataProvider::try_new(path)
                    .map_err(Into::into)
                    .map(convert_buffer_provider)
                    .into()
            }
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

            #[cfg(all(feature = "provider_test", feature = "any_provider"))]
            return convert_any_provider(icu_testdata::any());

            #[cfg(all(
                feature = "provider_test",
                feature = "buffer_provider",
                not(feature = "any_provider")
            ))]
            return convert_buffer_provider(icu_testdata::buffer());
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
            icu_provider_blob::BlobDataProvider::try_new_from_blob(Box::from(blob)) // allocates
                .map_err(Into::into)
                .map(convert_buffer_provider)
                .into()
        }

        /// Constructs an empty [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_provider_adapters::empty::EmptyDataProvider, Struct)]
        #[diplomat::rust_link(
            icu_provider_adapters::empty::EmptyDataProvider::new,
            FnInStruct,
            hidden
        )]
        pub fn create_empty() -> Box<ICU4XDataProvider> {
            Box::new(ICU4XDataProvider(ICU4XDataProviderInner::Empty))
        }

        /// Creates a provider that tries the current provider and then, if the current provider
        /// doesn't support the data key, another provider `other`.
        ///
        /// This takes ownership of the `other` provider, leaving an empty provider in its place.
        ///
        /// The providers must be the same type (Any or Buffer). This condition is satisfied if
        /// both providers originate from the same constructor, such as `create_from_byte_slice`
        /// or `create_fs`. If the condition is not upheld, a runtime error occurs.
        #[diplomat::rust_link(icu_provider_adapters::fork::ForkByKeyProvider, Typedef)]
        #[diplomat::rust_link(
            icu_provider_adapters::fork::predicates::MissingDataKeyPredicate,
            Struct,
            hidden
        )]
        pub fn fork_by_key(
            &mut self,
            other: &mut ICU4XDataProvider,
        ) -> DiplomatResult<(), ICU4XError> {
            let a = core::mem::take(&mut self.0);
            let b = core::mem::take(&mut other.0);
            match (a, b) {
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Any(a), ICU4XDataProviderInner::Any(b)) => {
                    self.0 = ICU4XDataProviderInner::Any(Box::from(
                        icu_provider_adapters::fork::ForkByKeyProvider::new(a, b),
                    ));
                    Ok(())
                }
                #[cfg(feature = "buffer_provider")]
                (ICU4XDataProviderInner::Buffer(a), ICU4XDataProviderInner::Buffer(b)) => {
                    self.0 = ICU4XDataProviderInner::Buffer(Box::from(
                        icu_provider_adapters::fork::ForkByKeyProvider::new(a, b),
                    ));
                    Ok(())
                }
                _ => {
                    let e = ICU4XError::DataMismatchedAnyBufferError;
                    crate::errors::log_conversion(
                        &"fork_by_key must be passed the same type of provider (Any or Buffer)",
                        e,
                    );
                    Err(e)
                }
            }
            .into()
        }

        /// Same as `fork_by_key` but forks by locale instead of key.
        #[diplomat::rust_link(
            icu_provider_adapters::fork::predicates::MissingLocalePredicate,
            Struct
        )]
        pub fn fork_by_locale(
            &mut self,
            other: &mut ICU4XDataProvider,
        ) -> DiplomatResult<(), ICU4XError> {
            let a = core::mem::take(&mut self.0);
            let b = core::mem::take(&mut other.0);
            match (a, b) {
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Any(a), ICU4XDataProviderInner::Any(b)) => {
                    self.0 = ICU4XDataProviderInner::Any(Box::from(
                        icu_provider_adapters::fork::ForkByErrorProvider::new_with_predicate(
                            a,
                            b,
                            MissingLocalePredicate,
                        ),
                    ));
                    Ok(())
                }
                #[cfg(feature = "buffer_provider")]
                (ICU4XDataProviderInner::Buffer(a), ICU4XDataProviderInner::Buffer(b)) => {
                    self.0 = ICU4XDataProviderInner::Buffer(Box::from(
                        icu_provider_adapters::fork::ForkByErrorProvider::new_with_predicate(
                            a,
                            b,
                            MissingLocalePredicate,
                        ),
                    ));
                    Ok(())
                }
                _ => {
                    let e = ICU4XError::DataMismatchedAnyBufferError;
                    crate::errors::log_conversion(
                        &"fork_by_locale must be passed the same type of provider (Any or Buffer)",
                        e,
                    );
                    Err(e)
                }
            }
            .into()
        }

        /// Enables locale fallbacking for data requests made to this provider.
        ///
        /// Note that the test provider (from `create_test`) already has fallbacking enabled.
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackProvider::try_new_unstable,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackProvider,
            Struct,
            compact
        )]
        pub fn enable_locale_fallback(&mut self) -> DiplomatResult<(), ICU4XError> {
            match core::mem::take(&mut self.0) {
                ICU4XDataProviderInner::Empty => Err(icu_provider::DataErrorKind::MissingDataKey
                    .into_error()
                    .into()),
                #[cfg(feature = "any_provider")]
                ICU4XDataProviderInner::Any(inner) => {
                    match LocaleFallbackProvider::try_new_with_any_provider(inner) {
                        Ok(x) => {
                            self.0 = ICU4XDataProviderInner::Any(Box::new(x));
                            Ok(())
                        }
                        Err(e) => Err(e.into()),
                    }
                }
                #[cfg(feature = "buffer_provider")]
                ICU4XDataProviderInner::Buffer(inner) => {
                    match LocaleFallbackProvider::try_new_with_buffer_provider(inner) {
                        Ok(x) => {
                            self.0 = ICU4XDataProviderInner::Buffer(Box::new(x));
                            Ok(())
                        }
                        Err(e) => Err(e.into()),
                    }
                }
            }
            .into()
        }

        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackProvider::new_with_fallbacker,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackProvider,
            Struct,
            compact
        )]
        #[allow(unused_variables)] // feature-gated
        pub fn enable_locale_fallback_with(
            &mut self,
            fallbacker: &ICU4XLocaleFallbacker,
        ) -> DiplomatResult<(), ICU4XError> {
            match core::mem::take(&mut self.0) {
                ICU4XDataProviderInner::Empty => Err(icu_provider::DataErrorKind::MissingDataKey
                    .into_error()
                    .into()),
                #[cfg(feature = "any_provider")]
                ICU4XDataProviderInner::Any(inner) => {
                    self.0 = ICU4XDataProviderInner::Any(Box::new(
                        LocaleFallbackProvider::new_with_fallbacker(inner, fallbacker.0.clone()),
                    ));
                    Ok(())
                }
                #[cfg(feature = "buffer_provider")]
                ICU4XDataProviderInner::Buffer(inner) => {
                    self.0 = ICU4XDataProviderInner::Buffer(Box::new(
                        LocaleFallbackProvider::new_with_fallbacker(inner, fallbacker.0.clone()),
                    ));
                    Ok(())
                }
            }
            .into()
        }
    }
}

#[cfg(not(any(feature = "any_provider", feature = "buffer_provider")))]
impl<M> DataProvider<M> for ICU4XDataProviderInner
where
    M: KeyedDataMarker + 'static,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        EmptyDataProvider::new().load(req)
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
            ICU4XDataProviderInner::Empty => EmptyDataProvider::new().load(req),
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
    M::Yokeable: MaybeSendSync,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Empty => EmptyDataProvider::new().load(req),
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
    M::Yokeable: MaybeSendSync,
    // Actual bound:
    //     for<'de> <M::Yokeable as Yokeable<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> YokeTraitHack<<M::Yokeable as Yokeable<'de>>::Output>: serde::Deserialize<'de>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Empty => EmptyDataProvider::new().load(req),
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
