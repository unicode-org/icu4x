// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::*;
use crate::prelude::*;
use icu_locid::LanguageIdentifier;

/// Filter a data provider based on the language identifier in a data request.
///
/// Import this trait to add the methods on it to all DataProviders:
///
/// ```
/// use icu_provider::filter::LanguageIdentifierFilter;
///
/// // now you can call `.filter_by_langid()` on any provider
/// ```
///
/// # Examples
///
/// ```
/// use icu_provider::prelude::*;
/// use icu_provider::hello_world::*;
/// use icu_provider::filter::LanguageIdentifierFilter;
/// use icu_provider::iter::IterableDataProviderCore;
/// use icu_locid::LanguageIdentifier;
/// use icu_locid_macros::{language, langid};
///
/// let provider = HelloWorldProvider::new_with_placeholder_data()
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
pub trait LanguageIdentifierFilter<'a>: Sized {
    type F: Fn(&DataRequest) -> bool + 'a;

    /// Filter out data requests with certain langids according to the predicate function. The
    /// predicate should return `true` to allow a langid and `false` to reject a langid.
    ///
    /// Data requests with no langid will be allowed. To reject data requests without a langid,
    /// chain this with [`Self::require_langid`].
    fn filter_by_langid(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + 'a,
    ) -> RequestFilterDataProvider<Self, Self::F>;

    /// Like `Self::filter_by_langid`, but with a string description to help with debugging.
    fn filter_by_langid_with_description(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + 'a,
        description: String,
    ) -> RequestFilterDataProvider<Self, Self::F>;

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
    /// use icu_provider::filter::LanguageIdentifierFilter;
    /// use icu_locid_macros::langid;
    ///
    /// let allowlist = vec![langid!("de"), langid!("zh")];
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
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
    fn filter_by_langid_allowlist_strict(
        self,
        allowlist: &'a [LanguageIdentifier],
    ) -> RequestFilterDataProvider<Self, Self::F>;

    /// Require that data requests contain a langid.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider::prelude::*;
    /// use icu_provider::hello_world::*;
    /// use icu_provider::filter::LanguageIdentifierFilter;
    /// use icu_locid_macros::langid;
    ///
    /// let provider = HelloWorldProvider::new_with_placeholder_data()
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
    fn require_langid(self) -> RequestFilterDataProvider<Self, Self::F>;
}

impl<'a, T> LanguageIdentifierFilter<'a> for T {
    type F = Box<dyn Fn(&DataRequest) -> bool + 'a>;

    fn filter_by_langid(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + 'a,
    ) -> RequestFilterDataProvider<Self, Self::F> {
        self.filter_by_langid_with_description(predicate, "Locale filter".to_string())
    }

    fn filter_by_langid_with_description(
        self,
        predicate: impl Fn(&LanguageIdentifier) -> bool + 'a,
        description: String,
    ) -> RequestFilterDataProvider<Self, Self::F> {
        RequestFilterDataProvider {
            inner: self,
            predicate: Box::new(move |request| -> bool {
                match &request.resource_path.options.langid {
                    Some(langid) => predicate(langid),
                    None => true,
                }
            }),
            description,
        }
    }

    fn filter_by_langid_allowlist_strict(
        self,
        allowlist: &'a [LanguageIdentifier],
    ) -> RequestFilterDataProvider<Self, Self::F> {
        RequestFilterDataProvider {
            inner: self,
            predicate: Box::new(move |request| -> bool {
                match &request.resource_path.options.langid {
                    Some(langid) => allowlist.contains(langid),
                    None => false,
                }
            }),
            description: format!("Locale filter (allowlist: {:?})", allowlist),
        }
    }

    fn require_langid(self) -> RequestFilterDataProvider<Self, Self::F> {
        RequestFilterDataProvider {
            inner: self,
            predicate: Box::new(move |request| -> bool {
                match &request.resource_path.options.langid {
                    Some(_) => true,
                    None => false,
                }
            }),
            description: "Locale is required".to_string(),
        }
    }
}
