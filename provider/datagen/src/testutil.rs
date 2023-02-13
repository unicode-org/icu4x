// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use elsa::FrozenMap;
use icu_provider::prelude::*;

/// A DataProvider that records input (requested) and output (resolved) locales.
pub(crate) struct ResolvedLocaleAdapter<P> {
    pub inner: P,
    pub resolved: FrozenMap<(DataKey, DataLocale), Box<Result<DataResponseMetadata, DataError>>>,
}

impl<P> ResolvedLocaleAdapter<P> {
    /// Creates a new [`ResolvedLocaleAdapter`] by wrapping another provider.
    pub fn new(provider: P) -> Self {
        Self {
            inner: provider,
            resolved: FrozenMap::new(),
        }
    }

    /// Gets the resolved [`DataLocale`] for a request that previously ran to completion.
    ///
    /// Returns `None` if the request didn't occur or was not successful.
    pub fn resolved_locale_for(
        &self,
        key: DataKey,
        input_locale: DataLocale,
    ) -> Option<&DataLocale> {
        self.resolved
            .get(&(key, input_locale))?
            .as_ref()
            .ok()?
            .locale
            .as_ref()
    }
}

impl<P, M> DataProvider<M> for ResolvedLocaleAdapter<P>
where
    P: DataProvider<M>,
    M: KeyedDataMarker,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        let result = self.inner.load(req);
        let key = (M::KEY, req.locale.clone());
        let value = result
            .as_ref()
            .map(|res| res.metadata.clone())
            .map_err(Clone::clone);
        let inserted_value = self.resolved.insert(key.clone(), Box::from(value.clone()));
        if &value != inserted_value {
            panic!(
                "Identical data requests returned different results: {key:?} {value:?} {inserted_value:?}"
            );
        }
        result
    }
}
