// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO

use std::fmt::Debug;

use calendrical_calculations::rata_die::RataDie;
use potential_utf::PotentialUtf16;
use resb::binary::BinaryDeserializerError;

use icu_locale_core::subtags::Region;
use icu_time::zone::UtcOffset;

#[cfg(feature = "chrono")]
mod chrono_impls;

mod rule;
use rule::*;
mod deserialize;

const EPOCH: RataDie = calendrical_calculations::iso::const_fixed_from_iso(1970, 1, 1);
const SECONDS_IN_UTC_DAY: i64 = 24 * 60 * 60;

#[derive(Debug)]
pub struct ZoneInfo64<'a> {
    zones: Vec<TzZone<'a>>,
    names: Vec<&'a PotentialUtf16>,
    rules: Vec<TzRule>,
    regions: Vec<Region>,
}

#[derive(Debug)]
enum TzZone<'a> {
    // The rule data is boxed here due to the large size difference between the
    // `TzDataRuleData` struct and `u32`. It's not strictly necessary.
    Table(Box<TzZoneData<'a>>),
    Int(u32),
}

struct TzZoneData<'a> {
    /// Transitions before the epoch of i32::MIN
    trans_pre32: &'a [(i32, i32)],
    /// Transitions with epoch values that can fit in an i32
    trans: &'a [i32],
    /// Transitions after the epoch of i32::MAX
    trans_post32: &'a [(i32, i32)],
    /// Map to offset from transitions. Treat [trans_pre32, trans, trans_post32]
    /// as a single array and use its corresponding index into this to get the index
    /// in type_offsets. The index in type_offsets is the *new* offset after the
    /// matching transition
    type_map: &'a [u8],
    /// Offsets. First entry is standard time, second entry is offset from standard time (if any)
    type_offsets: &'a [(i32, i32)],
    /// An index into the Rules table,
    /// its standard_offset_seconds, and its starting year.
    final_rule_offset_year: Option<(u32, i32, u32)>,
    #[allow(dead_code)]
    links: &'a [u32],
}

impl Debug for TzZoneData<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TzZoneData {{ ")?;

        fn dbg_timestamp(f: &mut std::fmt::Formatter<'_>, t: i64) -> std::fmt::Result {
            #[cfg(feature = "chrono")]
            let t = chrono::DateTime::from_timestamp(t, 0).unwrap();
            write!(f, "{t:?}, ")
        }

        write!(f, "transitions/offsets: [")?;
        let (std, rule) = self.type_offsets[0];
        write!(f, "{:?}, ", (std as f64 / 3600.0, rule as f64 / 3600.0))?;
        let mut i = 0;
        for &(hi, lo) in self.trans_pre32 {
            dbg_timestamp(f, ((hi as u32 as u64) << 32 | (lo as u32 as u64)) as i64)?;
            let (std, rule) = self.type_offsets[self.type_map[i] as usize];
            write!(f, "{:?}, ", (std as f64 / 3600.0, rule as f64 / 3600.0))?;
            i += 1;
        }
        for &t in self.trans {
            dbg_timestamp(f, t as i64)?;
            let (std, rule) = self.type_offsets[self.type_map[i] as usize];
            write!(f, "{:?}, ", (std as f64 / 3600.0, rule as f64 / 3600.0))?;
            i += 1;
        }
        for &(hi, lo) in self.trans_post32 {
            dbg_timestamp(f, ((hi as u32 as u64) << 32 | (lo as u32 as u64)) as i64)?;
            let (std, rule) = self.type_offsets[self.type_map[i] as usize];
            write!(f, "{:?}, ", (std as f64 / 3600.0, rule as f64 / 3600.0))?;
            i += 1;
        }
        write!(f, "], ")?;

        write!(f, "}}")
    }
}

impl<'a> ZoneInfo64<'a> {
    pub fn try_from_u32s(resb: &'a [u32]) -> Result<Self, BinaryDeserializerError> {
        crate::deserialize::deserialize(resb)
    }
    #[cfg(test)]
    fn is_alias(&self, iana: &str) -> bool {
        let Some(idx) = self
            .names
            .binary_search_by(|&n| n.chars().cmp(iana.chars()))
            .ok()
        else {
            return false;
        };

        #[expect(clippy::indexing_slicing)] // zones and names have the same length
        let zone = &self.zones[idx];

        matches!(zone, &TzZone::Int(_))
    }

