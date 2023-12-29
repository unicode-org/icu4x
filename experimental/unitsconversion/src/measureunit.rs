// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use smallvec::SmallVec;
use zerotrie::{ZeroTrie, ZeroTrieSimpleAscii, ZeroTrieSimpleAsciiCursor};
use zerovec::ZeroVec;

use crate::{
    power::get_power,
    provider::{Base, MeasureUnitItem, SiPrefix},
    si_prefix::get_si_prefix,
    ConversionError,
};

// TODO: add test cases for this parser after adding UnitsTest.txt to the test data.
/// A parser for the CLDR unit identifier (e.g. `meter-per-square-second`)
pub struct MeasureUnitParser<'data> {
    /// Contains the payload.
    payload: &'data ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,
}

impl<'data> MeasureUnitParser<'data> {
    // TODO: revisit the public nature of the API. Maybe we should make it private and add a function to create it from a ConverterFactory.
    /// Creates a new MeasureUnitParser from a ZeroTrie payload.
    #[cfg(feature = "datagen")]
    pub fn from_payload(payload: &'data ZeroTrieSimpleAscii<ZeroVec<u8>>) -> Self {
        Self { payload }
    }

    /// Get the unit id.
    /// NOTE:
    ///    if the unit id is found, the function will return (unit id, part without the unit id and without `-` at the beginning of the remaining part if it exists).
    ///    if the unit id is not found, the function will return an error.
    fn get_unit_id<'a>(&'a self, part: &'a str) -> Result<(u16, &str), ConversionError> {
        let mut cursor = self.payload.cursor();
        let mut longest_match = Err(ConversionError::InvalidUnit);

        for (i, byte) in part.bytes().enumerate() {
            cursor.step(byte);
            if cursor.is_empty() {
                break;
            }
            if let Some(value) = cursor.take_value() {
                longest_match = Ok((value as u16, &part[i + 1..]));
            }
        }
        longest_match
    }

    fn get_power<'a>(&'a self, part: &'a str) -> Result<(u8, &str), ConversionError> {
        let (power, part_without_power) = get_power(part);
        if part_without_power.len() == part.len() {
            return Ok((power, part));
        }

        if part_without_power.starts_with("-") {
            return Ok((power, &part_without_power[1..]));
        }

        Err(ConversionError::InvalidUnit)
    }

    fn get_si_prefix<'a>(&'a self, part: &'a str) -> (SiPrefix, &str) {
        let (si_prefix, part_without_si_prefix) = get_si_prefix(part);
        if part_without_si_prefix.len() == part.len() {
            return (si_prefix, part);
        }

        if part_without_si_prefix.starts_with("-") {
            return (si_prefix, &part_without_si_prefix[1..]);
        }

        (si_prefix, part_without_si_prefix)
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
        let mut identifier_part = identifier_part;
        while !identifier_part.is_empty() {
            let (power, identifier_part_without_power) = self.get_power(identifier_part)?;
            let (si_prefix, identifier_part_without_si_prefix) =
                self.get_si_prefix(identifier_part_without_power);
            let (unit_id, identifier_part_without_unit_id) =
                self.get_unit_id(identifier_part_without_si_prefix)?;

            result.push(MeasureUnitItem {
                power: sign * power as i8,
                si_prefix,
                unit_id,
            });

            identifier_part = match identifier_part_without_unit_id.len() {
                0 => identifier_part_without_unit_id,
                _ if identifier_part_without_unit_id.starts_with('-') => {
                    &identifier_part_without_unit_id[1..]
                }
                _ => return Err(ConversionError::InvalidUnit),
            };
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

// TODO NOTE: the MeasureUnitParser takes the trie and the ConverterFactory takes the full payload and an instance of MeasureUnitParser.
pub struct MeasureUnit {
    /// Contains the processed units.
    pub contained_units: SmallVec<[MeasureUnitItem; 8]>,
}
