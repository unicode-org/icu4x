// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(feature = "buffer_provider")]
use icu_provider::prelude::*;

#[cfg(feature = "buffer_provider")]
pub enum DataProviderInner {
    Destroyed,
    Buffer(alloc::boxed::Box<dyn BufferProvider + 'static>),
}

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
#[cfg(feature = "buffer_provider")]
pub mod ffi {
    use alloc::boxed::Box;

    use super::DataProviderInner;
    use crate::errors::ffi::DataError;

    #[diplomat::opaque]
    /// An ICU4X data provider, capable of loading ICU4X data keys from some source.
    #[diplomat::rust_link(icu_provider, Mod)]
    pub struct DataProvider(pub DataProviderInner);

    fn convert_buffer_provider<D: icu_provider::buf::BufferProvider + 'static>(
        x: D,
    ) -> DataProvider {
        DataProvider(super::DataProviderInner::Buffer(Box::new(x)))
    }

    impl DataProvider {
        // These will be unused if almost *all* components are turned off, which is tedious and unproductive to gate for
        #[allow(unused)]
        pub(crate) fn call_constructor<F, Ret>(
            &self,
            ctor: F,
        ) -> Result<Ret, icu_provider::DataError>
        where
            F: FnOnce(
                &(dyn icu_provider::buf::BufferProvider + 'static),
            ) -> Result<Ret, icu_provider::DataError>,
        {
            match &self.0 {
                DataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                ))?,
                DataProviderInner::Buffer(ref buffer_provider) => ctor(buffer_provider),
            }
        }

        #[allow(unused)]
        #[cfg(any(feature = "datetime", feature = "decimal", feature = "experimental"))]
        pub(crate) fn call_constructor_custom_err<F, Ret, Err>(&self, ctor: F) -> Result<Ret, Err>
        where
            Err: From<icu_provider::DataError>,
            F: FnOnce(&(dyn icu_provider::buf::BufferProvider + 'static)) -> Result<Ret, Err>,
        {
            match &self.0 {
                DataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                ))?,
                DataProviderInner::Buffer(ref buffer_provider) => ctor(buffer_provider),
            }
        }

