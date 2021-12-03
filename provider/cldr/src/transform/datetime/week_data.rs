// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::Error;
use crate::reader::open_reader;
use crate::support::KeyedDataProvider;
use crate::CldrPaths;
use icu_calendar::arithmetic::week_of::CalendarInfo;
use icu_datetime::provider::week_data::*;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::convert::TryFrom;
use tinystr::TinyStr4;

mod cldr_json {
    use core::convert::TryFrom;
    use serde::{Deserialize, Deserializer};
    use std::collections::BTreeMap;
    use std::num::ParseIntError;
    use std::str::FromStr;
    use tinystr::{tinystr4, TinyStr4};

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub enum Weekday {
        Mon,
        Tue,
        Wed,
        Thu,
        Fri,
        Sat,
        Sun,
    }

    impl From<&Weekday> for icu_calendar::types::IsoWeekday {
        fn from(day: &Weekday) -> Self {
            use icu_calendar::types::IsoWeekday;
            match day {
                Weekday::Mon => IsoWeekday::Monday,
                Weekday::Tue => IsoWeekday::Tuesday,
                Weekday::Wed => IsoWeekday::Wednesday,
                Weekday::Thu => IsoWeekday::Thursday,
                Weekday::Fri => IsoWeekday::Friday,
                Weekday::Sat => IsoWeekday::Saturday,
                Weekday::Sun => IsoWeekday::Sunday,
            }
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Territory {
        Region(TinyStr4),
        AltVariantRegion(TinyStr4),
    }

    /// The string used to represent the default territory.
    pub const DEFAULT_TERRITORY: Territory = Territory::Region(tinystr4!("001"));

    /// Suffix used to denote alternative week data variants for a given territory (e.g. English BC/AD v English BCE/CE).
    const ALT_VARIANT_SUFFIX: &str = "-alt-variant";

    impl<'de> Deserialize<'de> for Territory {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            struct TerritoryVisitor;

            impl<'de> serde::de::Visitor<'de> for TerritoryVisitor {
                type Value = Territory;

                fn expecting(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    write!(
                        formatter,
                        "a valid Unicode Language Identifier or default territory literal"
                    )
                }

                fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    if let Some(prefix) = s.strip_suffix(ALT_VARIANT_SUFFIX) {
                        return Ok(Territory::AltVariantRegion(
                            prefix
                                .parse::<TinyStr4>()
                                .map_err(serde::de::Error::custom)?,
                        ));
                    }

                    Ok(Territory::Region(
                        s.parse::<TinyStr4>().map_err(serde::de::Error::custom)?,
                    ))
                }
            }

            deserializer.deserialize_string(TerritoryVisitor)
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(try_from = "String")]
    pub struct U8(pub u8);

    impl TryFrom<String> for U8 {
        type Error = ParseIntError;

        fn try_from(s: String) -> Result<Self, Self::Error> {
            Ok(Self(u8::from_str(&s)?))
        }
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct WeekData {
        pub min_days: BTreeMap<Territory, U8>,
        pub first_day: BTreeMap<Territory, Weekday>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Supplemental {
        pub week_data: WeekData,
    }

    #[derive(Deserialize)]
    pub struct Resource {
        pub supplemental: Supplemental,
    }
}

/// A data provider reading from CLDR JSON weekData files.
#[derive(Debug)]
pub struct WeekDataProvider {
    default: CalendarInfo,
    week_data: cldr_json::WeekData,
}

impl TryFrom<&dyn CldrPaths> for WeekDataProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let path = cldr_paths.cldr_core()?.join("supplemental/weekData.json");

        let resource: cldr_json::Resource =
            serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
        let week_data = resource.supplemental.week_data;

