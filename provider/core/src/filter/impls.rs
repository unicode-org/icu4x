// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::prelude::*;
use icu_locid::LanguageIdentifier;

impl<D, F> RequestFilterDataProvider<D, F>
where
    F: Fn(&DataRequest) -> bool,
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
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::filter::Filterable;
    /// use icu_provider::iter::IterableDataProviderCore;
    /// use icu_locid::LanguageIdentifier;
    /// use icu_locid_macros::{language, langid};
    ///
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
    ///     .filterable()
    ///     .filter_by_langid(|langid| langid.language != language!("en"));
    ///
    /// // German requests should succeed:
    /// let req_de = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: key::HELLO_WORLD_V1,
    ///         options: langid!("de").into(),
    ///     }
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_payload(&req_de);
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // English requests should fail:
    /// let req_en = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: key::HELLO_WORLD_V1,
    ///         options: langid!("en-US").into(),
    ///     }
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_payload(&req_en);
    /// assert!(matches!(response, Err(DataError::FilteredResource(_, _))));
    ///
    /// // English should not appear in the iterator result:
    /// let supported_langids = provider.supported_options_for_key(&key::HELLO_WORLD_V1)
    ///     .expect("Should successfully make an iterator of supported locales")
    ///     .filter_map(|options| options.langid)
    ///     .collect::<Vec<LanguageIdentifier>>();
    /// assert!(supported_langids.contains(&langid!("de")));
    /// assert!(!supported_langids.contains(&langid!("en")));
    /// ```
    pub fn filter_by_langid<'a>(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + 'a,
    ) -> RequestFilterDataProvider<D, Box<dyn Fn(&DataRequest) -> bool + 'a>>
    where
        F: 'a,
    {
        self.filter_by_langid_with_description(predicate, "Locale filter".to_string())
    }

    /// Same as [`Self::filter_by_langid`] but with an extra argument to set a custom
    /// description for debugging.
    pub fn filter_by_langid_with_description<'a>(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + 'a,
        description: String,
    ) -> RequestFilterDataProvider<D, Box<dyn Fn(&DataRequest) -> bool + 'a>>
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
                match &request.resource_path.options.langid {
                    Some(langid) => predicate(langid),
                    None => true,
                }
            }),
            description,
        }
    }

    /// Filter out data request except those having a language identifier that exactly matches
    /// one in the allowlist.
    ///
    /// This will be replaced with a smarter algorithm for locale filtering; see
    /// https://github.com/unicode-org/icu4x/issues/834
    ///
    /// Data requests with no langid will be allowed. To reject data requests without a langid,
    /// chain this with [`Self::require_langid`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::filter::Filterable;
    /// use icu_locid_macros::langid;
    ///
    /// let allowlist = vec![langid!("de"), langid!("zh")];
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
    ///     .filterable()
    ///     .filter_by_langid_allowlist_strict(&allowlist);
    ///
    /// // German requests should succeed:
    /// let req_de = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: key::HELLO_WORLD_V1,
    ///         options: langid!("de").into(),
    ///     }
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_payload(&req_de);
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // English requests should fail:
    /// let req_en = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: key::HELLO_WORLD_V1,
    ///         options: langid!("en-US").into(),
    ///     }
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_payload(&req_en);
    /// assert!(matches!(response, Err(DataError::FilteredResource(_, _))));
    /// assert_eq!(
    ///     "Resource was filtered: Locale filter (allowlist: [de, zh]): icu4x/helloworld@1/en-US",
    ///     response.unwrap_err().to_string()
    /// );
    /// ```
    pub fn filter_by_langid_allowlist_strict<'a>(
        self,
        allowlist: &'a [LanguageIdentifier],
    ) -> RequestFilterDataProvider<D, Box<dyn Fn(&DataRequest) -> bool + 'a>>
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
                match &request.resource_path.options.langid {
                    Some(langid) => allowlist.contains(langid),
                    None => true,
                }
            }),
            description: format!("Locale filter (allowlist: {:?})", allowlist),
        }
    }

    /// Require that data requests contain a langid.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::filter::Filterable;
    /// use icu_locid_macros::langid;
    ///
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
    ///     .filterable()
    ///     .require_langid();
    ///
    /// // Requests with a langid should succeed:
    /// let req_with_langid = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: key::HELLO_WORLD_V1,
    ///         options: langid!("de").into(),
    ///     }
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_payload(&req_with_langid);
    /// assert!(matches!(response, Ok(_)));
    ///
    /// // Requests without a langid should fail:
    /// let req_no_langid = DataRequest {
    ///     resource_path: ResourcePath {
    ///         key: key::HELLO_WORLD_V1,
    ///         options: Default::default(),
    ///     }
    /// };
    /// let response: Result<DataResponse<HelloWorldV1Marker>, _> =
    ///     provider.load_payload(&req_no_langid);
    /// assert!(matches!(response, Err(DataError::FilteredResource(_, _))));
    /// ```
    pub fn require_langid<'a>(
        self,
    ) -> RequestFilterDataProvider<D, Box<dyn Fn(&DataRequest) -> bool + 'a>>
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
                request.resource_path.options.langid.is_some()
            }),
            description: "Locale is required".to_string(),
        }
    }
}
