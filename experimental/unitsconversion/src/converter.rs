// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use num_bigint::BigInt;
use num_rational::Ratio;

use crate::{
    measureunit::{MeasureUnit, MeasureUnitParser},
    provider::{Base, MeasureUnitItem, SiPrefix, Sign, UnitsInfoV1},
    ConversionError,
};
use litemap::LiteMap;
use zerovec::{ule::AsULE, ZeroSlice};

/// Represents the possible cases for the convertibility between two units.
#[derive(Debug, PartialEq)]
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
#[derive(Debug)]
pub struct Converter {
    /// The conversion rate between the input and output units.
    conversion_rate: Ratio<BigInt>,

    /// The offset between the input and output measurements.
    /// For example, converting between `celsius` and `fahrenheit` requires an offset of 32.
    offset: Ratio<BigInt>,

    /// The convertibility between the input and output units.
    /// For example, converting between `meter` and `foot` is convertible.
    /// and converting between `meter-per-second` and `second-per-meter` is reciprocal.
    /// and converting between `meter` and `second` is not convertible.
    convertibility: Convertibility,
}

/// A factory for creating a converter.
/// Also, it can check the convertibility between two units.
pub struct ConverterFactory<'data> {
    /// Contains the necessary data for the conversion factory.
    payload: &'data UnitsInfoV1<'data>,
}

impl<'data> ConverterFactory<'data> {
    /// Creates a new ConverterFactory from a UnitsInfoV1 payload.
    pub fn from_payload(payload: &'data UnitsInfoV1<'data>) -> Self {
        Self { payload }
    }

    pub fn parser(&self) -> MeasureUnitParser<'_> {
        MeasureUnitParser::from_payload(&self.payload.units_conversion_trie)
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

    fn apply_si_prefix(si_prefix: &SiPrefix, ratio: &mut Ratio<BigInt>) {
        match si_prefix.base {
            Base::Decimal => {
                *ratio *= Ratio::<BigInt>::from_integer(10.into()).pow(si_prefix.power as i32);
            }
            Base::Binary => {
                *ratio *= Ratio::<BigInt>::from_integer(2.into()).pow(si_prefix.power as i32);
            }
        }
    }

    fn apply_power(power: i8, ratio: &mut Ratio<BigInt>) {
        *ratio = ratio.pow(power as i32);
    }

    fn add_term(
        &self,
        unit_item: &MeasureUnitItem,
        sign: i8,
        conversion_rate: &mut Ratio<BigInt>,
        _offset: &mut Ratio<BigInt>,
    ) -> Result<(), ConversionError> {
        let conversion_info = self
            .payload
            .convert_infos
            .get(unit_item.unit_id as usize)
            .ok_or(ConversionError::InternalError)?;

        let factor_sign = match Sign::from_unaligned(conversion_info.factor_sign) {
            Sign::Positive => num_bigint::Sign::Plus,
            Sign::Negative => num_bigint::Sign::Minus,
        };

        let mut conversion_info_factor = Ratio::<BigInt>::new(
            BigInt::from_bytes_le(factor_sign, conversion_info.factor_num().as_ule_slice()),
            BigInt::from_bytes_le(
                num_bigint::Sign::Plus,
                conversion_info.factor_den().as_ule_slice(),
            ),
        );

        Self::apply_si_prefix(&unit_item.si_prefix, &mut conversion_info_factor);
        Self::apply_power(unit_item.power * sign, &mut conversion_info_factor);

        *conversion_rate *= conversion_info_factor;

        Ok(())
    }

    /// Creates a converter for converting between two units in the form of CLDR identifiers.
    pub fn converter(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Result<Converter, ConversionError> {
        let mut conversion_rate = Ratio::<BigInt>::from_integer(1.into());
        let mut offset = Ratio::<BigInt>::from_integer(0.into());
        let convertibility = self.extract_convertibility(input_unit, output_unit)?;

        if convertibility == Convertibility::NotConvertible {
            return Err(ConversionError::InvalidConversion);
        }

        for input_item in input_unit.contained_units.iter() {
            Self::add_term(self, input_item, 1, &mut conversion_rate, &mut offset)?;
        }

        let sign = match convertibility {
            Convertibility::Convertible => -1,
            Convertibility::Reciprocal => 1,
            Convertibility::NotConvertible => unreachable!(),
        };

        for output_item in output_unit.contained_units.iter() {
            Self::add_term(self, output_item, sign, &mut conversion_rate, &mut offset)?;
        }

        Ok(Converter {
            conversion_rate,
            offset,
            convertibility,
        })
    }
}

impl Converter {
    pub fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        let mut result: Ratio<BigInt> = value * &self.conversion_rate + &self.offset;
        if let Convertibility::Reciprocal = self.convertibility {
            result = result.recip();
        }

        result
    }
}
