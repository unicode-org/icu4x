// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use smallvec::SmallVec;
use zerotrie::ZeroTrie;
use zerovec::ZeroVec;

use crate::{
    power::get_power,
    provider::{Base, MeasureUnitItem, SiPrefix},
    si_prefix::{get_si_prefix_base_ten, get_si_prefix_base_two},
    ConversionError,
};

/// A parser for the CLDR unit identifier (e.g. `meter-per-square-second`)
pub struct MeasureUnitParser<'data> {
    /// Contains the zero-trie payload.
    zerotrie_payload: &'data ZeroTrie<ZeroVec<'data, u8>>,
}

impl<'data> MeasureUnitParser<'data> {
    /// Creates a new MeasureUnitParser from a ZeroTrie payload.
    pub fn new(zerotrie_payload: &'data ZeroTrie<ZeroVec<'data, u8>>) -> Self {
        Self { zerotrie_payload }
    }

    // TODO: complete all the cases for the prefixes.
    // TODO: consider using a trie for the prefixes.
    /// Extracts the SI prefix.
    /// NOTE:
    ///    if the prefix is found, the function will return (SiPrefix, part without the prefix string).
    ///    if the prefix is not found, the function will return (SiPrefix { power: 0, base: Base::Decimal }, part).
    fn get_si_prefix(part: &str) -> (SiPrefix, &str) {
        let (si_prefix_base_10, part) = get_si_prefix_base_ten(part);
        if si_prefix_base_10 != 0 {
            return (
                SiPrefix {
                    power: si_prefix_base_10,
                    base: Base::Decimal,
                },
                part,
            );
        }

        let (si_prefix_base_2, part) = get_si_prefix_base_two(part);
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

    /// Get the unit id.
    /// NOTE:
    ///    if the unit id is found, the function will return (unit id, part without the unit id and without `-` at the beginning of the remaining part if it exists).
    ///    if the unit id is not found, the function will return None.
    fn get_unit_id(&self, part: &'data str) -> Option<usize> {
        self.zerotrie_payload.get(part.as_bytes())
    }

    /// Process a part of an identifier.
    /// For example, if the whole identifier is: "square-kilometer-per-second",
    /// this function will be called for "square-kilometer" with sign (1) and "second" with sign (-1).
    fn analyze_identifier_part(
        &self,
        identifier_part: &str,
        sign: i8,
        result: &mut Vec<MeasureUnitItem>,
    ) -> Result<(), ConversionError> {
        if identifier_part.is_empty() {
            return Ok(());
        }
        let mut identifier_split = identifier_part.split('-');
        while let Some(mut part) = identifier_split.next() {
            let power = match get_power(part) {
                Some(power) => {
                    part = identifier_split
                        .next()
                        .ok_or(ConversionError::InvalidUnit)?;
                    power
                }
                None => 1,
            };

            let (si_prefix, identifier_after_si) = Self::get_si_prefix(part);
            let unit_id = self
                .get_unit_id(identifier_after_si)
                .ok_or(ConversionError::InvalidUnit)?;

            result.push(MeasureUnitItem {
                power: sign * power,
                si_prefix,
                unit_id: unit_id as u16,
            });
        }

        Ok(())
    }

    // TODO: add test cases for this function.
    /// Process an identifier.
    pub fn try_from_identifier(
        &self,
        identifier: &'data str,
    ) -> Result<Vec<MeasureUnitItem>, ConversionError> {
        if identifier.starts_with('-') {
            return Err(ConversionError::InvalidUnit);
        }

        let (num_part, den_part) = identifier
            .split_once("per-")
            .map(|(num_part, den_part)| (num_part.strip_suffix('-').unwrap_or(num_part), den_part))
            .unwrap_or((identifier, ""));

        let mut measure_unit_items = Vec::<MeasureUnitItem>::new();

        self.analyze_identifier_part(num_part, 1, &mut measure_unit_items)?;
        self.analyze_identifier_part(den_part, -1, &mut measure_unit_items)?;
        Ok(measure_unit_items)
    }
}

// TODO(#4369): split this struct to two structs: MeasureUnitParser for parsing the identifier and MeasureUnit to represent the unit.
// TODO NOTE: the MeasureUnitParser takes the trie and the ConverterFactory takes the full payload and an instance of MeasureUnitParser.
pub struct MeasureUnit {
    /// Contains the processed units.
    pub contained_units: SmallVec<[MeasureUnitItem; 8]>,
}
