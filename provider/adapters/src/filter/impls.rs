// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use alloc::boxed::Box;
use icu_provider::prelude::*;

use icu_locale::LanguageIdentifier;

type RequestFilterDataProviderOutput<'a, D> =
    RequestFilterDataProvider<D, Box<dyn Fn(DataRequest) -> bool + Sync + 'a>>;

impl<D, F> RequestFilterDataProvider<D, F>
where
    F: Fn(DataRequest) -> bool + Sync,
{
    /// Filter out data requests with certain langids according to the predicate function. The
    /// predicate should return `true` to allow a langid and `false` to reject a langid.
    ///
    /// Data requests with no langid will be allowed. To reject data requests without a langid,
    /// chain this with [`Self::require_langid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::LanguageIdentifier;
    /// use icu_locale::{langid, subtags::language};
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_adapters::filter::Filterable;
    ///
    /// let provider = HelloWorldProvider
    ///     .filterable("Demo no-English filter")
    ///     .filter_by_langid(|langid| langid.language != language!("en"));
    ///
    /// // German requests should succeed:
    /// let de = DataIdentifierCow::from_locale(langid!("de").into());
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load(DataRequest {
    ///         id: de.as_borrowed(),
    ///         ..Default::default()
    /// });
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // English requests should fail:
    /// let en = DataIdentifierCow::from_locale(langid!("en-US").into());
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load(DataRequest {
    ///     id: en.as_borrowed(),
    ///     ..Default::default()
    /// });
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load(DataRequest {
    ///     id: en.as_borrowed(),
    ///     ..Default::default()
    /// });
    /// assert!(matches!(
    ///     response,
    ///     Err(DataError {
    ///         kind: DataErrorKind::Filtered,
    ///         ..
    ///     })
    /// ));
    ///
    /// // English should not appear in the iterator result:
    /// let available_ids = provider
    ///     .iter_ids()
    ///     .expect("Should successfully make an iterator of supported locales");
    /// assert!(available_ids.contains(&DataIdentifierCow::from_locale(langid!("de").into())));
    /// assert!(!available_ids.contains(&DataIdentifierCow::from_locale(langid!("en").into())));
    /// ```
    pub fn filter_by_langid<'a>(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + Sync + 'a,
    ) -> RequestFilterDataProviderOutput<'a, D>
    where
        F: 'a,
    {
        let old_predicate = self.predicate;
        RequestFilterDataProvider {
            inner: self.inner,
            predicate: Box::new(move |request| -> bool {
                if !(old_predicate)(request) {
                    return false;
                }
                predicate(&request.id.locale.get_langid())
            }),
            filter_name: self.filter_name,
        }
    }

    /// Filter out data request except those having a language identifier that exactly matches
    /// one in the allowlist.
    ///
    /// This will be replaced with a smarter algorithm for locale filtering; see
    /// <https://github.com/unicode-org/icu4x/issues/834>
    ///
    /// Data requests with no langid will be allowed. To reject data requests without a langid,
    /// chain this with [`Self::require_langid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::langid;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_adapters::filter::Filterable;
    ///
    /// let allowlist = [langid!("de"), langid!("zh")];
    /// let provider = HelloWorldProvider
    ///     .filterable("Demo German+Chinese filter")
    ///     .filter_by_langid_allowlist_strict(&allowlist);
    ///
    /// // German requests should succeed:
    /// let de = DataIdentifierCow::from_locale(langid!("de").into());
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load(DataRequest {
    ///         id: de.as_borrowed(),
    ///         ..Default::default()
    /// });
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // English requests should fail:
    /// let en = DataIdentifierCow::from_locale(langid!("en-US").into());

    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load(DataRequest {
    ///     id: en.as_borrowed(),
    ///     ..Default::default()
    /// });
    /// assert!(matches!(
    ///     response,
    ///     Err(DataError {
    ///         kind: DataErrorKind::Filtered,
    ///         ..
    ///     })
    /// ));
    /// assert_eq!(
    ///     response.unwrap_err().str_context,
    ///     Some("Demo German+Chinese filter")
    /// );
    /// ```
    pub fn filter_by_langid_allowlist_strict<'a>(
        self,
        allowlist: &'a [LanguageIdentifier],
    ) -> RequestFilterDataProviderOutput<'a, D>
    where
        F: 'a,
    {
        let old_predicate = self.predicate;
        RequestFilterDataProvider {
            inner: self.inner,
            predicate: Box::new(move |request| -> bool {
                if !(old_predicate)(request) {
                    return false;
                }
                request.id.locale.is_langid_und()
                    || allowlist.contains(&request.id.locale.get_langid())
            }),
            filter_name: self.filter_name,
        }
    }

    /// Require that data requests contain a langid.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::langid;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::prelude::*;
    /// use icu_provider_adapters::filter::Filterable;
    ///
    /// let provider = HelloWorldProvider
    ///     .filterable("Demo require-langid filter")
    ///     .require_langid();
    ///
    /// // Requests with a data id should succeed:
    /// let id = DataIdentifierCow::from_locale(langid!("de").into());
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load(DataRequest {
    ///     id: id.as_borrowed(),
    ///     ..Default::default()
    /// });
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // Requests without a data should fail:
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load(DataRequest {
    ///     id: Default::default(),
    ///     ..Default::default()
    /// });
    /// assert!(matches!(
    ///     response,
    ///     Err(DataError {
    ///         kind: DataErrorKind::Filtered,
    ///         ..
    ///     })
    /// ));
    /// ```
    pub fn require_langid<'a>(self) -> RequestFilterDataProviderOutput<'a, D>
    where
        F: 'a,
    {
        let old_predicate = self.predicate;
        RequestFilterDataProvider {
            inner: self.inner,
            predicate: Box::new(move |request| -> bool {
                if !(old_predicate)(request) {
                    return false;
                }
                !request.id.locale.is_langid_und()
            }),
            filter_name: self.filter_name,
        }
    }
}
