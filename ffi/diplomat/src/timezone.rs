// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_timezone::CustomTimeZone;

#[diplomat::bridge]
pub mod ffi {
    use crate::datetime::ffi::ICU4XIsoDateTime;
    use crate::errors::ffi::ICU4XError;
    use crate::provider::ffi::ICU4XDataProvider;
    use alloc::boxed::Box;
    use core::fmt::Write;
    use core::str::FromStr;
    use diplomat_runtime::DiplomatResult;
    use icu_timezone::CustomTimeZone;
    use icu_timezone::GmtOffset;
    use icu_timezone::MetaZoneCalculator;
    use icu_timezone::ZoneVariant;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::CustomTimeZone, Struct)]
    pub struct ICU4XCustomTimeZone(pub CustomTimeZone);

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::MetaZoneCalculator, Struct)]
    pub struct ICU4XMetaZoneCalculator(pub MetaZoneCalculator);

    impl ICU4XCustomTimeZone {
        /// Creates a time zone from an offset string.
        pub fn create_from_str(s: &str) -> DiplomatResult<Box<ICU4XCustomTimeZone>, ICU4XError> {
            CustomTimeZone::from_str(s)
                .map(ICU4XCustomTimeZone::from)
                .map(Box::from)
                .map_err(Into::into)
                .into()
        }

        /// Creates a time zone with no information.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::new_empty, FnInStruct)]
        pub fn create_empty() -> Box<ICU4XCustomTimeZone> {
            Box::new(CustomTimeZone::new_empty().into())
        }

        /// Creates a time zone for UTC.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::utc, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset::utc, FnInStruct, hidden)]
        pub fn create_utc() -> Box<ICU4XCustomTimeZone> {
            Box::new(CustomTimeZone::utc().into())
        }

        /// Sets the `gmt_offset` field from offset seconds.
        ///
        /// Errors if the offset seconds are out of range.
        #[diplomat::rust_link(icu::timezone::GmtOffset::try_from_offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::new_with_offset, FnInStruct, hidden)]
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

        /// Clears the `gmt_offset` field.
        #[diplomat::rust_link(icu::timezone::GmtOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        pub fn clear_gmt_offset(&mut self) {
            self.0.gmt_offset.take();
        }

        /// Returns the value of the `gmt_offset` field as offset seconds.
        ///
        /// Errors if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::GmtOffset, Struct, compact)]
        pub fn gmt_offset_seconds(&self) -> DiplomatResult<i32, ICU4XError> {
            self.0
                .gmt_offset
                .map(|v| v.offset_seconds())
                .ok_or(ICU4XError::TimeZoneMissingInputError)
                .into()
        }

        /// Returns whether the `gmt_offset` field is positive.
        ///
        /// Errors if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::is_positive, FnInStruct)]
        pub fn is_gmt_offset_positive(&self) -> DiplomatResult<bool, ICU4XError> {
            self.0
                .gmt_offset
                .map(|v| v.is_positive())
                .ok_or(ICU4XError::TimeZoneMissingInputError)
                .into()
        }

        /// Returns whether the `gmt_offset` field is zero.
        ///
        /// Errors if the `gmt_offset` field is empty (which is not the same as zero).
        #[diplomat::rust_link(icu::timezone::GmtOffset::is_zero, FnInStruct)]
        pub fn is_gmt_offset_zero(&self) -> DiplomatResult<bool, ICU4XError> {
            self.0
                .gmt_offset
                .map(|v| v.is_zero())
                .ok_or(ICU4XError::TimeZoneMissingInputError)
                .into()
        }

        /// Returns whether the `gmt_offset` field has nonzero minutes.
        ///
        /// Errors if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::has_minutes, FnInStruct)]
        pub fn gmt_offset_has_minutes(&self) -> DiplomatResult<bool, ICU4XError> {
            self.0
                .gmt_offset
                .map(|v| v.has_minutes())
                .ok_or(ICU4XError::TimeZoneMissingInputError)
                .into()
        }

        /// Returns whether the `gmt_offset` field has nonzero seconds.
        ///
        /// Errors if the `gmt_offset` field is empty.
        #[diplomat::rust_link(icu::timezone::GmtOffset::has_seconds, FnInStruct)]
        pub fn gmt_offset_has_seconds(&self) -> DiplomatResult<bool, ICU4XError> {
            self.0
                .gmt_offset
                .map(|v| v.has_seconds())
                .ok_or(ICU4XError::TimeZoneMissingInputError)
                .into()
        }

        /// Sets the `time_zone_id` field from a string.
        ///
        /// Errors if the string is not a valid BCP-47 time zone ID.
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

        /// Clears the `time_zone_id` field.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        pub fn clear_time_zone_id(&mut self) {
            self.0.time_zone_id.take();
        }

        /// Writes the value of the `time_zone_id` field as a string.
        ///
        /// Errors if the `time_zone_id` field is empty.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        pub fn time_zone_id(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = match self.0.time_zone_id {
                Some(v) => write.write_str(v.0.as_str()).map_err(Into::into),
                None => Err(ICU4XError::TimeZoneMissingInputError),
            }
            .into();
            write.flush();
            result
        }

        /// Sets the `meta_zone_id` field from a string.
        ///
        /// Errors if the string is not a valid BCP-47 meta zone ID.
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

        /// Clears the `meta_zone_id` field.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::meta_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::MetaZoneId, Struct, compact)]
        pub fn clear_meta_zone_id(&mut self) {
            self.0.meta_zone_id.take();
        }

        /// Writes the value of the `meta_zone_id` field as a string.
        ///
        /// Errors if the `meta_zone_id` field is empty.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::meta_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::MetaZoneId, Struct, compact)]
        pub fn meta_zone_id(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = match self.0.meta_zone_id {
                Some(v) => write.write_str(v.0.as_str()).map_err(Into::into),
                None => Err(ICU4XError::TimeZoneMissingInputError),
            }
            .into();
            write.flush();
            result
        }

        /// Sets the `zone_variant` field from a string.
        ///
        /// Errors if the string is not a valid zone variant.
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

        /// Clears the `zone_variant` field.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        pub fn clear_zone_variant(&mut self) {
            self.0.zone_variant.take();
        }

        /// Writes the value of the `zone_variant` field as a string.
        ///
        /// Errors if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        pub fn zone_variant(
            &self,
            write: &mut diplomat_runtime::DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let result = match self.0.zone_variant {
                Some(v) => write.write_str(v.0.as_str()).map_err(Into::into),
                None => Err(ICU4XError::TimeZoneMissingInputError),
            }
            .into();
            write.flush();
            result
        }

        /// Sets the `zone_variant` field to standard time.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn set_standard_time(&mut self) {
            self.0.zone_variant = Some(ZoneVariant::standard())
        }

        /// Sets the `zone_variant` field to daylight time.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn set_daylight_time(&mut self) {
            self.0.zone_variant = Some(ZoneVariant::daylight())
        }

        /// Returns whether the `zone_variant` field is standard time.
        ///
        /// Errors if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn is_standard_time(&self) -> DiplomatResult<bool, ICU4XError> {
            self.0
                .zone_variant
                .as_ref()
                .map(|v| v == &ZoneVariant::standard())
                .ok_or(ICU4XError::TimeZoneMissingInputError)
                .into()
        }

        /// Returns whether the `zone_variant` field is daylight time.
        ///
        /// Errors if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::zone_variant, StructField, compact)]
        pub fn is_daylight_time(&self) -> DiplomatResult<bool, ICU4XError> {
            self.0
                .zone_variant
                .as_ref()
                .map(|v| v == &ZoneVariant::daylight())
                .ok_or(ICU4XError::TimeZoneMissingInputError)
                .into()
        }

        /// Sets the meta zone based on the time zone and the local timestamp.
        #[diplomat::rust_link(icu::timezone::CustomTimeZone::maybe_calculate_meta_zone, FnInStruct)]
        #[diplomat::rust_link(
            icu::timezone::MetaZoneCalculator::compute_metazone_from_timezone,
            FnInStruct,
            compact
        )]
        pub fn maybe_calculate_meta_zone(
            &mut self,
            metazone_calculator: &ICU4XMetaZoneCalculator,
            local_datetime: &ICU4XIsoDateTime,
        ) {
            self.0
                .maybe_calculate_meta_zone(&metazone_calculator.0, &local_datetime.0);
        }
    }

    impl ICU4XMetaZoneCalculator {
        #[diplomat::rust_link(icu::timezone::MetaZoneCalculator::try_new_unstable, FnInStruct)]
        pub fn try_new(
            provider: &ICU4XDataProvider,
        ) -> DiplomatResult<Box<ICU4XMetaZoneCalculator>, ICU4XError> {
            MetaZoneCalculator::try_new_unstable(&provider.0)
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
