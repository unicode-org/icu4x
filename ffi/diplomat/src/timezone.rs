// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_timezone::CustomTimeZone;
use icu_timezone::GmtOffset;
use icu_timezone::MetaZoneId;
use icu_timezone::TimeZoneBcp47Id;
use icu_timezone::ZoneVariant;

#[diplomat::bridge]
pub mod ffi {
    use crate::calendar::ffi::ICU4XIsoDateTime;
    use crate::errors::ffi::ICU4XError;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::str::FromStr;
    use diplomat_runtime::DiplomatResult;
    use icu_timezone::CustomTimeZone;
    use icu_timezone::GmtOffset;
    use icu_timezone::MetaZoneCalculator;
    use icu_timezone::TimeZoneBcp47Id;
    use icu_timezone::ZoneVariant;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::CustomTimeZone, Struct)]
    pub struct ICU4XCustomTimeZone(pub CustomTimeZone);

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::MetaZoneCalculator, Struct)]
    pub struct ICU4XMetaZoneCalculator(pub MetaZoneCalculator);

    impl ICU4XCustomTimeZone {
        pub fn create_from_str(s: &str) -> DiplomatResult<Box<ICU4XCustomTimeZone>, ICU4XError> {
            CustomTimeZone::from_str(s)
                .map(ICU4XCustomTimeZone::from)
                .map(Box::from)
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::timezone::GmtOffset::try_from_offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        pub fn try_set_gmt_offset_seconds(
            &mut self,
            offset_seconds: i32,
        ) -> DiplomatResult<(), ICU4XError> {
            match GmtOffset::try_from_offset_seconds(offset_seconds) {
                Ok(v) => {
                    self.0.gmt_offset = Some(v);
                    Ok(())
                }
                Err(e) => Err(ICU4XError::from(e)),
            }
            .into()
        }

        #[diplomat::rust_link(icu::timezone::GmtOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        pub fn gmt_offset_seconds(&self) -> DiplomatResult<i32, ()> {
            self.0
                .gmt_offset
                .map(|v| v.offset_seconds())
                .ok_or(())
                .into()
        }

        #[diplomat::rust_link(icu::timezone::GmtOffset::is_positive, FnInStruct)]
        pub fn is_gmt_offset_positive(&self) -> DiplomatResult<bool, ()> {
            self.0.gmt_offset.map(|v| v.is_positive()).ok_or(()).into()
        }

        #[diplomat::rust_link(icu::timezone::GmtOffset::is_zero, FnInStruct)]
        pub fn is_gmt_offset_zero(&self) -> DiplomatResult<bool, ()> {
            self.0.gmt_offset.map(|v| v.is_zero()).ok_or(()).into()
        }

        #[diplomat::rust_link(icu::timezone::GmtOffset::has_minutes, FnInStruct)]
        pub fn gmt_offset_has_minutes(&self) -> DiplomatResult<bool, ()> {
            self.0.gmt_offset.map(|v| v.has_minutes()).ok_or(()).into()
        }

        #[diplomat::rust_link(icu::timezone::GmtOffset::has_seconds, FnInStruct)]
        pub fn gmt_offset_has_seconds(&self) -> DiplomatResult<bool, ()> {
            self.0.gmt_offset.map(|v| v.has_seconds()).ok_or(()).into()
        }

        #[diplomat::rust_link(icu::timezone::CustomTimeZone::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        pub fn try_set_time_zone_id(&mut self, id: &str) -> DiplomatResult<(), ICU4XError> {
            match id.parse() {
                Ok(v) => {
                    self.0.time_zone_id = Some(v);
                    Ok(())
                }
                Err(e) => Err(ICU4XError::from(e)),
            }
            .into()
        }

        #[diplomat::rust_link(icu::timezone::CustomTimeZone::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        pub fn time_zone_id<'a>(&'a self) -> DiplomatResult<&'a str, ()> {
            self.0
                .time_zone_id
                .as_ref()
                .map(|v| v.0.as_str())
                .ok_or(())
                .into()
        }

        #[diplomat::rust_link(icu::timezone::CustomTimeZone::meta_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::MetaZoneId, Struct, compact)]
        pub fn try_set_meta_zone_id(&mut self, id: &str) -> DiplomatResult<(), ICU4XError> {
            match id.parse() {
                Ok(v) => {
                    self.0.meta_zone_id = Some(v);
                    Ok(())
                }
                Err(e) => Err(ICU4XError::from(e)),
            }
            .into()
        }

        #[diplomat::rust_link(icu::timezone::CustomTimeZone::meta_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::MetaZoneId, Struct, compact)]
        pub fn meta_zone_id<'a>(&'a self) -> DiplomatResult<&'a str, ()> {
            self.0
                .meta_zone_id
                .as_ref()
                .map(|v| v.0.as_str())
                .ok_or(())
                .into()
        }

        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        pub fn try_set_zone_variant(&mut self, id: &str) -> DiplomatResult<(), ICU4XError> {
            match id.parse() {
                Ok(v) => {
                    self.0.zone_variant = Some(v);
                    Ok(())
                }
                Err(e) => Err(ICU4XError::from(e)),
            }
            .into()
        }

        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        pub fn zone_variant<'a>(&'a self) -> DiplomatResult<&'a str, ()> {
            self.0
                .zone_variant
                .as_ref()
                .map(|v| v.0.as_str())
                .ok_or(())
                .into()
        }

        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn set_standard_time(&mut self) {
            self.0.zone_variant = Some(ZoneVariant::standard())
        }

        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn set_daylight_time(&mut self) {
            self.0.zone_variant = Some(ZoneVariant::daylight())
        }

        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn is_standard_time(&self) -> DiplomatResult<bool, ()> {
            self.0
                .zone_variant
                .as_ref()
                .map(|v| v == &ZoneVariant::standard())
                .ok_or(())
                .into()
        }

        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn is_daylight_time(&self) -> DiplomatResult<bool, ()> {
            self.0
                .zone_variant
                .as_ref()
                .map(|v| v == &ZoneVariant::daylight())
                .ok_or(())
                .into()
        }

        #[diplomat::rust_link(icu::timezone::CustomTimeZone::maybe_set_meta_zone, FnInStruct)]
        #[diplomat::rust_link(
            icu::timezone::MetaZoneCalculator::compute_metazone_from_timezone,
            FnInStruct,
            compact
        )]
        pub fn maybe_set_meta_zone(
            &mut self,
            local_datetime: &ICU4XIsoDateTime,
            metazone_calculator: &ICU4XMetaZoneCalculator,
        ) {
            self.0
                .maybe_set_meta_zone(&local_datetime.0, &metazone_calculator.0);
        }
    }

    impl ICU4XMetaZoneCalculator {
        #[diplomat::rust_link(icu::timezone::MetaZoneCalculator::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XMetaZoneCalculator>, ICU4XError> {
            use icu_provider::serde::AsDeserializingBufferProvider;
            let provider = provider.0.as_deserializing();

            MetaZoneCalculator::try_new_unstable(&provider)
                .map(|tf| Box::new(ICU4XMetaZoneCalculator(tf)))
                .map_err(Into::into)
                .into()
        }
    }
}

impl From<CustomTimeZone> for ffi::ICU4XCustomTimeZone {
    fn from(other: CustomTimeZone) -> Self {
        Self(other)
    }
}

impl From<ffi::ICU4XCustomTimeZone> for CustomTimeZone {
    fn from(other: ffi::ICU4XCustomTimeZone) -> Self {
        other.0
    }
}
