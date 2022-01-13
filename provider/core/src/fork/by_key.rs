// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Providers that invoke other providers based on the resource key.

use crate::prelude::*;
use alloc::vec::Vec;

/// A provider that returns data from one of two child providers based on the key.
///
/// The result of the first provider that supports a particular [`ResourceKey`] will be returned,
/// even if the request failed for other reasons (such as an unsupported language). Therefore,
/// you should add child providers that support disjoint sets of keys.
///
/// Note: It does not make sense to construct a forking [`DataProvider`], since that is
/// type-specific. Instead, make a forking [`BufferProvider`].
///
/// # Examples
///
/// Normal usage:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_locid_macros::langid;
/// use icu_provider::fork::by_key::ForkByKeyProvider;
///
/// struct DummyBufferProvider;
/// impl BufferProvider for DummyBufferProvider {
///     fn load_buffer(&self, req: &DataRequest) -> Result<DataResponse<BufferMarker>, DataError> {
///         Err(DataErrorKind::MissingResourceKey.with_req(req))
///     }
/// }
///
/// let forking_provider = ForkByKeyProvider(
///     DummyBufferProvider,
///     HelloWorldProvider::new_with_placeholder_data().into_json_provider()
/// );
///
/// let data_provider = forking_provider.as_deserializing();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = data_provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("de")),
///             }
///         }
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
/// ```
///
/// Stops at the first provider supporting a key, even if the locale is not supported:
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_locid_macros::{language, langid};
/// use icu_provider::filter::Filterable;
/// use icu_provider::fork::by_key::ForkByKeyProvider;
///
/// let forking_provider = ForkByKeyProvider(
///     HelloWorldProvider::new_with_placeholder_data()
///         .into_json_provider()
///         .filterable("Chinese")
///         .filter_by_langid(|langid| langid.language == language!("zh")),
///     HelloWorldProvider::new_with_placeholder_data()
///         .into_json_provider()
///         .filterable("German")
///         .filter_by_langid(|langid| langid.language == language!("de")),
/// );
///
/// let data_provider: &dyn DataProvider<HelloWorldV1Marker> = &forking_provider.as_deserializing();
///
/// // Chinese is the first provider, so this succeeds
/// let chinese_hello_world: DataPayload<HelloWorldV1Marker> = data_provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("zh")),
///             }
///         }
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("你好世界", chinese_hello_world.get().message);
///
/// // German is shadowed by Chinese, so this fails
/// data_provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("de")),
///             }
///         }
///     })
///     .expect_err("Should stop at the first provider, even though the second has data");
/// ```
pub struct ForkByKeyProvider<P0, P1>(pub P0, pub P1);

impl<P0: BufferProvider, P1: BufferProvider> BufferProvider for ForkByKeyProvider<P0, P1> {
    #[inline]
    fn load_buffer(&self, req: &DataRequest) -> Result<DataResponse<BufferMarker>, DataError> {
        let result = self.0.load_buffer(req);
        if !matches!(
            result,
            Err(DataError {
                kind: DataErrorKind::MissingResourceKey,
                ..
            })
        ) {
            return result;
        }
        self.1.load_buffer(req)
    }
}

impl<P0: AnyProvider, P1: AnyProvider> AnyProvider for ForkByKeyProvider<P0, P1> {
    #[inline]
    fn load_any(&self, req: &DataRequest) -> Result<AnyResponse, DataError> {
        let result = self.0.load_any(req);
        if !matches!(
            result,
            Err(DataError {
                kind: DataErrorKind::MissingResourceKey,
                ..
            })
        ) {
            return result;
        }
        self.1.load_any(req)
    }
}

/// A provider that returns data from the first child provider supporting the key.
///
/// The result of the first provider that supports a particular [`ResourceKey`] will be returned,
/// even if the request failed for other reasons (such as an unsupported language). Therefore,
/// you should add child providers that support disjoint sets of keys.
///
/// Note: It does not make sense to construct a forking [`DataProvider`], since that is
/// type-specific. Instead, make a forking [`BufferProvider`].
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_locid_macros::{language, langid};
/// use icu_provider::filter::Filterable;
/// use icu_provider::fork::by_key::MultiForkByKeyProvider;
///
/// let forking_provider = MultiForkByKeyProvider {
///     providers: vec![
///         HelloWorldProvider::new_with_placeholder_data()
///             .into_json_provider()
///             .filterable("Chinese")
///             .filter_by_langid(|langid| langid.language == language!("zh")),
///         HelloWorldProvider::new_with_placeholder_data()
///             .into_json_provider()
///             .filterable("German")
///             .filter_by_langid(|langid| langid.language == language!("de")),
///     ]
/// };
///
/// let data_provider: &dyn DataProvider<HelloWorldV1Marker> = &forking_provider.as_deserializing();
///
/// // Chinese is the first provider, so this succeeds
/// let chinese_hello_world: DataPayload<HelloWorldV1Marker> = data_provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("zh")),
///             }
///         }
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("你好世界", chinese_hello_world.get().message);
///
/// // German is shadowed by Chinese, so this fails
/// data_provider
///     .load_payload(&DataRequest {
///         resource_path: ResourcePath {
///             key: key::HELLO_WORLD_V1,
///             options: ResourceOptions {
///                 variant: None,
///                 langid: Some(langid!("de")),
///             }
///         }
///     })
///     .expect_err("Should stop at the first provider, even though the second has data");
/// ```
pub struct MultiForkByKeyProvider<P> {
    pub providers: Vec<P>,
}

impl<P: BufferProvider> BufferProvider for MultiForkByKeyProvider<P> {
    #[inline]
    fn load_buffer(&self, req: &DataRequest) -> Result<DataResponse<BufferMarker>, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_buffer(req);
            if !matches!(
                result,
                Err(DataError {
                    kind: DataErrorKind::MissingResourceKey,
                    ..
                })
            ) {
                return result;
            }
        }
        Err(DataErrorKind::MissingResourceKey.with_req(req))
    }
}

impl<P: AnyProvider> AnyProvider for MultiForkByKeyProvider<P> {
    #[inline]
    fn load_any(&self, req: &DataRequest) -> Result<AnyResponse, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_any(req);
            if !matches!(
                result,
                Err(DataError {
                    kind: DataErrorKind::MissingResourceKey,
                    ..
                })
            ) {
                return result;
            }
        }
        Err(DataErrorKind::MissingResourceKey.with_req(req))
    }
}
