// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::str::FromStr;

#[cfg(feature = "alloc")]
use crate::provider::legacy::TimezoneVariantsOffsetsV1;
use crate::provider::{TimezonePeriods, TimezonePeriodsV1};
use crate::TimeZone;
use icu_provider::prelude::*;

use displaydoc::Display;

use super::ZoneNameTimestamp;

/// The time zone offset was invalid. Must be within ±18:00:00.
#[derive(Display, Debug, Copy, Clone, PartialEq)]
#[allow(clippy::exhaustive_structs)]
pub struct InvalidOffsetError;

/// An offset from Coordinated Universal Time (UTC).
///
/// Supports ±18:00:00.
///
/// **The primary definition of this type is in the [`icu_time`](https://docs.rs/icu_time) crate. Other ICU4X crates re-export it for convenience.**
#[derive(Copy, Clone, Debug, PartialEq, Eq, Default, PartialOrd, Ord)]
pub struct UtcOffset(i32);

impl UtcOffset {
    /// Attempt to create a [`UtcOffset`] from a seconds input.
    ///
    /// Returns [`InvalidOffsetError`] if the seconds are out of bounds.
    pub fn try_from_seconds(seconds: i32) -> Result<Self, InvalidOffsetError> {
        if seconds.unsigned_abs() > 18 * 60 * 60 {
            Err(InvalidOffsetError)
        } else {
            Ok(Self(seconds))
        }
    }

    /// Creates a [`UtcOffset`] of zero.
    pub const fn zero() -> Self {
        Self(0)
    }

    /// Parse a [`UtcOffset`] from bytes.
    ///
    /// The offset must range from UTC-12 to UTC+14.
    ///
    /// The string must be an ISO-8601 time zone designator:
    /// e.g. Z
    /// e.g. +05
    /// e.g. +0500
    /// e.g. +05:00
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::time::zone::UtcOffset;
    ///
    /// let offset0: UtcOffset = UtcOffset::try_from_str("Z").unwrap();
    /// let offset1: UtcOffset = UtcOffset::try_from_str("+05").unwrap();
    /// let offset2: UtcOffset = UtcOffset::try_from_str("+0500").unwrap();
    /// let offset3: UtcOffset = UtcOffset::try_from_str("-05:00").unwrap();
    ///
    /// let offset_err0 =
    ///     UtcOffset::try_from_str("0500").expect_err("Invalid input");
    /// let offset_err1 =
    ///     UtcOffset::try_from_str("+05000").expect_err("Invalid input");
    ///
    /// assert_eq!(offset0.to_seconds(), 0);
    /// assert_eq!(offset1.to_seconds(), 18000);
    /// assert_eq!(offset2.to_seconds(), 18000);
    /// assert_eq!(offset3.to_seconds(), -18000);
    /// ```
    #[inline]
    pub fn try_from_str(s: &str) -> Result<Self, InvalidOffsetError> {
        Self::try_from_utf8(s.as_bytes())
    }

    /// See [`Self::try_from_str`]
    pub fn try_from_utf8(mut code_units: &[u8]) -> Result<Self, InvalidOffsetError> {
        fn try_get_time_component([tens, ones]: [u8; 2]) -> Option<i32> {
            Some(((tens as char).to_digit(10)? * 10 + (ones as char).to_digit(10)?) as i32)
        }

        let offset_sign = match code_units {
            [b'+', rest @ ..] => {
                code_units = rest;
                1
            }
            [b'-', rest @ ..] => {
                code_units = rest;
                -1
            }
            // Unicode minus ("\u{2212}" == [226, 136, 146])
            [226, 136, 146, rest @ ..] => {
                code_units = rest;
                -1
            }
            [b'Z'] => return Ok(Self(0)),
            _ => return Err(InvalidOffsetError),
        };

        let hours = match code_units {
            &[h1, h2, ..] => try_get_time_component([h1, h2]),
            _ => None,
        }
        .ok_or(InvalidOffsetError)?;

        let minutes = match code_units {
            /* ±hh */
            &[_, _] => Some(0),
            /* ±hhmm, ±hh:mm */
            &[_, _, m1, m2] | &[_, _, b':', m1, m2] => {
                try_get_time_component([m1, m2]).filter(|&m| m < 60)
            }
            _ => None,
        }
        .ok_or(InvalidOffsetError)?;

        Self::try_from_seconds(offset_sign * (hours * 60 + minutes) * 60)
    }

    /// Create a [`UtcOffset`] from a seconds input without checking bounds.
    #[inline]
    pub const fn from_seconds_unchecked(seconds: i32) -> Self {
        Self(seconds)
    }

    /// Returns the raw offset value in seconds.
    pub const fn to_seconds(self) -> i32 {
        self.0
    }

    /// Whether the [`UtcOffset`] is non-negative.
    pub fn is_non_negative(self) -> bool {
        self.0 >= 0
    }

