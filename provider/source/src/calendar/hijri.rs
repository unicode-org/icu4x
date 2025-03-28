// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::SourceDataProvider;
use icu::calendar::cal::{HijriObservational, HijriUmmAlQura};
use icu::calendar::provider::hijri::*;
use icu_provider::prelude::*;

impl DataProvider<CalendarHijriObservationalMeccaV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CalendarHijriObservationalMeccaV1>, DataError> {
        self.check_req::<CalendarHijriObservationalMeccaV1>(req)?;
        let cache = HijriObservational::new_mecca_always_calculating().build_cache(1317..1567);
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl crate::IterableDataProviderCached<CalendarHijriObservationalMeccaV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<CalendarHijriUmmalquraV1> for crate::SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CalendarHijriUmmalquraV1>, DataError> {
        self.check_req::<CalendarHijriUmmalquraV1>(req)?;

        // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L87
        const ICU4C_ENCODED_MONTH_LENGTHS: [u16; 1601 - 1300] = [
            0x0AAA, 0x0D54, 0x0EC9, 0x06D4, 0x06EA, 0x036C, 0x0AAD, 0x0555, 0x06A9, 0x0792, 0x0BA9,
            0x05D4, 0x0ADA, 0x055C, 0x0D2D, 0x0695, 0x074A, 0x0B54, 0x0B6A, 0x05AD, 0x04AE, 0x0A4F,
            0x0517, 0x068B, 0x06A5, 0x0AD5, 0x02D6, 0x095B, 0x049D, 0x0A4D, 0x0D26, 0x0D95, 0x05AC,
            0x09B6, 0x02BA, 0x0A5B, 0x052B, 0x0A95, 0x06CA, 0x0AE9, 0x02F4, 0x0976, 0x02B6, 0x0956,
            0x0ACA, 0x0BA4, 0x0BD2, 0x05D9, 0x02DC, 0x096D, 0x054D, 0x0AA5, 0x0B52, 0x0BA5, 0x05B4,
            0x09B6, 0x0557, 0x0297, 0x054B, 0x06A3, 0x0752, 0x0B65, 0x056A, 0x0AAB, 0x052B, 0x0C95,
            0x0D4A, 0x0DA5, 0x05CA, 0x0AD6, 0x0957, 0x04AB, 0x094B, 0x0AA5, 0x0B52, 0x0B6A, 0x0575,
            0x0276, 0x08B7, 0x045B, 0x0555, 0x05A9, 0x05B4, 0x09DA, 0x04DD, 0x026E, 0x0936, 0x0AAA,
            0x0D54, 0x0DB2, 0x05D5, 0x02DA, 0x095B, 0x04AB, 0x0A55, 0x0B49, 0x0B64, 0x0B71, 0x05B4,
            0x0AB5, 0x0A55, 0x0D25, 0x0E92, 0x0EC9, 0x06D4, 0x0AE9, 0x096B, 0x04AB, 0x0A93, 0x0D49,
            0x0DA4, 0x0DB2, 0x0AB9, 0x04BA, 0x0A5B, 0x052B, 0x0A95, 0x0B2A, 0x0B55, 0x055C, 0x04BD,
            0x023D, 0x091D, 0x0A95, 0x0B4A, 0x0B5A, 0x056D, 0x02B6, 0x093B, 0x049B, 0x0655, 0x06A9,
            0x0754, 0x0B6A, 0x056C, 0x0AAD, 0x0555, 0x0B29, 0x0B92, 0x0BA9, 0x05D4, 0x0ADA, 0x055A,
            0x0AAB, 0x0595, 0x0749, 0x0764, 0x0BAA, 0x05B5, 0x02B6, 0x0A56, 0x0E4D, 0x0B25, 0x0B52,
            0x0B6A, 0x05AD, 0x02AE, 0x092F, 0x0497, 0x064B, 0x06A5, 0x06AC, 0x0AD6, 0x055D, 0x049D,
            0x0A4D, 0x0D16, 0x0D95, 0x05AA, 0x05B5, 0x02DA, 0x095B, 0x04AD, 0x0595, 0x06CA, 0x06E4,
            0x0AEA, 0x04F5, 0x02B6, 0x0956, 0x0AAA, 0x0B54, 0x0BD2, 0x05D9, 0x02EA, 0x096D, 0x04AD,
            0x0A95, 0x0B4A, 0x0BA5, 0x05B2, 0x09B5, 0x04D6, 0x0A97, 0x0547, 0x0693, 0x0749, 0x0B55,
            0x056A, 0x0A6B, 0x052B, 0x0A8B, 0x0D46, 0x0DA3, 0x05CA, 0x0AD6, 0x04DB, 0x026B, 0x094B,
            0x0AA5, 0x0B52, 0x0B69, 0x0575, 0x0176, 0x08B7, 0x025B, 0x052B, 0x0565, 0x05B4, 0x09DA,
            0x04ED, 0x016D, 0x08B6, 0x0AA6, 0x0D52, 0x0DA9, 0x05D4, 0x0ADA, 0x095B, 0x04AB, 0x0653,
            0x0729, 0x0762, 0x0BA9, 0x05B2, 0x0AB5, 0x0555, 0x0B25, 0x0D92, 0x0EC9, 0x06D2, 0x0AE9,
            0x056B, 0x04AB, 0x0A55, 0x0D29, 0x0D54, 0x0DAA, 0x09B5, 0x04BA, 0x0A3B, 0x049B, 0x0A4D,
            0x0AAA, 0x0AD5, 0x02DA, 0x095D, 0x045E, 0x0A2E, 0x0C9A, 0x0D55, 0x06B2, 0x06B9, 0x04BA,
            0x0A5D, 0x052D, 0x0A95, 0x0B52, 0x0BA8, 0x0BB4, 0x05B9, 0x02DA, 0x095A, 0x0B4A, 0x0DA4,
            0x0ED1, 0x06E8, 0x0B6A, 0x056D, 0x0535, 0x0695, 0x0D4A, 0x0DA8, 0x0DD4, 0x06DA, 0x055B,
            0x029D, 0x062B, 0x0B15, 0x0B4A, 0x0B95, 0x05AA, 0x0AAE, 0x092E, 0x0C8F, 0x0527, 0x0695,
            0x06AA, 0x0AD6, 0x055D, 0x029D,
        ];

        // From https://github.com/unicode-org/icu/blob/1bf6bf774dbc8c6c2051963a81100ea1114b497f/icu4c/source/i18n/islamcal.cpp#L264
        const ICU4C_YEAR_START_ESTIMATE_FIX: [i64; 1601 - 1300] = [
            0, 0, -1, 0, -1, 0, 0, 0, 0, 0, -1, 0, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, 0, 0,
            1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, -1, 0, 0,
            0, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, -1,
            0, 1, 0, 0, 0, -1, 0, 1, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, -1, -1, 0, -1, 0, 1, 0, 0, 0,
            -1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, -1, -1, 0, -1, 0, 0,
            -1, -1, 0, -1, 0, -1, 0, 0, -1, -1, 0, 0, 0, 0, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, -1, 0,
            1, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, -1, 0, 1, 0, 0, -1, -1, 0, 0, 0, 1, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 1, 1, 0, 0, -1, 0, 1, 0, 1, 1, 0, 0, 0,
            0, 1, 0, 0, 0, -1, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, 0, -1, 0, -1, 0, 1, 0, 0, 0,
            -1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, -1, 0, 0, 0, 0, -1,
            -1, 0, -1, 0, 1, 0, 0, -1, -1, 0, 0, 1, 1, 0, 0, -1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1,
        ];

        let cache = HijriUmmAlQura::build_cache(
            1300,
            ICU4C_ENCODED_MONTH_LENGTHS
                .into_iter()
                .zip(ICU4C_YEAR_START_ESTIMATE_FIX)
                .map(|(l, ny)| (core::array::from_fn(|i| (1 << (11 - i)) & l != 0), ny)),
        );

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(cache),
        })
    }
}

impl crate::IterableDataProviderCached<CalendarHijriUmmalquraV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}
