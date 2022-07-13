// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::helpers::result_is_err_missing_resource_options;

/// A data provider wrapper that performs locale fallback. This enables arbitrary locales to be
/// handled at runtime.
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_provider::prelude::*;
/// use icu_plurals::PluralRules;
/// use icu_provider_adapters::fallback::LocaleFallbackProvider;
///
/// let provider = icu_testdata::get_provider();
///
/// // The provider does not have data for "ar-SA":
/// PluralRules::try_new_cardinal(locale!("ar-SA"), &provider)
///     .map(|_| ())
///     .expect_err("No data for ar-SA");
///
/// // But if we wrap the provider in a fallback provider...
/// let provider = LocaleFallbackProvider::try_new(provider)
///     .expect("Fallback data present");
///
/// // ...then we can load "ar-SA" based on "ar" data
/// let pr = PluralRules::try_new_cardinal(locale!("ar-SA"), &provider)
///     .expect("Using fallback provider");
///
/// assert_eq!(pr.select(5_usize), icu_plurals::PluralCategory::Few);
/// ```
pub struct LocaleFallbackProvider<P> {
    pub inner: P,
    fallbacker: LocaleFallbacker,
}

impl<P> LocaleFallbackProvider<P>
where
    P: ResourceProvider<LocaleFallbackLikelySubtagsV1Marker>
        + ResourceProvider<LocaleFallbackParentsV1Marker>,
{
    /// Create a [`LocaleFallbackProvider`] by wrapping another data provider and then loading
    /// fallback data from it.
    ///
    /// If the data provider being wrapped does not contain fallback data, use
    /// [`LocaleFallbackProvider::new_with_fallbacker`].
    pub fn try_new(provider: P) -> Result<Self, DataError> {
        let fallbacker = LocaleFallbacker::try_new(&provider)?;
        Ok(Self {
            inner: provider,
            fallbacker,
        })
    }
}

impl<P> LocaleFallbackProvider<P> {
    /// Wrap a provider with an arbitrary fallback engine.
    ///
    /// This relaxes the requirement that the wrapped provider contains its own fallback data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_adapters::fallback::{LocaleFallbacker, LocaleFallbackProvider};
    ///
    /// let provider = HelloWorldProvider;
    ///
    /// let req = DataRequest {
    ///     options: locale!("de-CH").into(),
    ///     metadata: Default::default(),
    /// };
    ///
    /// // There is no "de-CH" data in the `HelloWorldProvider`
    /// ResourceProvider::<HelloWorldV1Marker>::load_resource(&provider, &req)
    ///     .expect_err("No data for de-CH");
    ///
    /// // `HelloWorldProvider` does not contain fallback data,
    /// // but we can fetch it from `icu_testdata`, and then
    /// // use it to create the fallbacking data provider.
    /// let fallbacker = LocaleFallbacker::try_new(&icu_testdata::get_provider())
    ///     .expect("Fallback data present");
    /// let provider = LocaleFallbackProvider::new_with_fallbacker(provider, fallbacker);
    ///
    /// // Now we can load the "de-CH" data via fallback to "de".
    /// let german_hello_world: DataPayload<HelloWorldV1Marker> = provider
    ///     .load_resource(&req)
    ///     .expect("Loading should succeed")
    ///     .take_payload()
    ///     .expect("Data should be present");
    ///
    /// assert_eq!("Hallo Welt", german_hello_world.get().message);
    /// ```
    pub fn new_with_fallbacker(provider: P, fallbacker: LocaleFallbacker) -> Self {
        Self {
            inner: provider,
            fallbacker,
        }
    }
}

impl<P> AnyProvider for LocaleFallbackProvider<P>
where
    P: AnyProvider,
{
    fn load_any(&self, key: ResourceKey, base_req: &DataRequest) -> Result<AnyResponse, DataError> {
        let key_fallbacker = self.fallbacker.for_key(key);
        let mut fallback_iterator = key_fallbacker.fallback_for(base_req.clone());
        while !fallback_iterator.get().options.is_empty() {
            let result = self.inner.load_any(key, fallback_iterator.get());
            if !result_is_err_missing_resource_options(&result) {
                return result;
            }
            fallback_iterator.step();
        }
        Err(DataErrorKind::MissingResourceOptions.with_req(key, base_req))
    }
}

impl<P> BufferProvider for LocaleFallbackProvider<P>
where
    P: BufferProvider,
{
    fn load_buffer(
        &self,
        key: ResourceKey,
        base_req: &DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let key_fallbacker = self.fallbacker.for_key(key);
        let mut fallback_iterator = key_fallbacker.fallback_for(base_req.clone());
        while !fallback_iterator.get().options.is_empty() {
            let result = self.inner.load_buffer(key, fallback_iterator.get());
            if !result_is_err_missing_resource_options(&result) {
                return result;
            }
            fallback_iterator.step();
        }
        Err(DataErrorKind::MissingResourceOptions.with_req(key, base_req))
    }
}

impl<P, M> DynProvider<M> for LocaleFallbackProvider<P>
where
    P: DynProvider<M>,
    M: ResourceMarker,
{
    fn load_payload(
        &self,
        key: ResourceKey,
        base_req: &DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        let key_fallbacker = self.fallbacker.for_key(key);
        let mut fallback_iterator = key_fallbacker.fallback_for(base_req.clone());
        while !fallback_iterator.get().options.is_empty() {
            let result = self.inner.load_payload(key, fallback_iterator.get());
            if !result_is_err_missing_resource_options(&result) {
                return result;
            }
            fallback_iterator.step();
        }
        Err(DataErrorKind::MissingResourceOptions.with_req(key, base_req))
    }
}

impl<P, M> ResourceProvider<M> for LocaleFallbackProvider<P>
where
    P: ResourceProvider<M>,
    M: ResourceMarker,
{
    fn load_resource(&self, base_req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let key_fallbacker = self.fallbacker.for_key(M::KEY);
        let mut fallback_iterator = key_fallbacker.fallback_for(base_req.clone());
        while !fallback_iterator.get().options.is_empty() {
            let result = self.inner.load_resource(fallback_iterator.get());
            if !result_is_err_missing_resource_options(&result) {
                return result;
            }
            fallback_iterator.step();
        }
        Err(DataErrorKind::MissingResourceOptions.with_req(M::KEY, base_req))
    }
}