        let default = CalendarInfo {
            first_weekday: week_data
                .first_day
                .get(&cldr_json::DEFAULT_TERRITORY)
                .ok_or_else(|| {
                    Error::Custom(
                        "Missing default entry for firstDay in weekData.json".to_string(),
                        None,
                    )
                })?
                .into(),
            min_week_days: week_data
                .min_days
                .get(&cldr_json::DEFAULT_TERRITORY)
                .ok_or_else(|| {
                    Error::Custom(
                        "Missing default entry for minDays in weekData.json".to_string(),
                        None,
                    )
                })?
                .0,
        };

        Ok(Self { default, week_data })
    }
}

impl KeyedDataProvider for WeekDataProvider {
    fn supported_keys() -> Vec<ResourceKey> {
        vec![WeekDataV1Marker::KEY]
    }
}

impl IterableProvider for WeekDataProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let regions: HashSet<Option<TinyStr4>> = self
            .week_data
            .min_days
            .keys()
            .chain(self.week_data.first_day.keys())
            .filter_map(|t| match t {
                &cldr_json::DEFAULT_TERRITORY => Some(None),
                cldr_json::Territory::Region(r) => Some(Some(*r)),
                _ => None,
            })
            .collect();
        Ok(Box::new(regions.into_iter().map(|r| ResourceOptions {
            variant: r.map(|r| Cow::Owned(r.to_string())),
            langid: None,
        })))
    }
}

impl ResourceProvider<WeekDataV1Marker> for WeekDataProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<WeekDataV1Marker>, DataError> {
        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        let territory = req
            .options
            .variant
            .as_ref()
            .map(|v| -> Result<cldr_json::Territory, DataError> {
                Ok(cldr_json::Territory::Region(
                    TinyStr4::from_bytes(v.as_bytes()).map_err(|_| {
                        DataErrorKind::MissingVariant.with_req(WeekDataV1Marker::KEY, req)
                    })?,
                ))
            })
            .transpose()?
            .unwrap_or_else(|| cldr_json::DEFAULT_TERRITORY.clone());

        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(WeekDataV1(CalendarInfo {
                first_weekday: self
                    .week_data
                    .first_day
                    .get(&territory)
                    .map(|w| w.into())
                    .unwrap_or(self.default.first_weekday),
                min_week_days: self
                    .week_data
                    .min_days
                    .get(&territory)
                    .map(|c| c.0)
                    .unwrap_or(self.default.min_week_days),
            }))),
        })
    }
}

icu_provider::impl_dyn_provider!(WeekDataProvider, [WeekDataV1Marker,], SERDE_SE);

#[test]
fn basic_cldr_week_data() {
    use icu_calendar::types::IsoWeekday;
    use icu_locid_macros::langid;

    let cldr_paths = crate::cldr_paths::for_test();
    let provider = WeekDataProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();

    let default_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions {
                variant: None,
                langid: Some(langid!("en")), // We don't actually care about langid.
            },
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(1, default_week_data.get().0.min_week_days);
    assert_eq!(IsoWeekday::Monday, default_week_data.get().0.first_weekday);

    let fr_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions {
                variant: Some("FR".into()),
                langid: None,
            },
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, fr_week_data.get().0.min_week_days);
    assert_eq!(IsoWeekday::Monday, fr_week_data.get().0.first_weekday);

    let iq_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions {
                variant: Some("IQ".into()),
                langid: Some(langid!("fr-FR")),
            },
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    // Only first_weekday is defined for IQ, min_week_days uses the default.
    assert_eq!(
        default_week_data.get().0.min_week_days,
        iq_week_data.get().0.min_week_days
    );
    assert_eq!(IsoWeekday::Saturday, iq_week_data.get().0.first_weekday);

    let gg_week_data: DataPayload<WeekDataV1Marker> = provider
        .load_resource(&DataRequest {
            options: ResourceOptions {
                variant: Some("GG".into()),
                langid: None,
            },
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(4, gg_week_data.get().0.min_week_days);
    // Only min_week_days is defined for GG, first_weekday uses the default.
    assert_eq!(
        default_week_data.get().0.first_weekday,
        gg_week_data.get().0.first_weekday
    );
}
