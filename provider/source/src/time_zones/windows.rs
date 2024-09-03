use std::collections::HashSet;

use crate::{cldr_serde, SourceDataProvider};
use icu::timezone::provider::windows::{
    WindowsRegionCode, WindowsZoneIdentifier, WindowsZonesToIanaMapV1,
    WindowsZonesToIanaMapV1Marker,
};
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;
use zerovec::ZeroMap2d;

impl DataProvider<WindowsZonesToIanaMapV1Marker> for SourceDataProvider {
    fn load(
        &self,
        _: DataRequest,
    ) -> Result<DataResponse<WindowsZonesToIanaMapV1Marker>, DataError> {
        let resource: &cldr_serde::time_zones::windows_zones::WindowsResource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/windowsZones.json")?;

        let windows_zones = &resource.supplemental.windows_zones;

        let mut map: ZeroMap2d<'_, WindowsZoneIdentifier, WindowsRegionCode, str> =
            ZeroMap2d::default();
        for zone in &windows_zones.mapped_zones {
            let windows_zone = TinyAsciiStr::<32>::try_from_str(&zone.map_zone.windows_id)
                .map_err(|e| {
                    DataError::custom("Could not create windows timezone id")
                        .with_display_context(&e)
                })?;
            let region =
                TinyAsciiStr::<3>::try_from_str(&zone.map_zone.territory).map_err(|e| {
                    DataError::custom("Could not create windows territory id")
                        .with_display_context(&e)
                })?;
            let _ = map.insert(
                &WindowsZoneIdentifier(windows_zone),
                &WindowsRegionCode(region),
                zone.map_zone.iana_identifier.as_str(),
            );
        }

        let data_struct = WindowsZonesToIanaMapV1 { map };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(data_struct),
        })
    }
}

impl crate::IterableDataProviderCached<WindowsZonesToIanaMapV1Marker> for SourceDataProvider {
    fn iter_ids_cached(
        &self,
    ) -> Result<std::collections::HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use icu::timezone::provider::windows::{WindowsRegionCode, WindowsZonesToIanaMapV1Marker};
    use icu_provider::{DataProvider, DataRequest, DataResponse};
    use tinystr::tinystr;

    use crate::SourceDataProvider;

    #[test]
    fn windows_to_iana_basic_test() {
        let provider = SourceDataProvider::new_testing();
        let provider_response: DataResponse<WindowsZonesToIanaMapV1Marker> =
            provider.load(DataRequest::default()).unwrap();
        let windows_zones = provider_response.payload.get();

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "Eastern Standard Time").into(),
            &WindowsRegionCode::default(),
        );
        assert_eq!(result, Some("America/New_York"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "Central Standard Time").into(),
            &WindowsRegionCode::default(),
        );
        assert_eq!(result, Some("America/Chicago"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "Hawaiian Standard Time").into(),
            &WindowsRegionCode::default(),
        );
        assert_eq!(result, Some("Pacific/Honolulu"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "Central Europe Standard Time").into(),
            &WindowsRegionCode::default(),
        );
        assert_eq!(result, Some("Europe/Budapest"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "GMT Standard Time").into(),
            &WindowsRegionCode::default(),
        );
        assert_eq!(result, Some("Europe/London"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "SE Asia Standard Time").into(),
            &WindowsRegionCode::default(),
        );
        assert_eq!(result, Some("Asia/Bangkok"));
    }

    #[test]
    fn windows_to_iana_with_territories() {
        let provider = SourceDataProvider::new_testing();
        let provider_response: DataResponse<WindowsZonesToIanaMapV1Marker> =
            provider.load(DataRequest::default()).unwrap();
        let windows_zones = provider_response.payload.get();

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "Eastern Standard Time").into(),
            &WindowsRegionCode(tinystr!(3, "BS")),
        );
        assert_eq!(result, Some("America/Nassau"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "Central Standard Time").into(),
            &WindowsRegionCode(tinystr!(3, "MX")),
        );
        assert_eq!(result, Some("America/Matamoros America/Ojinaga"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "Central Europe Standard Time").into(),
            &WindowsRegionCode(tinystr!(3, "CZ")),
        );
        assert_eq!(result, Some("Europe/Prague"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "GMT Standard Time").into(),
            &WindowsRegionCode(tinystr!(3, "IE")),
        );
        assert_eq!(result, Some("Europe/Dublin"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "SE Asia Standard Time").into(),
            &WindowsRegionCode(tinystr!(3, "AQ")),
        );
        assert_eq!(result, Some("Antarctica/Davis"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "SE Asia Standard Time").into(),
            &WindowsRegionCode(tinystr!(3, "KH")),
        );
        assert_eq!(result, Some("Asia/Phnom_Penh"));

        let result = windows_zones.map.get_2d(
            &tinystr!(32, "SE Asia Standard Time").into(),
            &WindowsRegionCode(tinystr!(3, "VN")),
        );
        assert_eq!(result, Some("Asia/Saigon"));
    }
}
