// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::convert::compute_bcp47_tzids_btreemap;
use crate::transform::cldr::cldr_serde;
use icu_provider::prelude::*;
use icu_provider::datagen::IterableDataProvider;
use icu_timezone::provider::names::*;

impl DataProvider<IanaToBcp47MapV1Marker> for crate::DatagenProvider {
    fn load(&self, _: DataRequest) -> Result<DataResponse<IanaToBcp47MapV1Marker>, DataError> {
        let resource: &cldr_serde::time_zones::bcp47_tzid::Resource =
            self.source
                .cldr()?
                .bcp47()
                .read_and_parse("timezone.json")?;
        let bcp47_tzid_data = &compute_bcp47_tzids_btreemap(&resource.keyword.u.time_zones.values);
        let data_struct = IanaToBcp47MapV1 {
            map: bcp47_tzid_data
                .iter()
                .map(|(k, v)| (NormalizedTimeZoneIdStr::boxed_from_bytes(k.as_bytes()), v))
                .collect(),
        };
        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

impl IterableDataProvider<IanaToBcp47MapV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}
