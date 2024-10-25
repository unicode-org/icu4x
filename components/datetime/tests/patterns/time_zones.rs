// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::neo_skeleton::{NeoSkeletonLength, NeoTimeZoneSkeleton, NeoTimeZoneStyle};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTests(pub Vec<TimeZoneTest>);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneTest {
    pub locale: String,
    pub datetime: String,
    pub expectations: Vec<TimeZoneExpectation>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TimeZoneExpectation {
    pub patterns: Vec<String>,
    pub configs: Vec<TimeZoneFormatterConfig>,
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
    LocalizedOffsetLong,
    LocalizedOffsetShort,
    Iso8601(IsoFormat, IsoMinutes, IsoSeconds),
}

impl TimeZoneFormatterConfig {
    pub fn to_semantic_skeleton(self) -> NeoTimeZoneSkeleton {
        match self {
            TimeZoneFormatterConfig::GenericNonLocationLong => {
                NeoTimeZoneSkeleton::for_length_and_components(
                    NeoSkeletonLength::Long,
                    NeoTimeZoneStyle::Generic,
                )
            }
            TimeZoneFormatterConfig::GenericNonLocationShort => {
                NeoTimeZoneSkeleton::for_length_and_components(
                    NeoSkeletonLength::Short,
                    NeoTimeZoneStyle::Generic,
                )
            }
            TimeZoneFormatterConfig::GenericLocation => {
                NeoTimeZoneSkeleton::for_length_and_components(
                    NeoSkeletonLength::Long,
                    NeoTimeZoneStyle::Location,
                )
            }
            TimeZoneFormatterConfig::SpecificNonLocationLong => {
                NeoTimeZoneSkeleton::for_length_and_components(
                    NeoSkeletonLength::Long,
                    NeoTimeZoneStyle::Specific,
                )
            }
            TimeZoneFormatterConfig::SpecificNonLocationShort => {
                NeoTimeZoneSkeleton::for_length_and_components(
                    NeoSkeletonLength::Short,
                    NeoTimeZoneStyle::Specific,
                )
            }
            TimeZoneFormatterConfig::LocalizedOffsetLong => {
                NeoTimeZoneSkeleton::for_length_and_components(
                    NeoSkeletonLength::Long,
                    NeoTimeZoneStyle::Offset,
                )
            }
            TimeZoneFormatterConfig::LocalizedOffsetShort => {
                NeoTimeZoneSkeleton::for_length_and_components(
                    NeoSkeletonLength::Short,
                    NeoTimeZoneStyle::Offset,
                )
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
    }
}
