// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use core::fmt::Write;

    use crate::errors::ffi::TimeZoneInvalidIdError;
    use crate::errors::ffi::TimeZoneInvalidOffsetError;
    #[cfg(feature = "compiled_data")]
    use crate::errors::ffi::TimeZoneUnknownError;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::timezone::TimeZone, Struct)]
    pub struct TimeZone(pub icu_timezone::TimeZone);

    impl TimeZone {
        /// Creates a time zone from an offset string.
        #[diplomat::rust_link(icu::timezone::TimeZone::try_from_str, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::TimeZone::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::TimeZone::from_str, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::TimeZone::from_parts, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::try_from_str, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::try_from_utf8, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::from_str, FnInStruct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor)]
        #[diplomat::demo(default_constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn from_string(s: &DiplomatStr) -> Result<Box<TimeZone>, TimeZoneUnknownError> {
            Ok(Box::new(TimeZone::from(
                icu_timezone::TimeZone::try_from_utf8(s)?,
            )))
        }

        /// Creates a time zone for UTC (Coordinated Universal Time).
        #[diplomat::rust_link(icu::timezone::TimeZone::utc, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::zero, FnInStruct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor)]
        pub fn utc() -> Box<TimeZone> {
            Box::new(icu_timezone::TimeZone::utc().into())
        }

        /// Creates a `TimeZone` from an offset seconds and a BCP-47 string.
        ///
        /// Errors if the string is not a valid BCP-47 time zone ID, or if the offset seconds are out of range.
        #[diplomat::rust_link(icu::timezone::TimeZone::new, StructField)]
        #[diplomat::rust_link(icu::timezone::UtcOffset, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        #[diplomat::attr(auto, constructor)]
        pub fn create(
            offset_seconds: i32,
            id: &DiplomatStr,
        ) -> Result<Box<Self>, TimeZoneUnknownError> {
            Ok(Box::new(Self(icu_timezone::TimeZone::new(
                icu_timezone::UtcOffset::try_from_offset_seconds(offset_seconds)
                    .map_err(|_| TimeZoneUnknownError)?,
                icu_timezone::TimeZoneBcp47Id(
                    tinystr::TinyAsciiStr::try_from_utf8(id).map_err(|_| TimeZoneUnknownError)?,
                ),
            ))))
        }

        /// Crates a `TimeZone` from offset seconds.
        ///
        /// Errors if the offset seconds are out of range.
        #[diplomat::rust_link(icu::timezone::TimeZone::new_with_offset, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset::try_from_offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset, Struct, compact)]
        #[diplomat::rust_link(
            icu::timezone::UtcOffset::from_offset_seconds_unchecked,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(auto, named_constructor = "from_offset_seconds")]
        pub fn create_from_offset_seconds(
            offset_seconds: i32,
        ) -> Result<Box<Self>, TimeZoneInvalidOffsetError> {
            Ok(Box::new(Self(icu_timezone::TimeZone::new_with_offset(
                icu_timezone::UtcOffset::try_from_offset_seconds(offset_seconds)?,
            ))))
        }

        /// Creates a `TimeZone` from a BCP-47 string.
        ///
        /// Errors if the string is not a valid BCP-47 time zone ID.
        #[diplomat::rust_link(icu::timezone::TimeZone::new_with_bcp47_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id::from_str, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id::deref, FnInStruct, hidden)]
        #[diplomat::rust_link(
            icu::timezone::TimeZoneBcp47Id::Target,
            AssociatedTypeInStruct,
            hidden
        )]
        #[diplomat::attr(auto, named_constructor = "from_bcp47_id")]
        pub fn create_from_bcp47_id(id: &DiplomatStr) -> Result<Box<Self>, TimeZoneInvalidIdError> {
            Ok(Box::new(Self(icu_timezone::TimeZone::new_with_bcp47_id(
                icu_timezone::TimeZoneBcp47Id(
                    tinystr::TinyAsciiStr::try_from_utf8(id).map_err(|_| TimeZoneInvalidIdError)?,
                ),
            ))))
        }

        /// Gets the `offset` field from offset as eighths of an hour.
        #[diplomat::rust_link(icu::timezone::UtcOffset::offset_eighths_of_hour, FnInStruct)]
        pub fn offset_eighths_of_hour(&self) -> Option<i8> {
            self.0
                .offset()
                .map(icu_timezone::UtcOffset::offset_eighths_of_hour)
        }

        /// Returns the value of the `offset` field as offset seconds.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::offset_seconds, FnInStruct)]
        #[diplomat::rust_link(icu::timezone::UtcOffset, Struct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_seconds(&self) -> Option<i32> {
            self.0.offset().map(icu_timezone::UtcOffset::offset_seconds)
        }

        /// Returns whether the `offset` field is positive.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::is_positive, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn is_offset_positive(&self) -> Option<bool> {
            self.0.offset().map(icu_timezone::UtcOffset::is_positive)
        }

        /// Returns whether the `offset` field is zero.
        ///
        /// Returns null if the `offset` field is empty (which is not the same as zero).
        #[diplomat::rust_link(icu::timezone::UtcOffset::is_zero, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn is_offset_zero(&self) -> Option<bool> {
            self.0.offset().map(icu_timezone::UtcOffset::is_zero)
        }

        /// Returns whether the `offset` field has nonzero minutes.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::has_minutes, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_has_minutes(&self) -> Option<bool> {
            self.0.offset().map(icu_timezone::UtcOffset::has_minutes)
        }

        /// Returns whether the `offset` field has nonzero seconds.
        ///
        /// Returns null if the `offset` field is empty.
        #[diplomat::rust_link(icu::timezone::UtcOffset::has_seconds, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        pub fn offset_has_seconds(&self) -> Option<bool> {
            self.0.offset().map(icu_timezone::UtcOffset::has_seconds)
        }

        /// Writes the value of the `bcp47_id` field as a string.
        ///
        /// Returns null if the `bcp47_id` field is empty.
        #[diplomat::rust_link(icu::timezone::TimeZone::bcp47_id, StructField)]
        #[diplomat::rust_link(icu::timezone::TimeZoneBcp47Id, Struct, compact)]
        #[diplomat::attr(auto, getter)]
        pub fn bcp47_id(&self, write: &mut diplomat_runtime::DiplomatWrite) -> Option<()> {
            let _infallible = write.write_str(self.0.bcp47_id()?.0.as_str());
            Some(())
        }
    }
}

impl From<icu_timezone::TimeZone> for ffi::TimeZone {
    fn from(other: icu_timezone::TimeZone) -> Self {
        Self(other)
    }
}

impl From<ffi::TimeZone> for icu_timezone::TimeZone {
    fn from(other: ffi::TimeZone) -> Self {
        other.0
    }
}
