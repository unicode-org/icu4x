// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{fmt::Debug, marker::PhantomData};

use potential_utf::PotentialUtf16;
use resb::include_bytes_as_u32;
use serde::Deserialize;

use icu::{locale::subtags::Region, time::zone::UtcOffset};

#[derive(Debug)]
struct ZoneInfo64<'a> {
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

#[derive(Debug)]
struct TzZoneData<'a> {
    type_offsets: &'a [(i32, i32)],
    trans: &'a [i32],
    trans_pre32: &'a [(i32, i32)],
    trans_post32: &'a [(i32, i32)],
    type_map: &'a [u8],
    final_rule_offset_year: Option<(u32, i32, u32)>,
    #[allow(dead_code)]
    links: Option<&'a [u32]>,
}

#[derive(Debug)]
struct TzRule {
    additional_offset_secs: i32,
    start: TzRuleDate,
    end: TzRuleDate,
}

#[derive(Debug)]
struct TzRuleDate {
    day: i8,
    day_of_week: i8,
    month: u8,
    millis_of_day: u32,
    time_mode: TimeMode,
    mode: RuleMode,
}

#[derive(Debug)]
enum TimeMode {
    Wall = 0,
    Standard = 1,
    Utc = 2,
}

#[derive(Debug, PartialEq)]
#[allow(non_camel_case_types)]
enum RuleMode {
    DOW_IN_MONTH,
    DOM,
    DOW_GE_DOM,
    DOW_LE_DOM,
}

impl TzRuleDate {
    fn new(
        mut day: i8,
        mut day_of_week: i8,
        month: u8,
        millis_of_day: u32,
        time_mode: i8,
    ) -> Option<Self> {
        const GREGORIAN_MONTHS: [i8; 12] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

        if day == 0 {
            return None;
        }
        if month > 11 {
            return None;
        }
        if millis_of_day > 24 * 60 * 60 * 1000 {
            return None;
        }

        let time_mode = match time_mode {
            0 => TimeMode::Wall,
            1 => TimeMode::Standard,
            2 => TimeMode::Utc,
            _ => return None,
        };

        let mode;

        if day_of_week == 0 {
            mode = RuleMode::DOM;
        } else {
            if day_of_week > 0 {
                mode = RuleMode::DOW_IN_MONTH
            } else {
                day_of_week = -day_of_week;
                if day > 0 {
                    mode = RuleMode::DOW_GE_DOM;
                } else {
                    day = -day;
                    mode = RuleMode::DOW_LE_DOM;
                }
            }
            if day_of_week > 7 {
                return None;
            }
        }

        if mode == RuleMode::DOW_IN_MONTH {
            if !(-5..=5).contains(&day) {
                return None;
            }
        } else if day < 1 || day > GREGORIAN_MONTHS[month as usize] {
            return None;
        }

        Some(Self {
            day,
            day_of_week,
            month,
            millis_of_day,
            time_mode,
            mode,
        })
    }
}

