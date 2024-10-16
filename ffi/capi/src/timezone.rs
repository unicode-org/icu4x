// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use core::fmt::Write;
    use icu_timezone::TimeZoneBcp47Id;

    use crate::{datetime::ffi::IsoDateTime, errors::ffi::TimeZoneInvalidOffsetError};

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::TimeZoneInfo, Struct)]
    pub struct TimeZoneInfo(pub icu_timezone::TimeZoneInfo);

    impl TimeZoneInfo {
        /// Creates a time zone with no information.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::unknown, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id::unknown, FnInStruct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        pub fn unknown() -> Box<TimeZoneInfo> {
            Box::new(icu_timezone::TimeZoneInfo::unknown().into())
        }

        /// Creates a time zone for UTC (Coordinated Universal Time).
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::utc, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::zero, FnInStruct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor)]
        pub fn utc() -> Box<TimeZoneInfo> {
            Box::new(icu_timezone::TimeZoneInfo::utc().into())
        }

        /// Sets the `offset` field from offset seconds.
        ///
        /// Errors if the offset seconds are out of range.
        #[diplomat::rust_link(icu::timezone::UtcOffset::try_from_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::from_seconds_unchecked, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::new_with_offset, FnInStruct, hidden)]
        pub fn try_set_offset_seconds(
            &mut self,
            offset_seconds: i32,
        ) -> Result<(), TimeZoneInvalidOffsetError> {
            self.0.offset = Some(icu_timezone::UtcOffset::try_from_seconds(offset_seconds)?);
            Ok(())
        }

        /// Sets the `offset` field from offset as eighths of an hour.
        #[diplomat::rust_link(icu::timezone::UtcOffset::from_eighths_of_hour, FnInStruct)]
        pub fn set_offset_eighths_of_hour(&mut self, offset_eighths_of_hour: i8) {
            self.0.offset = Some(icu_timezone::UtcOffset::from_eighths_of_hour(
                offset_eighths_of_hour,
            ));
        }

        /// Sets the `offset` field from a string.
        #[diplomat::rust_link(icu::timezone::UtcOffset::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::from_str, FnInStruct, hidden)]
        pub fn try_set_offset_str(
            &mut self,
            offset: &DiplomatStr,
        ) -> Result<(), TimeZoneInvalidOffsetError> {
            self.0.offset = Some(
                icu_timezone::UtcOffset::try_from_utf8(offset)
                    .map_err(|_| TimeZoneInvalidOffsetError)?,
            );
            Ok(())
        }

        /// Gets the `offset` field from offset as eighths of an hour.
        #[diplomat::rust_link(icu::timezone::UtcOffset::to_eighths_of_hour, FnInStruct)]
        pub fn offset_eighths_of_hour(&self) -> Option<i8> {
            self.0
                .offset
                .map(icu_timezone::UtcOffset::to_eighths_of_hour)
        }

        /// Clears the `offset` field.
        #[diplomat::rust_link(icu::timezone::UtcOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset, Struct, compact)]
        pub fn clear_offset(&mut self) {
            self.0.offset.take();
        }

        /// Returns the value of the `offset` field as offset seconds.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::to_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset, Struct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_seconds(&self) -> Option<i32> {
            self.0.offset.map(icu_timezone::UtcOffset::to_seconds)
        }

        /// Returns whether the `offset` field is positive.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::is_positive, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn is_offset_positive(&self) -> Option<bool> {
            self.0.offset.map(icu_timezone::UtcOffset::is_positive)
        }

        /// Returns whether the `offset` field is zero.
        ///
        /// Returns null if the `offset` field is empty (which is not the same as zero).
        #[diplomat::rust_link(icu::timezone::UtcOffset::is_zero, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn is_offset_zero(&self) -> Option<bool> {
            self.0.offset.map(icu_timezone::UtcOffset::is_zero)
        }

        /// Returns the hours part of the the `offset` field.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::hours_part, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_hours_part(&self) -> Option<i32> {
            self.0.offset.map(|o| o.hours_part())
        }

        /// Returns the minutes part of the the `offset` field.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::minutes_part, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_minutes_part(&self) -> Option<u32> {
            self.0.offset.map(|o| o.minutes_part())
        }

        /// Returns the seconds part of the the `offset` field.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::seconds_part, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_seconds_part(&self) -> Option<u32> {
            self.0.offset.map(|o| o.seconds_part())
        }

        /// Sets the `time_zone_id` field from a BCP-47 string.
        ///
        /// Errors if the string is not a valid BCP-47 time zone ID.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id::from_str, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id::deref, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::timezone::TimeZoneBcp47Id::Target,
            AssociatedTypeInStruct,
            hidden
        )]
        pub fn set_time_zone_id(&mut self, id: &DiplomatStr) {
            self.0.time_zone_id = tinystr::TinyAsciiStr::try_from_utf8(id)
                .map(TimeZoneBcp47Id)
                .unwrap_or(TimeZoneBcp47Id::unknown());
        }

        /// Sets the `time_zone_id` field from an IANA string by looking up
        /// the corresponding BCP-47 string.
        pub fn set_iana_time_zone_id(
            &mut self,
            mapper: &crate::timezone_mapper::ffi::TimeZoneIdMapper,
            id: &DiplomatStr,
        ) {
            self.0.time_zone_id = mapper.0.as_borrowed().iana_bytes_to_bcp47(id);
        }

        /// Writes the value of the `time_zone_id` field as a string.
        ///
        /// Returns null if the `time_zone_id` field is empty.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::time_zone_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn time_zone_id(&self, write: &mut diplomat_runtime::DiplomatWrite) {
            let _infallible = write.write_str(self.0.time_zone_id.0.as_str());
        }

        /// Sets the `zone_variant` field from a string.
        ///
        /// Returns null if the string is not a valid zone variant.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant::from_str, FnInStruct, hidden)]
        pub fn try_set_zone_variant(&mut self, id: &DiplomatStr) -> Option<()> {
            self.0.zone_variant = Some(icu_timezone::ZoneVariant(
                tinystr::TinyAsciiStr::try_from_utf8(id).ok()?,
            ));
            Some(())
        }

        /// Clears the `zone_variant` field.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        pub fn clear_zone_variant(&mut self) {
            self.0.zone_variant.take();
        }

        /// Writes the value of the `zone_variant` field as a string.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::zone_variant, StructField)]
        #[diplomat::rust_link(icu::timezone::ZoneVariant, Struct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn zone_variant(&self, write: &mut diplomat_runtime::DiplomatWrite) -> Option<()> {
            let _infallible = write.write_str(self.0.zone_variant?.0.as_str());
            Some(())
        }

        /// Sets the `zone_variant` field to "standard" time, which may or may
        /// not correspond to a display name with "Standard" in its name.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::zone_variant, StructField, compact)]
        pub fn set_standard_time(&mut self) {
            self.0.zone_variant = Some(icu_timezone::ZoneVariant::standard())
        }

        /// Sets the `zone_variant` field to "daylight" time, which may or may
        /// not correspond to a display name with "Daylight" in its name.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::zone_variant, StructField, compact)]
        pub fn set_daylight_time(&mut self) {
            self.0.zone_variant = Some(icu_timezone::ZoneVariant::daylight())
        }

        /// Returns whether the `zone_variant` field is standard time.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::standard, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::zone_variant, StructField, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn is_standard_time(&self) -> Option<bool> {
            Some(self.0.zone_variant? == icu_timezone::ZoneVariant::standard())
        }

        /// Returns whether the `zone_variant` field is daylight time.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::timezone::ZoneVariant::daylight, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::zone_variant, StructField, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn is_daylight_time(&self) -> Option<bool> {
            Some(self.0.zone_variant? == icu_timezone::ZoneVariant::daylight())
        }

        /// Sets the `local_time` field.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::local_time, StructField)]
        pub fn set_local_time(&mut self, datetime: &IsoDateTime) {
            self.0.local_time = Some((datetime.0.date, datetime.0.time.clone()));
        }

        /// Clears the `local_time` field.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::local_time, StructField)]
        pub fn clear_local_time(&mut self) {
            self.0.local_time.take();
        }

        /// Returns a copy of the `local_time` field.
        #[diplomat::rust_link(icu::timezone::TimeZoneInfo::local_time, StructField, compact)]
        pub fn get_local_time(&self) -> Option<Box<IsoDateTime>> {
            self.0
                .local_time
                .map(|(date, time)| Box::new(IsoDateTime(icu_calendar::DateTime { date, time })))
        }
    }
}

impl From<icu_timezone::TimeZoneInfo> for ffi::TimeZoneInfo {
    fn from(other: icu_timezone::TimeZoneInfo) -> Self {
        Self(other)
    }
}

impl From<ffi::TimeZoneInfo> for icu_timezone::TimeZoneInfo {
    fn from(other: ffi::TimeZoneInfo) -> Self {
        other.0
    }
}
