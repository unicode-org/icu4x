// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerotrie::ZeroTrie;
use zerovec::ZeroVec;

use crate::provider::{Base, MeasureUnitItem, SiPrefix, Sign};

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
    /// Contains the processed units.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub contained_units: ZeroVec<'data, MeasureUnitItem>,
}

impl MeasureUnit<'_> {
    // TODO: consider returning Option<(u8, &str)> instead of (1, part) for the case when the power is not found.
    // TODO: complete all the cases for the powers.
    // TODO: consider using a trie for the powers.
    /// Get the power of the unit.
    /// NOTE:
    ///    if the power is found, the function will return (power, part without the power).
    ///    if the power is not found, the function will return (1, part).
    fn get_power(part: &str) -> (u8, &str) {
        if let Some(part) = part.strip_prefix("square-") {
            (2, part)
        } else if let Some(part) = part.strip_prefix("pow2-") {
            (2, part)
        } else if let Some(part) = part.strip_prefix("cubic-") {
            (3, part)
        } else if let Some(part) = part.strip_prefix("pow3-") {
            (3, part)
        } else if let Some(part) = part.strip_prefix("pow4-") {
            (4, part)
        } else if let Some(part) = part.strip_prefix("pow5-") {
            (5, part)
        } else if let Some(part) = part.strip_prefix("pow6-") {
            (6, part)
        } else if let Some(part) = part.strip_prefix("pow7-") {
            (7, part)
        } else if let Some(part) = part.strip_prefix("pow8-") {
            (8, part)
        } else if let Some(part) = part.strip_prefix("pow9-") {
            (9, part)
        } else {
            (1, part)
        }
    }

    // TODO: complete all the cases for the prefixes.
    // TODO: consider using a trie for the prefixes.
    /// Get the SI prefix.
    /// NOTE:
    ///    if the prefix is found, the function will return (power, base, part without the prefix).
    ///    if the prefix is not found, the function will return (0, Base::NotExist, part).
    fn get_si_prefix(part: &str) -> (Option<SiPrefix>, &str) {
        let (si_prefix_base_10, part) = Self::get_si_prefix_base_10(part);
        if si_prefix_base_10 != 0 {
            return (
                Some(SiPrefix {
                    power: si_prefix_base_10.unsigned_abs(),
                    base: Base::Decimal,
                    sign: if si_prefix_base_10 >= 0 {
                        Sign::Positive
                    } else {
                        Sign::Negative
                    },
                }),
                part,
            );
        }

        let (si_prefix_base_2, part) = Self::get_si_prefix_base_two(part);
        if si_prefix_base_2 != 0 {
            return (
                Some(SiPrefix {
                    power: si_prefix_base_2.unsigned_abs(),
                    base: Base::Binary,
                    sign: if si_prefix_base_2 >= 0 {
                        Sign::Positive
                    } else {
                        Sign::Negative
                    },
                }),
                part,
            );
        }

        (None, part)
    }

    // TODO: consider returning Option<(i8, &str)> instead of (0, part) for the case when the prefix is not found.
    // TODO: consider using a trie for the prefixes.
    // TODO: complete all the cases for the prefixes.
    /// Get the SI prefix for base 10.
    /// NOTE:
    ///    if the prefix is found, the function will return (power, part without the prefix).
    ///   if the prefix is not found, the function will return (0, part).
    fn get_si_prefix_base_10(part: &str) -> (i8, &str) {
        if let Some(part) = part.strip_prefix("yotta") {
            (24, part)
        } else if let Some(part) = part.strip_prefix("zetta") {
            (21, part)
        } else if let Some(part) = part.strip_prefix("exa") {
            (18, part)
        } else if let Some(part) = part.strip_prefix("peta") {
            (15, part)
        } else if let Some(part) = part.strip_prefix("tera") {
            (12, part)
        } else if let Some(part) = part.strip_prefix("giga") {
            (9, part)
        } else if let Some(part) = part.strip_prefix("mega") {
            (6, part)
        } else if let Some(part) = part.strip_prefix("kilo") {
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
        } else if let Some(part) = part.strip_prefix("ronto") {
            (-27, part)
        } else if let Some(part) = part.strip_prefix("quecto") {
            (-30, part)
        } else {
            (0, part)
        }
    }

    // TODO: consider returning Option<(i8, &str)> instead of (0, part) for the case when the prefix is not found.
    // TODO: consider using a trie for the prefixes.
    // TODO: complete all the cases for the prefixes.
    /// Get the SI prefix for base 2.
    /// NOTE:
    ///     if the prefix is found, the function will return (power, part without the prefix).
    ///     if the prefix is not found, the function will return (0, part).
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

    // TODO: consider using a sufficient trie search for finding the unit id.
    /// Get the unit id.
    /// NOTE:
    ///    if the unit id is found, the function will return (unit id, part without the unit id and without `-` at the beginning of the remaining part if it exists).
    ///    if the unit id is not found, the function will return None.
    fn get_unit_id<'data>(
        part: &'data str,
        trie: &ZeroTrie<ZeroVec<'data, u8>>,
    ) -> Option<(usize, &'data str)> {
        // TODO: this is inefficient way to search for an item in a trie.
        // we must implement a way to search for a prefix in a trie.
        for (index, _) in part.char_indices() {
            let identifier = &part[..=index];
            if let Some(value) = trie.get(identifier.as_bytes()) {
                return Some((value, &part[identifier.len()..]));
            }
        }

        None
    }

    /// Process a part of an identifier.
    /// For example, if the whole identifier is: "square-kilometer-per-second",
    /// this function will be called for "square-kilometer" with sign (1) and "second" with sign (-1).
    fn analyze_identifier_part(
        identifier_part: &str,
        sign: i8,
        trie: &ZeroTrie<ZeroVec<'_, u8>>,
    ) -> Option<Vec<MeasureUnitItem>> {
        let mut identifier = identifier_part;
        let mut measure_unit_items = Vec::<MeasureUnitItem>::new();
        while !identifier.is_empty() {
            let (power, identifier_power) = Self::get_power(identifier);
            let (si_prefix, identifier_si) = Self::get_si_prefix(identifier_power);
            let (unit_id, identifier_unit) = Self::get_unit_id(identifier_si, trie)?;

            measure_unit_items.push(MeasureUnitItem {
                power: power as i8 * sign,
                si_prefix,
                unit_id: unit_id as u16,
            });

            identifier = identifier_unit.get(1..).unwrap_or("");
        }

        Some(measure_unit_items)
    }

    // TODO: add test cases for this function.
    /// Process an identifier.
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