        /// Constructs an `FsDataProvider` and returns it as an [`DataProvider`].
        /// Requires the `provider_fs` Cargo feature.
        /// Not supported in WASM.
        #[diplomat::rust_link(icu_provider_fs::FsDataProvider, Struct)]
        #[cfg(all(
            feature = "provider_fs",
            not(any(target_arch = "wasm32", target_os = "none"))
        ))]
        #[diplomat::attr(any(dart, js), disable)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor)]
        pub fn from_fs(path: &DiplomatStr) -> Result<Box<DataProvider>, DataError> {
            Ok(Box::new(convert_buffer_provider(
                icu_provider_fs::FsDataProvider::try_new(
                    // In the future we can start using OsString APIs to support non-utf8 paths
                    core::str::from_utf8(path)
                        .map_err(|_| DataError::Io)?
                        .into(),
                )?,
            )))
        }

        /// Constructs a `BlobDataProvider` and returns it as an [`DataProvider`].
        #[diplomat::rust_link(icu_provider_blob::BlobDataProvider, Struct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor)]
        #[diplomat::attr(not(supports = static_slices), disable)]
        pub fn from_byte_slice(
            blob: &'static [DiplomatByte],
        ) -> Result<Box<DataProvider>, DataError> {
            Ok(Box::new(convert_buffer_provider(
                icu_provider_blob::BlobDataProvider::try_new_from_static_blob(blob)?,
            )))
        }

        /// Creates a provider that tries the current provider and then, if the current provider
        /// doesn't support the data key, another provider `other`.
        ///
        /// This takes ownership of the `other` provider, leaving an empty provider in its place.
        ///
        /// The providers must be the same type (Any or Buffer). This condition is satisfied if
        /// both providers originate from the same constructor, such as `create_from_byte_slice`
        /// or `create_fs`. If the condition is not upheld, a runtime error occurs.
        #[diplomat::rust_link(icu_provider_adapters::fork::ForkByMarkerProvider, Typedef)]
        #[diplomat::rust_link(
            icu_provider_adapters::fork::predicates::MarkerNotFoundPredicate,
            Struct,
            hidden
        )]
        pub fn fork_by_key(&mut self, other: &mut DataProvider) -> Result<(), DataError> {
            #[allow(unused_imports)]
            use DataProviderInner::*;
            *self = match (
                core::mem::replace(&mut self.0, Destroyed),
                core::mem::replace(&mut other.0, Destroyed),
            ) {
                (Destroyed, _) | (_, Destroyed) => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                ))?,
                (Buffer(a), Buffer(b)) => convert_buffer_provider(
                    icu_provider_adapters::fork::ForkByMarkerProvider::new(a, b),
                ),
            };
            Ok(())
        }

        /// Same as `fork_by_key` but forks by locale instead of key.
        #[diplomat::rust_link(
            icu_provider_adapters::fork::predicates::IdentifierNotFoundPredicate,
            Struct
        )]
        pub fn fork_by_locale(&mut self, other: &mut DataProvider) -> Result<(), DataError> {
            #[allow(unused_imports)]
            use DataProviderInner::*;
            *self = match (
                core::mem::replace(&mut self.0, Destroyed),
                core::mem::replace(&mut other.0, Destroyed),
            ) {
                (Destroyed, _) | (_, Destroyed) => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                ))?,
                (Buffer(a), Buffer(b)) => convert_buffer_provider(
                    icu_provider_adapters::fork::ForkByErrorProvider::new_with_predicate(
                        a,
                        b,
                        icu_provider_adapters::fork::predicates::IdentifierNotFoundPredicate,
                    ),
                ),
            };
            Ok(())
        }

        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackProvider::new,
            FnInStruct
        )]
        #[diplomat::rust_link(
            icu_provider_adapters::fallback::LocaleFallbackProvider,
            Struct,
            compact
        )]
        #[allow(unused_variables)] // feature-gated
        #[cfg(feature = "locale")]
        pub fn enable_locale_fallback_with(
            &mut self,
            fallbacker: &crate::fallbacker::ffi::LocaleFallbacker,
        ) -> Result<(), DataError> {
            use DataProviderInner::*;
            *self = match core::mem::replace(&mut self.0, Destroyed) {
                Destroyed => Err(icu_provider::DataError::custom(
                    "This provider has been destroyed",
                ))?,
                Buffer(inner) => convert_buffer_provider(
                    icu_provider_adapters::fallback::LocaleFallbackProvider::new(
                        inner,
                        fallbacker.0.clone(),
                    ),
                ),
            };
            Ok(())
        }
    }
}

#[cfg(feature = "buffer_provider")]
macro_rules! load {
    () => {
        fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
            use DataProviderInner::*;
            match self {
                Destroyed => Err(DataError::custom("This provider has been destroyed"))?,
                #[cfg(feature = "buffer_provider")]
                Buffer(buffer_provider) => buffer_provider.as_deserializing().load(req),
            }
        }
    };
}

#[cfg(feature = "buffer_provider")]
impl<M> DataProvider<M> for DataProviderInner
where
    M: DataMarker,
    // Actual bound:
    //     for<'de> <M::DataStruct as DataStruct<'de>>::Output: Deserialize<'de>,
    // Necessary workaround bound (see `yoke::trait_hack` docs):
    for<'de> yoke::trait_hack::YokeTraitHack<<M::DataStruct as yoke::Yokeable<'de>>::Output>:
        serde::Deserialize<'de>,
{
    load!();
}

#[macro_export]
macro_rules! call_constructor {
    ($buffer:path, $provider:expr $(, $args:expr)* $(,)?) => {
        match &$provider.0 {
            $crate::provider::DataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                "This provider has been destroyed",
            ))?,
            $crate::provider::DataProviderInner::Buffer(buffer_provider) => $buffer(buffer_provider, $($args,)*),
        }
    };
}

#[macro_export]
macro_rules! call_constructor_unstable {
    ($unstable:path, $provider:expr $(, $args:expr)* $(,)?) => {
        match &$provider.0 {
            $crate::provider::DataProviderInner::Destroyed => Err(icu_provider::DataError::custom(
                "This provider has been destroyed",
            ))?,
            $crate::provider::DataProviderInner::Buffer(buffer_provider) => $unstable(&icu_provider::buf::AsDeserializingBufferProvider::as_deserializing(buffer_provider), $($args,)*),
        }
    };
}
