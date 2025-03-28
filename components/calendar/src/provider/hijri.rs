// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Data provider struct definitions for chinese-based calendars.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use core::fmt;
use icu_provider::prelude::*;
use zerovec::ule::{AsULE, ULE};
use zerovec::ZeroVec;

icu_provider::data_marker!(
    /// Precomputed data for the Hijri obsevational calendar
    CalendarHijriSimulatedMeccaV1,
    "calendar/hijri/simulated/mecca/v1",
    HijriCache<'static>,
    is_singleton = true,
);

icu_provider::data_marker!(
    /// Precomputed data for the Hijri Umm-Al-Qura calendar
    CalendarHijriUmmalquraV1,
    "calendar/hijri/ummalqura/v1",
    HijriCache<'static>,
    is_singleton = true,
);

/// Cached/precompiled data for a certain range of years for a chinese-based
/// calendar. Avoids the need to perform lunar calendar arithmetic for most calendrical
/// operations.
#[derive(Debug, PartialEq, Clone, Default, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_calendar::provider::hijri))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct HijriCache<'data> {
    /// The extended year corresponding to the first data entry for this year
    pub first_extended_year: i32,
    /// A list of precomputed data for each year beginning with first_extended_year
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub data: ZeroVec<'data, PackedHijriYearInfo>,
}

icu_provider::data_struct!(
    HijriCache<'_>,
    #[cfg(feature = "datagen")]
);

/// The struct containing compiled Hijri YearInfo
///
/// Bit structure (little endian: note that shifts go in the opposite direction!)
///
/// ```text
/// Bit:             0   1   2   3   4   5   6   7
/// Byte 0:          [  month lengths .............
/// Byte 1:         .. months    ] | [ ny offset    ]
/// ```
///
/// Where the New Year Offset is a signed offset from `epoch + MEAN_SYNODIC_MONTH * year * 12` for the given
/// calendar. This number does not appear to be less than 2, however we use all remaining bits for it in case of drift
/// in the math.
/// The month lengths are stored as 1 = 30, 0 = 29 for each month including the leap month.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, ULE)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_calendar::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[repr(C, packed)]
pub struct PackedHijriYearInfo(pub u8, pub u8);

impl fmt::Debug for PackedHijriYearInfo {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.debug_struct("PackedHijriYearInfo")
            .field("ny_offset", &self.ny_offset())
            .field("month_lengths", &self.month_lengths())
            .finish()
    }
}

impl PackedHijriYearInfo {
    pub(crate) fn new(month_lengths: [bool; 12], ny_offset: i64) -> Self {
        debug_assert!(
            -8 < ny_offset && ny_offset < 8,
            "Year offset too big to store"
        );
        let ny_offset = ny_offset as i8;

        let mut all = 0u16; // last byte unused

        for (month, length_30) in month_lengths.iter().enumerate() {
            if *length_30 {
                all |= 1 << month as u16;
            }
        }

        if ny_offset < 0 {
            all |= 1 << 12;
        }
        all |= u16::from(ny_offset.unsigned_abs()) << 13;
        let le = all.to_le_bytes();
        Self(le[0], le[1])
    }

    fn month_lengths(self) -> [u8; 12] {
        let months: [u8; 12] = core::array::from_fn(|i| 1 + i as u8);
        months.map(|x| if self.month_has_30_days(x) { 30 } else { 29 })
    }

    // Get the new year offset from the mean synodic new year
    pub(crate) fn ny_offset(self) -> i64 {
        let masked = (self.1 >> 5) as i8;
        (if (self.1 & 0b10000) != 0 {
            -masked
        } else {
            masked
        }) as i64
    }

    // Whether a particular month has 30 days (month is 1-indexed)
    pub(crate) fn month_has_30_days(self, month: u8) -> bool {
        let months = u16::from_le_bytes([self.0, self.1]);
        months & (1 << (month - 1) as u16) != 0
    }

    // Which day of year is the last day of a month (month is 1-indexed)
    pub(crate) fn last_day_of_month(self, month: u8) -> u16 {
        let months = u16::from_le_bytes([self.0, self.1]);
        // month is 1-indexed, so `29 * month` includes the current month
        let mut prev_month_lengths = 29 * month as u16;
        // month is 1-indexed, so `1 << month` is a mask with all zeroes except
        // for a 1 at the bit index at the next month. Subtracting 1 from it gets us
        // a bitmask for all months up to now
        let long_month_bits = months & ((1 << month as u16) - 1);
        prev_month_lengths += long_month_bits.count_ones().try_into().unwrap_or(0);
        prev_month_lengths
    }
}

impl AsULE for PackedHijriYearInfo {
    type ULE = Self;
    fn to_unaligned(self) -> Self {
        self
    }
    fn from_unaligned(other: Self) -> Self {
        other
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_roundtrip(month_lengths: [bool; 12], ny_offset: i64) {
        let packed = PackedHijriYearInfo::new(month_lengths, ny_offset);
        for i in 0..12 {
            assert_eq!(packed.month_has_30_days(i + 1), month_lengths[i as usize], "Month lengths must match for testcase {month_lengths:?} / {ny_offset}, with packed repr: {packed:?}");
        }
        assert_eq!(packed.ny_offset(), ny_offset, "Month lengths must match for testcase {month_lengths:?} / {ny_offset}, with packed repr: {packed:?}");
    }
    const ALL_FALSE: [bool; 12] = [false; 12];
    const ALL_TRUE: [bool; 12] = [true; 12];
    const MIXED1: [bool; 12] = [
        true, false, true, false, true, false, true, false, true, false, true, false,
    ];
    const MIXED2: [bool; 12] = [
        false, false, true, true, true, false, true, false, false, false, true, true,
    ];
    #[test]
    fn test_hijri_packed_roundtrip() {
        single_roundtrip(ALL_FALSE, 0);
        single_roundtrip(ALL_TRUE, 0);
        single_roundtrip(MIXED1, 0);
        single_roundtrip(MIXED2, 0);

        single_roundtrip(MIXED1, -7);
        single_roundtrip(MIXED2, 7);
        single_roundtrip(MIXED2, 4);
        single_roundtrip(MIXED2, 1);
        single_roundtrip(MIXED2, -1);
        single_roundtrip(MIXED2, -4);
    }
}