impl<'de> Deserialize<'de> for ZoneInfo64<'de> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
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
                        if value.as_ptr().align_offset(core::mem::align_of::<i32>()) != 0
                            || value.len() != core::mem::size_of::<i32>() * 11
                        {
                            return Err(A::Error::custom("Wrong length or align"));
                        }
                        let value = unsafe { &*(value.as_ptr() as *const [i32; 11]) };

                        vec.push((
                            key,
                            TzRule {
                                additional_offset_secs: value[10],
                                start: TzRuleDate::new(
                                    value[1] as i8,
                                    value[2] as i8,
                                    value[0] as u8,
                                    value[3] as u32,
                                    value[4] as i8,
                                )
                                .unwrap(),
                                end: TzRuleDate::new(
                                    value[6] as i8,
                                    value[7] as i8,
                                    value[5] as u8,
                                    value[8] as u32,
                                    value[9] as i8,
                                )
                                .unwrap(),
                            },
                        ));
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
                        if bytes.as_ptr().align_offset(core::mem::align_of::<u16>()) != 0
                            || bytes.len() % core::mem::size_of::<u16>() != 0
                        {
                            return Err(A::Error::custom("Wrong length or align"));
                        }

                        // Safety: The check gurantees length and alignment
                        let utf16 = potential_utf::PotentialUtf16::from_slice(unsafe {
                            core::slice::from_raw_parts(
                                bytes.as_ptr() as *const u16,
                                bytes.len() / core::mem::size_of::<u16>(),
                            )
                        });

                        let mut utf16 = utf16.chars();

                        let Ok(region) = Region::try_from_raw([
                            // interpreting the UTF-16 code points as UTF-8 code points, which is fine because they're
                            // guaranteed ASCII
                            utf16.next().unwrap_or_default() as u8,
                            utf16.next().unwrap_or_default() as u8,
                            utf16.next().unwrap_or_default() as u8,
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
        } = ZoneInfo64Raw::deserialize(deserializer)?;

        if zones.len() != names.len() || names.len() != regions.len() {
            return Err(D::Error::custom(
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
                        return Err(D::Error::custom("invalid alias idx"));
                    };
                    if let TzZoneRaw::Int(_) = alias {
                        return Err(D::Error::custom("multi-step alias"));
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

                    if trans.len() + trans_post32.len() + trans_pre32.len() != type_map.len()
                        || type_offsets.is_empty()
                    {
                        return Err(D::Error::custom("inconsistent offset data"));
                    }

                    for &idx in type_map {
                        if idx as usize >= type_offsets.len() {
                            return Err(D::Error::custom("invalid offset map"));
                        }
                    }

                    let final_rule_offset_year = match (final_rule, final_raw, final_year) {
                        (Some(name), Some(offset), Some(year)) => {
                            let Some(idx) = rules_lookup(name) else {
                                return Err(D::Error::custom("invalid rule id"));
                            };
                            Some((idx as u32, offset, year))
                        }
                        (None, None, None) => None,
                        _ => {
                            return Err(D::Error::custom(
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
}

impl<'a> ZoneInfo64<'a> {
    fn get(&'a self, iana: &str) -> Option<(Transitions<'a>, Region)> {
        let idx = self
            .names
            .binary_search_by(|&n| n.chars().cmp(iana.chars()))
            .ok()?;

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

        let &TzZoneData {
            type_offsets,
            trans,
            trans_pre32,
            trans_post32,
            type_map,
            final_rule_offset_year,
            ..
        } = zone.as_ref();

        #[expect(clippy::indexing_slicing)] // rules indices are all valid
        let final_rule = final_rule_offset_year
            .map(|(idx, offset, year)| (&self.rules[idx as usize], offset, year))
            .map(|(inner, offset_seconds, start_year)| Rule {
                start_year,
                standard_offset_seconds: offset_seconds,
                inner,
            });

        Some((
            Transitions {
                trans_pre32,
                trans,
                trans_post32,
                type_map,
                type_offsets,
                final_rule,
            },
            region,
        ))
    }
}

#[derive(Debug)]
pub struct Transitions<'a> {
    trans_pre32: &'a [(i32, i32)],
    trans: &'a [i32],
    trans_post32: &'a [(i32, i32)],
    type_map: &'a [u8],
    type_offsets: &'a [(i32, i32)],
    final_rule: Option<Rule<'a>>,
}

#[derive(Debug)]
struct Rule<'a> {
    start_year: u32,
    standard_offset_seconds: i32,
    inner: &'a TzRule,
}

impl Rule<'_> {
    fn previous_transition(&self, seconds_since_epoch: i64) -> Transition {
        // TODO
        let rule_applies = true;
        let transition = seconds_since_epoch;

        let _ = self.start_year;

        let _ = self.inner.start.month;
        let _ = self.inner.start.day;
        let _ = self.inner.start.day_of_week;
        let _ = self.inner.start.millis_of_day;
        let _ = self.inner.start.time_mode;
        let _ = self.inner.start.mode;

        let _ = self.inner.end.month;
        let _ = self.inner.end.day;
        let _ = self.inner.end.day_of_week;
        let _ = self.inner.end.millis_of_day;
        let _ = self.inner.end.time_mode;
        let _ = self.inner.end.mode;

        Transition {
            transition,
            rule_applies,
            offset: UtcOffset::from_seconds_unchecked(
                self.standard_offset_seconds
                    + if rule_applies {
                        self.inner.additional_offset_secs
                    } else {
                        0
                    },
            ),
        }
    }
}

#[derive(Debug)]
pub struct Transition {
    pub transition: i64,
    pub offset: UtcOffset,
    pub rule_applies: bool,
}

impl Transitions<'_> {
    fn previous_transition(&self, seconds_since_epoch: i64) -> Transition {
        let idx = if seconds_since_epoch < i32::MIN as i64 {
            self.trans_pre32
                .binary_search(&(
                    (seconds_since_epoch >> 32) as i32,
                    (seconds_since_epoch & 0xFFFFFFFF) as i32,
                ))
                .map(|i| i as isize)
                // binary_search returns the index of the next (higher) transition, so we subtract one
                .unwrap_or_else(|i| i as isize - 1)
        } else if seconds_since_epoch <= i32::MAX as i64 {
            self.trans_pre32.len() as isize
                + self
                    .trans
                    .binary_search(&(seconds_since_epoch as i32))
                    .map(|i| i as isize)
                    // binary_search returns the index of the next (higher) transition, so we subtract one
                    .unwrap_or_else(|i| i as isize - 1)
        } else {
            self.trans_pre32.len() as isize
                + self.trans.len() as isize
                + self
                    .trans_post32
                    .binary_search(&(
                        (seconds_since_epoch >> 32) as i32,
                        (seconds_since_epoch & 0xFFFFFFFF) as i32,
                    ))
                    .map(|i| i as isize)
                    // binary_search returns the index of the next (higher) transition, so we subtract one
                    .unwrap_or_else(|i| i as isize - 1)
        };

        // before first transition don't use `type_map`, just the first entry in `type_offsets`
        if idx == -1 {
            #[expect(clippy::unwrap_used)] // type_offsets non-empty by invariant
            let &(standard, rule_additional) = self.type_offsets.first().unwrap();
            return Transition {
                transition: i64::MIN,
                offset: UtcOffset::from_seconds_unchecked(standard + rule_additional),
                rule_applies: rule_additional > 0,
            };
        }

        let idx = idx as usize;

        // after the last transition, respect the rule
        if idx == self.type_map.len() - 1 {
            if let Some(rule) = self.final_rule.as_ref() {
                return rule.previous_transition(seconds_since_epoch);
            }
        }

        #[expect(clippy::indexing_slicing)]
        // type_map has length sum(trans*), and type_map values are validated to be valid indices in type_offsets
        let (standard, rule_additional) = self.type_offsets[self.type_map[idx] as usize];

        #[expect(clippy::indexing_slicing)] // by guards or invariant
        let transition = if idx < self.trans_pre32.len() {
            let (lo, hi) = self.trans_pre32[idx];
            (lo as i64) << 32 | hi as i64
        } else if idx - self.trans_pre32.len() < self.trans.len() {
            self.trans[idx - self.trans_pre32.len()] as i64
        } else {
            let (lo, hi) = self.trans_pre32[idx - self.trans_pre32.len() - self.trans.len()];
            (lo as i64) << 32 | hi as i64
        };

        Transition {
            transition,
            offset: UtcOffset::from_seconds_unchecked(standard + rule_additional),
            rule_applies: rule_additional > 0,
        }
    }
}

fn main() {
    let in_bytes = include_bytes_as_u32!("data/zoneinfo64.res");

    let tzdb = resb::binary::from_words::<ZoneInfo64>(in_bytes)
        .expect("Error processing resource bundle file");

    let id = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "Europe/Zurich".into());
    let seconds_since_epoch = std::env::args()
        .nth(2)
        .unwrap_or_default()
        .parse()
        .unwrap_or_default();

    let (zone, region) = tzdb.get(&id).unwrap();
    let transition = zone.previous_transition(seconds_since_epoch);

    let chrono = chrono::offset::TimeZone::offset_from_utc_datetime(
        &<chrono_tz::Tz as core::str::FromStr>::from_str(&id).unwrap(),
        &chrono::DateTime::from_timestamp(seconds_since_epoch, 0)
            .unwrap()
            .naive_utc(),
    );

    assert_eq!(
        transition.offset.to_seconds(),
        (chrono_tz::OffsetComponents::base_utc_offset(&chrono)
            + chrono_tz::OffsetComponents::dst_offset(&chrono))
        .num_seconds() as i32
    );
    assert_eq!(
        transition.rule_applies,
        !chrono_tz::OffsetComponents::dst_offset(&chrono).is_zero()
    );

    println!(
        "Zone {id:?} ({region}) at {time:?}: {offset:?} ({is_dst}) since {transition:?}",
        time = chrono::DateTime::from_timestamp(seconds_since_epoch, 0).unwrap(),
        region = region.as_str(),
        offset = transition.offset,
        is_dst = if transition.rule_applies {
            "Daylight"
        } else {
            "Standard"
        },
        transition = chrono::DateTime::from_timestamp(transition.transition, 0).unwrap(),
    );
}
