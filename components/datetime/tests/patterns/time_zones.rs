// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::neo_skeleton::{NeoSkeletonLength, NeoTimeZoneSkeleton, NeoTimeZoneStyle};
use icu_datetime::time_zone;
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
    LocalizedGmt,
}

impl From<FallbackFormat> for time_zone::TimeZoneFormatterOptions {
    fn from(other: FallbackFormat) -> time_zone::TimeZoneFormatterOptions {
        let mut options = time_zone::TimeZoneFormatterOptions::default();
        options.fallback_format = match other {
            FallbackFormat::Iso8601(iso_format, iso_minutes, iso_seconds) => {
                time_zone::FallbackFormat::Iso8601(
                    iso_format.into(),
                    iso_minutes.into(),
                    iso_seconds.into(),
                )
            }
            FallbackFormat::LocalizedGmt => time_zone::FallbackFormat::LocalizedGmt,
        };
        options
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoSeconds {
    Optional,
    Never,
}

impl From<IsoSeconds> for time_zone::IsoSeconds {
    fn from(other: IsoSeconds) -> time_zone::IsoSeconds {
        match other {
            IsoSeconds::Optional => time_zone::IsoSeconds::Optional,
            IsoSeconds::Never => time_zone::IsoSeconds::Never,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum IsoMinutes {
    Required,
    Optional,
}

impl From<IsoMinutes> for time_zone::IsoMinutes {
    fn from(other: IsoMinutes) -> time_zone::IsoMinutes {
        match other {
            IsoMinutes::Required => time_zone::IsoMinutes::Required,
            IsoMinutes::Optional => time_zone::IsoMinutes::Optional,
        }
    }
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

impl From<IsoFormat> for time_zone::IsoFormat {
    fn from(other: IsoFormat) -> time_zone::IsoFormat {
        match other {
            IsoFormat::Basic => time_zone::IsoFormat::Basic,
            IsoFormat::Extended => time_zone::IsoFormat::Extended,
            IsoFormat::UtcBasic => time_zone::IsoFormat::UtcBasic,
            IsoFormat::UtcExtended => time_zone::IsoFormat::UtcExtended,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TimeZoneFormatterConfig {
    GenericNonLocationLong,
    GenericNonLocationShort,
    GenericLocation,
    SpecificNonLocationLong,
    SpecificNonLocationShort,
    LocalizedGMT,
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
            TimeZoneFormatterConfig::LocalizedGMT => {
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
