// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::neo_skeleton::{NeoSkeletonLength, NeoTimeZoneSkeleton, NeoTimeZoneStyle};
use serde::{Deserialize, Serialize};
use tinystr::TinyAsciiStr;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneConfig {
    pub time_zone_id: Option<TinyAsciiStr<8>>,
    pub metazone_id: Option<TinyAsciiStr<4>>,
    pub zone_variant: Option<TinyAsciiStr<2>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTests(pub Vec<TimeZoneTest>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTest {
    pub locale: String,
    pub config: TimeZoneConfig,
    pub datetime: String,
    pub expectations: Vec<TimeZoneExpectation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneExpectation {
    pub patterns: Vec<String>,
    pub configs: Vec<TimeZoneFormatterConfig>,
    pub fallback_formats: Vec<FallbackFormat>,
    pub expected: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FallbackFormat {
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
    LocalizedOffset,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoSeconds {
    Optional,
    Never,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoMinutes {
    Required,
    Optional,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum ZeroPadding {
    On,
    Off,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoFormat {
    Basic,
    Extended,
    UtcBasic,
    UtcExtended,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimeZoneFormatterConfig {
    GenericNonLocationLong,
    GenericNonLocationShort,
    GenericLocation,
    SpecificNonLocationLong,
    SpecificNonLocationShort,
    LocalizedOffset,
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
}

impl TimeZoneFormatterConfig {
    pub fn to_semantic_skeleton(self) -> NeoTimeZoneSkeleton {
        let mut skeleton = NeoTimeZoneSkeleton::default();
        match self {
            TimeZoneFormatterConfig::GenericNonLocationLong => {
                skeleton.length = Some(NeoSkeletonLength::Long);
                skeleton.style = NeoTimeZoneStyle::NonLocation;
            }
            TimeZoneFormatterConfig::GenericNonLocationShort => {
                skeleton.length = Some(NeoSkeletonLength::Short);
                skeleton.style = NeoTimeZoneStyle::NonLocation;
            }
            TimeZoneFormatterConfig::GenericLocation => {
                skeleton.length = Some(NeoSkeletonLength::Long);
                skeleton.style = NeoTimeZoneStyle::Location;
            }
            TimeZoneFormatterConfig::SpecificNonLocationLong => {
                skeleton.length = Some(NeoSkeletonLength::Long);
                skeleton.style = NeoTimeZoneStyle::SpecificNonLocation;
            }
            TimeZoneFormatterConfig::SpecificNonLocationShort => {
                skeleton.length = Some(NeoSkeletonLength::Short);
                skeleton.style = NeoTimeZoneStyle::SpecificNonLocation;
            }
            TimeZoneFormatterConfig::LocalizedOffset => {
                skeleton.length = Some(NeoSkeletonLength::Long);
                skeleton.style = NeoTimeZoneStyle::Offset;
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::UtcBasic,
                IsoMinutes::Optional,
                IsoSeconds::Never,
            ) => {
                // TODO: X
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::UtcBasic,
                IsoMinutes::Required,
                IsoSeconds::Never,
            ) => {
                // TODO: XX
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::UtcExtended,
                IsoMinutes::Required,
                IsoSeconds::Never,
            ) => {
                // TODO: XXX
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::UtcBasic,
                IsoMinutes::Required,
                IsoSeconds::Optional,
            ) => {
                // TODO: XXXX
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::UtcExtended,
                IsoMinutes::Required,
                IsoSeconds::Optional,
            ) => {
                // TODO: XXXXX
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::Basic,
                IsoMinutes::Optional,
                IsoSeconds::Never,
            ) => {
                // TODO: x
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::Basic,
                IsoMinutes::Required,
                IsoSeconds::Never,
            ) => {
                // TODO: xx
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::Extended,
                IsoMinutes::Required,
                IsoSeconds::Never,
            ) => {
                // TODO: xxx
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::Basic,
                IsoMinutes::Required,
                IsoSeconds::Optional,
            ) => {
                // TODO: xxxx
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(
                IsoFormat::Extended,
                IsoMinutes::Required,
                IsoSeconds::Optional,
            ) => {
                // TODO: xxxxx
                todo!()
            }
            TimeZoneFormatterConfig::Iso8601(_, _, _) => {
                todo!()
            }
        }
        skeleton
    }
}
