// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Helpers for computing the difference between multiple providers.

use alloc::collections::BTreeSet;
#[cfg(feature = "export")]
use icu_provider::export::{ExportMarker, ExportableProvider};
use icu_provider::prelude::yoke::Yokeable;
use icu_provider::prelude::*;

/// A data provider that only yields data from `target` (`P0`) whose payloads
/// differ from or are absent in `base` (`P1`).
///
/// Loading data loads from both providers, and returns
/// [`DataErrorKind::IdentifierNotFound`] if the payloads are equal. Iteration
/// only yields the identifiers that differ.
///
/// [`ExportableProvider::supported_markers`] retains all non-singleton markers,
/// even if they contain no differing identifiers, but drops singleton markers
/// that don't differ (as a singleton marker cannot be exported without data).
#[derive(Debug)]
pub struct DifferenceDataProvider<P0, P1> {
    target: P0,
    base: P1,
}

impl<P0, P1> DifferenceDataProvider<P0, P1> {
    /// Creates a new [`DifferenceDataProvider`] yielding the data in `target` that
    /// differs from or is absent in `base`.
    pub fn new(target: P0, base: P1) -> Self {
        Self { target, base }
    }
}

/// Returns the identifiers in `ids` for which `differs` returns `true`.
fn filter_ids<'a, M: DynamicDataMarker>(
    ids: BTreeSet<DataIdentifierCow<'a>>,
    differs: impl Fn(DataRequest) -> Result<DataResponse<M>, DataError>,
) -> Result<BTreeSet<DataIdentifierCow<'a>>, DataError> {
    let mut result = BTreeSet::new();
    for id in ids {
        let req = {
            let mut metadata = DataRequestMetadata::default();
            metadata.silent = true;
            DataRequest {
                id: id.as_borrowed(),
                metadata,
            }
        };
        if differs(req).allow_identifier_not_found()?.is_some() {
            result.insert(id);
        }
    }
    Ok(result)
}

/// Returns `target`, or [`DataErrorKind::IdentifierNotFound`] if `base` has an equal payload.
fn diff_response<M>(
    marker: DataMarkerInfo,
    req: DataRequest,
    target: DataResponse<M>,
    base: Result<DataResponse<M>, DataError>,
) -> Result<DataResponse<M>, DataError>
where
    M: DynamicDataMarker,
    for<'a> <M::DataStruct as Yokeable<'a>>::Output: PartialEq,
{
    match base.allow_identifier_not_found()? {
        Some(base) if base.payload == target.payload => {
            Err(DataErrorKind::IdentifierNotFound.with_req(marker, req))
        }
        _ => Ok(target),
    }
}

impl<M, P0, P1> DynamicDataProvider<M> for DifferenceDataProvider<P0, P1>
where
    M: DynamicDataMarker,
    for<'a> <M::DataStruct as Yokeable<'a>>::Output: PartialEq,
    P0: DynamicDataProvider<M>,
    P1: DynamicDataProvider<M>,
{
    fn load_data(
        &self,
        marker: DataMarkerInfo,
        req: DataRequest,
    ) -> Result<DataResponse<M>, DataError> {
        diff_response(
            marker,
            req,
            self.target.load_data(marker, req)?,
            self.base.load_data(marker, req),
        )
    }
}

impl<M, P0, P1> DataProvider<M> for DifferenceDataProvider<P0, P1>
where
    M: DataMarker,
    for<'a> <M::DataStruct as Yokeable<'a>>::Output: PartialEq,
    P0: DataProvider<M>,
    P1: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        diff_response(M::INFO, req, self.target.load(req)?, self.base.load(req))
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
        filter_ids(self.target.iter_ids_for_marker(marker)?, |id| {
            self.load_data(marker, id)
        })
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
        filter_ids(self.target.iter_ids()?, |req| self.load(req))
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