    pub fn get(&'a self, iana: &str) -> Option<Zone<'a>> {
        let idx = self
            .names
            .binary_search_by(|&n| n.chars().cmp(iana.chars()))
            .ok()?;

        #[expect(clippy::indexing_slicing)] // binary search
        let name = self.names[idx];

        #[expect(clippy::indexing_slicing)] // regions and names have the same length
        let region = self.regions[idx];
        #[expect(clippy::indexing_slicing)] // zones and names have the same length
        let mut zone = &self.zones[idx];

        #[expect(clippy::indexing_slicing)] // TzZone::Ints are validated as indices
        if let &TzZone::Int(i) = zone {
            zone = &self.zones[i as usize];
        }
        let TzZone::Table(ref zone) = zone else {
            unreachable!() // data validate to have at most one alias jump
        };

        #[expect(clippy::indexing_slicing)] // rules indices are all valid
        let final_rule = zone
            .final_rule_offset_year
            .map(|(idx, offset, year)| (&self.rules[idx as usize], offset, year))
            .map(|(inner, offset_seconds, start_year)| Rule {
                start_year,
                standard_offset_seconds: offset_seconds,
                inner,
            });

        Some(Zone {
            simple: zone.as_ref(),
            final_rule,
            region,
            name,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Zone<'a> {
    simple: &'a TzZoneData<'a>,
    final_rule: Option<Rule<'a>>,
    pub name: &'a PotentialUtf16,
    pub region: Region,
}

/// A resolved offset for a given point in time
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Offset {
    /// The offset from UTC of this time zone
    pub offset: UtcOffset,
    /// Whether or not the Rule (i.e. "non standard" time) applies
    pub rule_applies: bool,
}

/// A transition (from the non-rule transition array in the data)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transition {
    /// When the transition starts
    pub since: i64,
    /// The offset from UTC after this transition
    pub offset: UtcOffset,
    /// Whether or not the Rule (i.e. "non standard" time) applies
    pub rule_applies: bool,
}

impl From<Transition> for Offset {
    fn from(other: Transition) -> Self {
        Self {
            offset: other.offset,
            rule_applies: other.rule_applies,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum PossibleOffset {
    /// There is a single possible offset
    Single(Offset),
    /// There are multiple possible offsets (sorted based on which transition comes first)
    ///
    /// Note: Temporal requires these to be in ascending order of offset, Temporal consumers should sort them
    // <https://tc39.es/proposal-temporal/#sec-getnamedtimezoneepochnanoseconds>
    Ambiguous(Offset, Offset),
    /// There is no possible offset, this is a gap transition
    None,
}

impl Zone<'_> {
    /// Returns the index of the previous transition.
    ///
    /// As this can be -1, it returns an isize.
    ///
    /// Does not consider rule transitions
    fn prev_transition_offset_idx(&self, seconds_since_epoch: i64) -> isize {
        if seconds_since_epoch < i32::MIN as i64 {
            self.simple
                .trans_pre32
                .binary_search(&(
                    (seconds_since_epoch >> 32) as i32,
                    (seconds_since_epoch & 0xFFFFFFFF) as i32,
                ))
                .map(|i| i as isize)
                // binary_search returns the index of the next (higher) transition, so we subtract one
                .unwrap_or_else(|i| i as isize - 1)
        } else if seconds_since_epoch <= i32::MAX as i64 {
            self.simple.trans_pre32.len() as isize
                + self
                    .simple
                    .trans
                    .binary_search(&(seconds_since_epoch as i32))
                    .map(|i| i as isize)
                    // binary_search returns the index of the next (higher) transition, so we subtract one
                    .unwrap_or_else(|i| i as isize - 1)
        } else {
            self.simple.trans_pre32.len() as isize
                + self.simple.trans.len() as isize
                + self
                    .simple
                    .trans_post32
                    .binary_search(&(
                        (seconds_since_epoch >> 32) as i32,
                        (seconds_since_epoch & 0xFFFFFFFF) as i32,
                    ))
                    .map(|i| i as isize)
                    // binary_search returns the index of the next (higher) transition, so we subtract one
                    .unwrap_or_else(|i| i as isize - 1)
        }
    }

    /// Gets the information for the transition offset at idx.
    ///
    /// Invariant: idx must be in-range for the transitions table. It is allowed to be 0
    /// when the table is empty, and it is allowed to be -1 to refer to the offsets before the transitions table.
    ///
    /// Does not handle rule transitions
    fn transition_offset_at(&self, idx: isize) -> Transition {
        // before first transition don't use `type_map`, just the first entry in `type_offsets`
        if idx < 0 || self.simple.type_map.is_empty() {
            #[expect(clippy::unwrap_used)] // type_offsets non-empty by invariant
            let &(standard, rule_additional) = self.simple.type_offsets.first().unwrap();
            return Transition {
                since: i64::MIN,
                offset: UtcOffset::from_seconds_unchecked(standard + rule_additional),
                rule_applies: rule_additional > 0,
            };
        }

        let idx = idx as usize;

        if idx >= self.simple.type_map.len() {
            debug_assert!(false, "Called transition_offset_at with out-of-range index (got {idx}, but only have {} transitions)", self.simple.type_map.len());
            // GIGO behavior
            return Transition {
                since: i64::MIN,
                rule_applies: false,
                offset: Default::default(),
            };
        }

        let idx = core::cmp::min(idx, self.simple.type_map.len() - 1);

        #[expect(clippy::indexing_slicing)]
        // type_map has length sum(trans*), and type_map values are validated to be valid indices in type_offsets
        let (standard, rule_additional) =
            self.simple.type_offsets[self.simple.type_map[idx] as usize];

        #[expect(clippy::indexing_slicing)] // by guards or invariant
        let since = if idx < self.simple.trans_pre32.len() {
            let (hi, lo) = self.simple.trans_pre32[idx];
            ((hi as u32 as u64) << 32 | (lo as u32 as u64)) as i64
        } else if idx - self.simple.trans_pre32.len() < self.simple.trans.len() {
            self.simple.trans[idx - self.simple.trans_pre32.len()] as i64
        } else {
            let (hi, lo) = self.simple.trans_post32
                [idx - self.simple.trans_pre32.len() - self.simple.trans.len()];
            ((hi as u32 as u64) << 32 | (lo as u32 as u64)) as i64
        };

        Transition {
            since,
            offset: UtcOffset::from_seconds_unchecked(standard + rule_additional),
            rule_applies: rule_additional > 0,
        }
    }

    /// Get the possible offsets matching to a timestamp given in *local* (wall) time
    pub fn for_date_time(
        &self,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> PossibleOffset {
        let day_before_year = calendrical_calculations::iso::day_before_year(year);
        let day_in_year =
            calendrical_calculations::iso::days_before_month(year, month) + day as u16;
        let local_time_of_day = (hour as i32 * 60 + minute as i32) * 60 + second as i32;

        // `resolve_local` quickly returns if the rule doesn't apply
        if let Some(rule) = self.final_rule {
            // If rule applies, use it
            if let Some(result) =
                rule.resolve_local(year, day_before_year, day_in_year, local_time_of_day)
            {
                return result;
            }
        }

        // If we have reached this point, the rule does not apply.

        // Pretend date time is UTC to get a candidate

        let seconds_since_local_epoch = (day_before_year + day_in_year as i64 - EPOCH)
            * SECONDS_IN_UTC_DAY
            + local_time_of_day as i64;

        let idx = core::cmp::min(
            self.prev_transition_offset_idx(seconds_since_local_epoch),
            self.simple.type_map.len() as isize - 1,
        );

        // `transition_offset_at` always returns a transition that it thinks is *before* this one
        // We are using local time here, so it *could* be wrong. We need to check
        // the transition it returned (the transition it thinks is before this one),
        // and the transition it thinks is after this one.
        //
        // We do not need to check transitions that are further back or forward;
        // since the data does not have any duplicate transitions (invariant: monotonic-transition-times),
        // and we know that prior transitions are far enough away that there is no chance of their
        // wall times overlapping (invariant: transition-local-times-do-not-overlap)
        let first_candidate = self.transition_offset_at(idx);

        let second_candidate = if idx + 1 == self.simple.type_map.len() as isize {
            // If out of range, just constrain to first_candidate
            first_candidate
        } else {
            self.transition_offset_at(idx + 1)
        };

        // Even though we do not need to *check* transitions that are further back,
        // we do need the transition before `first_candidate` to understand the offset
        // that came before it.
        //
        // When called at the beginning of the array, this just constrains to
        // first_candidate
        let before_first_candidate = self.transition_offset_at(idx - 1);

        // Wall time for `first_candidate`'s transition time, before and after its transition
        let first_candidate_wall_prev = first_candidate
            .since
            .saturating_add(before_first_candidate.offset.to_seconds() as i64);
        let first_candidate_wall_next = first_candidate
            .since
            .saturating_add(first_candidate.offset.to_seconds() as i64);

        // Wall time for `second_candidate`'s transition time, before and after its transition
        let second_candidate_wall_prev = second_candidate
            .since
            .saturating_add(first_candidate.offset.to_seconds() as i64);

        let second_candidate_wall_next = second_candidate
            .since
            .saturating_add(second_candidate.offset.to_seconds() as i64);

        // We are within the first candidate's transition
        if seconds_since_local_epoch < first_candidate_wall_prev
            && seconds_since_local_epoch >= first_candidate_wall_next
        {
            // This is mathematically impossible: if the candidates are equal then
            // seconds_since_local_epoch would not be >= wall_prev but <= wall_next
            debug_assert!(before_first_candidate != first_candidate);
            return PossibleOffset::Ambiguous(
                before_first_candidate.into(),
                first_candidate.into(),
            );
        }

        // We are within the second candidate's transition
        if seconds_since_local_epoch < second_candidate_wall_prev
            && seconds_since_local_epoch >= second_candidate_wall_next
        {
            // This is mathematically impossible: if the candidates are equal then
            // seconds_since_local_epoch would not be >= wall_prev but <= wall_next
            debug_assert!(first_candidate != second_candidate);
            return PossibleOffset::Ambiguous(first_candidate.into(), second_candidate.into());
        }

        // We are before the first transition entirely
        if seconds_since_local_epoch < first_candidate_wall_prev {
            return PossibleOffset::Single(before_first_candidate.into());
        }

        // We are between the two transitions
        if seconds_since_local_epoch < second_candidate_wall_prev {
            return PossibleOffset::Single(first_candidate.into());
        }

        // We are after the second transition entirely
        if seconds_since_local_epoch >= second_candidate_wall_next {
            return PossibleOffset::Single(second_candidate.into());
        }

        // The only other cases are gap transitions (seconds >= wall_prev && seconds < wall_next)
        PossibleOffset::None
    }

    /// Get the offset matching to a timestamp given in UTC time.
    ///
    /// seconds_since_epoch must resolve to a year that is in-range for i32
    pub fn for_timestamp(&self, seconds_since_epoch: i64) -> Offset {
        let mut idx = self.prev_transition_offset_idx(seconds_since_epoch);
        // We add 1 to idx here since idx is the index of the previous transition
        // but if the previous transition is the last one then we still need to check
        // against the rule
        if idx + 1 >= self.simple.type_map.len() as isize {
            if let Some(rule) = self.final_rule {
                if let Some(resolved) = rule.resolve_utc(seconds_since_epoch) {
                    return resolved;
                }
            }
            // Rule doesn't apply, use the last index instead
            idx = self.simple.type_map.len() as isize - 1;
        }
        self.transition_offset_at(idx).into()
    }

    pub fn prev_transition(
        &self,
        seconds_since_epoch: i64,
        seconds_exact: bool,
    ) -> Option<Transition> {
        let mut idx = self.prev_transition_offset_idx(seconds_since_epoch);
        // We add 1 to idx here since idx is the index of the previous transition
        // but if the previous transition is the last one then we still need to check
        // against the rule
        if idx + 1 >= self.simple.type_map.len() as isize {
            if let Some(rule) = self.final_rule {
                if let Some(resolved) = rule.prev_transition(seconds_since_epoch, seconds_exact) {
                    return Some(resolved);
                }
            }
            // Rule doesn't apply, use the last index instead
            idx = self.simple.type_map.len() as isize - 1;
        }

        let candidate = self.transition_offset_at(idx);
        if candidate.since == seconds_since_epoch && seconds_exact {
            if idx == -1 {
                None
            } else {
                Some(self.transition_offset_at(idx - 1))
            }
        } else {
            Some(candidate)
        }
    }

    pub fn next_transition(&self, seconds_since_epoch: i64) -> Option<Transition> {
        let idx = self.prev_transition_offset_idx(seconds_since_epoch);
        if idx + 1 >= self.simple.type_map.len() as isize {
            if let Some(rule) = self.final_rule {
                return Some(rule.next_transition(seconds_since_epoch));
            } else {
                return None;
            }
        }

        Some(self.transition_offset_at(idx + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Tz;
    use itertools::Itertools;
    use std::{str::FromStr, sync::LazyLock};

    pub(crate) static TZDB: LazyLock<ZoneInfo64> = LazyLock::new(|| {
        ZoneInfo64::try_from_u32s(resb::include_bytes_as_u32!("../tests/data/zoneinfo64.res"))
            .expect("Error processing resource bundle file")
    });

    /// This tests invariants we rely on in our code
    ///
    /// These invariants not being upheld should never cause a panic, but can produce garbage behavior.
    #[test]
    fn test_transitions_monotonic() {
        for chrono in chrono_tz::TZ_VARIANTS {
            let iana = chrono.name();

            let zoneinfo64 = TZDB.get(iana).unwrap();

            let mut prev_offset = zoneinfo64.transition_offset_at(-1);
            for idx in 0..zoneinfo64.simple.type_map.len() {
                let offset = zoneinfo64.transition_offset_at(idx as isize);
                if prev_offset.since >= offset.since {
                    debug_assert!(prev_offset.since == offset.since);
                    debug_assert!(
                        idx == 0 || idx == zoneinfo64.simple.type_map.len() - 1,
                        "{iana}: Transition times are monotonically increasing, \
                                  with no overlaps except at the beginning/end (found at {idx}) \
                                  (invariant: monotonic-transition-times)"
                    );
                    continue;
                }

                let prev_offset_wall = prev_offset
                    .since
                    .saturating_add(prev_offset.offset.to_seconds() as i64);
                let offset_wall = offset
                    .since
                    .saturating_add(offset.offset.to_seconds() as i64);

                debug_assert!(prev_offset_wall < offset_wall, "{iana}: Transition times are never so close as to create \
                                                               a potential region of ambiguity with multiple transitions \
                                                               {prev_offset_wall} < {offset_wall} \
                                                               (invariant: transition-local-times-do-not-overlap)");

                prev_offset = offset;
            }
        }
    }

    // Tests pre32 transitions
    // 1938-04-24T22:00:00Z
    const PAST: i64 = -1_000_000_000 - 800;
    // Tests rules and post32 transitions
    // 2033-05-18T03:00:00Z
    const FUTURE: i64 = 3_000_000_000 - 2000;

    // To test all timezones, set EXHAUSTIVE_TZ_TEST=1
    //
    // We recommend testing with `--profile release-with-assertions`
    fn time_zones_to_test() -> &'static [Tz] {
        if std::env::var("EXHAUSTIVE_TZ_TEST").is_err() {
            // Keep this list small, this test is slow
            &[
                // Some normal timezones with DST
                // Wall rule
                Tz::America__Los_Angeles,
                // Standard rules
                Tz::Europe__London,
                Tz::Europe__Zurich,
                // Utc rule
                Tz::America__Santiago,
                // Transition skips a day
                Tz::Pacific__Apia,
                // Transition removes sub-second offset
                Tz::Pacific__Niue,
                // Has a single transition into a rule
                Tz::Antarctica__Troll,
            ][..]
        } else {
            &chrono_tz::TZ_VARIANTS[..]
        }
    }

    fn has_rearguard_diff(iana: &str) -> bool {
        matches!(
            iana,
            "Africa/Casablanca"
                | "Africa/El_Aaiun"
                | "Africa/Windhoek"
                | "Europe/Dublin"
                | "Eire"
                | "Europe/Bratislava"
                | "Europe/Prague"
        )
    }

    #[test]
    fn test_against_chrono() {
        use chrono::Offset;
        use chrono::TimeZone;
        use chrono_tz::OffsetComponents;

        for chrono in time_zones_to_test() {
            let iana = chrono.name();

            if TZDB.is_alias(iana) {
                continue;
            }

            println!("{iana}");

            let zoneinfo64 = TZDB.get(iana).unwrap();

            for seconds_since_epoch in (PAST..FUTURE).step_by(60 * 60) {
                let utc_datetime = chrono::DateTime::from_timestamp(seconds_since_epoch, 0)
                    .unwrap()
                    .naive_utc();

                let zoneinfo64_date = zoneinfo64.from_utc_datetime(&utc_datetime);
                let chrono_date = chrono.from_utc_datetime(&utc_datetime);
                assert_eq!(
                    zoneinfo64_date.offset().fix(),
                    chrono_date.offset().fix(),
                    "{seconds_since_epoch}, {iana:?}",
                );

                let local_datetime = chrono_date.naive_local();
                assert_eq!(
                    zoneinfo64
                        .offset_from_local_datetime(&local_datetime)
                        .map(|o| o.fix()),
                    chrono
                        .offset_from_local_datetime(&local_datetime)
                        .map(|o| o.fix()),
                    "{seconds_since_epoch}, {zoneinfo64:?} {local_datetime}",
                );

                // Rearguard / vanguard diffs
                if has_rearguard_diff(iana) {
                    continue;
                }

                assert_eq!(
                    zoneinfo64_date.offset().rule_applies(),
                    !chrono_date.offset().dst_offset().is_zero(),
                    "{seconds_since_epoch}, {iana:?}",
                );
            }
        }
    }

    #[test]
    fn test_transition_against_jiff() {
        for zone in time_zones_to_test() {
            let iana = zone.name();

            if TZDB.is_alias(iana) || has_rearguard_diff(iana) {
                continue;
            }

            println!("{iana}");

            let jiff = jiff::tz::TimeZone::get(iana).unwrap();
            let zoneinfo64 = TZDB.get(iana).unwrap();

            let mut transitions = jiff
                .preceding(jiff::Timestamp::from_str("2222-01-01T00:00:00Z").unwrap())
                .collect::<Vec<_>>()
                .into_iter()
                .rev()
                .dedup_by(|a, b| a.offset() == b.offset())
                .filter(|t| {
                    t.timestamp() > jiff::Timestamp::from_str("1920-01-01T00:00:00Z").unwrap()
                })
                .map(|t| Transition {
                    since: t.timestamp().as_second(),
                    offset: UtcOffset::from_seconds_unchecked(t.offset().seconds()),
                    rule_applies: t.dst().is_dst(),
                })
                .collect::<Vec<_>>();

            if transitions.first().is_some_and(|t| t.offset.is_zero()) {
                transitions.remove(0);
            }

            for t in &transitions {
                let exact = zoneinfo64.prev_transition(t.since, true);
                let after1 = zoneinfo64.prev_transition(t.since, false);

                let before1 = zoneinfo64.prev_transition(t.since - 1, true);
                let before2 = zoneinfo64.prev_transition(t.since - 1, false);

                let after2 = zoneinfo64.prev_transition(t.since + 1, true);
                let after3 = zoneinfo64.prev_transition(t.since + 1, false);

                assert_eq!(before1, before2);
                assert_eq!(before1, exact);

                assert_ne!(exact, after1);

                assert_eq!(after1, after2);
                assert_eq!(after2, after3);

                assert_eq!(after1, Some(*t));
            }

            // for t in &transitions {
            //     let before = zoneinfo64.next_transition(t.since - 1);
            //     let exact = zoneinfo64.next_transition(t.since);
            //     let after = zoneinfo64.next_transition(t.since + 1);

            //     assert_ne!(before, exact);

            //     assert_eq!(exact, after);

            //     assert_eq!(before, Some(*t));
            // }
        }
    }
}
