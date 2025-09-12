// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::any::TypeId;

use icu_provider::{buf::DeserializingBufferProvider, prelude::*};
use icu_time::provider::TimezonePeriodsV1;

use crate::provider::{
    neo::{DatetimeNamesMonthGregorianV1, MonthNames},
    DatetimePatternsDateJapaneseV1, PackedPatterns,
};

/// Data provider for compatibility with old 2.x buffer providers
pub(crate) struct CompatProvider<P0>(pub(crate) P0);

impl<M, P0> DataProvider<M> for CompatProvider<P0>
where
    M: DataMarker,
    P0: BufferProvider,
    for<'a> DeserializingBufferProvider<'a, P0>: DataProvider<M>,
{
    fn load(&self, req: DataRequest) -> Result<DataResponse<M>, DataError> {
        let p = self.0.as_deserializing();
        let original_error = match p.load(req) {
            Err(
                e @ DataError {
                    kind: DataErrorKind::MarkerNotFound,
                    ..
                },
            ) => e,
            other => return other,
        };
        if TypeId::of::<M>() == TypeId::of::<TimezonePeriodsV1>() {
            crate::provider::time_zones::legacy::metazone_timezone_compat(&self.0, req)
                .and_then(DataResponse::dynamic_cast)
        } else if TypeId::of::<M>() == TypeId::of::<DatetimeNamesMonthGregorianV1>() {
            DataProvider::<DatetimeNamesMonthBuddhistV1>::load(&p, req)
                .and_then(|p| p.dynamic_cast())
                .or_else(|_| {
                    DataProvider::<DatetimeNamesMonthJapaneseV1>::load(&p, req)
                        .and_then(|p| p.dynamic_cast())
                })
                .or_else(|_| {
                    DataProvider::<DatetimeNamesMonthJapanextV1>::load(&p, req)
                        .and_then(|p| p.dynamic_cast())
                })
                .or_else(|_| {
                    DataProvider::<DatetimeNamesMonthRocV1>::load(&p, req)
                        .and_then(|p| p.dynamic_cast())
                })
        } else if TypeId::of::<M>() == TypeId::of::<DatetimePatternsDateJapaneseV1>() {
            DataProvider::<DatetimePatternsDateJapanextV1>::load(&p, req)
                .and_then(|p| p.dynamic_cast())
        } else {
            Err(original_error)
        }
    }
}

icu_provider::data_marker!(
    /// `DatetimeNamesMonthBuddhistV1`
    DatetimeNamesMonthBuddhistV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthJapaneseV1`
    DatetimeNamesMonthJapaneseV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthJapanextV1`
    DatetimeNamesMonthJapanextV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);
icu_provider::data_marker!(
    /// `DatetimeNamesMonthRocV1`
    DatetimeNamesMonthRocV1,
    MonthNames<'static>,
    #[cfg(feature = "datagen")]
    attributes_domain = "datetime_month_length"
);

icu_provider::data_marker!(
    /// `DatetimePatternsDateJapanextV1`
    DatetimePatternsDateJapanextV1,
    PackedPatterns<'static>
);
