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
    Destroyed,
    Empty,
    Baked,
    #[cfg(feature = "any_provider")]
    Any(Box<dyn AnyProvider + 'static>),
    #[cfg(feature = "buffer_provider")]
    Buffer(Box<dyn BufferProvider + 'static>),
}

impl Default for ICU4XDataProviderInner {
    fn default() -> Self {
        Self::Destroyed
    }
}

struct BakedProvider;
mod baked {
    include!(concat!(core::env!("ICU4X_FFI_BAKED_ROOT"), "/mod.rs"));
    impl_data_provider!(super::BakedProvider, COMPLETE);
    #[cfg(feature = "any_provider")]
    impl_any_provider!(super::BakedProvider);
}

#[diplomat::bridge]
pub mod ffi {
    #[allow(unused_imports)] // feature-gated
    use super::BakedProvider;
    use super::ICU4XDataProviderInner;
    use crate::errors::ffi::ICU4XError;
    use crate::fallbacker::ffi::ICU4XLocaleFallbacker;
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatResult;
    #[allow(unused_imports)] // feature-gated
    use icu_provider_adapters::empty::EmptyDataProvider;
    #[allow(unused_imports)] // feature-gated
    use icu_provider_adapters::fallback::LocaleFallbackProvider;
    #[allow(unused_imports)] // feature-gated
    use icu_provider_adapters::fork::predicates::MissingLocalePredicate;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct ICU4XDataProvider(pub ICU4XDataProviderInner);

    #[cfg(feature = "any_provider")]
    #[allow(dead_code)] // feature-specific
    fn convert_any_provider<D: icu_provider::AnyProvider + 'static>(x: D) -> ICU4XDataProvider {
        ICU4XDataProvider(super::ICU4XDataProviderInner::Any(Box::new(x)))
    }

    #[cfg(feature = "buffer_provider")]
    fn convert_buffer_provider<D: icu_provider::BufferProvider + 'static>(
        x: D,
    ) -> ICU4XDataProvider {
        ICU4XDataProvider(super::ICU4XDataProviderInner::Buffer(Box::new(x)))
    }

    impl ICU4XDataProvider {
        /// Constructs an `FsDataProvider` and returns it as an [`ICU4XDataProvider`].
        /// Requires the `provider_fs` Cargo feature.
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
                    .map(Box::new)
                    .into()
            }
        }

        /// Constructs a testdata provider and returns it as an [`ICU4XDataProvider`].
        /// Requires the `provider_test` Cargo feature.
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
            return Box::new(convert_any_provider(icu_testdata::any()));

            #[cfg(all(
                feature = "provider_test",
                feature = "buffer_provider",
                not(feature = "any_provider")
            ))]
            return Box::new(convert_buffer_provider(icu_testdata::buffer()));
        }

        /// Constructs a `BlobDataProvider` and returns it as an [`ICU4XDataProvider`].
        #[diplomat::rust_link(icu_provider_blob::BlobDataProvider, Struct)]
        #[allow(unused_variables)] // conditional on features
        pub fn create_from_byte_slice(
            blob: &'static [u8],
        ) -> DiplomatResult<Box<ICU4XDataProvider>, ICU4XError> {
            #[cfg(not(feature = "buffer_provider"))]
            panic!("Requires feature 'buffer_provider'");

            #[cfg(feature = "buffer_provider")]
            icu_provider_blob::BlobDataProvider::try_new_from_static_blob(blob)
                .map_err(Into::into)
                .map(convert_buffer_provider)
                .map(Box::new)
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

        /// Constructs a [`ICU4XDataProvider`] containing baked data.
        ///
        /// When compiling the Rust library, set the `ICU4X_FFI_BAKED_ROOT`
        /// environment variable to the baked data folder.
        ///
        /// If no data is supplied, this behaves like an empty provider.
        pub fn create_baked() -> Box<ICU4XDataProvider> {
            Box::new(ICU4XDataProvider(ICU4XDataProviderInner::Baked))
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
            *self = match (core::mem::take(&mut self.0), core::mem::take(&mut other.0)) {
                (ICU4XDataProviderInner::Empty, b) | (b, ICU4XDataProviderInner::Empty) => {
                    ICU4XDataProvider(b)
                }
                (ICU4XDataProviderInner::Baked, ICU4XDataProviderInner::Baked) => {
                    ICU4XDataProvider(ICU4XDataProviderInner::Baked)
                }
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Any(a), ICU4XDataProviderInner::Baked) => {
                    convert_any_provider(icu_provider_adapters::fork::ForkByKeyProvider::new(
                        a,
                        BakedProvider,
                    ))
                }
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Baked, ICU4XDataProviderInner::Any(b)) => {
                    convert_any_provider(icu_provider_adapters::fork::ForkByKeyProvider::new(
                        BakedProvider,
                        b,
                    ))
                }
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Any(a), ICU4XDataProviderInner::Any(b)) => {
                    convert_any_provider(icu_provider_adapters::fork::ForkByKeyProvider::new(a, b))
                }
                #[cfg(feature = "buffer_provider")]
                (ICU4XDataProviderInner::Buffer(a), ICU4XDataProviderInner::Buffer(b)) => {
                    convert_buffer_provider(icu_provider_adapters::fork::ForkByKeyProvider::new(
                        a, b,
                    ))
                }
                _ => {
                    let e = ICU4XError::DataMismatchedAnyBufferError;
                    crate::errors::log_conversion(
                        &"fork_by_key must be passed the same type of provider (Any or Buffer)",
                        e,
                    );
                    return Err(e).into();
                }
            };
            Ok(()).into()
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
            *self = match (core::mem::take(&mut self.0), core::mem::take(&mut other.0)) {
                (ICU4XDataProviderInner::Empty, b) | (b, ICU4XDataProviderInner::Empty) => {
                    ICU4XDataProvider(b)
                }
                (ICU4XDataProviderInner::Baked, ICU4XDataProviderInner::Baked) => {
                    ICU4XDataProvider(ICU4XDataProviderInner::Baked)
                }
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Baked, ICU4XDataProviderInner::Any(b)) => {
                    convert_any_provider(
                        icu_provider_adapters::fork::ForkByErrorProvider::new_with_predicate(
                            BakedProvider,
                            b,
                            MissingLocalePredicate,
                        ),
                    )
                }
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Any(a), ICU4XDataProviderInner::Baked) => {
                    convert_any_provider(
                        icu_provider_adapters::fork::ForkByErrorProvider::new_with_predicate(
                            a,
                            BakedProvider,
                            MissingLocalePredicate,
                        ),
                    )
                }
                #[cfg(feature = "any_provider")]
                (ICU4XDataProviderInner::Any(a), ICU4XDataProviderInner::Any(b)) => {
                    convert_any_provider(
                        icu_provider_adapters::fork::ForkByErrorProvider::new_with_predicate(
                            a,
                            b,
                            MissingLocalePredicate,
                        ),
                    )
                }
                #[cfg(feature = "buffer_provider")]
                (ICU4XDataProviderInner::Buffer(a), ICU4XDataProviderInner::Buffer(b)) => {
                    convert_buffer_provider(
                        icu_provider_adapters::fork::ForkByErrorProvider::new_with_predicate(
                            a,
                            b,
                            MissingLocalePredicate,
                        ),
                    )
                }
                _ => {
                    let e = ICU4XError::DataMismatchedAnyBufferError;
                    crate::errors::log_conversion(
                        &"fork_by_locale must be passed the same type of provider (Any or Buffer)",
                        e,
                    );
                    return Err(e).into();
                }
            };
            Ok(()).into()
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
                ICU4XDataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                )),
                ICU4XDataProviderInner::Empty => {
                    Ok(ICU4XDataProvider(ICU4XDataProviderInner::Empty))
                }
                #[cfg(not(feature = "any_provider"))]
                ICU4XDataProviderInner::Baked => panic!(
                    "Locale fallback for baked providers requires the 'any_provider' feature"
                ),
                #[cfg(feature = "any_provider")]
                ICU4XDataProviderInner::Baked => {
                    LocaleFallbackProvider::try_new_with_any_provider(BakedProvider)
                        .map(convert_any_provider)
                }
                #[cfg(feature = "any_provider")]
                ICU4XDataProviderInner::Any(inner) => {
                    LocaleFallbackProvider::try_new_with_any_provider(inner)
                        .map(convert_any_provider)
                }
                #[cfg(feature = "buffer_provider")]
                ICU4XDataProviderInner::Buffer(inner) => {
                    LocaleFallbackProvider::try_new_with_buffer_provider(inner)
                        .map(convert_buffer_provider)
                }
            }
            .map(|p| *self = p)
            .map_err(Into::into)
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
            *self = match core::mem::take(&mut self.0) {
                ICU4XDataProviderInner::Destroyed => {
                    return Err(
                        icu_provider::DataError::custom("This provider has been destroyed").into(),
                    )
                    .into()
                }
                ICU4XDataProviderInner::Empty => ICU4XDataProvider(ICU4XDataProviderInner::Empty),
                #[cfg(not(feature = "any_provider"))]
                ICU4XDataProviderInner::Baked => panic!(
                    "Locale fallback for baked providers requires the 'any_provider' feature"
                ),
                #[cfg(feature = "any_provider")]
                ICU4XDataProviderInner::Baked => {
                    convert_any_provider(LocaleFallbackProvider::new_with_fallbacker(
                        BakedProvider,
                        fallbacker.0.clone(),
                    ))
                }
                #[cfg(feature = "any_provider")]
                ICU4XDataProviderInner::Any(inner) => convert_any_provider(
                    LocaleFallbackProvider::new_with_fallbacker(inner, fallbacker.0.clone()),
                ),
                #[cfg(feature = "buffer_provider")]
                ICU4XDataProviderInner::Buffer(inner) => convert_buffer_provider(
                    LocaleFallbackProvider::new_with_fallbacker(inner, fallbacker.0.clone()),
                ),
            };
            Ok(()).into()
        }
    }
}

