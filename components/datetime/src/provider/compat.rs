// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::any::TypeId;

use icu_provider::prelude::*;
use icu_time::provider::TimezonePeriodsV1;

/// Data provider for compatibility with old 2.x buffer providers
pub(crate) struct CompatProvider<P0, P1>(pub(crate) P0, pub(crate) P1);

impl<M, P0, P1> DataProvider<M> for CompatProvider<P0, P1>
where
    M: DataMarker,
    P0: DataProvider<M>,
    P1: BufferProvider,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        let original_error = match self.0.load(req) {
            Err(
                e @ DataError {
                    kind: DataErrorKind::MarkerNotFound,
                    ..
                },
            ) => e,
            other => return other,
        };
        if TypeId::of::<M>() == TypeId::of::<TimezonePeriodsV1>() {
            crate::provider::time_zones::legacy::metazone_timezone_compat(&self.1, req)
                .and_then(DataResponse::dynamic_cast)
        } else {
            Err(original_error)
        }
    }
}