    /// Whether the [`UtcOffset`] is zero.
    pub fn is_zero(self) -> bool {
        self.0 == 0
    }

    /// Returns the hours part of if the [`UtcOffset`]
    pub fn hours_part(self) -> i32 {
        self.0 / 3600
    }

    /// Returns the minutes part of if the [`UtcOffset`].
    pub fn minutes_part(self) -> u32 {
        (self.0 % 3600 / 60).unsigned_abs()
    }

    /// Returns the seconds part of if the [`UtcOffset`].
    pub fn seconds_part(self) -> u32 {
        (self.0 % 60).unsigned_abs()
    }
}

impl FromStr for UtcOffset {
    type Err = InvalidOffsetError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from_str(s)
    }
}

#[derive(Debug)]
enum OffsetData {
    #[cfg(feature = "alloc")] // doesn't alloc, but ZeroMap are behind the alloc feature
    Old(DataPayload<TimezoneVariantsOffsetsV1>),
    New(DataPayload<TimezonePeriodsV1>),
}

#[derive(Debug)]
enum OffsetDataBorrowed<'a> {
    #[cfg(feature = "alloc")]
    Old(&'a zerovec::ZeroMap2d<'a, TimeZone, ZoneNameTimestamp, VariantOffsets>),
    New(&'a TimezonePeriods<'a>),
}

/// [`VariantOffsetsCalculator`] uses data from the [data provider] to calculate time zone offsets.
///
/// [data provider]: icu_provider
#[derive(Debug)]
#[deprecated(
    since = "2.1.0",
    note = "this API is a bad approximation of a time zone database"
)]
pub struct VariantOffsetsCalculator {
    offset_period: OffsetData,
}

/// The borrowed version of a  [`VariantOffsetsCalculator`]
#[derive(Debug)]
#[deprecated(
    since = "2.1.0",
    note = "this API is a bad approximation of a time zone database"
)]
pub struct VariantOffsetsCalculatorBorrowed<'a> {
    offset_period: OffsetDataBorrowed<'a>,
}

#[cfg(feature = "compiled_data")]
#[allow(deprecated)]
impl Default for VariantOffsetsCalculatorBorrowed<'static> {
    fn default() -> Self {
        VariantOffsetsCalculator::new()
    }
}

#[allow(deprecated)]
impl VariantOffsetsCalculator {
    /// Constructs a `VariantOffsetsCalculator` using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    #[expect(clippy::new_ret_no_self)]
    pub const fn new() -> VariantOffsetsCalculatorBorrowed<'static> {
        VariantOffsetsCalculatorBorrowed::new()
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER, Self::new)]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        use icu_provider::buf::AsDeserializingBufferProvider;
        {
            Ok(Self {
                offset_period: match DataProvider::<TimezonePeriodsV1>::load(
                    &provider.as_deserializing(),
                    Default::default(),
                ) {
                    Ok(payload) => OffsetData::New(payload.payload),
                    Err(_e) => {
                        #[cfg(feature = "alloc")]
                        {
                            OffsetData::Old(
                                DataProvider::<TimezoneVariantsOffsetsV1>::load(
                                    &provider.as_deserializing(),
                                    Default::default(),
                                )?
                                .payload,
                            )
                        }
                        #[cfg(not(feature = "alloc"))]
                        return Err(_e);
                    }
                },
            })
        }
    }

    #[doc = icu_provider::gen_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<TimezonePeriodsV1> + ?Sized),
    ) -> Result<Self, DataError> {
        let offset_period = provider.load(Default::default())?.payload;
        Ok(Self {
            offset_period: OffsetData::New(offset_period),
        })
    }

    /// Returns a borrowed version of the calculator that can be queried.
    ///
    /// This avoids a small potential indirection cost when querying.
    pub fn as_borrowed(&self) -> VariantOffsetsCalculatorBorrowed<'_> {
        VariantOffsetsCalculatorBorrowed {
            offset_period: match self.offset_period {
                OffsetData::New(ref payload) => OffsetDataBorrowed::New(payload.get()),
                #[cfg(feature = "alloc")]
                OffsetData::Old(ref payload) => OffsetDataBorrowed::Old(payload.get()),
            },
        }
    }
}

#[allow(deprecated)]
impl VariantOffsetsCalculatorBorrowed<'static> {
    /// Constructs a `VariantOffsetsCalculatorBorrowed` using compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    #[inline]
    pub const fn new() -> Self {
        Self {
            offset_period: OffsetDataBorrowed::New(
                crate::provider::Baked::SINGLETON_TIMEZONE_PERIODS_V1,
            ),
        }
    }

    /// Cheaply converts a [`VariantOffsetsCalculatorBorrowed<'static>`] into a [`VariantOffsetsCalculator`].
    ///
    /// Note: Due to branching and indirection, using [`VariantOffsetsCalculator`] might inhibit some
    /// compile-time optimizations that are possible with [`VariantOffsetsCalculatorBorrowed`].
    pub fn static_to_owned(&self) -> VariantOffsetsCalculator {
        VariantOffsetsCalculator {
            offset_period: match self.offset_period {
                OffsetDataBorrowed::New(p) => OffsetData::New(DataPayload::from_static_ref(p)),
                #[cfg(feature = "alloc")]
                OffsetDataBorrowed::Old(p) => OffsetData::Old(DataPayload::from_static_ref(p)),
            },
        }
    }
}

