// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    measureunit::{MeasureUnit, MeasureUnitParser},
    provider::{Base, MeasureUnitItem, SiPrefix, Sign, SignULE, UnitsInfoV1},
    ConversionError,
};
use litemap::LiteMap;
use num::{rational::Ratio, BigInt, Zero};
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::{ule::AsULE, ZeroSlice, ZeroVec};

/// Represents the possible cases for the convertibility between two units.
#[derive(Debug, PartialEq)]
enum Convertibility {
    /// The units are convertible.
    /// For example, `meter` and `foot` are convertible.
    Convertible,

    /// The units are reciprocal.
    /// For example, `gallon-per-mile` and `100-kilometer-per-liter` are reciprocal.
    Reciprocal,

    /// The units are not convertible.
    /// For example, `meter` and `second` are not convertible.
    NotConvertible,
}

/// A converter for converting between two single units.
/// For example: 1- `meter` to `foot`.
///              2- `square-meter` to `square-foot`.
///              3- `100-kilometer-per-liter` to `gallon-per-mile`.
/// NOTE:
///    The converter is not able to convert between two units that are not single. such as "foot-and-inch" to "meter".
#[derive(Debug)]
pub struct Converter {
    // TODO(#4554): Implement a New Struct `IcuRatio` to Encapsulate `Ratio<BigInt>`.
    /// The conversion rate between the input and output units.
    conversion_rate: Ratio<BigInt>,

    /// The offset between the input and output measurements.
    /// For example, converting between `celsius` and `fahrenheit` requires an offset of 32.
    offset: Ratio<BigInt>,

    /// Determines if the units are reciprocal or not.
    /// For example, `meter-per-second` and `second-per-meter` are reciprocal.
    /// Real world case, `gallon-per-mile` and `100-kilometer-per-liter` which are reciprocal.
    reciprocal: bool,
}

/// A factory for creating a converter.
/// Also, it can check the convertibility between two units.
pub struct ConverterFactory<'data> {
    // TODO(#4522): Make the converter factory owns the data.
    /// Contains the necessary data for the conversion factory.
    payload: &'data UnitsInfoV1<'data>,
    payload_store: &'data ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,
}

impl<'data> ConverterFactory<'data> {
    #[cfg(feature = "datagen")]
    pub fn from_payload(
        payload: &'data UnitsInfoV1<'data>,
        payload_store: &'data ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,
    ) -> Self {
        Self {
            payload,
            payload_store,
        }
    }

