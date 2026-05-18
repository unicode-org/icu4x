// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs and markers for day periods rules.

use icu_provider::prelude::*;

/// Day period rules representing the active period for each hour of the day.
///
/// Lookup assumes that at hour 0, the active period is the *last* present period in the
/// sequence of present periods (sorted by enum value). This is because night periods
/// often spill over past midnight (e.g., from 21:00 to 06:00), meaning hour 0 is
/// typically covered by the last chronological period, which is usually a night period.
/// Transitions move to the next present period in the sequence, wrapping around.
#[derive(Debug, PartialEq, Clone, Copy, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct DayPeriodRules {
    /// A bitmask of present day periods. Bit `i` is set if the period with
    /// enum value `i` is present.
    pub presence: u8,
    /// A 24-bit map (packed into 3 bytes) where bit `h` is set if a transition
    /// to the next present period occurs at hour `h`.
    pub transitions: [u8; 3],
}

icu_provider::data_struct!(
    DayPeriodRules,
    #[cfg(feature = "datagen")]
);

icu_provider::data_marker!(
    /// `DayPeriodRulesV1` marker
    DayPeriodRulesV1,
    DayPeriodRules,
);

/// Day periods.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::day_periods))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(missing_docs, reason = "Trivial representation of CLDR day periods")]
pub enum DayPeriod {
    Morning1 = 0,
    Morning2 = 1,
    Afternoon1 = 2,
    Afternoon2 = 3,
    Evening1 = 4,
    Evening2 = 5,
    Night1 = 6,
    Night2 = 7,
}

impl DayPeriod {
    /// Parses a CLDR day period name into a `DayPeriod`.
    #[cfg(feature = "datagen")]
    pub fn from_cldr_name(name: &str) -> Option<Self> {
        match name {
            "morning1" => Some(DayPeriod::Morning1),
            "morning2" => Some(DayPeriod::Morning2),
            "afternoon1" => Some(DayPeriod::Afternoon1),
            "afternoon2" => Some(DayPeriod::Afternoon2),
            "evening1" => Some(DayPeriod::Evening1),
            "evening2" => Some(DayPeriod::Evening2),
            "night1" => Some(DayPeriod::Night1),
            "night2" => Some(DayPeriod::Night2),
            _ => None,
        }
    }

    /// Converts a u8 index to a `DayPeriod` enum.
    pub(crate) fn from_u8(v: u8) -> Option<Self> {
        match v {
            0 => Some(DayPeriod::Morning1),
            1 => Some(DayPeriod::Morning2),
            2 => Some(DayPeriod::Afternoon1),
            3 => Some(DayPeriod::Afternoon2),
            4 => Some(DayPeriod::Evening1),
            5 => Some(DayPeriod::Evening2),
            6 => Some(DayPeriod::Night1),
            7 => Some(DayPeriod::Night2),
            _ => None,
        }
    }
}

impl DayPeriodRules {
    /// Looks up the day period for a given hour (0-23).
    pub fn lookup(&self, hour: u8) -> DayPeriod {
        debug_assert!(hour < 24, "hour must be in 0..24");

        // GIGO: If presence is 0 (bad data), we return Morning1 as a safe fallback.
        if self.presence == 0 {
            debug_assert!(false, "presence must not be zero");
            return DayPeriod::Morning1;
        }

        let count = self.presence.count_ones() as usize;

        // Combine transitions bytes into a u32 for easier bit manipulation.
        let transitions_u32 = u32::from_le_bytes([
            self.transitions[0],
            self.transitions[1],
            self.transitions[2],
            0,
        ]);

        // Shifting left by `31 - hour` moves bits `0..=hour` to the top of the `u32`,
        // shifting out all later transitions. `count_ones()` then counts exactly the
        // transitions that occurred up to `hour`.
        let transitions = (transitions_u32 << (31 - hour)).count_ones() as usize;

        // Assume first period (at hour 0) is the last present period in the sorted sequence.
        // Adding `transitions` moves us forward in the sequence.
        // We subtract 1 and modulo `count` to start from the last period (index count - 1)
        // and wrap around correctly.
        // target_period is the target period's index amongst the present periods in sorted order.
        let target_period = (transitions + count - 1) % count;

        // Find the target_period-th set bit in presence (0-indexed).
        // Number of period bits found so far
        let mut found_count = 0;
        // The bit index we are currently inspecting
        let mut i = 0;
        let period_idx = loop {
            // Is period at bit index `i` present?
            if (self.presence & (1 << i)) != 0 {
                // Is this the target_periodth index?
                if found_count == target_period {
                    break i;
                }
                // Found a period bit
                found_count += 1;
            }
            i += 1;
            if i >= 8 {
                // Unreachable: `target_period` is % count, which caps it to `count`,
                // the number of set bits in `presence` (minus one), which must be at most 7
                // `found_count` only increases when a bit is found, so it will
                // iterate from 0 to `count - 1`, so the above loop is guaranteed to eventually
                // find target_period
                debug_assert!(false, "target_period >= presence.count_ones()");
                break 0;
            }
        };

        if let Some(period) = DayPeriod::from_u8(period_idx) {
            period
        } else {
            // Unreachable since `i` should not go above 8 above, and `period_idx` is
            // assigned from `i`
            debug_assert!(false, "Unreachable day period index: {}", period_idx);
            DayPeriod::Morning1 // Fallback
        }
    }
}

