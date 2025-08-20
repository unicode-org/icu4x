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
    final_rule_offset_year: Option<(u32, i32, i32)>,
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

    fn transition_count(&self) -> isize {
        self.simple.type_map.len() as isize
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

        debug_assert!(idx < self.transition_count(), "Called transition_offset_at with out-of-range index (got {idx}, but only have {} transitions)", self.transition_count());

        let idx = core::cmp::min(idx, self.transition_count() - 1);

        let idx = idx as usize;

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

    /// Get the possible offsets for a local datetime.
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
        let seconds_since_local_epoch = (day_before_year
            + calendrical_calculations::iso::days_before_month(year, month) as i64
            + day as i64
            - EPOCH)
            * SECONDS_IN_UTC_DAY
            + ((hour as i64 * 60 + minute as i64) * 60 + second as i64);

        let rule = self.final_rule.filter(|rule| year >= rule.start_year);
        let mut idx = 0;

        // Compute the candidate transition and the offset that was used before the transition (which is
        // required to to validates times around the first transition).
        // This is either from the rule or the transitions.
        let (before_first_candidate, first_candidate) = if let Some(rule) = rule {
            // The rule applies and we use this year's first transition as the first candidate.
            let (before, candidate) = rule.transition(year, day_before_year, false);
            (Some(before), candidate)
        } else {
            // Pretend date time is UTC to get a candidate
            idx = self.prev_transition_offset_idx(seconds_since_local_epoch);

            // We use the transition before the "timestamp" as the first candidate.
            //
            // We do not need to check transitions that are further back or forward;
            // since the data does not have any duplicate transitions (`test_monotonic_transition_times`),
            // and we know that prior transitions are far enough away that there is no chance of their
            // wall times overlapping (`test_transition_local_times_do_not_overlap`)
            (
                (idx >= 0).then(|| self.transition_offset_at(idx - 1).into()),
                self.transition_offset_at(idx),
            )
        };

        // There's only an actual transition into `first_candidate` if there's an offset before it.
        if let Some(before_first_candidate) = before_first_candidate {
            let wall_before =
                first_candidate.since + before_first_candidate.offset.to_seconds() as i64;
            let wall_after = first_candidate.since + first_candidate.offset.to_seconds() as i64;

            match (
                seconds_since_local_epoch < wall_before,
                seconds_since_local_epoch < wall_after,
            ) {
                // We are before the first transition entirely
                (true, true) => return PossibleOffset::Single(before_first_candidate),
                // We are within the first candidate's transition
                (true, false) => {
                    // This is impossible: if the candidates are equal then
                    // there can be no repeated local times.
                    debug_assert_ne!(before_first_candidate.offset, first_candidate.offset);

                    return PossibleOffset::Ambiguous(
                        before_first_candidate,
                        first_candidate.into(),
                    );
                }
                // We are in the first candidate's gap
                (false, true) => return PossibleOffset::None,
                // We are after the first candidate, try the second
                (false, false) => {}
            }
        }

        let second_candidate = if let Some(rule) = rule {
            // The rule applies and we use this year's second transition as the second candidate.
            Some(rule.transition(year, day_before_year, true).1)
        } else {
            // We use the transition after the "timestamp" as the second candidate.
            (idx + 1 < self.transition_count()).then(|| self.transition_offset_at(idx + 1))
        };

        if let Some(second_candidate) = second_candidate {
            let wall_before = second_candidate.since + first_candidate.offset.to_seconds() as i64;
            let wall_after = second_candidate.since + second_candidate.offset.to_seconds() as i64;

            match (
                seconds_since_local_epoch < wall_before,
                seconds_since_local_epoch < wall_after,
            ) {
                // We are before the second transition entirely
                (true, true) => return PossibleOffset::Single(first_candidate.into()),
                // We are within the second candidate's transition
                (true, false) => {
                    // This is impossible: if the candidates are equal then
                    // there can be no repeated local times.
                    debug_assert_ne!(first_candidate.offset, second_candidate.offset);

                    return PossibleOffset::Ambiguous(
                        first_candidate.into(),
                        second_candidate.into(),
                    );
                }
                // We are in the second candidate's gap
                (false, true) => return PossibleOffset::None,
                // We are after the second candidate
                (false, false) => return PossibleOffset::Single(second_candidate.into()),
            }
        }

        PossibleOffset::Single(first_candidate.into())
    }

    /// Get the offset for a timestamp.
    pub fn for_timestamp(&self, seconds_since_epoch: i64) -> Offset {
        let idx = self.prev_transition_offset_idx(seconds_since_epoch);
        // If the previous transition is the last one then we need to check
        // against the rule
        if idx == self.transition_count() - 1 {
            if let Some(rule) = self.final_rule {
                if let Some(resolved) = rule.for_timestamp(seconds_since_epoch) {
                    return resolved;
                }
            }
        }
        self.transition_offset_at(idx).into()
    }

    /// Get the last transition before a timestamp.
    ///
    /// If `seconds_exact` is false, the transition at `x` is considered
    /// to be before the timestamp `x`.
    pub fn prev_transition(
        &self,
        seconds_since_epoch: i64,
        seconds_exact: bool,
    ) -> Option<Transition> {
        let idx = self.prev_transition_offset_idx(seconds_since_epoch);

        // `self.transition_offset_at()` returns a synthetic transition at `i64::MIN`
        // for the time range before the actual first transition.
        if idx == -1 {
            return None;
        }

        if idx == self.transition_count() - 1 {
            // If the previous transition is the last one then we need to check
            // against the rule
            if let Some(rule) = self.final_rule {
                if let Some(resolved) = rule.prev_transition(seconds_since_epoch, seconds_exact) {
                    return Some(resolved);
                }
            }
        }

        let candidate = self.transition_offset_at(idx);
        if candidate.since == seconds_since_epoch && seconds_exact {
            // If the transition is an exact hit, we actually want the one before
            (idx > 0).then(|| self.transition_offset_at(idx - 1))
        } else {
            Some(candidate)
        }
    }

    /// Get the first transition after a timestamp.
    pub fn next_transition(&self, seconds_since_epoch: i64) -> Option<Transition> {
        let idx = self.prev_transition_offset_idx(seconds_since_epoch);
        Some(if idx == self.transition_count() - 1 {
            // If the previous transition is the last one then the next one
            // can only be the rule.
            self.final_rule?.next_transition(seconds_since_epoch)
        } else {
            self.transition_offset_at(idx + 1)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono_tz::Tz;
    use itertools::Itertools;
    use std::sync::LazyLock;

    pub(crate) static TZDB: LazyLock<ZoneInfo64> = LazyLock::new(|| {
        ZoneInfo64::try_from_u32s(resb::include_bytes_as_u32!("../tests/data/zoneinfo64.res"))
            .expect("Error processing resource bundle file")
    });

    /// Tests an invariant we rely on in our code
    #[test]
    fn test_monotonic_transition_times() {
        for chrono in time_zones_to_test() {
            let iana = chrono.name();
            let zoneinfo64 = TZDB.get(iana).unwrap();

            for (prev, curr) in (-1..zoneinfo64.transition_count())
                .map(|idx| zoneinfo64.transition_offset_at(idx))
                .tuple_windows::<(_, _)>()
            {
                assert!(
                    prev.since < curr.since,
                    "{iana}: Transition times should be strictly increasing ({prev:?}, {curr:?})"
                );
            }
        }
    }

    /// Tests an invariant we rely on in our code
    #[test]
    fn test_transition_local_times_do_not_overlap() {
        for chrono in time_zones_to_test() {
            let iana = chrono.name();
            let zoneinfo64 = TZDB.get(iana).unwrap();

            for (prev, curr) in (-1..zoneinfo64.transition_count())
                .map(|idx| zoneinfo64.transition_offset_at(idx))
                .tuple_windows::<(_, _)>()
            {
                let prev_wall = prev.since.saturating_add(prev.offset.to_seconds() as i64);
                let curr_wall = curr.since.saturating_add(curr.offset.to_seconds() as i64);

                assert!(
                    prev_wall < curr_wall,
                    "{iana}: Transitions should not be so close as to create a ambiguity ({prev:?}, {curr:?}"
                );
            }
        }
    }

    pub(crate) fn time_zones_to_test() -> impl Iterator<Item = Tz> {
        chrono_tz::TZ_VARIANTS
            .iter()
            .copied()
            .filter(|tz| !TZDB.is_alias(tz.name()))
    }

    fn has_rearguard_diff(iana: &str) -> bool {
        matches!(
            iana,
            "Africa/Casablanca"
                | "Africa/El_Aaiun"
                | "Africa/Windhoek"
                | "Europe/Dublin"
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

            let zoneinfo64 = TZDB.get(iana).unwrap();

            for seconds_since_epoch in transitions(iana)
                .into_iter()
                // 30-minute increments around a transition
                .flat_map(|t| (-3..=3).map(move |h| t.since + h * 30 * 60))
            {
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

    fn transitions(iana: &str) -> Vec<Transition> {
        let tz = tzdb::tz_by_name(iana).unwrap();
        let mut transitions = tz
            .transitions()
            .iter()
            .map(|t| {
                let o = tz.find_local_time_type(t.unix_leap_time()).unwrap();
                Transition {
                    since: t.unix_leap_time(),
                    offset: UtcOffset::from_seconds_unchecked(o.ut_offset()),
                    rule_applies: o.is_dst(),
                }
            })
            .collect::<Vec<_>>();

        // tzdb returns transitions also if only the name changes, we don't
        transitions.retain(|t| {
            let before = tz.find_local_time_type(t.since - 1).unwrap();
            before.ut_offset() != t.offset.to_seconds()
                || before.is_dst() != t.rule_applies
                // This is a super weird transition that would be removed by our rule,
                // but we want to keep it because it's in zoneinfo64.
                // 1944-04-03T01:00:00Z, (1.0, 1.0)
                // 1944-08-24T22:00:00Z, (0.0, 2.0) <- same offset and also DST
                // 1944-10-07T23:00:00Z, (0.0, 1.0)
                || (iana == "Europe/Paris" && t.since == -800071200)
        });

        transitions
    }

    #[test]
    fn test_transition_against_tzdb() {
        for zone in time_zones_to_test() {
            let iana = zone.name();
            let transitions = transitions(iana);

            if has_rearguard_diff(iana) || transitions.is_empty() {
                continue;
            }

            let zoneinfo64 = TZDB.get(iana).unwrap();

            assert_eq!(zoneinfo64.prev_transition(i64::MIN + 1, true), None);
            assert_eq!(zoneinfo64.prev_transition(i64::MIN + 1, false), None);

            assert_eq!(
                zoneinfo64.next_transition(i64::MIN),
                transitions.first().copied()
            );

            for ts in transitions.windows(2) {
                let &[prev, curr] = ts else { unreachable!() };

                assert_eq!(zoneinfo64.prev_transition(curr.since - 1, true), Some(prev));
                assert_eq!(
                    zoneinfo64.prev_transition(curr.since - 1, false),
                    Some(prev)
                );
                assert_eq!(zoneinfo64.prev_transition(curr.since, true), Some(prev));

                assert_eq!(zoneinfo64.prev_transition(curr.since, false), Some(curr));
                assert_eq!(zoneinfo64.prev_transition(curr.since + 1, true), Some(curr));
                assert_eq!(
                    zoneinfo64.prev_transition(curr.since + 1, false),
                    Some(curr)
                );

                assert_eq!(zoneinfo64.next_transition(prev.since - 1), Some(prev));
                assert_eq!(zoneinfo64.next_transition(prev.since), Some(curr));
                assert_eq!(zoneinfo64.next_transition(prev.since + 1), Some(curr))
            }

            if zoneinfo64.final_rule.is_none() {
                assert_eq!(
                    zoneinfo64.next_transition(transitions.last().unwrap().since),
                    None
                );
            }
        }
    }
}
