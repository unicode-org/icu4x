// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Types for loading a single display name at a time.
//!
//! This submodule is useful for applications that only need to display one or
//! two specific names, such as the name of the current region.
//!
//! ### Status
//!
//! Currently, this module has limited support. It supports regions, scripts,
//! and variants, but support for languages and locales is currently missing.
//! More features are on their way.
//!
//! If you have any feedback, please let us know at
//! <https://github.com/unicode-org/icu4x/issues/7825>.
//!
//! See [the parent module](mod@super) for a comparison of single and multi.

mod region;
mod script;
mod variant;

// Re-export from submodules
pub use region::{RegionDisplayName, RegionDisplayNameOwned};
pub use script::{ScriptDisplayName, ScriptDisplayNameOwned};
pub use variant::{VariantDisplayName, VariantDisplayNameOwned};

use crate::displaynames::DisplayNamesPreferences;
use icu_provider::prelude::*;
use zerovec::VarZeroCow;

pub(crate) fn try_new_unstable<M, D>(
    provider: &D,
    prefs: DisplayNamesPreferences,
    attributes: &DataMarkerAttributes,
) -> Result<DataPayload<M>, DataError>
where
    M: DataMarker<DataStruct = VarZeroCow<'static, str>>,
    D: DataProvider<M> + ?Sized,
{
    let locale = M::make_locale(prefs.locale_preferences);
    let payload = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale),
            ..Default::default()
        })?
        .payload;
    Ok(payload)
}

pub(crate) fn try_new_short_unstable<MShort, MLong, D>(
    provider: &D,
    prefs: DisplayNamesPreferences,
    attributes: &DataMarkerAttributes,
) -> Result<DataPayload<MLong>, DataError>
where
    MShort: DataMarker<DataStruct = VarZeroCow<'static, str>>,
    MLong: DataMarker<DataStruct = VarZeroCow<'static, str>>,
    D: DataProvider<MShort> + DataProvider<MLong> + ?Sized,
{
    let locale = MShort::make_locale(prefs.locale_preferences);
    let id = DataIdentifierBorrowed::for_marker_attributes_and_locale(attributes, &locale);
    let mut metadata = DataRequestMetadata::default();
    metadata.silent = true;
    let result: Result<DataResponse<MShort>, DataError> =
        provider.load(DataRequest { id, metadata });

    match result {
        Ok(response) => Ok(response.payload.cast()),
        Err(DataError {
            kind: DataErrorKind::IdentifierNotFound,
            ..
        }) => try_new_unstable(provider, prefs, attributes),
        Err(e) => Err(e),
    }
}
