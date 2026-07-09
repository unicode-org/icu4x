// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Module for tests that need to access `#[cfg(test)]` code and dependencies.
//!
//! Most tests should either be in-module unit tests or integration tests.

#[cfg(feature = "networking")]
mod download_repo_sources;
mod make_testdata;

include!("data.rs");

use crate::SourceDataProvider;
use crate::cldr_cache::CldrCache;
use crate::source::{RscdCache, SerdeCache, TzdbCache};
use std::sync::{Arc, OnceLock};

impl SourceDataProvider {
    // This is equivalent to `new` for the files defined in `tools/testdata-scripts/globs.rs.data`.
    pub(crate) fn new_testing() -> Self {
        // Singleton so that all instantiations share the same caches.
        static SINGLETON: OnceLock<SourceDataProvider> = OnceLock::new();
        SINGLETON
            .get_or_init(|| Self {
                cldr_paths: Some(Arc::new(CldrCache::new(cldr_data()))),
                icuexport_paths: Some(Arc::new(SerdeCache::new(icuexport_data()))),
                segmenter_lstm_paths: Some(Arc::new(SerdeCache::new(lstm_data()))),
                rscd_paths: Some(Arc::new(RscdCache::new_local(rscd_data()))),
                tzdb_paths: Some(Arc::new(TzdbCache::new(tzdb_data()))),
                ..SourceDataProvider::new_custom()
            })
            .clone()
    }
}
