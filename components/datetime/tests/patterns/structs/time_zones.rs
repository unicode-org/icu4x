// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::time_zone;
use serde::{Deserialize, Serialize};
use tinystr::TinyStr8;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneConfig {
    pub time_zone_id: Option<String>,
    pub metazone_id: Option<String>,
    pub time_variant: Option<TinyStr8>,
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
    pub configs: Vec<TimeZoneFormatConfig>,
    pub fallback_formats: Vec<FallbackFormat>,
    pub expected: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FallbackFormat {
    None,
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
    LocalizedGmt,
}

impl From<FallbackFormat> for Option<time_zone::FallbackFormat> {
    fn from(other: FallbackFormat) -> Option<time_zone::FallbackFormat> {
        match other {
            FallbackFormat::Iso8601(iso_format, iso_minutes, iso_seconds) => {
                Some(time_zone::FallbackFormat::Iso8601(
                    iso_format.into(),
                    iso_minutes.into(),
                    iso_seconds.into(),
                ))
            }
            FallbackFormat::LocalizedGmt => Some(time_zone::FallbackFormat::LocalizedGmt),
            FallbackFormat::None => None,
        }
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

impl From<ZeroPadding> for time_zone::ZeroPadding {
    fn from(other: ZeroPadding) -> time_zone::ZeroPadding {
        match other {
            ZeroPadding::On => time_zone::ZeroPadding::On,
            ZeroPadding::Off => time_zone::ZeroPadding::Off,
        }
    }
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
pub enum TimeZoneFormatConfig {
    GenericNonLocationLong,
    GenericNonLocationShort,
    GenericLocation,
    SpecificNonLocationLong,
    SpecificNonLocationShort,
    LocalizedGMT,
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
}

impl From<TimeZoneFormatConfig> for time_zone::TimeZoneFormatConfig {
    fn from(other: TimeZoneFormatConfig) -> time_zone::TimeZoneFormatConfig {
        match other {
            TimeZoneFormatConfig::GenericNonLocationLong => {
                time_zone::TimeZoneFormatConfig::GenericNonLocationLong
            }
            TimeZoneFormatConfig::GenericNonLocationShort => {
                time_zone::TimeZoneFormatConfig::GenericNonLocationShort
            }
            TimeZoneFormatConfig::GenericLocation => {
                time_zone::TimeZoneFormatConfig::GenericLocation
            }
            TimeZoneFormatConfig::SpecificNonLocationLong => {
                time_zone::TimeZoneFormatConfig::SpecificNonLocationLong
            }
            TimeZoneFormatConfig::SpecificNonLocationShort => {
                time_zone::TimeZoneFormatConfig::SpecificNonLocationShort
            }
            TimeZoneFormatConfig::LocalizedGMT => time_zone::TimeZoneFormatConfig::LocalizedGMT,
            TimeZoneFormatConfig::Iso8601(iso_format, iso_minutes, iso_seconds) => {
                time_zone::TimeZoneFormatConfig::Iso8601(
                    iso_format.into(),
                    iso_minutes.into(),
                    iso_seconds.into(),
                )
            }
        }
    }
}
