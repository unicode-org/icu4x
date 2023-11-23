// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::ops::DerefMut;

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
        if part.starts_with("square-") {
            return (2, &part[7..]);
        } else if part.starts_with("cubic-") {
            return (3, &part[6..]);
        } else {
            return (1, part);
        }
    }

    fn get_si_prefix_base_10(part: &str) -> (i8, &str) {
        if part.starts_with("kilo") {
            return (3, &part[4..]);
        } else if part.starts_with("hecto") {
            return (2, &part[5..]);
        } else if part.starts_with("deca") {
            return (1, &part[4..]);
        } else if part.starts_with("deci") {
            return (-1, &part[4..]);
        } else if part.starts_with("centi") {
            return (-2, &part[5..]);
        } else if part.starts_with("milli") {
            return (-3, &part[5..]);
        } else if part.starts_with("micro") {
            return (-6, &part[5..]);
        } else if part.starts_with("nano") {
            return (-9, &part[4..]);
        } else if part.starts_with("pico") {
            return (-12, &part[4..]);
        } else if part.starts_with("femto") {
            return (-15, &part[5..]);
        } else if part.starts_with("atto") {
            return (-18, &part[4..]);
        } else if part.starts_with("zepto") {
            return (-21, &part[5..]);
        } else if part.starts_with("yocto") {
            return (-24, &part[5..]);
        } else {
            return (0, part);
        }
    }

    fn get_si_prefix_base_two(part: &str) -> (i8, &str) {
        if part.starts_with("kibi") {
            return (10, &part[4..]);
        } else if part.starts_with("mebi") {
            return (20, &part[4..]);
        } else if part.starts_with("gibi") {
            return (30, &part[4..]);
        } else if part.starts_with("tebi") {
            return (40, &part[4..]);
        } else if part.starts_with("pebi") {
            return (50, &part[4..]);
        } else if part.starts_with("exbi") {
            return (60, &part[4..]);
        } else if part.starts_with("zebi") {
            return (70, &part[4..]);
        } else if part.starts_with("yobi") {
            return (80, &part[4..]);
        } else {
            return (0, part);
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
                return Some((value as usize, &part[i..]));
            }
        }

        None
    }

    fn analyze_identifier_part<'data>(
        part: &str,
        sign: i8,
        trie: &ZeroTrie<ZeroVec<'data, u8>>,
    ) -> Option<Vec<MeasureUnitItem>> {
        let mut result = Vec::<MeasureUnitItem>::new();
        while !part.is_empty() {
            let (power, part) = Self::get_power(part);
            let (si_prefix, base, part) = {
                let (si_prefix_base_10, part) = Self::get_si_prefix_base_10(part);
                if si_prefix_base_10 != 0 {
                    (si_prefix_base_10, 10, part)
                } else {
                    let (si_prefix_base_2, part) = Self::get_si_prefix_base_two(part);
                    (si_prefix_base_2, 2, part)
                }
            };
            let (unit_id, part) = match Self::get_unit_id(part, trie) {
                Some((id, remaining_part)) => (id, remaining_part),
                None => return None,
            };

            let base = if base == 2 {
                Base::Binary
            } else {
                Base::Decimal
            };

            result.push(MeasureUnitItem {
                power: power as i8 * sign,
                si_base: base,
                unit_id: unit_id as u16,
            });
        }

        Some(result)
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
