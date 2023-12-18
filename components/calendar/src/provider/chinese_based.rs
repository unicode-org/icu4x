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

use calendrical_calculations::rata_die::RataDie;
use core::num::NonZeroU8;
use zerovec::ule::{AsULE, ULE};

/// The struct containing compiled ChineseData
///
/// Bit structure (little endian: note that shifts go in the opposite direction!)
///
/// ```text
/// Bit:             0   1   2   3   4   5   6   7
/// Byte 0:          [  month lengths .............
/// Byte 1:         .. month lengths ] | [ leap month index ..
/// Byte 2:          ] | [   NY offset   ] | unused
/// ```
///
/// Where the New Year Offset is the offset from ISO Jan 21 of that year for Chinese New Year,
/// the month lengths are stored as 1 = 30, 0 = 29 for each month including the leap month.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ULE)]
#[repr(packed)]
pub struct PackedChineseBasedYearInfo(pub u8, pub u8, pub u8);

impl PackedChineseBasedYearInfo {
    /// The first day of the ISO year on which Chinese New Year may occur
    ///
    /// According to Reingold & Dershowitz, ch 19.6, Chinese New Year occurs on Jan 21 - Feb 21 inclusive.
    ///
    /// Chinese New Year in the year 30 AD is January 20 (30-01-20)
    pub(crate) const FIRST_NY: u8 = 20;

    pub(crate) fn new(
        month_lengths: [bool; 13],
        leap_month_idx: Option<NonZeroU8>,
        ny_offset: u8,
    ) -> Self {
        debug_assert!(
            !month_lengths[12] || leap_month_idx.is_some(),
            "Last month length should not be set for non-leap years"
        );
        debug_assert!(ny_offset < 32, "Year offset too big to store");
        debug_assert!(
            leap_month_idx.map(|l| l.get() <= 13).unwrap_or(true),
            "Leap month indices must be 1 <= i <= 13"
        );
        let mut all = 0u32; // last byte unused

        for (month, length_30) in month_lengths.iter().enumerate() {
            #[allow(clippy::indexing_slicing)]
            if *length_30 {
                all |= 1 << month as u32;
            }
        }
        let leap_month_idx = leap_month_idx.map(|x| x.get()).unwrap_or(0);
        all |= (leap_month_idx as u32) << (8 + 5);
        all |= (ny_offset as u32) << (16 + 1);
        let le = all.to_le_bytes();
        Self(le[0], le[1], le[2])
    }

    // Get the new year offset from January 21
    pub(crate) fn ny_offset(self) -> u8 {
        self.2 >> 1
    }

    pub(crate) fn ny_rd(self, related_iso: i32) -> RataDie {
        let ny_offset = self.ny_offset();
        let iso_ny = calendrical_calculations::iso::fixed_from_iso(related_iso, 1, 1);
        // -1 because `iso_ny` is itself in the year, and `FIRST_NY` is 1-indexed
        iso_ny + i64::from(Self::FIRST_NY) + i64::from(ny_offset) - 1
    }

    pub(crate) fn leap_month_idx(self) -> Option<NonZeroU8> {
        let low_bits = self.1 >> 5;
        let high_bits = (self.2 & 0b1) << 3;

        NonZeroU8::new(low_bits + high_bits)
    }

    // Whether a particular month has 30 days (month is 1-indexed)
    #[cfg(test)]
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

    pub(crate) fn days_in_year(self) -> u16 {
        if self.leap_month_idx().is_some() {
            self.last_day_of_month(13)
        } else {
            self.last_day_of_month(12)
        }
    }
}

impl AsULE for PackedChineseBasedYearInfo {
    type ULE = Self;
    fn to_unaligned(self) -> Self {
        self
    }
    fn from_unaligned(other: Self) -> Self {
        other
    }
}
