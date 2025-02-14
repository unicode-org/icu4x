// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ffi::TimeZoneInfo;

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use core::fmt::Write;
    use icu_time::{
        zone::{TimeZoneVariant, UtcOffset},
        TimeZone,
    };
    use tinystr::TinyAsciiStr;

    use crate::{
        date::ffi::IsoDate, datetime::ffi::IsoDateTime, errors::ffi::TimeZoneInvalidOffsetError,
        time::ffi::Time,
    };

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::time::TimeZoneInfo, Struct)]
    pub struct TimeZoneInfo {
        pub(crate) time_zone_id: icu_time::TimeZone,
        pub(crate) offset: Option<UtcOffset>,
        pub(crate) zone_variant: Option<TimeZoneVariant>,
        pub(crate) local_time: Option<(icu_calendar::Date<icu_calendar::Iso>, icu_time::Time)>,
    }

    impl TimeZoneInfo {
        /// Creates a time zone with no information.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::unknown, FnInStruct)]
        #[diplomat::rust_link(icu::time::TimeZone::unknown, FnInStruct, hidden)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn unknown() -> Box<TimeZoneInfo> {
            Box::new(icu_time::TimeZoneInfo::unknown().into())
        }

        /// Creates a time zone for UTC (Coordinated Universal Time).
        #[diplomat::rust_link(icu::time::TimeZoneInfo::utc, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset::zero, FnInStruct, hidden)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn utc() -> Box<TimeZoneInfo> {
            Box::new(icu_time::TimeZoneInfo::utc().into())
        }

        /// Creates a time zone.
        #[diplomat::attr(auto, constructor)]
        pub fn from_parts(
            bcp47_id: &DiplomatStr,
            offset_seconds: i32,
            dst: bool,
        ) -> Box<TimeZoneInfo> {
            Box::new(Self {
                time_zone_id: TinyAsciiStr::try_from_utf8(bcp47_id)
                    .ok()
                    .map(TimeZone)
                    .unwrap_or(TimeZone::unknown()),
                offset: UtcOffset::try_from_seconds(offset_seconds).ok(),
                zone_variant: Some(if dst {
                    TimeZoneVariant::Daylight
                } else {
                    TimeZoneVariant::Standard
                }),
                local_time: None,
            })
        }

        /// Sets the `offset` field from offset seconds.
        ///
        /// Errors if the offset seconds are out of range.
        #[diplomat::rust_link(icu::time::UtcOffset, Struct, compact)]
        #[diplomat::rust_link(icu::time::TimeZone::with_offset, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset::try_from_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset::from_seconds_unchecked, FnInStruct, hidden)]
        pub fn try_set_offset_seconds(
            &mut self,
            offset_seconds: i32,
        ) -> Result<(), TimeZoneInvalidOffsetError> {
            self.offset = Some(UtcOffset::try_from_seconds(offset_seconds)?);
            Ok(())
        }

        /// Sets the `offset` field from offset as eighths of an hour.
        #[diplomat::rust_link(icu::time::UtcOffset, Struct, compact)]
        #[diplomat::rust_link(icu::time::TimeZone::with_offset, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset::from_eighths_of_hour, FnInStruct)]
        pub fn set_offset_eighths_of_hour(&mut self, offset_eighths_of_hour: i8) {
            self.offset = Some(UtcOffset::from_eighths_of_hour(offset_eighths_of_hour));
        }

        /// Sets the `offset` field from a string.
        #[diplomat::rust_link(icu::time::UtcOffset, Struct, compact)]
        #[diplomat::rust_link(icu::time::TimeZone::with_offset, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::time::UtcOffset::from_str, FnInStruct, hidden)]
        pub fn try_set_offset_str(
            &mut self,
            offset: &DiplomatStr,
        ) -> Result<(), TimeZoneInvalidOffsetError> {
            self.offset =
                Some(UtcOffset::try_from_utf8(offset).map_err(|_| TimeZoneInvalidOffsetError)?);
            Ok(())
        }

        /// Gets the `offset` field from offset as eighths of an hour.
        #[diplomat::rust_link(icu::time::UtcOffset::to_eighths_of_hour, FnInStruct)]
        pub fn offset_eighths_of_hour(&self) -> Option<i8> {
            self.offset.map(UtcOffset::to_eighths_of_hour)
        }

        /// Clears the `offset` field.
        #[diplomat::rust_link(icu::time::UtcOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset, Struct, compact)]
        #[diplomat::rust_link(icu::time::TimeZone::without_offset, FnInStruct, compact)]
        pub fn clear_offset(&mut self) {
            self.offset.take();
        }

        /// Returns the value of the `offset` field as offset seconds.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::offset, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset::to_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::time::UtcOffset, Struct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_seconds(&self) -> Option<i32> {
            self.offset.map(UtcOffset::to_seconds)
        }

        /// Returns whether the `offset` field is positive.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::time::UtcOffset::is_non_negative, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn is_offset_non_negative(&self) -> Option<bool> {
            self.offset.map(UtcOffset::is_non_negative)
        }

        /// Returns whether the `offset` field is zero.
        ///
        /// Returns null if the `offset` field is empty (which is not the same as zero).
        #[diplomat::rust_link(icu::time::UtcOffset::is_zero, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn is_offset_zero(&self) -> Option<bool> {
            self.offset.map(UtcOffset::is_zero)
        }

        /// Returns the hours part of the the `offset` field.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::time::UtcOffset::hours_part, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_hours_part(&self) -> Option<i32> {
            self.offset.map(|o| o.hours_part())
        }

        /// Returns the minutes part of the the `offset` field.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::time::UtcOffset::minutes_part, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_minutes_part(&self) -> Option<u32> {
            self.offset.map(|o| o.minutes_part())
        }

        /// Returns the seconds part of the the `offset` field.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::time::UtcOffset::seconds_part, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_seconds_part(&self) -> Option<u32> {
            self.offset.map(|o| o.seconds_part())
        }

        /// Sets the `time_zone_id` field from a BCP-47 string.
        ///
        /// Errors if the string is not a valid BCP-47 time zone ID.
        #[diplomat::rust_link(icu::time::TimeZone, Struct, compact)]
        #[diplomat::rust_link(icu::time::TimeZone::from_str, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::time::TimeZone::deref, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::time::TimeZone::Target, AssociatedTypeInStruct, hidden)]
        pub fn set_time_zone_id(&mut self, id: &DiplomatStr) {
            self.time_zone_id = tinystr::TinyAsciiStr::try_from_utf8(id)
                .map(TimeZone)
                .unwrap_or(TimeZone::unknown());
        }

        /// Sets the `time_zone_id` field from an IANA string by looking up
        /// the corresponding BCP-47 string.
        pub fn set_iana_time_zone_id(
            &mut self,
            mapper: &crate::timezone_mapper::ffi::IanaParser,
            id: &DiplomatStr,
        ) {
            self.time_zone_id = mapper.0.as_borrowed().iana_bytes_to_bcp47(id);
        }

        /// Writes the value of the `time_zone_id` field as a string.
        ///
        /// Returns null if the `time_zone_id` field is empty.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::time_zone_id, FnInStruct)]
        #[diplomat::rust_link(icu::time::TimeZone, Struct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn time_zone_id(&self, write: &mut diplomat_runtime::DiplomatWrite) {
            let _infallible = write.write_str(self.time_zone_id.0.as_str());
        }

        /// Clears the `zone_variant` field.
        #[diplomat::rust_link(icu::time::TimeZoneVariant, Enum, compact)]
        pub fn clear_zone_variant(&mut self) {
            self.zone_variant.take();
        }

        /// Sets the `zone_variant` field to standard time, which may or may
        /// not correspond to a display name with Standard in its name.
        #[diplomat::rust_link(icu::time::TimeZoneVariant::Standard, EnumVariant)]
        #[diplomat::rust_link(icu::time::TimeZoneInfo::with_zone_variant, FnInStruct)]
        pub fn set_standard_time(&mut self) {
            self.zone_variant = Some(TimeZoneVariant::Standard)
        }

        /// Sets the `zone_variant` field to "daylight" time, which may or may
        /// not correspond to a display name with "Daylight" in its name.
        #[diplomat::rust_link(icu::time::TimeZoneVariant::Daylight, EnumVariant)]
        #[diplomat::rust_link(icu::time::TimeZoneInfo::with_zone_variant, FnInStruct)]
        pub fn set_daylight_time(&mut self) {
            self.zone_variant = Some(TimeZoneVariant::Daylight)
        }

        /// Returns whether the `zone_variant` field is standard time.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::time::TimeZoneVariant::Standard, EnumVariant)]
        #[diplomat::rust_link(icu::time::TimeZoneInfo::zone_variant, FnInStruct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn is_standard_time(&self) -> Option<bool> {
            Some(self.zone_variant? == TimeZoneVariant::Standard)
        }

        /// Returns whether the `zone_variant` field is daylight time.
        ///
        /// Returns null if the `zone_variant` field is empty.
        #[diplomat::rust_link(icu::time::TimeZoneVariant::Daylight, EnumVariant)]
        #[diplomat::rust_link(icu::time::TimeZoneInfo::zone_variant, FnInStruct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn is_daylight_time(&self) -> Option<bool> {
            Some(self.zone_variant? == TimeZoneVariant::Daylight)
        }

        /// Sets the `local_time` field.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::at_time, FnInStruct)]
        pub fn set_local_time(&mut self, date: &IsoDate, time: &Time) {
            self.local_time = Some((date.0, time.0));
        }

        /// Clears the `local_time` field.
        pub fn clear_local_time(&mut self) {
            self.local_time.take();
        }

        /// Returns a copy of the `local_time` field.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::local_time, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn get_local_time(&self) -> Option<IsoDateTime> {
            self.local_time.map(|(date, time)| IsoDateTime {
                date: Box::new(IsoDate(date)),
                time: Box::new(Time(time)),
            })
        }
    }
}

impl From<icu_time::TimeZoneInfo<icu_time::zone::models::Base>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::Base>) -> Self {
        Self {
            time_zone_id: other.time_zone_id(),
            offset: other.offset(),
            zone_variant: None,
            local_time: None,
        }
    }
}

impl From<icu_time::TimeZoneInfo<icu_time::zone::models::Full>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::Full>) -> Self {
        Self {
            time_zone_id: other.time_zone_id(),
            offset: other.offset(),
            zone_variant: Some(other.zone_variant()),
            local_time: Some(other.local_time()),
        }
    }
}
