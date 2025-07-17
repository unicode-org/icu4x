// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use ffi::TimeZoneInfo;

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::unstable::{
        date::ffi::IsoDate,
        datetime::ffi::IsoDateTime,
        time::ffi::Time,
        variant_offset::ffi::{UtcOffset, VariantOffsetsCalculator},
    };

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::time::TimeZone, Struct)]
    pub struct TimeZone(pub(crate) icu_time::TimeZone);

    impl TimeZone {
        /// The unknown time zone.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::unknown, FnInStruct)]
        #[diplomat::rust_link(icu::time::TimeZone::unknown, FnInStruct, hidden)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn unknown() -> Box<TimeZone> {
            Box::new(TimeZone(icu_time::TimeZone::UNKNOWN))
        }

        /// Whether the time zone is the unknown zone.
        #[diplomat::rust_link(icu::time::TimeZone::is_unknown, FnInStruct)]
        pub fn is_unknown(&self) -> bool {
            self.0.is_unknown()
        }

        /// Creates a time zone from a BCP-47 string.
        ///
        /// Returns the unknown time zone if the string is not a valid BCP-47 subtag.
        #[diplomat::rust_link(icu::time::TimeZone, Struct, compact)]
        #[diplomat::attr(auto, named_constructor = "from_bcp47")]
        #[diplomat::demo(default_constructor)]
        pub fn create_from_bcp47(id: &DiplomatStr) -> Box<Self> {
            icu_locale_core::subtags::Subtag::try_from_utf8(id)
                .map(icu_time::TimeZone)
                .map(TimeZone)
                .map(Box::new)
                .unwrap_or(Self::unknown())
        }

        #[diplomat::rust_link(icu::time::TimeZone::with_offset, FnInStruct)]
        pub fn with_offset(&self, offset: &UtcOffset) -> Box<TimeZoneInfo> {
            Box::new(self.0.with_offset(Some(offset.0)).into())
        }

        #[diplomat::rust_link(icu::time::TimeZone::without_offset, FnInStruct)]
        pub fn without_offset(&self) -> Box<TimeZoneInfo> {
            Box::new(self.0.without_offset().into())
        }
    }

    #[diplomat::enum_convert(icu_time::zone::TimeZoneVariant, needs_wildcard)]
    #[non_exhaustive]
    /// DEPRECATED
    pub enum TimeZoneVariant {
        // TimeZoneVariant in Rust doesn't have a default, but it is useful to have one
        // here for consistent behavior.
        #[diplomat::attr(auto, default)]
        Standard,
        Daylight,
    }

    impl TimeZoneVariant {
        #[diplomat::rust_link(icu::time::zone::TimeZoneVariant::from_rearguard_isdst, FnInEnum)]
        #[diplomat::rust_link(icu::time::TimeZoneInfo::with_variant, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::TimeZoneVariant, Enum, compact)]
        /// DEPRECATED
        #[allow(deprecated)] // remove in 3.0
        pub fn from_rearguard_isdst(isdst: bool) -> Self {
            icu_time::zone::TimeZoneVariant::from_rearguard_isdst(isdst).into()
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::time::TimeZoneInfo, Struct)]
    #[diplomat::rust_link(icu::time::zone::models::AtTime, Struct, hidden)]
    #[diplomat::rust_link(icu::time::zone::models::Base, Struct, hidden)]
    #[diplomat::rust_link(icu::time::zone::models::Full, Struct, hidden)]
    #[derive(Clone, Copy)]
    pub struct TimeZoneInfo {
        pub(crate) id: icu_time::TimeZone,
        pub(crate) offset: Option<icu_time::zone::UtcOffset>,
        pub(crate) zone_name_timestamp: Option<icu_time::zone::ZoneNameTimestamp>,
    }

    impl TimeZoneInfo {
        /// Creates a time zone for UTC (Coordinated Universal Time).
        #[diplomat::rust_link(icu::time::TimeZoneInfo::utc, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::UtcOffset::zero, FnInStruct, hidden)]
        #[diplomat::attr(auto, named_constructor)]
        pub fn utc() -> Box<TimeZoneInfo> {
            Box::new(icu_time::TimeZoneInfo::utc().into())
        }

        /// Creates a time zone info from parts.
        ///
        /// `variant` is ignored.
        #[diplomat::attr(auto, constructor)]
        pub fn from_parts(
            id: &TimeZone,
            offset: Option<&UtcOffset>,
            _variant: Option<TimeZoneVariant>,
        ) -> Box<TimeZoneInfo> {
            Box::new(Self {
                id: id.0,
                offset: offset.map(|o| o.0),
                zone_name_timestamp: None,
            })
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::id, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // this just returns a constructor argument
        #[diplomat::attr(auto, getter)]
        pub fn id(&self) -> Box<TimeZone> {
            Box::new(TimeZone(self.id))
        }

        /// Sets the datetime at which to interpret the time zone
        /// for display name lookup.
        ///
        /// Notes:
        ///
        /// - If not set, the formatting datetime is used if possible.
        /// - If the offset is not set, the datetime is interpreted as UTC.
        /// - The constraints are the same as with `ZoneNameTimestamp` in Rust.
        /// - Set to year 1000 or 9999 for a reference far in the past or future.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::at_date_time_iso, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::ZoneNameTimestamp, Struct, compact)]
        #[diplomat::rust_link(
            icu::time::TimeZoneInfo::with_zone_name_timestamp,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(
            icu::time::zone::ZoneNameTimestamp::from_date_time_iso,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(
            icu::time::zone::ZoneNameTimestamp::far_in_future,
            FnInStruct,
            hidden
        )] // documented
        #[diplomat::rust_link(icu::time::zone::ZoneNameTimestamp::far_in_past, FnInStruct, hidden)] // documented
        pub fn at_date_time_iso(&self, date: &IsoDate, time: &Time) -> Box<Self> {
            Box::new(Self {
                zone_name_timestamp: Some(
                    icu_time::zone::ZoneNameTimestamp::from_zoned_date_time_iso(
                        icu_time::ZonedDateTime {
                            date: date.0,
                            time: time.0,
                            zone: self.offset.unwrap_or(icu_time::zone::UtcOffset::zero()),
                        },
                    ),
                ),
                ..*self
            })
        }

        /// Sets the timestamp, in milliseconds since Unix epoch, at which to interpret the time zone
        /// for display name lookup.
        ///
        /// Notes:
        ///
        /// - If not set, the formatting datetime is used if possible.
        /// - The constraints are the same as with `ZoneNameTimestamp` in Rust.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::with_zone_name_timestamp, FnInStruct)]
        #[diplomat::rust_link(
            icu::time::zone::ZoneNameTimestamp::from_zoned_date_time_iso,
            FnInStruct,
            compact
        )]
        #[diplomat::rust_link(icu::time::zone::ZoneNameTimestamp, Struct, compact)]
        pub fn at_timestamp(&self, timestamp: i64) -> Box<Self> {
            Box::new(Self {
                zone_name_timestamp: Some(
                    icu_time::zone::ZoneNameTimestamp::from_zoned_date_time_iso(
                        icu_time::ZonedDateTime::from_epoch_milliseconds_and_utc_offset(
                            timestamp,
                            icu_time::zone::UtcOffset::zero(),
                        ),
                    ),
                ),
                ..*self
            })
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::zone_name_timestamp, FnInStruct)]
        #[diplomat::rust_link(
            icu::time::zone::ZoneNameTimestamp::to_zoned_date_time_iso,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(
            icu::time::zone::ZoneNameTimestamp::to_date_time_iso,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(auto, getter)]
        /// Returns the DateTime for the UTC zone name reference time
        pub fn zone_name_date_time(&self) -> Option<IsoDateTime> {
            let datetime = self.zone_name_timestamp?.to_zoned_date_time_iso();
            Some(IsoDateTime {
                date: Box::new(IsoDate(datetime.date)),
                time: Box::new(Time(datetime.time)),
            })
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::with_variant, FnInStruct)]
        /// DEPRECATED
        ///
        /// Just clones
        pub fn with_variant(&self, _time_variant: TimeZoneVariant) -> Box<Self> {
            Box::new(*self)
        }

        #[diplomat::attr(auto, getter)]
        pub fn offset(&self) -> Option<Box<UtcOffset>> {
            self.offset.map(UtcOffset).map(Box::new)
        }

        /// DEPRECATED
        ///
        /// No-op
        #[diplomat::rust_link(icu::time::TimeZoneInfo::infer_variant, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::TimeZoneVariant, Enum, compact)]
        pub fn infer_variant(
            &mut self,
            _offset_calculator: &VariantOffsetsCalculator,
        ) -> Option<()> {
            Some(())
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::variant, FnInStruct)]
        #[diplomat::attr(demo_gen, disable)] // this just returns a constructor argument
        /// DEPRECATED
        pub fn variant(&self) -> Option<TimeZoneVariant> {
            None
        }
    }
}

impl From<icu_time::zone::UtcOffset> for TimeZoneInfo {
    fn from(other: icu_time::zone::UtcOffset) -> Self {
        Self {
            id: icu_time::TimeZone::UNKNOWN,
            offset: Some(other),
            zone_name_timestamp: None,
        }
    }
}

impl From<icu_time::TimeZoneInfo<icu_time::zone::models::Base>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::Base>) -> Self {
        Self {
            id: other.id(),
            offset: other.offset(),
            zone_name_timestamp: None,
        }
    }
}

impl From<icu_time::TimeZoneInfo<icu_time::zone::models::AtTime>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::AtTime>) -> Self {
        Self {
            id: other.id(),
            offset: other.offset(),
            zone_name_timestamp: Some(other.zone_name_timestamp()),
        }
    }
}

#[allow(deprecated)]
impl From<icu_time::TimeZoneInfo<icu_time::zone::models::Full>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::Full>) -> Self {
        Self {
            id: other.id(),
            offset: other.offset(),
            zone_name_timestamp: Some(other.zone_name_timestamp()),
        }
    }
}
