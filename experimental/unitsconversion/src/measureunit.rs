// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use zerotrie::ZeroTrie;
use zerovec::ZeroVec;

use crate::{
    provider::{Base, MeasureUnitItem, SiPrefix},
    ConversionError,
};

// TODO(#4369): split this struct to two structs: MeasureUnitParser for parsing the identifier and MeasureUnit to represent the unit.
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
    fn get_power(part: &str) -> Option<i8> {
        match part {
            "square" | "pow2" => Some(2),
            "cubic" | "pow3" => Some(3),
            "pow4" => Some(4),
            "pow5" => Some(5),
            "pow6" => Some(6),
            "pow7" => Some(7),
            "pow8" => Some(8),
            "pow9" => Some(9),
            _ => None,
        }
    }

    // TODO: complete all the cases for the prefixes.
    // TODO: consider using a trie for the prefixes.
    /// Get the SI prefix.
    /// NOTE:
    ///    if the prefix is found, the function will return (power, base, part without the prefix).
    ///    if the prefix is not found, the function will return (0, Base::NotExist, part).
    fn get_si_prefix(part: &str) -> (SiPrefix, &str) {
        let (si_prefix_base_10, part) = Self::get_si_prefix_base_10(part);
        if si_prefix_base_10 != 0 {
            return (
                SiPrefix {
                    power: si_prefix_base_10,
                    base: Base::Decimal,
                },
                part,
            );
        }

        let (si_prefix_base_2, part) = Self::get_si_prefix_base_two(part);
        if si_prefix_base_2 != 0 {
            return (
                SiPrefix {
                    power: si_prefix_base_2,
                    base: Base::Binary,
                },
                part,
            );
        }

        (
            SiPrefix {
                power: 0,
                base: Base::Decimal,
            },
            part,
        )
    }

    // TODO: consider returning Option<(i8, &str)> instead of (0, part) for the case when the prefix is not found.
    // TODO: consider using a trie for the prefixes.
    // TODO: complete all the cases for the prefixes.
    /// Get the SI prefix for base 10.
    /// NOTE:
    ///    if the prefix is found, the function will return (power, part without the prefix).
    ///   if the prefix is not found, the function will return (0, part).
    fn get_si_prefix_base_10(part: &str) -> (i8, &str) {
        if let Some(part) = part.strip_prefix("quetta") {
            (30, part)
        } else if let Some(part) = part.strip_prefix("ronna") {
            (27, part)
        } else if let Some(part) = part.strip_prefix("yotta") {
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

    /// Get the unit id.
    /// NOTE:
    ///    if the unit id is found, the function will return (unit id, part without the unit id and without `-` at the beginning of the remaining part if it exists).
    ///    if the unit id is not found, the function will return None.
    fn get_unit_id<'data>(part: &'data str, trie: &ZeroTrie<ZeroVec<'data, u8>>) -> Option<usize> {
        if let Some(unit_id) = trie.get(part.as_bytes()) {
            Some(unit_id)
        } else {
            None
        }
    }

    /// Process a part of an identifier.
    /// For example, if the whole identifier is: "square-kilometer-per-second",
    /// this function will be called for "square-kilometer" with sign (1) and "second" with sign (-1).
    fn analyze_identifier_part(
        identifier_part: &str,
        sign: i8,
        result: &mut Vec<MeasureUnitItem>,
        trie: &ZeroTrie<ZeroVec<'_, u8>>,
    ) -> Result<(), ConversionError> {
        if identifier_part.is_empty() {
            return Ok(());
        }
        let mut identifier_split = identifier_part.split('-');
        loop {
            let mut part = match identifier_split.next() {
                Some(part) => part,
                None => break,
            };

            let mut power = Self::get_power(part);
            if power.is_none() {
                power = Some(1);
            } else {
                power = Some(power.unwrap());
                part = match identifier_split.next() {
                    Some(part) => part,
                    None => return Err(ConversionError::InvalidUnit),
                };
            }

            let (si_prefix, identifier_after_si) = Self::get_si_prefix(part);
            let unit_id = match Self::get_unit_id(identifier_after_si, trie) {
                Some(unit_id) => unit_id,
                None => return Err(ConversionError::InvalidUnit),
            };

            result.push(MeasureUnitItem {
                power: sign * power.unwrap(),
                si_prefix,
                unit_id: unit_id as u16,
            });
        }

        Ok(())
    }

    // TODO: add test cases for this function.
    /// Process an identifier.
    pub fn try_from_identifier<'data>(
        identifier: &'data str,
        trie: &ZeroTrie<ZeroVec<'data, u8>>,
    ) -> Result<Vec<MeasureUnitItem>, ConversionError> {
        let (num_part, den_part) = identifier
            .split_once("-per-")
            .or_else(|| identifier.split_once("per-"))
            .unwrap_or((identifier, ""));

        let mut measure_unit_items = Vec::<MeasureUnitItem>::new();

        Self::analyze_identifier_part(num_part, 1, &mut measure_unit_items, trie)?;
        Self::analyze_identifier_part(den_part, -1, &mut measure_unit_items, trie)?;
        Ok(measure_unit_items)
    }
}
