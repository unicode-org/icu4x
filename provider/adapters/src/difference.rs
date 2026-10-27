// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Helpers for computing the difference between multiple providers.

use alloc::collections::{BTreeMap, BTreeSet};
#[cfg(feature = "export")]
use icu_provider::export::{ExportMarker, ExportableProvider};
use icu_provider::prelude::yoke::Yokeable;
use icu_provider::prelude::*;
use spin::Mutex;

/// A data provider that only yields data identifiers from `target` (`P0`)
/// whose payloads differ from or are absent in `base` (`P1`).
///
/// Loading data is forwarded to `target`; only iteration is filtered.
///
/// [`ExportableProvider::supported_markers`] retains all non-singleton markers,
/// even if they contain no differing identifiers, but drops singleton markers
/// that don't differ (as a singleton marker cannot be exported without data).
#[derive(Debug)]
pub struct DifferenceDataProvider<P0, P1> {
    target: P0,
    base: P1,
    /// The differing identifiers per marker, computed on first iteration.
    diff_ids: Mutex<BTreeMap<DataMarkerInfo, BTreeSet<DataIdentifierCow<'static>>>>,
}

impl<P0, P1> DifferenceDataProvider<P0, P1> {
    /// Creates a new [`DifferenceDataProvider`] yielding the data in `target` that
    /// differs from or is absent in `base`.
    pub fn new(target: P0, base: P1) -> Self {
        Self {
            target,
            base,
            diff_ids: Mutex::new(BTreeMap::new()),
        }
    }

    /// Returns the (cached) identifiers of `marker` for which `differs` returns `true`.
    fn diff_ids<'a>(
        &self,
        marker: DataMarkerInfo,
        target_ids: impl FnOnce() -> Result<BTreeSet<DataIdentifierCow<'a>>, DataError>,
        differs: impl Fn(DataIdentifierBorrowed) -> Result<bool, DataError>,
    ) -> Result<BTreeSet<DataIdentifierCow<'static>>, DataError> {
        if let Some(ids) = self.diff_ids.lock().get(&marker) {
            return Ok(ids.clone());
        }
        // Computed without holding the lock; concurrent calls for the same marker
        // may duplicate work but produce the same result.
        let mut ids = BTreeSet::new();
        for id in target_ids()? {
            if differs(id.as_borrowed())? {
                ids.insert(id.as_borrowed().into_owned());
            }
        }
        self.diff_ids.lock().insert(marker, ids.clone());
        Ok(ids)
    }
}

fn silent_request(id: DataIdentifierBorrowed) -> DataRequest {
    let mut metadata = DataRequestMetadata::default();
    metadata.silent = true;
    DataRequest { id, metadata }
}

fn payload_differs<M>(
    target: Result<DataResponse<M>, DataError>,
    base: impl FnOnce() -> Result<DataResponse<M>, DataError>,
) -> Result<bool, DataError>
where
    M: DynamicDataMarker,
    for<'a> <M::DataStruct as Yokeable<'a>>::Output: PartialEq,
{
    let Some(target) = target.allow_identifier_not_found()? else {
        return Ok(false);
    };
    Ok(match base().allow_identifier_not_found()? {
        Some(base) => target.payload != base.payload,
        None => true,
    })
}

impl<M, P0, P1> DynamicDataProvider<M> for DifferenceDataProvider<P0, P1>
where
    M: DynamicDataMarker,
    P0: DynamicDataProvider<M>,
{
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        self.target.load_data(marker, req)
    }
}

impl<M, P0, P1> DataProvider<M> for DifferenceDataProvider<P0, P1>
where
    M: DataMarker,
    P0: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        self.target.load(req)
    }
}

impl<M, P0, P1> IterableDynamicDataProvider<M> for DifferenceDataProvider<P0, P1>
where
    M: DynamicDataMarker,
    for<'a> <M::DataStruct as Yokeable<'a>>::Output: PartialEq,
    P0: IterableDynamicDataProvider<M>,
    P1: DynamicDataProvider<M>,
{
    fn iter_ids_for_marker(
        &self,
        marker: DataMarkerInfo,
    ) -> Result<BTreeSet<DataIdentifierCow<'_>>, DataError> {
        self.diff_ids(
            marker,
            || self.target.iter_ids_for_marker(marker),
            |id| {
                let req = silent_request(id);
                payload_differs(self.target.load_data(marker, req), || {
                    self.base.load_data(marker, req)
                })
            },
        )
    }
}

impl<M, P0, P1> IterableDataProvider<M> for DifferenceDataProvider<P0, P1>
where
    M: DataMarker,
    for<'a> <M::DataStruct as Yokeable<'a>>::Output: PartialEq,
    P0: IterableDataProvider<M>,
    P1: DataProvider<M>,
{
    fn iter_ids(&self) -> Result<BTreeSet<DataIdentifierCow<'_>>, DataError> {
        self.diff_ids(
            M::INFO,
            || self.target.iter_ids(),
            |id| {
                let req = silent_request(id);
                payload_differs(self.target.load(req), || self.base.load(req))
            },
        )
    }
}

#[cfg(feature = "export")]
impl<P0, P1> ExportableProvider for DifferenceDataProvider<P0, P1>
where
    P0: ExportableProvider,
    P1: DynamicDataProvider<ExportMarker> + Sync,
{
    fn supported_markers(&self) -> BTreeSet<DataMarkerInfo> {
        // Don't drop non-singleton markers without differing ids, so that the
        // difference can be forked with `base` to reassemble `target`. Singleton
        // markers cannot be empty, so drop them if they don't differ.
        self.target
            .supported_markers()
            .into_iter()
            .filter(|&marker| {
                !marker.is_singleton
                    // Keep markers that error, so that the error surfaces during export.
                    || self
                        .iter_ids_for_marker(marker)
                        .map(|ids| !ids.is_empty())
                        .unwrap_or(true)
            })
            .collect()
    }
}
