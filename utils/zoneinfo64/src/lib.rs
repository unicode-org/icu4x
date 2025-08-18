// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO

use std::{fmt::Debug, marker::PhantomData};

use calendrical_calculations::rata_die::RataDie;
use potential_utf::PotentialUtf16;
use resb::binary::BinaryDeserializerError;
use serde::Deserialize;

use icu_locale_core::subtags::Region;
use icu_time::zone::UtcOffset;

#[cfg(feature = "chrono")]
mod chrono_impls;

mod rule;
use rule::*;

const EPOCH: RataDie = calendrical_calculations::iso::const_fixed_from_iso(1970, 1, 1);
const SECONDS_IN_UTC_DAY: i64 = 86400;

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
        use serde::de::*;

        #[derive(Debug, Deserialize)]
        #[serde(rename = "zoneinfo64")]
        #[serde(rename_all = "PascalCase")]
        struct ZoneInfo64Raw<'a> {
            #[serde(borrow)]
            zones: Vec<TzZoneRaw<'a>>,
            #[serde(borrow, deserialize_with = "resb::binary::helpers::vec_utf_16")]
            names: Vec<&'a PotentialUtf16>,
            #[serde(borrow, deserialize_with = "rules")]
            rules: Vec<(&'a str, TzRule)>,
            #[serde(deserialize_with = "regions")]
            regions: Vec<Region>,
        }

        #[derive(Debug)]
        pub enum TzZoneRaw<'a> {
            Table(TzZoneDataRaw<'a>),
            Int(u32),
        }

        impl<'de: 'a, 'a> Deserialize<'de> for TzZoneRaw<'a> {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct TzDataRuleEnumVisitor<'a> {
                    phantom: PhantomData<TzZoneRaw<'a>>,
                }

                impl<'de: 'a, 'a> Visitor<'de> for TzDataRuleEnumVisitor<'a> {
                    type Value = TzZoneRaw<'a>;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                        formatter.write_str("an unsigned 32-bit integer or a table of rule data")
                    }

                    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
                    where
                        E: Error,
                    {
                        Ok(TzZoneRaw::Int(v))
                    }

                    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
                    where
                        A: MapAccess<'de>,
                    {
                        let value =
                            TzZoneDataRaw::deserialize(value::MapAccessDeserializer::new(map))?;

                        Ok(TzZoneRaw::Table(value))
                    }
                }

                deserializer.deserialize_any(TzDataRuleEnumVisitor {
                    phantom: PhantomData,
                })
            }
        }

        #[derive(Debug, Deserialize)]
        #[serde(rename_all = "camelCase")]
        pub struct TzZoneDataRaw<'a> {
            #[serde(borrow, deserialize_with = "resb::binary::helpers::i32_tuple")]
            type_offsets: &'a [(i32, i32)],
            #[serde(
                borrow,
                default,
                deserialize_with = "resb::binary::helpers::option_i32"
            )]
            trans: Option<&'a [i32]>,
            #[serde(
                borrow,
                default,
                deserialize_with = "resb::binary::helpers::option_i32_tuple"
            )]
            trans_pre32: Option<&'a [(i32, i32)]>,
            #[serde(
                borrow,
                default,
                deserialize_with = "resb::binary::helpers::option_i32_tuple"
            )]
            trans_post32: Option<&'a [(i32, i32)]>,
            type_map: Option<&'a [u8]>,
            #[serde(
                borrow,
                default,
                deserialize_with = "resb::binary::helpers::option_utf_16"
            )]
            final_rule: Option<&'a PotentialUtf16>,
            final_raw: Option<i32>,
            final_year: Option<u32>,
            #[allow(dead_code)]
            #[serde(
                borrow,
                default,
                deserialize_with = "resb::binary::helpers::option_u32"
            )]
            links: Option<&'a [u32]>,
        }

        fn rules<'de, D: Deserializer<'de>>(
            deserializer: D,
        ) -> Result<Vec<(&'de str, TzRule)>, D::Error> {
            struct RulesVisitor;

            impl<'de> Visitor<'de> for RulesVisitor {
                type Value = Vec<(&'de str, TzRule)>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, "a sequence of UTF-16 slices")
                }

                fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
                where
                    A: MapAccess<'de>,
                {
                    let mut vec = vec![];
                    while let Some((key, value)) = map.next_entry::<&str, &[u8]>()? {
                        if value
                            .as_ptr()
                            .align_offset(core::mem::align_of::<[i32; 11]>())
                            != 0
                            || value.len() != core::mem::size_of::<[i32; 11]>()
                        {
                            return Err(A::Error::custom("Wrong length or align"));
                        }
                        let value = unsafe { &*(value.as_ptr() as *const [i32; 11]) };

                        vec.push((key, TzRule::from_raw(value)));
                    }
                    Ok(vec)
                }
            }

            deserializer.deserialize_map(RulesVisitor)
        }

        fn regions<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Vec<Region>, D::Error> {
            struct RegionsVisitor;

            impl<'de> Visitor<'de> for RegionsVisitor {
                type Value = Vec<Region>;

                fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                    write!(formatter, "a sequence of UTF-16 slices")
                }

                fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
                where
                    A: SeqAccess<'de>,
                {
                    let mut vec = vec![];
                    while let Some(bytes) = seq.next_element::<&[u8]>()? {
                        let utf16 = potential_utf::PotentialUtf16::from_slice(
                            // Safety: all byte representations are valid u16s
                            unsafe {
                                resb::binary::helpers::cast_bytes_to_slice::<_, A::Error>(bytes)?
                            },
                        );

                        let mut utf16 = utf16.chars();

                        let Ok(region) = Region::try_from_raw([
                            utf16.next().filter(char::is_ascii).unwrap_or_default() as u8,
                            utf16.next().filter(char::is_ascii).unwrap_or_default() as u8,
                            utf16.next().filter(char::is_ascii).unwrap_or_default() as u8,
                        ]) else {
                            return Err(A::Error::custom("Invalid region code"));
                        };

                        vec.push(region);
                    }
                    Ok(vec)
                }
            }

            deserializer.deserialize_seq(RegionsVisitor)
        }

        let ZoneInfo64Raw {
            zones,
            names,
            rules,
            regions,
        } = resb::binary::from_words::<ZoneInfo64Raw>(resb)?;

        if zones.len() != names.len() || names.len() != regions.len() {
            return Err(BinaryDeserializerError::unknown(
                "zones, names, regions need to have matching lengths",
            ));
        }

        // Translate rule string keys to indices
        let rules_lookup =
            |name: &PotentialUtf16| rules.iter().position(|&(n, _)| name.chars().eq(n.chars()));

        let raw_zones = zones;
        let mut zones = Vec::with_capacity(raw_zones.len());

        for zone in &raw_zones {
            match *zone {
                TzZoneRaw::Int(i) => {
                    let Some(alias) = raw_zones.get(i as usize) else {
                        return Err(BinaryDeserializerError::unknown("invalid alias idx"));
                    };
                    if let TzZoneRaw::Int(_) = alias {
                        return Err(BinaryDeserializerError::unknown("multi-step alias"));
                    }
                    zones.push(TzZone::Int(i))
                }
                TzZoneRaw::Table(TzZoneDataRaw {
                    type_offsets,
                    trans,
                    trans_pre32,
                    trans_post32,
                    type_map,
                    final_rule,
                    final_raw,
                    final_year,
                    links,
                }) => {
                    let trans = trans.unwrap_or_default();
                    let trans_pre32 = trans_pre32.unwrap_or_default();
                    let trans_post32 = trans_post32.unwrap_or_default();
                    let type_map = type_map.unwrap_or_default();

                    let links = links.unwrap_or_default();

                    if trans.len() + trans_post32.len() + trans_pre32.len() != type_map.len()
                        || type_offsets.is_empty()
                    {
                        return Err(BinaryDeserializerError::unknown("inconsistent offset data"));
                    }

                    for &idx in type_map {
                        if idx as usize >= type_offsets.len() {
                            return Err(BinaryDeserializerError::unknown("invalid offset map"));
                        }
                    }

                    let final_rule_offset_year = match (final_rule, final_raw, final_year) {
                        (Some(name), Some(offset), Some(year)) => {
                            let Some(idx) = rules_lookup(name) else {
                                return Err(BinaryDeserializerError::unknown("invalid rule id"));
                            };
                            Some((idx as u32, offset, year))
                        }
                        (None, None, None) => None,
                        _ => {
                            return Err(BinaryDeserializerError::unknown(
                                "inconsisent finalRule, finalRaw, finalYear",
                            ))
                        }
                    };

                    zones.push(TzZone::Table(Box::new(TzZoneData {
                        type_offsets,
                        trans,
                        trans_pre32,
                        trans_post32,
                        type_map,
                        final_rule_offset_year,
                        links,
                    })))
                }
            }
        }

        let rules = rules.into_iter().map(|(_name, rule)| rule).collect();

        Ok(Self {
            zones,
            names,
            rules,
            regions,
        })
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
    fn transition_offset_idx(&self, seconds_since_epoch: i64) -> isize {
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
        const EPOCH: RataDie = calendrical_calculations::iso::const_fixed_from_iso(1970, 1, 1);
        let seconds_since_local_epoch =
            (((calendrical_calculations::iso::fixed_from_iso(year, month, day) - EPOCH) * 24
                + hour as i64)
                * 60
                + minute as i64)
                * 60
                + second as i64;

        // Pretend date time is UTC to get a candidate
        let mut idx = self.transition_offset_idx(seconds_since_local_epoch);

        // If the index is at the end of the array
        // Note that transition_offset_idx returns in-bounds values or -1,
        // so we can't check if it's out of bounds, we need to check if it's the last
        // element
        if idx + 1 == self.simple.type_map.len() as isize {
            if let Some(rule) = self.final_rule {
                // If rule applies, use it
                if let Some(result) = rule.resolve_local(year, month, day, hour, minute, second) {
                    return result;
                }
            }

            // Rule doesn't apply. Make sure to reset to a valid index
            idx = self.simple.type_map.len() as isize - 1;
        }

        // If we have reached this point, the rule does not apply, either because
        // idx is not the last index, or because the rule (if any) does not apply yet.

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
        let mut idx = self.transition_offset_idx(seconds_since_epoch);
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Datelike;
    use std::sync::LazyLock;

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

    /// This tests invariants we rely on in our code
    ///
    /// These invariants not being upheld should never cause a panic, but can produce garbage behavior.
    #[test]
    fn test_rule_not_at_year_boundary() {
        for chrono in chrono_tz::TZ_VARIANTS {
            let iana = chrono.name();

            let zoneinfo64 = TZDB.get(iana).unwrap();

            if let Some(rule) = zoneinfo64.final_rule {
                let final_offset = zoneinfo64.transition_offset_idx(i64::MAX);
                let offset = zoneinfo64.transition_offset_at(final_offset);
                let utc_datetime = chrono::DateTime::from_timestamp(offset.since, 0)
                    .unwrap()
                    .naive_utc();

                assert!(
                    utc_datetime.year() < rule.start_year as i32,
                    "{iana}: Expected last transition to not be in rule year {} < {} \
                    (invariant: last-transition-not-in-rule-year)",
                    utc_datetime.year(),
                    rule.start_year
                );

                let max_delta = (rule.standard_offset_seconds.abs()
                    + rule.inner.additional_offset_secs.abs())
                    as u32;
                for date in [&rule.inner.start, &rule.inner.end] {
                    let seconds_of_day = date.transition_time;
                    if date.month == 0 && date.day == 1 {
                        assert!(
                            seconds_of_day > max_delta,
                            "{iana}: Rule at beginning should not cross year boundary \
                                 {seconds_of_day} > Δ{max_delta} \
                                 (invariant: rule-stays-inside-year)"
                        );
                    }
                    if date.month == 11 && date.day == 31 {
                        assert!(
                            seconds_of_day + max_delta < SECONDS_IN_UTC_DAY as u32,
                            "{iana}: Rule at end of year should not cross year boundary \
                                 {seconds_of_day} + Δ{max_delta} < 24h \
                                 (invariant: rule-stays-inside-year)"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_against_chrono() {
        use chrono::Offset;
        use chrono::TimeZone;
        use chrono_tz::OffsetComponents;
        use chrono_tz::Tz;

        // Tests pre32 transitions
        // 1938-04-24T22:00:00Z
        const PAST: i64 = -1_000_000_000 - 800;
        // Tests rules and post32 transitions
        // 2033-05-18T03:00:00Z
        const FUTURE: i64 = 3_000_000_000 - 2000;

        // To test all timezones, set EXHAUSTIVE_TZ_TEST=1
        //
        // We recommend testing with `--profile release-with-assertions`
        let time_zones_to_test = if std::env::var("EXHAUSTIVE_TZ_TEST").is_err() {
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
        };

        for chrono in time_zones_to_test {
            let iana = chrono.name();

            if TZDB.is_alias(iana) {
                continue;
            }

            println!("{iana}");

            let zoneinfo64 = TZDB.get(iana).unwrap();

            // Temporary cap until rules are implemented
            let max_working_timestamp = if zoneinfo64.final_rule.is_some() {
                zoneinfo64
                    .simple
                    .trans
                    .len()
                    .checked_sub(2)
                    .map(|i| zoneinfo64.simple.trans[i])
                    .unwrap_or(i32::MAX) as i64
            } else {
                FUTURE
            };

            for seconds_since_epoch in (PAST..max_working_timestamp).step_by(60 * 60) {
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
                if [
                    "Africa/Casablanca",
                    "Africa/El_Aaiun",
                    "Africa/Windhoek",
                    "Europe/Dublin",
                    "Eire",
                    "Europe/Bratislava",
                    "Europe/Prague",
                ]
                .contains(&chrono.name())
                {
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
}