    pub fn parser(&self) -> MeasureUnitParser<'_> {
        MeasureUnitParser::from_payload(self.payload_store)
    }

    // TODO(#4512): the need needs to be bikeshedded.
    /// Extract the convertibility from the given units in the form of CLDR identifiers.
    fn extract_convertibility(&self, unit1: &MeasureUnit, unit2: &MeasureUnit) -> Convertibility {
        /// A struct that contains the sum and difference of base unit powers.
        /// For example:
        ///     For the input unit `meter-per-second`, the base units are `meter` (power: 1) and `second` (power: -1).
        ///     For the output unit `foot-per-second`, the base units are `meter` (power: 1) and `second` (power: -1).
        ///     The differences are: meter: 1 - 1 = 0, second: -1 - (-1) = 0.
        ///     The sums are: meter: 1 + 1 = 2, second: -1 + (-1) = -2.
        ///     If all the sums are zeros, then the units are reciprocal.
        ///     If all the subtractions are zeros, then the units are convertible.
        ///     This means the result for the example is convertible.
        #[derive(Debug)]
        struct DetermineConvertibility {
            difference: i16,
            sum: i16,
        }

        /// Inserting the units item into the map.
        /// NOTE:
        ///     This will require to go through the basic units of the given unit items.
        ///     For example, `newton` has the basic units:  `gram`, `meter`, and `second` (each one has it is own power and si prefix).
        fn insert_non_basic_units(
            factory: &ConverterFactory,
            units: &[MeasureUnitItem],
            sign: i16,
            map: &mut LiteMap<u16, DetermineConvertibility>,
        ) -> Result<(), ConversionError> {
            for item in units {
                let items_from_item = factory
                    .payload
                    .convert_infos
                    .get(item.unit_id as usize)
                    .ok_or(ConversionError::DataNotFoundError)?;

                insert_base_units(items_from_item.basic_units(), item.power as i16, sign, map);
            }

            Ok(())
        }

        /// Inserting the basic units into the map.
        /// NOTE:   
        ///     The base units should be multiplied by the original power.
        ///     For example, `square-foot` , the base unit is `meter` with power 1.
        ///     Thus, the inserted power should be `1 * 2 = 2`.
        fn insert_base_units(
            basic_units: &ZeroSlice<MeasureUnitItem>,
            original_power: i16,
            sign: i16,
            map: &mut LiteMap<u16, DetermineConvertibility>,
        ) {
            for item in basic_units.iter() {
                let item_power = (item.power as i16) * original_power;
                let signed_item_power = item_power * sign;
                if let Some(determine_convertibility) = map.get_mut(&item.unit_id) {
                    determine_convertibility.difference += signed_item_power;
                    determine_convertibility.sum += item_power;
                } else {
                    map.insert(
                        item.unit_id,
                        DetermineConvertibility {
                            difference: (signed_item_power),
                            sum: (item_power),
                        },
                    );
                }
            }
        }

        let unit1 = &unit1.contained_units;
        let unit2 = &unit2.contained_units;

        let mut map = LiteMap::new();
        let first_insert_result = insert_non_basic_units(self, unit1, 1, &mut map);
        let second_insert_result = insert_non_basic_units(self, unit2, -1, &mut map);

        debug_assert!(first_insert_result.is_ok());
        debug_assert!(second_insert_result.is_ok());

        if first_insert_result.is_err() || second_insert_result.is_err() {
            return Convertibility::NotConvertible;
        }

        let (unit1_unit2_power_sums_are_zero, unit1_unit2_power_diffs_are_zero) = map
            .iter_values()
            .fold((true, true), |(sums, subs), determine_convertibility| {
                (
                    sums && determine_convertibility.sum == 0,
                    subs && determine_convertibility.difference == 0,
                )
            });

        if unit1_unit2_power_diffs_are_zero {
            Convertibility::Convertible
        } else if unit1_unit2_power_sums_are_zero {
            Convertibility::Reciprocal
        } else {
            Convertibility::NotConvertible
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

    fn compute_conversion_term(
        &self,
        unit_item: &MeasureUnitItem,
        sign: i8,
    ) -> Option<Ratio<BigInt>> {
        let conversion_info = self.payload.convert_infos.get(unit_item.unit_id as usize);
        debug_assert!(conversion_info.is_some(), "Failed to get conversion info");
        let conversion_info = conversion_info?;

        let mut conversion_info_factor = Self::extract_ratio_from_unaligned(
            &conversion_info.factor_sign,
            conversion_info.factor_num(),
            conversion_info.factor_den(),
        );

        Self::apply_si_prefix(&unit_item.si_prefix, &mut conversion_info_factor);
        conversion_info_factor = conversion_info_factor.pow((unit_item.power * sign) as i32);
        Some(conversion_info_factor)
    }

    fn extract_ratio_from_unaligned(
        sign_ule: &SignULE,
        num_ule: &ZeroSlice<u8>,
        den_ule: &ZeroSlice<u8>,
    ) -> Ratio<BigInt> {
        let sign = match Sign::from_unaligned(*sign_ule) {
            Sign::Positive => num_bigint::Sign::Plus,
            Sign::Negative => num_bigint::Sign::Minus,
        };

        Ratio::<BigInt>::new(
            BigInt::from_bytes_le(sign, num_ule.as_ule_slice()),
            BigInt::from_bytes_le(num_bigint::Sign::Plus, den_ule.as_ule_slice()),
        )
    }

    fn compute_offset(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Option<Ratio<BigInt>> {
        // In order to have an offset, the input and output units should be simple.
        // This means, the input and output units should have only one unit item with power 1 and si prefix 0.
        // For example:
        //           1 - `meter` and `foot` are simple units.
        //           2 - `meter-per-second` and `foot-per-second` are not simple units.
        if !(input_unit.contained_units.len() == 1
            && output_unit.contained_units.len() == 1
            && input_unit.contained_units[0].power == 1
            && output_unit.contained_units[0].power == 1
            && input_unit.contained_units[0].si_prefix.power == 0
            && output_unit.contained_units[0].si_prefix.power == 0)
        {
            return Some(Ratio::<BigInt>::from_integer(0.into()));
        }

        let input_conversion_info = self
            .payload
            .convert_infos
            .get(input_unit.contained_units[0].unit_id as usize);
        debug_assert!(
            input_conversion_info.is_some(),
            "Failed to get input conversion info"
        );
        let input_conversion_info = input_conversion_info?;

        let output_conversion_info = self
            .payload
            .convert_infos
            .get(output_unit.contained_units[0].unit_id as usize);
        debug_assert!(
            output_conversion_info.is_some(),
            "Failed to get output conversion info"
        );
        let output_conversion_info = output_conversion_info?;

        let input_offset = Self::extract_ratio_from_unaligned(
            &input_conversion_info.offset_sign,
            input_conversion_info.offset_num(),
            input_conversion_info.offset_den(),
        );

        let output_offset = Self::extract_ratio_from_unaligned(
            &output_conversion_info.offset_sign,
            output_conversion_info.offset_num(),
            output_conversion_info.offset_den(),
        );

        if input_offset.is_zero() && output_offset.is_zero() {
            return Some(Ratio::<BigInt>::from_integer(0.into()));
        }

        // To calculate the offset:
        // Let's assume the unit 1 conversion rate is : N1/D1 + OffsetN1/OffsetD1 to be converted to the root.
        // Let's assume the unit 2 conversion rate is : N2/D2 + OffsetN2/OffsetD2 to be converted to the root.
        // To flip the conversion from the root to unit 2, we will have:
        //      D2/N2 - OffsetN2/OffsetD2 * (D2/N2).
        // Then, If V is the value that will be converted from unit 1 to unit 2, then:
        // V * (N1/D1 + OffsetN1/OffsetD1) = (V * N1/D1) + OffsetN1/OffsetD1 which we will call it V_Root.
        // Then, to convert V_Root to the unit 2, we will have:
        // V_Root * D2/N2 - OffsetN2/OffsetD2 *(D2/N2)
        // Then, by substituting V_Root, we will have:
        // ((V * N1/D1) + OffsetN1/OffsetD1) * D2/N2 - OffsetN2/OffsetD2 *(D2/N2)
        // Then, by simplifying the equation, we will have:
        // V * (N1/D1) * (D2/N2) + OffsetN1/OffsetD1 * (D2/N2) - OffsetN2/OffsetD2 *(D2/N2)
        // Then, by looking at the constants part (which is the offset), we will have:
        // Offset = OffsetN1/OffsetD1 * (D2/N2) - OffsetN2/OffsetD2 *(D2/N2)
        //        = (OffsetN1/OffsetD1 - OffsetN2/OffsetD2) * (D2/N2).
        // Then,
        // Offset = (Offset1 - Offset2) * (1/ConversionRate2).

        let output_conversion_rate_recip = Self::extract_ratio_from_unaligned(
            &output_conversion_info.factor_sign,
            // Because we are computing the reciprocal, the numerator and denominator are swapped.
            output_conversion_info.factor_den(),
            output_conversion_info.factor_num(),
        );

        Some((input_offset - output_offset) * output_conversion_rate_recip)
    }

    /// Creates a converter for converting between two units in the form of CLDR identifiers.
    pub fn converter(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Option<Converter> {
        let mut conversion_rate = Ratio::<BigInt>::from_integer(1.into());
        let convertibility = self.extract_convertibility(input_unit, output_unit);

        let direction_sign = match convertibility {
            Convertibility::Convertible => -1,
            Convertibility::Reciprocal => 1,
            Convertibility::NotConvertible => {
                return None;
            }
        };

        for input_item in input_unit.contained_units.iter() {
            match Self::compute_conversion_term(self, input_item, 1) {
                Some(term) => conversion_rate *= term,
                None => return None,
            }
        }

        for output_item in output_unit.contained_units.iter() {
            match Self::compute_conversion_term(self, output_item, direction_sign) {
                Some(term) => conversion_rate *= term,
                None => return None,
            }
        }

        let offset = Self::compute_offset(self, input_unit, output_unit)?;

        Some(Converter {
            conversion_rate,
            offset,
            reciprocal: convertibility == Convertibility::Reciprocal,
        })
    }
}

impl Converter {
    /// Converts the given value from the input unit to the output unit.
    pub fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        let mut result: Ratio<BigInt> = value * &self.conversion_rate + &self.offset;
        if self.reciprocal {
            result = result.recip();
        }

        result
    }
}