#[allow(deprecated)]
impl VariantOffsetsCalculatorBorrowed<'_> {
    /// Calculate zone offsets from timezone and local datetime.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::calendar::Date;
    /// use icu::locale::subtags::subtag;
    /// use icu::time::zone::UtcOffset;
    /// use icu::time::zone::VariantOffsetsCalculator;
    /// use icu::time::zone::ZoneNameTimestamp;
    /// use icu::time::Time;
    /// use icu::time::TimeZone;
    ///
    /// let zoc = VariantOffsetsCalculator::new();
    ///
    /// // America/Denver observes DST
    /// let offsets = zoc
    ///     .compute_offsets_from_time_zone_and_name_timestamp(
    ///         TimeZone(subtag!("usden")),
    ///         ZoneNameTimestamp::far_in_future(),
    ///     )
    ///     .unwrap();
    /// assert_eq!(
    ///     offsets.standard,
    ///     UtcOffset::try_from_seconds(-7 * 3600).unwrap()
    /// );
    /// assert_eq!(
    ///     offsets.daylight,
    ///     Some(UtcOffset::try_from_seconds(-6 * 3600).unwrap())
    /// );
    ///
    /// // America/Phoenix does not
    /// let offsets = zoc
    ///     .compute_offsets_from_time_zone_and_name_timestamp(
    ///         TimeZone(subtag!("usphx")),
    ///         ZoneNameTimestamp::far_in_future(),
    ///     )
    ///     .unwrap();
    /// assert_eq!(
    ///     offsets.standard,
    ///     UtcOffset::try_from_seconds(-7 * 3600).unwrap()
    /// );
    /// assert_eq!(offsets.daylight, None);
    /// ```
    pub fn compute_offsets_from_time_zone_and_name_timestamp(
        &self,
        time_zone_id: TimeZone,
        timestamp: ZoneNameTimestamp,
    ) -> Option<VariantOffsets> {
        match self.offset_period {
            OffsetDataBorrowed::New(p) => p.get(time_zone_id, timestamp).map(|(os, _)| os),
            #[cfg(feature = "alloc")]
            OffsetDataBorrowed::Old(p) => {
                use zerovec::ule::AsULE;
                let mut offsets = None;
                for (bytes, id) in p.get0(&time_zone_id)?.iter1_copied().rev() {
                    if timestamp >= ZoneNameTimestamp::from_unaligned(*bytes) {
                        offsets = Some(id);
                        break;
                    }
                }
                Some(offsets?)
            }
        }
    }
}

#[deprecated(
    since = "2.1.0",
    note = "this API is a bad approximation of a time zone database"
)]
pub use crate::provider::VariantOffsets;

#[test]
#[allow(deprecated)]
pub fn test_legacy_offsets_data() {
    use crate::ZonedDateTime;
    use icu_locale_core::subtags::subtag;
    use icu_provider_blob::BlobDataProvider;

    let c = VariantOffsetsCalculator::try_new_with_buffer_provider(
        &BlobDataProvider::try_new_from_static_blob(
            // icu4x-datagen --markers TimezoneVariantsOffsetsV1 --format blob
            include_bytes!("../../tests/data/offset_periods_old.blob"),
        )
        .unwrap(),
    )
    .unwrap();

    let tz = TimeZone(subtag!("aqcas"));

    for timestamp in [
        "1970-01-01 00:00Z",
        "2009-10-17 18:00Z",
        "2010-03-04 15:00Z",
        "2011-10-27 18:00Z",
        "2012-02-21 17:00Z",
        "2016-10-21 16:00Z",
        "2018-03-10 17:00Z",
        "2018-10-06 20:00Z",
        "2019-03-16 16:00Z",
        "2019-10-03 19:00Z",
        "2020-03-07 16:00Z",
        "2021-03-13 13:00Z",
        "2022-03-12 13:00Z",
        "2023-03-08 16:00Z",
    ] {
        let t = ZoneNameTimestamp::from_zoned_date_time_iso(
            ZonedDateTime::try_offset_only_from_str(timestamp, icu_calendar::Iso).unwrap(),
        );

        assert_eq!(
            c.as_borrowed()
                .compute_offsets_from_time_zone_and_name_timestamp(tz, t),
            VariantOffsetsCalculator::new()
                .compute_offsets_from_time_zone_and_name_timestamp(tz, t),
            "{timestamp:?}",
        );
    }
}
