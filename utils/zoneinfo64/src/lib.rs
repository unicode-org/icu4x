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

#[derive(Debug, Clone, Copy)]
struct Rule<'a> {
    /// The year the rule starts applying
    start_year: u32,
    /// The offset of standard time
    standard_offset_seconds: i32,
    inner: &'a TzRule,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Offset {
    pub since: i64,
    pub offset: UtcOffset,
    pub rule_applies: bool,
}

pub enum PossibleOffset {
    Single(Offset),
    Ambiguous(Offset, Offset),
    None,
}

impl Zone<'_> {
    // Returns the index of the previous transition. As this can be -1, it returns an isize.
    fn offset_idx(&self, seconds_since_epoch: i64) -> isize {
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

    fn offset_at(&self, idx: isize, seconds_since_epoch: i64) -> Offset {
        // before first transition don't use `type_map`, just the first entry in `type_offsets`
        if idx < 0 || self.simple.type_map.is_empty() {
            #[expect(clippy::unwrap_used)] // type_offsets non-empty by invariant
            let &(standard, rule_additional) = self.simple.type_offsets.first().unwrap();
            return Offset {
                since: i64::MIN,
                offset: UtcOffset::from_seconds_unchecked(standard + rule_additional),
                rule_applies: rule_additional > 0,
            };
        }

        let idx = idx as usize;

        // after the last transition, respect the rule
        if idx >= self.simple.type_map.len() - 1 {
            if let Some(rule) = self.final_rule.as_ref() {
                let (additional_offset_seconds, transition) = rule
                    .inner
                    .additional_offset_since(seconds_since_epoch, rule.start_year);
                return Offset {
                    since: transition,
                    rule_applies: additional_offset_seconds != 0,
                    offset: UtcOffset::from_seconds_unchecked(
                        rule.standard_offset_seconds + additional_offset_seconds,
                    ),
                };
            }
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

        Offset {
            since,
            offset: UtcOffset::from_seconds_unchecked(standard + rule_additional),
            rule_applies: rule_additional > 0,
        }
    }

    pub fn for_date_time(
        &self,
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> PossibleOffset {
        const EPOCH: RataDie =
            calendrical_calculations::gregorian::const_fixed_from_gregorian(1970, 1, 1);
        let seconds_since_local_epoch =
            (((calendrical_calculations::gregorian::fixed_from_gregorian(year, month, day)
                - EPOCH)
                * 24
                + hour as i64)
                * 60
                + minute as i64)
                * 60
                + second as i64;

        // Pretend date time is UTC to get a candidate
        let idx = self.offset_idx(seconds_since_local_epoch);

        let candidate = self.offset_at(idx, seconds_since_local_epoch);
        let before_candidate = self.offset_at(idx - 1, seconds_since_local_epoch);
        let after_candidate = self.offset_at(idx + 1, seconds_since_local_epoch);

        let before_candidate_local_until = candidate
            .since
            .saturating_add(before_candidate.offset.to_seconds() as i64);

        let candidate_local_since = candidate
            .since
            .saturating_add(candidate.offset.to_seconds() as i64);
        let candidate_local_until = after_candidate
            .since
            .saturating_add(candidate.offset.to_seconds() as i64);

        let after_candidate_local_since = after_candidate
            .since
            .saturating_add(after_candidate.offset.to_seconds() as i64);

        if seconds_since_local_epoch < before_candidate_local_until
            && seconds_since_local_epoch >= candidate_local_since
            && before_candidate != candidate
        {
            return PossibleOffset::Ambiguous(before_candidate, candidate);
        }

        if seconds_since_local_epoch < candidate_local_until
            && seconds_since_local_epoch >= after_candidate_local_since
            && candidate != after_candidate
        {
            return PossibleOffset::Ambiguous(candidate, after_candidate);
        }

        if seconds_since_local_epoch < before_candidate_local_until {
            return PossibleOffset::Single(before_candidate);
        }
        if seconds_since_local_epoch < candidate_local_until {
            return PossibleOffset::Single(candidate);
        }
        if seconds_since_local_epoch >= after_candidate_local_since {
            return PossibleOffset::Single(after_candidate);
        }

        PossibleOffset::None
    }

    pub fn for_timestamp(&self, seconds_since_epoch: i64) -> Offset {
        self.offset_at(self.offset_idx(seconds_since_epoch), seconds_since_epoch)
    }
}

#[test]
fn test() {
    use chrono::Offset;
    use chrono::TimeZone;
    use chrono_tz::OffsetComponents;

    // Tests pre32 transitions
    // 1938-04-24T22:00:00Z
    const PAST: i64 = -1_000_000_000 - 800;
    // Tests rules and post32 transitions
    // 2033-05-18T03:00:00Z
    const FUTURE: i64 = 3_000_000_000 - 2000;

    let tzdb =
        ZoneInfo64::try_from_u32s(resb::include_bytes_as_u32!("../tests/data/zoneinfo64.res"))
            .expect("Error processing resource bundle file");

    for chrono in chrono_tz::TZ_VARIANTS {
        let iana = chrono.name();

        if std::env::var("EXHAUSTIVE_TZ_TEST").is_err() && iana != "Europe/Zurich" {
            continue;
        }

        println!("{iana}");

        let zoneinfo64 = tzdb.get(iana).unwrap();

        // TODO
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
