// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Providers that invoke other providers based on the resource key.

use alloc::vec::Vec;
#[cfg(feature = "datagen")]
use icu_provider::datagen;
use icu_provider::prelude::*;

use crate::helpers::result_is_err_missing_data_key;

/// A provider that returns data from one of two child providers based on the key.
///
/// The result of the first provider that supports a particular [`DataKey`] will be returned,
/// even if the request failed for other reasons (such as an unsupported language). Therefore,
/// you should add child providers that support disjoint sets of keys.
///
/// Note: A forking [`DataProvider`] does not make sense, since there is only one key that
/// type can support. Instead, you can create a forking [`AnyProvider`], [`BufferProvider`],
/// or [`DynamicDataProvider`].
///
/// # Examples
///
/// Normal usage:
///
/// ```
/// # #[cfg(feature = "deserialize_json")] {
/// use icu_locid::locale;
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::fork::by_key::ForkByKeyProvider;
///
/// struct DummyBufferProvider;
/// impl BufferProvider for DummyBufferProvider {
///     fn load_buffer(
///         &self,
///         key: DataKey,
///         req: &DataRequest,
///     ) -> Result<DataResponse<BufferMarker>, DataError> {
///         Err(DataErrorKind::MissingDataKey.with_req(key, req))
///     }
/// }
///
/// let forking_provider = ForkByKeyProvider(
///     DummyBufferProvider,
///     HelloWorldProvider.into_json_provider(),
/// );
///
/// let data_provider = forking_provider.as_deserializing();
///
/// let german_hello_world: DataPayload<HelloWorldV1Marker> = data_provider
///     .load_resource(&DataRequest {
///         options: locale!("de").into(),
///         metadata: Default::default(),
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("Hallo Welt", german_hello_world.get().message);
/// # }
/// ```
///
/// Stops at the first provider supporting a key, even if the locale is not supported:
///
/// ```
/// # #[cfg(feature = "deserialize_json")] {
/// use icu_locid::{subtags_language as language, locale};
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::filter::Filterable;
/// use icu_provider_adapters::fork::by_key::ForkByKeyProvider;
///
/// let forking_provider = ForkByKeyProvider(
///     HelloWorldProvider
///         .into_json_provider()
///         .filterable("Chinese")
///         .filter_by_langid(|langid| langid.language == language!("zh")),
///     HelloWorldProvider
///         .into_json_provider()
///         .filterable("German")
///         .filter_by_langid(|langid| langid.language == language!("de")),
/// );
///
/// let data_provider: &dyn DataProvider<HelloWorldV1Marker> =
///     &forking_provider.as_deserializing();
///
/// // Chinese is the first provider, so this succeeds
/// let chinese_hello_world = data_provider
///     .load_resource(&DataRequest {
///         options: locale!("zh").into(),
///         metadata: Default::default(),
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("你好世界", chinese_hello_world.get().message);
///
/// // German is shadowed by Chinese, so this fails
/// data_provider
///     .load_resource(&DataRequest {
///         options: locale!("de").into(),
///         metadata: Default::default(),
///     })
///     .expect_err("Should stop at the first provider, even though the second has data");
/// # }
/// ```
#[derive(Debug, PartialEq, Eq)]
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ForkByKeyProvider<P0, P1>(pub P0, pub P1);

impl<P0, P1> BufferProvider for ForkByKeyProvider<P0, P1>
where
    P0: BufferProvider,
    P1: BufferProvider,
{
    fn load_buffer(
        &self,
        key: DataKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let result = self.0.load_buffer(key, req);
        if !result_is_err_missing_data_key(&result) {
            return result;
        }
        self.1.load_buffer(key, req)
    }
}

impl<P0, P1> AnyProvider for ForkByKeyProvider<P0, P1>
where
    P0: AnyProvider,
    P1: AnyProvider,
{
    fn load_any(&self, key: DataKey, req: &DataRequest) -> Result<AnyResponse, DataError> {
        let result = self.0.load_any(key, req);
        if !result_is_err_missing_data_key(&result) {
            return result;
        }
        self.1.load_any(key, req)
    }
}

impl<M, P0, P1> DynamicDataProvider<M> for ForkByKeyProvider<P0, P1>
where
    M: DataMarker,
    P0: DynamicDataProvider<M>,
    P1: DynamicDataProvider<M>,
{
    fn load_payload(&self, key: DataKey, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let result = self.0.load_payload(key, req);
        if !result_is_err_missing_data_key(&result) {
            return result;
        }
        self.1.load_payload(key, req)
    }
}

#[cfg(feature = "datagen")]
impl<M, P0, P1> datagen::IterableDynamicDataProvider<M> for ForkByKeyProvider<P0, P1>
where
    M: DataMarker,
    P0: datagen::IterableDynamicDataProvider<M>,
    P1: datagen::IterableDynamicDataProvider<M>,
{
    fn supported_options_for_key(&self, key: DataKey) -> Result<Vec<DataOptions>, DataError> {
        let result = self.0.supported_options_for_key(key);
        if !result_is_err_missing_data_key(&result) {
            return result;
        }
        self.1.supported_options_for_key(key)
    }
}

