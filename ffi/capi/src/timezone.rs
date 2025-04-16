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
            Box::new(TimeZone(icu_time::TimeZone::unknown()))
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
    pub enum TimeZoneVariant {
        Standard,
        Daylight,
    }

    impl TimeZoneVariant {
        /// Sets the `variant` field to "daylight" time.
        #[diplomat::rust_link(icu::time::zone::TimeZoneVariant::from_rearguard_isdst, FnInEnum)]
        #[diplomat::rust_link(icu::time::TimeZoneInfo::with_variant, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::TimeZoneVariant, Enum, compact)]
        pub fn from_rearguard_isdst(&mut self, isdst: bool) -> Self {
            icu_time::zone::TimeZoneVariant::from_rearguard_isdst(isdst).into()
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::time::TimeZoneInfo, Struct)]
    #[diplomat::rust_link(icu::time::zone::models::AtTime, Struct, hidden)]
    #[diplomat::rust_link(icu::time::zone::models::Base, Struct, hidden)]
    #[diplomat::rust_link(icu::time::zone::models::Full, Struct, hidden)]
    pub struct TimeZoneInfo {
        pub(crate) id: icu_time::TimeZone,
        pub(crate) offset: Option<icu_time::zone::UtcOffset>,
        pub(crate) variant: Option<icu_time::zone::TimeZoneVariant>,
        pub(crate) local_time: Option<(icu_calendar::Date<icu_calendar::Iso>, icu_time::Time)>,
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
        #[diplomat::attr(auto, constructor)]
        pub fn from_parts(
            id: &TimeZone,
            offset: Option<&UtcOffset>,
            variant: Option<TimeZoneVariant>,
        ) -> Box<TimeZoneInfo> {
            Box::new(Self {
                id: id.0,
                offset: offset.map(|o| o.0),
                variant: variant.map(Into::into),
                local_time: None,
            })
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::id, FnInStruct)]
        pub fn id(&self) -> Box<TimeZone> {
            Box::new(TimeZone(self.id))
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::at_time, FnInStruct)]
        pub fn at_time(&self, date: &IsoDate, time: &Time) -> Box<Self> {
            Box::new(Self {
                local_time: Some((date.0, time.0)),
                ..*self
            })
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::local_time, FnInStruct)]
        pub fn local_time(&self) -> Option<IsoDateTime> {
            Some(IsoDateTime {
                date: Box::new(IsoDate(self.local_time?.0)),
                time: Box::new(Time(self.local_time?.1)),
            })
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::with_variant, FnInStruct)]
        pub fn with_variant(&self, time_variant: TimeZoneVariant) -> Box<Self> {
            Box::new(Self {
                variant: Some(time_variant.into()),
                ..*self
            })
        }

        /// Infers the zone variant.
        ///
        /// Requires the offset and local time to be set.
        #[diplomat::rust_link(icu::time::TimeZoneInfo::infer_variant, FnInStruct)]
        #[diplomat::rust_link(icu::time::zone::TimeZoneVariant, Enum, compact)]
        pub fn infer_variant(
            &mut self,
            offset_calculator: &VariantOffsetsCalculator,
        ) -> Option<()> {
            let info = self
                .id
                .with_offset(self.offset)
                .at_time(self.local_time?)
                .infer_variant(offset_calculator.0.as_borrowed());

            self.id = info.id();
            self.variant = Some(info.variant());
            Some(())
        }

        #[diplomat::rust_link(icu::time::TimeZoneInfo::variant, FnInStruct)]
        pub fn variant(&self) -> Option<TimeZoneVariant> {
            self.variant.map(Into::into)
        }
    }
}

impl From<icu_time::zone::UtcOffset> for TimeZoneInfo {
    fn from(other: icu_time::zone::UtcOffset) -> Self {
        Self {
            id: icu_time::TimeZone::unknown(),
            offset: Some(other),
            variant: None,
            local_time: None,
        }
    }
}

impl From<icu_time::TimeZoneInfo<icu_time::zone::models::Base>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::Base>) -> Self {
        Self {
            id: other.id(),
            offset: other.offset(),
            variant: None,
            local_time: None,
        }
    }
}

impl From<icu_time::TimeZoneInfo<icu_time::zone::models::AtTime>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::AtTime>) -> Self {
        Self {
            id: other.id(),
            offset: other.offset(),
            variant: None,
            local_time: Some(other.local_time()),
        }
    }
}

impl From<icu_time::TimeZoneInfo<icu_time::zone::models::Full>> for TimeZoneInfo {
    fn from(other: icu_time::TimeZoneInfo<icu_time::zone::models::Full>) -> Self {
        Self {
            id: other.id(),
            offset: other.offset(),
            variant: Some(other.variant()),
            local_time: Some(other.local_time()),
        }
    }
}
