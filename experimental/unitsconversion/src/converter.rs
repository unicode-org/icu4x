// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: we do not want to use `std`
use std::collections::HashMap;

use num::BigRational;

use crate::{
    measureunit::{MeasureUnit, MeasureUnitParser},
    provider::{MeasureUnitItem, UnitsInfoV1},
    ConversionError,
};
use litemap::LiteMap;
use zerovec::ZeroSlice;

/// Represents the possible cases for the convertibility between two units.
pub enum Convertibility {
    /// The units are convertible.
    /// For example, `meter` and `foot` are convertible.
    Convertible,

    /// The units are reciprocal.
    /// For example, `meter-per-second` and `second-per-meter` are reciprocal.
    Reciprocal,

    /// The units are not convertible.
    /// For example, `meter` and `second` are not convertible.
    NotConvertible,
}


/// A converter for converting between two units.
/// For example, converting between `meter` and `foot`.
pub struct Converter {
    conversion_rate: BigRational,
    offset: BigRational,
    convertibility: Convertibility,
}


/// A factory for creating a converter.
/// Also, it can check the convertibility between two units.
pub struct ConverterFactory<'data> {
    /// Contains the necessary data for the conversion factory.
    payload: &'data UnitsInfoV1<'data>,
}



impl<'data> ConverterFactory<'data> {

    pub fn parser(&self) -> MeasureUnitParser<'_> {
        MeasureUnitParser::from_payload(&self.payload.units_conversion_trie)
    }

impl ConverterFactory<'_> {
    fn parser(&self) -> MeasureUnitParser<'_> {
        MeasureUnitParser::new(&self.payload.units_conversion_trie)
    }

    /// Extract the convertibility from the given units in the form of CLDR identifiers.
    pub fn extract_convertibility(
        &self,
        unit1: &MeasureUnit,
        unit2: &MeasureUnit,
    ) -> Result<Convertibility, ConversionError> {
        let unit1 = &unit1.contained_units;
        let unit2 = &unit2.contained_units;

        struct DetermineConvertibility {
            convertible: i16,
            reciprocal: i16,
        }

        /// Inserting the non basic units into the map.
        /// Thus means that from the given unis
        fn insert_non_basic_units(
            factory: &ConverterFactory,
            units: &[MeasureUnitItem],
            sign: i8,
            map: &mut LiteMap<u16, DetermineConvertibility>,
        ) -> Result<(), ConversionError> {
            for item in units {
                let items_from_item = factory
                    .payload
                    .convert_infos
                    .get(item.unit_id as usize)
                    .ok_or(ConversionError::InternalError)?;

                insert_units_powers(items_from_item.basic_units(), item.power, sign, map)?;
            }

            Ok(())
        }

        fn insert_units_powers(
            basic_units: &ZeroSlice<MeasureUnitItem>,
            original_power: i8,
            sign: i8,
            map: &mut LiteMap<u16, DetermineConvertibility>,
        ) -> Result<(), ConversionError> {
            for item in basic_units.iter() {
                let item_power = item
                    .power
                    .checked_mul(original_power)
                    .ok_or(ConversionError::InternalError)?;
                let item_power_signed = item_power
                    .checked_mul(sign)
                    .ok_or(ConversionError::InternalError)?;
                if let Some(determine_convertibility) = map.get_mut(&item.unit_id) {
                    determine_convertibility.convertible += (item_power_signed) as i16;
                    determine_convertibility.reciprocal += (item_power) as i16;
                } else {
                    map.insert(
                        item.unit_id,
                        DetermineConvertibility {
                            convertible: (item_power_signed) as i16,
                            reciprocal: (item_power) as i16,
                        },
                    );
                }
            }

            Ok(())
        }

        let mut map = LiteMap::<u16, DetermineConvertibility>::new();
        insert_non_basic_units(self, unit1, 1, &mut map)?;
        insert_non_basic_units(self, unit2, -1, &mut map)?;

        let (convertible_sum, reciprocal_sum) = map.iter_values().fold(
            (0, 0),
            |(convertible_sum, reciprocal_sum), determine_convertibility| {
                (
                    convertible_sum + determine_convertibility.convertible,
                    reciprocal_sum + determine_convertibility.reciprocal,
                )
            },
        );

        if convertible_sum == 0 {
            Ok(Convertibility::Convertible)
        } else if reciprocal_sum == 0 {
            Ok(Convertibility::Reciprocal)
        } else {
            Ok(Convertibility::NotConvertible)
        }
    }

    /// Creates a converter for converting between two units in the form of CLDR identifiers.
    pub fn converter(
        &self,
        _input_unit: &str,
        _output_unit: &str,
    ) -> Result<Converter, ConversionError> {
        todo!("Implement ConverterFactory::converter")
    }
}

impl Converter {
    pub fn convert(&self, value: &BigRational) -> BigRational {
        let mut result: BigRational = value * &self.conversion_rate + &self.offset;
        if let Convertibility::Reciprocal = self.convertibility {
            result = result.recip();
        }

        result
    }
}