#[cfg(feature = "datagen")]
impl DayPeriodRules {
    /// Computes `DayPeriodRules` from a set of periods/rules.
    ///
    /// Entries is a map from `(start_hour, end_hour)` tuple ranges to `DayPeriod`.
    /// Returns `None` if entries is empty, or if there are overlaps or gaps in 24-hour coverage.
    #[allow(
        clippy::indexing_slicing,
        clippy::expect_used,
        reason = "Datagen is allowed to panic"
    )]
    pub fn from_periods(
        entries: &alloc::collections::BTreeMap<(u8, u8), DayPeriod>,
    ) -> Result<Self, &'static str> {
        if entries.is_empty() {
            return Err("empty");
        }

        let mut presence = 0u8;
        for &period in entries.values() {
            presence |= 1 << (period as u8);
        }

        let mut hour_periods = [None; 24];
        for (&(start, end), &period) in entries {
            let mut h = start;
            loop {
                if hour_periods[h as usize].is_some() {
                    return Err("overlapping period");
                }
                hour_periods[h as usize] = Some(period as u8);
                h = (h + 1) % 24;
                if h == end || (h == 0 && end == 24) {
                    break;
                }
            }
        }

        for p in &hour_periods {
            if p.is_none() {
                return Err("gap between periods");
            }
        }

        // TODO: change this to use array_try_map
        // <https://github.com/rust-lang/rust/issues/79711>
        #[allow(clippy::unwrap_used)] // just checked
        let hour_periods = hour_periods.map(|p| p.unwrap());

        let mut current_period = entries
            .values()
            .map(|&p| p as u8)
            .max()
            .expect("Must be at least one entry, checked above");
        let mut transitions_u32 = 0u32;

        for (h, &actual_period) in hour_periods.iter().enumerate() {
            let expected_period = current_period;

            if actual_period != expected_period {
                transitions_u32 |= 1 << h;
                let mut next = current_period + 1;
                current_period = loop {
                    next %= 8;
                    if (presence & (1 << next)) != 0 {
                        break next;
                    }
                    next += 1;
                };
                if actual_period != current_period {
                    current_period = actual_period;
                }
            }
        }

        let bytes = transitions_u32.to_le_bytes();
        let transitions = [bytes[0], bytes[1], bytes[2]];

        Ok(DayPeriodRules {
            presence,
            transitions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn test_rules(periods: &[((u8, u8), DayPeriod)]) {
        let rules = DayPeriodRules::from_periods(&periods.iter().cloned().collect()).unwrap();

        for &((start, end), period) in periods {
            for hour in start..end {
                assert_eq!(rules.lookup(hour % 24), period, "{hour}");
            }
        }
    }

    #[test]
    fn test_roundtrip() {
        test_rules(&[
            ((6, 12), DayPeriod::Morning1),
            ((12, 18), DayPeriod::Afternoon1),
            ((18, 6), DayPeriod::Night1),
        ]);

        test_rules(&[
            ((6, 12), DayPeriod::Morning1),
            ((12, 18), DayPeriod::Afternoon1),
            ((18, 21), DayPeriod::Evening1),
            ((21, 6), DayPeriod::Night1),
        ]);

        test_rules(&[((12, 12), DayPeriod::Morning1)]);

        test_rules(&[
            ((0, 12), DayPeriod::Morning1),
            ((12, 18), DayPeriod::Afternoon1),
            ((18, 21), DayPeriod::Evening1),
            ((21, 24), DayPeriod::Night1),
        ]);

        test_rules(&[
            ((0, 5), DayPeriod::Night1),
            ((5, 8), DayPeriod::Morning1),
            ((8, 12), DayPeriod::Morning2),
            ((12, 13), DayPeriod::Afternoon1),
            ((13, 19), DayPeriod::Afternoon2),
            ((19, 24), DayPeriod::Evening1),
        ]);

        test_rules(&[
            ((0, 5), DayPeriod::Night1),
            ((5, 10), DayPeriod::Morning1),
            ((10, 12), DayPeriod::Morning2),
            ((12, 13), DayPeriod::Afternoon1),
            ((13, 19), DayPeriod::Afternoon2),
            ((19, 24), DayPeriod::Evening1),
        ]);
    }

    #[test]
    fn test_empty() {
        assert!(DayPeriodRules::from_periods(&Default::default()).is_err());
    }

    #[test]
    fn test_overlap() {
        assert!(DayPeriodRules::from_periods(
            &[
                ((0, 12), DayPeriod::Morning1),
                ((12, 19), DayPeriod::Afternoon1),
                // Overlaps
                ((18, 21), DayPeriod::Evening1),
                ((21, 24), DayPeriod::Night1),
            ]
            .into_iter()
            .collect(),
        )
        .is_err());
    }

    #[test]
    fn test_gap() {
        assert!(DayPeriodRules::from_periods(
            &[((0, 12), DayPeriod::Morning1),].into_iter().collect(),
        )
        .is_err());
    }
}
