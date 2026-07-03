// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structs and markers for day periods rules.

#[cfg(feature = "datagen")]
use alloc::string::String;
use icu_time::Hour;

/// Day period rules mapping each hour of the day to an index.
///
/// Lookup assumes that at hour 0, the active period is the *last* index. This is
/// because night periods often spill over past midnight (e.g., from 21:00
/// to 06:00), meaning hour 0 is typically covered by the last index.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct DayPeriodRules {
    /// A 24-bit map (packed into 3 bytes) where bit `h` is set if a transition
    /// to the next index occurs at hour `h`.
    pub transitions: [u8; 3],
}

impl DayPeriodRules {
    /// Looks up the index for a given hour (0-23).
    pub fn name_offset(&self, hour: Hour) -> usize {
        // Combine transitions bytes into a u32 for easier bit manipulation.
        let transitions = u32::from_le_bytes([
            self.transitions[0],
            self.transitions[1],
            self.transitions[2],
            0,
        ]);

        // Shifting left by `31 - hour` moves bits `0..=hour` to the top of the `u32`,
        // shifting out all later transitions. `count_ones()` then counts exactly the
        // transitions that occurred up to, and including, `hour`.
        let num_transitions_up_to_hour = (transitions << (31 - hour.number())).count_ones();

        // This is branch-free as it compiles to a conditional move.
        let periods_before = if num_transitions_up_to_hour == 0 {
            // If we're before the first transition, we're in the last period
            transitions.count_ones() - 1
        } else {
            num_transitions_up_to_hour - 1
        };

        periods_before as usize
    }

    /// Decodes rules from a string
    pub fn decode_from_str(s: &str) -> Option<Self> {
        let bytes = <&[u8; 4]>::try_from(s.as_bytes()).ok()?;
        let raw = (bytes[0] as u32) << 17
            | (bytes[1] as u32) << 10
            | (bytes[2] as u32) << 3
            | (bytes[3] as u32);

        let [b, c, d, _] = u32::to_le_bytes(raw);

        Some(Self {
            transitions: [b, c, d],
        })
    }

    /// Encodes rules into a string
    #[cfg(feature = "datagen")]
    pub fn encode_to_string(&self) -> String {
        let [a, b, c] = self.transitions;
        let raw = u32::from_le_bytes([a, b, c, 0]);

        let bytes = [
            (raw >> 17) as u8 & 0b0111_1111,
            (raw >> 10) as u8 & 0b0111_1111,
            (raw >> 3) as u8 & 0b0111_1111,
            raw as u8 & 0b0000_0111,
        ];

        #[allow(clippy::unwrap_used)] // all ascii
        String::from(core::str::from_utf8(&bytes).unwrap())
    }
}

#[cfg(feature = "datagen")]
impl DayPeriodRules {
    /// Computes `DayPeriodRules` from a set of periods and names.
    ///
    /// Entries is a map from `(start_hour, end_hour)` to a name. This method returns a
    /// `DayPeriodName` and an iterator of names. The `name_offset` function of the
    /// `DayPeriodName` will return the index of the hour's name in the iterator.
    ///
    /// Returns `None` if entries is empty, and errors if there are overlaps or gaps in
    /// 24-hour coverage.
    pub fn from_periods(
        entries: alloc::collections::BTreeMap<(u8, u8), &str>,
    ) -> Result<(Self, impl Iterator<Item = &str>), &'static str> {
        if entries.is_empty() {
            return Err("empty");
        }

        let mut hour_periods = [None; 24];
        for (id, (&(start, end), _)) in entries.iter().enumerate() {
            let mut h = start % 24;
            #[allow(clippy::indexing_slicing)] // h in 0..=24
            loop {
                if hour_periods[h as usize].is_some() {
                    return Err("overlapping period");
                }
                hour_periods[h as usize] = Some(id);
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

        let mut current_period = entries.len() - 1;
        let mut transitions_u32 = 0u32;

        for (h, &actual_period) in hour_periods.iter().enumerate() {
            let expected_period = current_period;

            if actual_period != expected_period {
                transitions_u32 |= 1 << h;
                let mut next = current_period + 1;
                current_period = loop {
                    next %= 8;
                    if next < entries.len() {
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

        Ok((DayPeriodRules { transitions }, entries.into_values()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[track_caller]
    fn test_rules(periods: &[((u8, u8), &'static str)]) {
        let (rules, names) =
            DayPeriodRules::from_periods(periods.iter().cloned().collect()).unwrap();

        let names = names.collect::<Vec<_>>();

        assert_eq!(
            DayPeriodRules::decode_from_str(&rules.encode_to_string()),
            Some(rules)
        );

        for &((start, end), name) in periods {
            for hour in start..end {
                assert_eq!(
                    names[rules.name_offset((hour % 24).try_into().unwrap())],
                    name,
                    "{hour}"
                );
            }
        }
    }

    #[test]
    fn test_roundtrip() {
        test_rules(&[((6, 12), "foo"), ((12, 18), "bar"), ((18, 6), "baz")]);

        test_rules(&[
            ((6, 12), "foo"),
            ((12, 18), "bar"),
            ((18, 21), "baz"),
            ((21, 6), "qu"),
        ]);

        test_rules(&[((12, 12), "foo")]);

        test_rules(&[
            ((0, 12), "foo"),
            ((12, 18), "bar"),
            ((18, 21), "baz"),
            ((21, 24), "qux"),
        ]);

        test_rules(&[
            ((0, 5), "foo"),
            ((5, 8), "bar"),
            ((8, 12), "batz"),
            ((12, 13), "qux"),
            ((13, 19), "quux"),
            ((19, 24), "quuux"),
        ]);

        test_rules(&[
            ((0, 5), "foo"),
            ((5, 10), "bar"),
            ((10, 12), "batz"),
            ((12, 13), "qux"),
            ((13, 19), "quux"),
            ((19, 24), "quuux"),
        ]);
    }

    #[test]
    fn test_empty() {
        assert!(DayPeriodRules::from_periods(Default::default()).is_err());
    }

    #[test]
    fn test_overlap() {
        assert!(
            DayPeriodRules::from_periods(
                [
                    ((0, 12), "foo"),
                    ((12, 19), "bar"),
                    // Overlaps
                    ((18, 21), "baz"),
                    ((21, 24), "qux"),
                ]
                .into_iter()
                .collect(),
            )
            .is_err()
        );
    }

    #[test]
    fn test_gap() {
        assert!(DayPeriodRules::from_periods([((0, 12), "foo"),].into_iter().collect()).is_err());
    }
}