/// A provider that returns data from the first child provider supporting the key.
///
/// The result of the first provider that supports a particular [`DataKey`] will be returned,
/// even if the request failed for other reasons (such as an unsupported language). Therefore,
/// you should add child providers that support disjoint sets of keys.
///
/// Note: A forking [`DataProvider`] does not make sense, since there is only one key that
/// type can support. Instead, you can create a forking [`AnyProvider`], [`BufferProvider`],
/// or [`DynamicDataProvider`].
///
/// # Examples
///
/// ```
/// # #[cfg(feature = "deserialize_json")] {
/// use icu_locid::{subtags_language as language, locale};
/// use icu_provider::hello_world::*;
/// use icu_provider::prelude::*;
/// use icu_provider_adapters::filter::Filterable;
/// use icu_provider_adapters::fork::by_key::MultiForkByKeyProvider;
///
/// let forking_provider = MultiForkByKeyProvider {
///     providers: vec![
///         HelloWorldProvider
///             .into_json_provider()
///             .filterable("Chinese")
///             .filter_by_langid(|langid| langid.language == language!("zh")),
///         HelloWorldProvider
///             .into_json_provider()
///             .filterable("German")
///             .filter_by_langid(|langid| langid.language == language!("de")),
///     ],
/// };
///
/// let data_provider: &dyn DataProvider<HelloWorldV1Marker> =
///     &forking_provider.as_deserializing();
///
/// // Chinese is the first provider, so this succeeds
/// let chinese_hello_world = data_provider
///     .load_resource(&DataRequest {
///         options: locale!("zh").into(),
///         metadata: Default::default(),
///     })
///     .expect("Loading should succeed")
///     .take_payload()
///     .expect("Data should be present");
///
/// assert_eq!("你好世界", chinese_hello_world.get().message);
///
/// // German is shadowed by Chinese, so this fails
/// data_provider
///     .load_resource(&DataRequest {
///         options: locale!("de").into(),
///         metadata: Default::default(),
///     })
///     .expect_err("Should stop at the first provider, even though the second has data");
/// # }
/// ```
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct MultiForkByKeyProvider<P> {
    pub providers: Vec<P>,
}

impl<P> BufferProvider for MultiForkByKeyProvider<P>
where
    P: BufferProvider,
{
    fn load_buffer(
        &self,
        key: DataKey,
        req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_buffer(key, req);
            if !result_is_err_missing_data_key(&result) {
                return result;
            }
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

impl<P> AnyProvider for MultiForkByKeyProvider<P>
where
    P: AnyProvider,
{
    fn load_any(&self, key: DataKey, req: &DataRequest) -> Result<AnyResponse, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_any(key, req);
            if !result_is_err_missing_data_key(&result) {
                return result;
            }
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

impl<M, P> DynamicDataProvider<M> for MultiForkByKeyProvider<P>
where
    M: DataMarker,
    P: DynamicDataProvider<M>,
{
    fn load_payload(&self, key: DataKey, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        for provider in self.providers.iter() {
            let result = provider.load_payload(key, req);
            if !result_is_err_missing_data_key(&result) {
                return result;
            }
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

#[cfg(feature = "datagen")]
impl<M, P> datagen::IterableDynamicDataProvider<M> for MultiForkByKeyProvider<P>
where
    M: DataMarker,
    P: datagen::IterableDynamicDataProvider<M>,
{
    fn supported_options_for_key(&self, key: DataKey) -> Result<Vec<DataOptions>, DataError> {
        for provider in self.providers.iter() {
            let result = provider.supported_options_for_key(key);
            if !result_is_err_missing_data_key(&result) {
                return result;
            }
        }
        Err(DataErrorKind::MissingDataKey.with_key(key))
    }
}

#[cfg(feature = "datagen")]
impl<P, MFrom, MTo> datagen::DataConverter<MFrom, MTo> for MultiForkByKeyProvider<P>
where
    P: datagen::DataConverter<MFrom, MTo>,
    MFrom: DataMarker,
    MTo: DataMarker,
{
    fn convert(
        &self,
        key: DataKey,
        mut from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, datagen::ReturnedPayloadError<MFrom>> {
        use datagen::ReturnedPayloadError;

        for provider in self.providers.iter() {
            let result = provider.convert(key, from);
            match result {
                Ok(_) => return result,
                Err(e) => {
                    let ReturnedPayloadError(returned, error) = e;
                    if error.kind != DataErrorKind::MissingDataKey {
                        return Err(ReturnedPayloadError(returned, error));
                    }
                    from = returned;
                }
            }
        }
        Err(ReturnedPayloadError(
            from,
            DataErrorKind::MissingDataKey.with_key(key),
        ))
    }
}

#[cfg(feature = "datagen")]
impl<P0, P1, MFrom, MTo> datagen::DataConverter<MFrom, MTo> for ForkByKeyProvider<P0, P1>
where
    P0: datagen::DataConverter<MFrom, MTo>,
    P1: datagen::DataConverter<MFrom, MTo>,
    MFrom: DataMarker,
    MTo: DataMarker,
{
    fn convert(
        &self,
        key: DataKey,
        mut from: DataPayload<MFrom>,
    ) -> Result<DataPayload<MTo>, datagen::ReturnedPayloadError<MFrom>> {
        use datagen::ReturnedPayloadError;
        let result = self.0.convert(key, from);
        match result {
            Ok(_) => return result,
            Err(e) => {
                let ReturnedPayloadError(returned, error) = e;
                if error.kind != DataErrorKind::MissingDataKey {
                    return Err(ReturnedPayloadError(returned, error));
                }
                from = returned;
            }
        }
        self.1.convert(key, from)
    }
}
