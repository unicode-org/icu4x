// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::collections::{BTreeMap, BTreeSet};
use core::cell::RefCell;

use icu_provider::prelude::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct ResolvedLocaleInfo {
    pub requested_locale: DataLocale,
    pub resolved_locale: Option<DataLocale>,
}

/// TODO: Docs
#[derive(Debug)]
pub struct ResolvedLocaleAdapter<P> {
    inner: P,
    resolved_locales: RefCell<BTreeMap<DataKey, ResolvedLocaleInfo>>,
    drop_payloads: bool,
}

impl<P> ResolvedLocaleAdapter<P> {
    pub fn into_inner(self) -> P {
        self.inner
    }

    pub fn clear(&mut self) {
        self.resolved_locales.borrow_mut().clear()
    }

    pub fn take_resolved_locale_for_key(&mut self, key: DataKey) -> Option<DataLocale> {
        self.resolved_locales
            .borrow_mut()
            .remove(&key)
            .and_then(|info| info.resolved_locale)
    }

    pub fn take_all_resolved_locales(&mut self) -> BTreeSet<DataLocale> {
        let map = self.resolved_locales.take();
        map.into_iter()
            .filter_map(|(_, info)| info.resolved_locale)
            .collect()
    }

    pub fn saw_last_resort_fallback(&self) -> bool {
        self.resolved_locales.borrow().values().any(|info| {
            info.resolved_locale
                .as_ref()
                .map(|l| l.is_langid_und())
                .unwrap_or(false)
        })
    }
}

impl<P: BufferProvider> BufferProvider for ResolvedLocaleAdapter<P> {
    fn load_buffer(
        &self,
        key: DataKey,
        req: DataRequest,
    ) -> Result<DataResponse<BufferMarker>, DataError> {
        let mut response = self.inner.load_buffer(key, req)?;
        self.resolved_locales.borrow_mut().insert(
            key,
            ResolvedLocaleInfo {
                requested_locale: req.locale.clone(),
                resolved_locale: response.metadata.locale.take(),
            },
        );
        Ok(response)
    }
}