#[cfg(not(any(feature = "any_provider", feature = "buffer_provider")))]
impl<M> DataProvider<M> for ICU4XDataProviderInner
where
    M: KeyedDataMarker + 'static,
    BakedProvider: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                "This provider has been destroyed",
            )),
            ICU4XDataProviderInner::Empty => EmptyDataProvider::new().load(req),
            ICU4XDataProviderInner::Baked => BakedProvider.load(req),
        }
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
    BakedProvider: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                "This provider has been destroyed",
            )),
            ICU4XDataProviderInner::Empty => EmptyDataProvider::new().load(req),
            ICU4XDataProviderInner::Baked => BakedProvider.load(req),
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
    BakedProvider: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                "This provider has been destroyed",
            )),
            ICU4XDataProviderInner::Empty => EmptyDataProvider::new().load(req),
            ICU4XDataProviderInner::Baked => BakedProvider.load(req),
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
    BakedProvider: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        match self {
            ICU4XDataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                "This provider has been destroyed",
            )),
            ICU4XDataProviderInner::Empty => EmptyDataProvider::new().load(req),
            ICU4XDataProviderInner::Baked => BakedProvider.load(req),
            ICU4XDataProviderInner::Any(any_provider) => any_provider.as_downcasting().load(req),
            ICU4XDataProviderInner::Buffer(buffer_provider) => {
                buffer_provider.as_deserializing().load(req)
            }
        }
    }
}
