// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::{BTreeMap, BTreeSet, HashSet};

use crate::{cldr_serde, SourceDataProvider};
use icu::timezone::{
    provider::windows::{WindowsZonesToBcp47MapV1, WindowsZonesToBcp47MapV1Marker},
    TimeZoneBcp47Id,
};
use icu_provider::prelude::*;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ZeroVec;

impl DataProvider<WindowsZonesToBcp47MapV1Marker> for SourceDataProvider {
    fn load(
        &self,
        _: DataRequest,
    ) -> Result<DataResponse<WindowsZonesToBcp47MapV1Marker>, DataError> {
        let resource: &cldr_serde::time_zones::windows_zones::WindowsResource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/windowsZones.json")?;

        let iana2bcp = self.iana_to_bcp47_map()?;

        let windows_zones = &resource.supplemental.windows_zones;

        let mut bcp47_set: BTreeSet<TimeZoneBcp47Id> = BTreeSet::default();
        let intermediary: Vec<(String, TimeZoneBcp47Id)> = windows_zones
            .mapped_zones
            .iter()
            .map(|zone| {
                let primary_iana_id = zone
                    .map_zone
                    .iana_identifier
                    .split_ascii_whitespace()
                    .next()
                    .unwrap_or(&zone.map_zone.iana_identifier);

                let bcp_47 = iana2bcp.get(primary_iana_id).unwrap();
                let _ = bcp47_set.insert(*bcp_47);

                (
                    (zone.map_zone.windows_id.clone() + "/" + &zone.map_zone.territory),
                    *bcp_47,
                )
            })
            .collect();

        let bcp47_ids: ZeroVec<TimeZoneBcp47Id> = bcp47_set.iter().copied().collect();

        let windows2bcp_map: BTreeMap<Vec<u8>, usize> = intermediary
            .iter()
            .map(|(name, id)| {
                (
                    name.as_bytes().to_vec(),
                    bcp47_ids.binary_search(id).unwrap(),
                )
            })
            .collect();

        let data_struct = WindowsZonesToBcp47MapV1 {
            map: ZeroTrieSimpleAscii::try_from(&windows2bcp_map)
                .map_err(|e| {
                    DataError::custom("Could not map windowsZones.json data")
                        .with_display_context(&e)
                })?
                .convert_store(),
            bcp47_ids,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<WindowsZonesToBcp47MapV1Marker> for SourceDataProvider {
    fn iter_ids_cached(
        &self,
    ) -> Result<std::collections::HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use icu::timezone::{provider::windows::WindowsZonesToBcp47MapV1Marker, TimeZoneBcp47Id};
    use icu_provider::{DataProvider, DataRequest, DataResponse};
    use tinystr::tinystr;

    use crate::SourceDataProvider;

    #[test]
    fn windows_to_iana_basic_test() {
        let provider = SourceDataProvider::new_testing();
        let provider_response: DataResponse<WindowsZonesToBcp47MapV1Marker> =
            provider.load(DataRequest::default()).unwrap();
        let windows_zones = provider_response.payload.get();

        let index = windows_zones.map.get("Eastern Standard Time/001").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "usnyc"))));

        let index = windows_zones.map.get("Central Standard Time/001").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "uschi"))));

        let index = windows_zones.map.get("Hawaiian Standard Time/001").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "ushnl"))));

        let index = windows_zones
            .map
            .get("Central Europe Standard Time/001")
            .unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "hubud"))));

        let index = windows_zones.map.get("GMT Standard Time/001").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "gblon"))));

        let index = windows_zones.map.get("SE Asia Standard Time/001").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "thbkk"))));
    }

    #[test]
    fn windows_to_iana_with_territories() {
        let provider = SourceDataProvider::new_testing();
        let provider_response: DataResponse<WindowsZonesToBcp47MapV1Marker> =
            provider.load(DataRequest::default()).unwrap();
        let windows_zones = provider_response.payload.get();

        let index = windows_zones.map.get("Eastern Standard Time/BS").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "bsnas"))));

        let index = windows_zones.map.get("Central Standard Time/MX").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "mxmam"))));

        let index = windows_zones
            .map
            .get("Central Europe Standard Time/CZ")
            .unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "czprg"))));

        let index = windows_zones.map.get("GMT Standard Time/IE").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "iedub"))));

        let index = windows_zones.map.get("SE Asia Standard Time/AQ").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "aqdav"))));

        let index = windows_zones.map.get("SE Asia Standard Time/KH").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "khpnh"))));

        let index = windows_zones.map.get("SE Asia Standard Time/VN").unwrap();
        let result = windows_zones.bcp47_ids.get(index);
        assert_eq!(result, Some(TimeZoneBcp47Id(tinystr!(8, "vnsgn"))));
    }
}
