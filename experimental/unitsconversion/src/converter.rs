// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// TODO: we do not want to use `std`
use std::collections::HashMap;

use num::BigRational;

use crate::{
    measureunit::MeasureUnitParser,
    provider::{MeasureUnitItem, UnitsInfoV1},
    ConversionError,
};

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

/// A factory for creating a converter.
/// Also, it can check the convertibility between two units.
pub struct ConverterFactory<'data> {
    /// Contains the necessary data for the conversion factory.
    payload: UnitsInfoV1<'data>,
}

/// A converter for converting between two units.
/// For example, converting between `meter` and `foot`.
pub struct Converter {
    conversion_rate: BigRational,
    offset: BigRational,
    convertibility: Convertibility,
}

impl ConverterFactory<'_> {
    fn parser(&self) -> MeasureUnitParser<'_> {
        MeasureUnitParser::new(&self.payload.units_conversion_trie)
    }

    /// Extract the convertibility from the given units in the form of CLDR identifiers.
    pub fn extract_convertibility(
        &self,
        unit1: &str,
        unit2: &str,
    ) -> Result<Convertibility, ConversionError> {
        let parser = self.parser();
        let unit1 = parser.try_from_identifier(unit1)?;
        let unit2 = parser.try_from_identifier(unit2)?;

        struct DetermineConvertibility {
            convertible: i16,
            reciprocal: i16,
        }

        fn insert_units_powers(
            unit_items: &Vec<MeasureUnitItem>,
            sign: i8,
            map: &mut HashMap<u16, DetermineConvertibility>,
        ) {
            for item in unit_items {
                if let Some(determine_convertibility) = map.get_mut(&item.unit_id) {
                    determine_convertibility.convertible += (item.power * sign) as i16;
                    determine_convertibility.reciprocal += (item.power * sign) as i16;
                } else {
                    map.insert(
                        item.unit_id,
                        DetermineConvertibility {
                            convertible: (item.power * sign) as i16,
                            reciprocal: (item.power * sign) as i16,
                        },
                    );
                }
            }
        }

        let mut map = HashMap::<u16, DetermineConvertibility>::new();
        insert_units_powers(&unit1, 1, &mut map);
        insert_units_powers(&unit2, -1, &mut map);

        let (convertible_sum, reciprocal_sum) = map.values().fold(
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
