// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[cfg(debug_assertions)]
use calendrical_calculations::chinese_based::WELL_BEHAVED_ASTRONOMICAL_RANGE;
use calendrical_calculations::rata_die::RataDie;

/// The struct containing compiled ChineseData
///
/// Bit structure (little endian: note that shifts go in the opposite direction!)
///
/// ```text
/// Bit:             0   1   2   3   4   5   6   7
/// Byte 0:          [  month lengths .............
/// Byte 1:         .. month lengths ] | [ leap month index ..
/// Byte 2:          ] | [   NY offset       ] | unused
/// ```
///
/// Where the New Year Offset is the offset from ISO Jan 19 of that year for Chinese New Year,
/// the month lengths are stored as 1 = 30, 0 = 29 for each month including the leap month.
/// The largest possible offset is 33, which requires 6 bits of storage.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct PackedChineseBasedYearInfo(u8, u8, u8);

impl PackedChineseBasedYearInfo {
    /// The first day on which Chinese New Year may occur
    ///
    /// According to Reingold & Dershowitz, ch 19.6, Chinese New Year occurs on Jan 21 - Feb 21 inclusive.
    ///
    /// Our simple approximation sometimes returns Feb 22.
    ///
    /// We allow it to occur as early as January 19 which is the earliest the second new moon
    /// could occur after the Winter Solstice if the solstice is pinned to December 20.
    const fn earliest_ny(related_iso: i32) -> RataDie {
        calendrical_calculations::gregorian::fixed_from_gregorian(related_iso, 1, 19)
    }

    /// It clamps some values to avoid debug assertions on calendrical invariants.
    pub(crate) const fn new(
        related_iso: i32,
        month_lengths: [bool; 13],
        leap_month: Option<u8>,
        new_year: RataDie,
    ) -> Self {
        // These assertions are API correctness assertions and even bad calendar arithmetic
        // should not produce this
        if let Some(l) = leap_month {
            debug_assert!(2 <= l && l <= 13, "Leap month indices must be 2 <= i <= 13");
        } else {
            debug_assert!(
                !month_lengths[12],
                "Last month length should not be set for non-leap years"
            )
        }

        let ny_offset = new_year.since(Self::earliest_ny(related_iso));

        #[cfg(debug_assertions)]
        let out_of_valid_astronomical_range = WELL_BEHAVED_ASTRONOMICAL_RANGE.start.to_i64_date()
            > new_year.to_i64_date()
            || new_year.to_i64_date() > WELL_BEHAVED_ASTRONOMICAL_RANGE.end.to_i64_date();

        // Assert the offset is in range, but allow it to be out of
        // range when out_of_valid_astronomical_range=true
        #[cfg(debug_assertions)]
        debug_assert!(
            ny_offset >= 0 || out_of_valid_astronomical_range,
            "Year offset too small to store"
        );
        // The maximum new-year's offset we have found is 34
        #[cfg(debug_assertions)]
        debug_assert!(
            ny_offset < 35 || out_of_valid_astronomical_range,
            "Year offset too big to store"
        );

        // Just clamp to something we can represent when things get of range.
        //
        // This will typically happen when out_of_valid_astronomical_range
        // is true.
        //
        // We can store up to 6 bytes for ny_offset, even if our
        // maximum asserted value is otherwise 33.
        let ny_offset = ny_offset & (0x40 - 1);

        let mut all = 0u32; // last byte unused

        let mut month = 0;
        while month < month_lengths.len() {
            #[allow(clippy::indexing_slicing)] // const iteration
            if month_lengths[month] {
                all |= 1 << month as u32;
            }
            month += 1;
        }
        let leap_month_idx = if let Some(leap_month_idx) = leap_month {
            leap_month_idx
        } else {
            0
        };
        all |= (leap_month_idx as u32) << (8 + 5);
        all |= (ny_offset as u32) << (16 + 1);
        let le = all.to_le_bytes();
        Self(le[0], le[1], le[2])
    }

    pub(crate) fn new_year(self, related_iso: i32) -> RataDie {
        Self::earliest_ny(related_iso) + (self.2 as i64 >> 1)
    }

    pub(crate) fn leap_month(self) -> Option<u8> {
        let bits = (self.1 >> 5) + ((self.2 & 0b1) << 3);

        (bits != 0).then_some(bits)
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

#[test]
fn test_roundtrip_packed() {
    fn packed_roundtrip_single(
        month_lengths: [bool; 13],
        leap_month_idx: Option<u8>,
        ny_offset: i64,
    ) {
        let ny = calendrical_calculations::gregorian::fixed_from_gregorian(1000, 1, 1) + ny_offset;
        let packed = PackedChineseBasedYearInfo::new(1000, month_lengths, leap_month_idx, ny);

        assert_eq!(
            ny,
            packed.new_year(1000),
            "Roundtrip with {month_lengths:?}, {leap_month_idx:?}, {ny_offset}"
        );
        assert_eq!(
            leap_month_idx,
            packed.leap_month(),
            "Roundtrip with {month_lengths:?}, {leap_month_idx:?}, {ny_offset}"
        );
        assert_eq!(
            month_lengths,
            core::array::from_fn(|i| packed.month_has_30_days(i as u8 + 1)),
            "Roundtrip with {month_lengths:?}, {leap_month_idx:?}, {ny_offset}"
        );
    }

    const SHORT: [bool; 13] = [false; 13];
    const LONG: [bool; 13] = [true; 13];
    const ALTERNATING1: [bool; 13] = [
        false, true, false, true, false, true, false, true, false, true, false, true, false,
    ];
    const ALTERNATING2: [bool; 13] = [
        true, false, true, false, true, false, true, false, true, false, true, false, false,
    ];
    const RANDOM1: [bool; 13] = [
        true, true, false, false, true, true, false, true, true, true, true, false, false,
    ];
    const RANDOM2: [bool; 13] = [
        false, true, true, true, true, false, true, true, true, false, false, true, false,
    ];
    packed_roundtrip_single(SHORT, None, 18 + 5);
    packed_roundtrip_single(SHORT, None, 18 + 10);
    packed_roundtrip_single(SHORT, Some(11), 18 + 15);
    packed_roundtrip_single(LONG, Some(12), 18 + 15);
    packed_roundtrip_single(ALTERNATING1, None, 18 + 2);
    packed_roundtrip_single(ALTERNATING1, Some(3), 18 + 5);
    packed_roundtrip_single(ALTERNATING2, None, 18 + 9);
    packed_roundtrip_single(ALTERNATING2, Some(7), 18 + 26);
    packed_roundtrip_single(RANDOM1, None, 18 + 29);
    packed_roundtrip_single(RANDOM1, Some(12), 18 + 29);
    packed_roundtrip_single(RANDOM1, Some(2), 18 + 21);
    packed_roundtrip_single(RANDOM2, None, 18 + 25);
    packed_roundtrip_single(RANDOM2, Some(2), 18 + 19);
    packed_roundtrip_single(RANDOM2, Some(5), 18 + 2);
    packed_roundtrip_single(RANDOM2, Some(12), 18 + 5);
}
