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
use zerovec::ule::AsULE;

/// The struct containing compiled Hijri YearInfo
///
/// * `start_day` has to be within 5 days of the start of the year of the [`TabularAlgorithm`].
/// * `month_lengths[n - 1]` has either 6 or 7 long months.
///
/// Bit structure
///
/// ```text
/// Bit:              F.........C  B.............0
/// Value:           [ start day ][ month lengths ]
/// ```
///
/// The start day is encoded as a signed offset from `Self::mean_tabular_start_day`. This number does not
/// appear to be less than 2, however we use all remaining bits for it in case of drift in the math.
/// The month lengths are stored as 1 = 30, 0 = 29 for each month including the leap month.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_calendar::provider))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PackedHijriYearInfo(pub u16);

impl PackedHijriYearInfo {
    pub(crate) const fn try_new(
        extended_year: i32,
        month_lengths: [bool; 12],
        start_day: RataDie,
    ) -> Option<Self> {
        let start_offset = start_day.since(Self::mean_tabular_start_day(extended_year));

        if !(-8 < start_offset && start_offset < 8
            || calendrical_calculations::islamic::WELL_BEHAVED_ASTRONOMICAL_RANGE
                .start
                .to_i64_date()
                > start_day.to_i64_date()
            || calendrical_calculations::islamic::WELL_BEHAVED_ASTRONOMICAL_RANGE
                .end
                .to_i64_date()
                < start_day.to_i64_date())
        {
            return None;
        }
        let start_offset = start_offset as i8 & 0b1000_0111u8 as i8;

        let mut all = 0u16;

        let mut num_days = 29 * 12;

        let mut i = 0;
        while i < 12 {
            #[expect(clippy::indexing_slicing)]
            if month_lengths[i] {
                all |= 1 << i;
                num_days += 1;
            }
            i += 1;
        }

        if !matches!(num_days, 354 | 355) {
            return None;
        }

        if start_offset < 0 {
            all |= 1 << 12;
        }
        all |= (start_offset.unsigned_abs() as u16) << 13;
        Some(Self(all))
    }

    pub(crate) const fn new_unchecked(
        extended_year: i32,
        month_lengths: [bool; 12],
        start_day: RataDie,
    ) -> Self {
        let start_offset = start_day.since(Self::mean_tabular_start_day(extended_year));

        let start_offset = start_offset as i8 & 0b1000_0111u8 as i8;

        let mut all = 0u16;

        let mut i = 0;
        while i < 12 {
            #[expect(clippy::indexing_slicing)]
            if month_lengths[i] {
                all |= 1 << i;
            }
            i += 1;
        }

        if start_offset < 0 {
            all |= 1 << 12;
        }
        all |= (start_offset.unsigned_abs() as u16) << 13;
        Self(all)
    }

    pub(crate) fn start_day(self, extended_year: i32) -> RataDie {
        let start_offset = if (self.0 & 0b1_0000_0000_0000) != 0 {
            -((self.0 >> 13) as i64)
        } else {
            (self.0 >> 13) as i64
        };
        Self::mean_tabular_start_day(extended_year) + start_offset
    }

    pub(crate) fn month_has_30_days(self, month: u8) -> bool {
        self.0 & (1 << (month - 1) as u16) != 0
    }

    pub(crate) fn is_leap(self) -> bool {
        (self.0 & ((1 << 12) - 1)).count_ones() == 7
    }

    pub(crate) fn last_day_of_month(self, month: u8) -> u16 {
        // month is 1-indexed, so `29 * month` includes the current month
        let mut prev_month_lengths = 29 * month as u16;
        // month is 1-indexed, so `1 << month` is a mask with all zeroes except
        // for a 1 at the bit index at the next month. Subtracting 1 from it gets us
        // a bitmask for all months up to now
        let long_month_bits = self.0 & ((1 << month as u16) - 1);
        prev_month_lengths += long_month_bits.count_ones().try_into().unwrap_or(0);
        prev_month_lengths
    }

    const fn mean_tabular_start_day(extended_year: i32) -> RataDie {
        // -1 because the epoch is new year of year 1
        calendrical_calculations::islamic::ISLAMIC_EPOCH_FRIDAY
            .add((extended_year as i64 - 1) * (354 * 30 + 11) / 30)
    }
}

impl AsULE for PackedHijriYearInfo {
    type ULE = <u16 as AsULE>::ULE;
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        Self(<u16 as AsULE>::from_unaligned(unaligned))
    }
    fn to_unaligned(self) -> Self::ULE {
        <u16 as AsULE>::to_unaligned(self.0)
    }
}

#[test]
fn test_hijri_packed_roundtrip() {
    fn single_roundtrip(month_lengths: [bool; 12], start_day: RataDie) -> Option<()> {
        let packed = PackedHijriYearInfo::try_new(1600, month_lengths, start_day)?;
        for i in 0..12 {
            assert_eq!(packed.month_has_30_days(i + 1), month_lengths[i as usize]);
        }
        assert_eq!(packed.start_day(1600), start_day);
        Some(())
    }

    let l = true;
    let s = false;
    let all_short = [s; 12];
    let all_long = [l; 12];
    let mixed1 = [l, s, l, s, l, s, l, s, l, s, l, s];
    let mixed2 = [s, s, l, l, l, s, l, s, s, s, l, l];

    let start_1600 = PackedHijriYearInfo::mean_tabular_start_day(1600);
    assert_eq!(single_roundtrip(all_short, start_1600), None);
    assert_eq!(single_roundtrip(all_long, start_1600), None);
    single_roundtrip(mixed1, start_1600).unwrap();
    single_roundtrip(mixed2, start_1600).unwrap();

    single_roundtrip(mixed1, start_1600 - 7).unwrap();
    single_roundtrip(mixed2, start_1600 + 7).unwrap();
    single_roundtrip(mixed2, start_1600 + 4).unwrap();
    single_roundtrip(mixed2, start_1600 + 1).unwrap();
    single_roundtrip(mixed2, start_1600 - 1).unwrap();
    single_roundtrip(mixed2, start_1600 - 4).unwrap();
}
