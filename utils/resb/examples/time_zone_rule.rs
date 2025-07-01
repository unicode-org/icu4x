// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::{collections::HashMap, fmt::Debug, marker::PhantomData};

use serde::{
    de::{self, Visitor},
    Deserialize, Serialize,
};

use icu::{locale::subtags::Region, time::zone::UtcOffset};

use zerovec::ZeroVec;

/// A zero-copy representation of a little-endian UTF-16 string.
///
/// Unlike `String`, the contents are not required to be valid UTF-16. Consumers
/// are expected to validate the contents or use `try_into::<String>()`. No zero
/// terminator is included.
#[derive(Deserialize, Serialize)]
#[serde(transparent)]
pub struct ZeroUTF16String<'a> {
    #[serde(borrow)]
    units: ZeroVec<'a, u16>,
}

impl ZeroUTF16String<'_> {
    /// Gets whether the UTF-16 string is empty.
    pub fn is_empty(&self) -> bool {
        self.units.is_empty()
    }

    /// Gets the count of units in the string.
    ///
    /// This value does not necessarily equal the length of the string in
    /// characters, as characters outside the Basic Multilingual Plane are
    /// represented by 2 units.
    pub fn len(&self) -> usize {
        self.units.len()
    }

    /// Gets an iterator for the units of the string.
    ///
    /// See `len` for details on why this does not correspond to characters.
    pub fn chars(&self) -> impl Iterator<Item = char> + '_ {
        char::decode_utf16(self.units.iter()).map(|c| c.unwrap_or(char::REPLACEMENT_CHARACTER))
    }
}

impl From<&'_ ZeroUTF16String<'_>> for String {
    fn from(value: &'_ ZeroUTF16String<'_>) -> Self {
        value.chars().collect::<String>()
    }
}

impl Debug for ZeroUTF16String<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let decoded = String::from(self);
        write!(f, "{decoded}")
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TzDataRuleData<'a> {
    #[serde(borrow)]
    // values are standard and special time
    type_offsets: ZeroVec<'a, (i32, i32)>,
    #[serde(borrow)]
    trans: Option<ZeroVec<'a, i32>>,
    #[serde(borrow)]
    // values are an i64 encoded as two i32s?
    trans_pre32: Option<ZeroVec<'a, (i32, i32)>>,
    #[serde(borrow)]
    // values are an i64 encoded as two i32s?
    trans_post32: Option<ZeroVec<'a, (i32, i32)>>,
    type_map: Option<&'a [u8]>,
    #[serde(borrow)]
    final_rule: Option<ZeroUTF16String<'a>>,
    final_raw: Option<i32>,
    final_year: Option<u32>,
    #[serde(borrow)]
    links: Option<ZeroVec<'a, u32>>,
}

#[derive(Debug)]
pub enum TzDataRule<'a> {
    // The rule data is boxed here due to the large size difference between the
    // `TzDataRuleData` struct and `u32`. It's not strictly necessary.
    Table(Box<TzDataRuleData<'a>>),
    Int(u32),
}

impl<'de: 'a, 'a> Deserialize<'de> for TzDataRule<'a> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(TzDataRuleEnumVisitor {
            phantom: PhantomData,
        })
    }
}

struct TzDataRuleEnumVisitor<'a> {
    phantom: PhantomData<TzDataRule<'a>>,
}

impl<'de: 'a, 'a> Visitor<'de> for TzDataRuleEnumVisitor<'a> {
    type Value = TzDataRule<'a>;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("an unsigned 32-bit integer or a table of rule data")
    }

    fn visit_u32<E>(self, v: u32) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(TzDataRule::Int(v))
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        let value = TzDataRuleData::deserialize(de::value::MapAccessDeserializer::new(map))?;

        Ok(TzDataRule::Table(Box::new(value)))
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "zoneinfo64")]
#[serde(rename_all = "PascalCase")]
struct ZoneInfo64<'a> {
    #[serde(borrow)]
    zones: Vec<TzDataRule<'a>>,
    #[serde(borrow)]
    names: Vec<ZeroUTF16String<'a>>,
    #[serde(borrow)]
    rules: HashMap<&'a str, ZeroVec<'a, i32>>,
    #[serde(borrow)]
    regions: Vec<ZeroUTF16String<'a>>,
}

