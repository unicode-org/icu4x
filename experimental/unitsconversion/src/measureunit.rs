// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerotrie::ZeroTrie;
use zerovec::ZeroVec;

use crate::provider::{Base, MeasureUnitItem};

#[zerovec::make_varule(MeasureUnitULE)]
#[derive(Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(
    feature = "datagen",
    derive(databake::Bake),
    databake(path = icu_unitsconversion::provider),
)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize),
    zerovec::derive(Serialize)
)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    zerovec::derive(Deserialize)
)]
#[zerovec::derive(Debug)]
pub struct MeasureUnit<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub contained_units: ZeroVec<'data, MeasureUnitItem>,
}

impl MeasureUnit<'_> {
    fn get_power(part: &str) -> (u8, &str) {
        if let Some(part) = part.strip_prefix("square-") {
            (2, part)
        } else if let Some(part) = part.strip_prefix("cubic-") {
            (3, part)
        } else if let Some(part) = part.strip_prefix("pow4-") {
            (4, part)
        } else {
            (1, part)
        }
    }

    fn get_si_prefix(part: &str) -> (i8, Base, &str) {
        let (si_prefix_base_10, part) = Self::get_si_prefix_base_10(part);
        if si_prefix_base_10 != 0 {
            return (si_prefix_base_10, Base::Decimal, part);
        }

        let (si_prefix_base_2, part) = Self::get_si_prefix_base_two(part);
        if si_prefix_base_2 != 0 {
            return (si_prefix_base_2, Base::Binary, part);
        }

        (0, Base::NotExist, part)
    }

    fn get_si_prefix_base_10(part: &str) -> (i8, &str) {
        if let Some(part) = part.strip_prefix("kilo") {
            (3, part)
        } else if let Some(part) = part.strip_prefix("hecto") {
            (2, part)
        } else if let Some(part) = part.strip_prefix("deca") {
            (1, part)
        } else if let Some(part) = part.strip_prefix("deci") {
            (-1, part)
        } else if let Some(part) = part.strip_prefix("centi") {
            (-2, part)
        } else if let Some(part) = part.strip_prefix("milli") {
            (-3, part)
        } else if let Some(part) = part.strip_prefix("micro") {
            (-6, part)
        } else if let Some(part) = part.strip_prefix("nano") {
            (-9, part)
        } else if let Some(part) = part.strip_prefix("pico") {
            (-12, part)
        } else if let Some(part) = part.strip_prefix("femto") {
            (-15, part)
        } else if let Some(part) = part.strip_prefix("atto") {
            (-18, part)
        } else if let Some(part) = part.strip_prefix("zepto") {
            (-21, part)
        } else if let Some(part) = part.strip_prefix("yocto") {
            (-24, part)
        } else {
            (0, part)
        }
    }

    fn get_si_prefix_base_two(part: &str) -> (i8, &str) {
        if let Some(part) = part.strip_prefix("kibi") {
            (10, part)
        } else if let Some(part) = part.strip_prefix("mebi") {
            (20, part)
        } else if let Some(part) = part.strip_prefix("gibi") {
            (30, part)
        } else if let Some(part) = part.strip_prefix("tebi") {
            (40, part)
        } else if let Some(part) = part.strip_prefix("pebi") {
            (50, part)
        } else if let Some(part) = part.strip_prefix("exbi") {
            (60, part)
        } else if let Some(part) = part.strip_prefix("zebi") {
            (70, part)
        } else if let Some(part) = part.strip_prefix("yobi") {
            (80, part)
        } else {
            (0, part)
        }
    }

    fn get_unit_id<'data>(
        part: &'data str,
        trie: &ZeroTrie<ZeroVec<'data, u8>>,
    ) -> Option<(usize, &'data str)> {
        // TODO: this is inefficient way to search for an item in a trie.
        // we must implement a way to search for a prefix in a trie.
        for i in 1..part.len() + 1 {
            let sub_part = &part[..i];
            if let Some(value) = trie.get(sub_part.as_bytes()) {
                return Some((value, &part[i..]));
            }
        }

        None
    }

    fn analyze_identifier_part(
        identifier: &str,
        sign: i8,
        trie: &ZeroTrie<ZeroVec<'_, u8>>,
    ) -> Option<Vec<MeasureUnitItem>> {
        let mut identifier = identifier;
        let mut measure_unit_items = Vec::<MeasureUnitItem>::new();
        while !identifier.is_empty() {
            let (power, identifier_power) = Self::get_power(identifier);
            let (si_prefix, base, identifier_si) = Self::get_si_prefix(identifier_power);
            let (unit_id, identifier_unit) = Self::get_unit_id(identifier_si, trie)?;

            measure_unit_items.push(MeasureUnitItem {
                power: power as i8 * sign,
                si_base: base,
                si_prefix: si_prefix * sign,
                unit_id: unit_id as u16,
            });

            identifier = identifier_unit.get(1..).unwrap_or("");
        }

        Some(measure_unit_items)
    }

    pub fn try_from_identifier<'data>(
        identifier: &'data str,
        trie: &ZeroTrie<ZeroVec<'data, u8>>,
    ) -> Option<Vec<MeasureUnitItem>> {
        let mut per_split = if identifier.contains("-per-") {
            identifier.split("-per-")
        } else {
            identifier.split("per-")
        };

        let num_part = per_split.next().unwrap_or("");
        let den_part = per_split.next().unwrap_or("");
        if per_split.next().is_some() {
            return None;
        }

        let measure_unit_items = [
            Self::analyze_identifier_part(num_part, 1, trie)?,
            Self::analyze_identifier_part(den_part, -1, trie)?,
        ]
        .concat();
        Some(measure_unit_items)
    }
}