#[derive(Debug)]
pub struct Zone<'a> {
    // values are an i64 encoded as two i32s?
    trans_pre32: ZeroVec<'a, (i32, i32)>,
    trans: ZeroVec<'a, i32>,
    // values are an i64 encoded as two i32s?
    trans_post32: ZeroVec<'a, (i32, i32)>,
    type_map: &'a [u8],
    // values are standard and relative dst offset
    type_offsets: ZeroVec<'a, (i32, i32)>,
    final_rule: Option<Rule>,
    pub region: Region,
}

#[derive(Debug)]
struct Rule {
    start_year: u32,

    std: i32,
    dst: i32,

    savings_start_month: i8,
    savings_start_day: i8,
    savings_start_day_of_week: i8,
    savings_start_time_seconds_of_day: i32,
    savings_start_time_mode: i8,

    savings_end_month: i8,
    savings_end_day: i8,
    savings_end_day_of_week: i8,
    savings_end_time_seconds_of_day: i32,
    savings_end_time_mode: i8,
}

impl Rule {
    fn previous_transition(&self, seconds_since_epoch: i64) -> Transition {
        // TODO
        let is_dst = true;
        let transition = seconds_since_epoch;

        let _ = self.start_year;

        let _ = self.savings_start_month;
        let _ = self.savings_start_day;
        let _ = self.savings_start_day_of_week;
        let _ = self.savings_start_time_seconds_of_day;
        let _ = self.savings_start_time_mode;

        let _ = self.savings_end_month;
        let _ = self.savings_end_day;
        let _ = self.savings_end_day_of_week;
        let _ = self.savings_end_time_seconds_of_day;
        let _ = self.savings_end_time_mode;

        Transition {
            transition,
            is_dst,
            offset: UtcOffset::from_seconds_unchecked(self.std + if is_dst { self.dst } else { 0 }),
        }
    }
}

#[derive(Debug)]
pub struct Transition {
    pub transition: i64,
    pub offset: UtcOffset,
    pub is_dst: bool,
}

impl Zone<'_> {
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
            let (std, dst) = self.type_offsets.get(0).unwrap();
            return Transition {
                transition: i64::MIN,
                offset: UtcOffset::from_seconds_unchecked(std + dst),
                is_dst: dst > 0,
            };
        }

        let idx = idx as usize;

        // after the last transition, respect the rule
        if idx == self.type_map.len() - 1 {
            if let Some(rule) = self.final_rule.as_ref() {
                return rule.previous_transition(seconds_since_epoch);
            }
        }

        let (std, dst) = self
            .type_offsets
            .get(*self.type_map.get(idx).unwrap() as usize)
            .unwrap();

        let transition = if idx < self.trans_pre32.len() {
            let (lo, hi) = self.trans_pre32.get(idx).unwrap();
            (lo as i64) << 32 | hi as i64
        } else if idx - self.trans_pre32.len() < self.trans.len() {
            self.trans.get(idx - self.trans_pre32.len()).unwrap() as i64
        } else {
            let (lo, hi) = self
                .trans_pre32
                .get(idx - self.trans_pre32.len() - self.trans.len())
                .unwrap();
            (lo as i64) << 32 | hi as i64
        };

        Transition {
            transition,
            offset: UtcOffset::from_seconds_unchecked(std + dst),
            is_dst: dst > 0,
        }
    }
}

impl<'a> ZoneInfo64<'a> {
    fn zone(&'a self, name: &str) -> Option<Zone<'a>> {
        let idx = self
            .names
            .binary_search_by(|n| n.chars().cmp(name.chars()))
            .ok()?;

        let mut zone = self.zones.get(idx)?; // invalid data
        if let &TzDataRule::Int(i) = zone {
            zone = self.zones.get(i as usize)?; // invalid data
        }
        let TzDataRule::Table(ref zone) = zone else {
            return None; // invalid data
        };

        let TzDataRuleData {
            ref type_offsets,
            ref trans,
            ref trans_pre32,
            ref trans_post32,
            ref type_map,
            ref final_rule,
            ref final_raw,
            ref final_year,
            ..
        } = zone.as_ref();

        let final_rule = final_rule
            .as_ref()
            .and_then(|r| self.rule(&String::from(r)))
            .zip(*final_raw)
            .zip(*final_year)
            .map(|((rule, raw), year)| Rule {
                start_year: year,

                std: raw,
                dst: rule.get(10).unwrap(),

                savings_start_month: rule.get(0).unwrap() as i8,
                savings_start_day: rule.get(1).unwrap() as i8,
                savings_start_day_of_week: rule.get(2).unwrap() as i8,
                savings_start_time_seconds_of_day: rule.get(3).unwrap(),
                savings_start_time_mode: rule.get(4).unwrap() as i8,

                savings_end_month: rule.get(5).unwrap() as i8,
                savings_end_day: rule.get(6).unwrap() as i8,
                savings_end_day_of_week: rule.get(7).unwrap() as i8,
                savings_end_time_seconds_of_day: rule.get(8).unwrap(),
                savings_end_time_mode: rule.get(9).unwrap() as i8,
            });

        let mut region = self.regions.get(idx)?.chars(); // invalid data
        let region = Region::try_from_raw([
            region.next().unwrap_or(0 as char) as u8,
            region.next().unwrap_or(0 as char) as u8,
            region.next().unwrap_or(0 as char) as u8,
        ])
        .unwrap();

        assert_eq!(
            trans.as_ref().map(|z| z.len()).unwrap_or_default()
                + trans_post32.as_ref().map(|z| z.len()).unwrap_or_default()
                + trans_pre32.as_ref().map(|z| z.len()).unwrap_or_default(),
            type_map.map(|z| z.len()).unwrap_or_default()
        );

        assert_eq!(type_offsets.len() % 2, 0);
        assert!(!type_offsets.is_empty());

        Some(Zone {
            trans_pre32: trans_pre32.clone().unwrap_or_default(),
            trans: trans.clone().unwrap_or_default(),
            trans_post32: trans_post32.clone().unwrap_or_default(),
            type_map: (*type_map).unwrap_or_default(),
            type_offsets: type_offsets.clone(),
            final_rule,
            region,
        })
    }

    pub fn rule(&'a self, name: &str) -> Option<&'a ZeroVec<'a, i32>> {
        self.rules.get(name)
    }
}

fn main() {
    let in_bytes = include_bytes!("data/zoneinfo64.res");

    let tzdb = resb::binary::from_bytes::<ZoneInfo64>(in_bytes)
        .expect("Error processing resource bundle file");

    let id = std::env::args().nth(1).unwrap_or_default();
    let seconds_since_epoch = std::env::args().nth(2).unwrap_or_default().parse().unwrap();

    let zone = tzdb.zone(&id).unwrap();

    let transition = zone.previous_transition(seconds_since_epoch);

    println!(
        "Zone {id:?} ({region}) at {time:?}: {offset:?} ({is_dst}) since {transition:?}",
        time = chrono::DateTime::from_timestamp(seconds_since_epoch, 0).unwrap(),
        region = zone.region.as_str(),
        offset = transition.offset,
        is_dst = if transition.is_dst {
            "Daylight"
        } else {
            "Standard"
        },
        transition = chrono::DateTime::from_timestamp(transition.transition, 0).unwrap(),
    );
}
