// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    measureunit::{MeasureUnit, MeasureUnitParser},
    provider::{Base, MeasureUnitItem, SiPrefix, Sign, SignULE, UnitsInfoV1},
    ConversionError,
};
use litemap::LiteMap;
use num::{rational::Ratio, BigInt};
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::{ule::AsULE, ZeroSlice, ZeroVec};

/// LinearConverter is responsible for converting between two units that are linearly related.
/// For example: 1- `meter` to `foot`.
///              2- `square-meter` to `square-foot`.
///              3- `100-kilometer-per-liter` to `gallon-per-mile`.
///
/// However, it cannot convert between two units that are not linearly related such as `celsius` to `fahrenheit`.
/// NOTE:
///    The converter is not able to convert between two units that are not single. such as "foot-and-inch" to "meter".
#[derive(Debug)]
pub struct LinearConverter {
    // TODO(#4554): Implement a New Struct `IcuRatio` to Encapsulate `Ratio<BigInt>`.
    /// The conversion rate between the input and output units.
    conversion_rate: Ratio<BigInt>,

    /// Determines if the units are reciprocal or not.
    /// For example, `meter-per-second` and `second-per-meter` are reciprocal.
    /// Real world case, `gallon-per-mile` and `100-kilometer-per-liter` which are reciprocal.
    reciprocal: bool,
}

/// ConverterFactory is a factory for creating a converter.
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
    /// Checks if the given units are reciprocal or not.
    /// If it is not reciprocal, this means that the units are convertible.
    /// NOTE:
    ///   If the units are not convertible or reciprocal, the function will return `None`
    ///   which means that the units are not compatible.
    fn is_reciprocal(&self, unit1: &MeasureUnit, unit2: &MeasureUnit) -> Option<bool> {
        /// A struct that contains the sum and difference of base unit powers.
        /// For example:
        ///     For the input unit `meter-per-second`, the base units are `meter` (power: 1) and `second` (power: -1).
        ///     For the output unit `foot-per-second`, the base units are `meter` (power: 1) and `second` (power: -1).
        ///     The differences are: meter: 1 - 1 = 0, second: -1 - (-1) = 0.
        ///     The sums are: meter: 1 + 1 = 2, second: -1 + (-1) = -2.
        ///     If all the sums are zeros, then the units are reciprocal.
        ///     If all the diffs are zeros, then the units are convertible.
        ///     If none of the above, then the units are not convertible.
        #[derive(Debug)]
        struct PowersInfo {
            diffs: i16,
            sums: i16,
        }

        /// Inserting the units item into the map.
        /// NOTE:
        ///     This will require to go through the basic units of the given unit items.
        ///     For example, `newton` has the basic units:  `gram`, `meter`, and `second` (each one has it is own power and si prefix).
        fn insert_non_basic_units(
            factory: &ConverterFactory,
            units: &[MeasureUnitItem],
            sign: i16,
            map: &mut LiteMap<u16, PowersInfo>,
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
            map: &mut LiteMap<u16, PowersInfo>,
        ) {
            for item in basic_units.iter() {
                let item_power = (item.power as i16) * original_power;
                let signed_item_power = item_power * sign;
                if let Some(powers) = map.get_mut(&item.unit_id) {
                    powers.diffs += signed_item_power;
                    powers.sums += item_power;
                } else {
                    map.insert(
                        item.unit_id,
                        PowersInfo {
                            diffs: (signed_item_power),
                            sums: (item_power),
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
            return None;
        }

        let (power_sums_are_zero, power_diffs_are_zero) =
            map.iter_values()
                .fold((true, true), |(sums, diffs), determine_convertibility| {
                    (
                        sums && determine_convertibility.sums == 0,
                        diffs && determine_convertibility.diffs == 0,
                    )
                });

        if power_diffs_are_zero {
            Some(false)
        } else if power_sums_are_zero {
            Some(true)
        } else {
            None
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

    /// Creates a converter for converting between two units in the form of CLDR identifiers.
    pub fn converter(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Option<LinearConverter> {
        let is_reciprocal = self.is_reciprocal(input_unit, output_unit)?;

        // Determine the sign of the powers of the units from root to unit2.
        let root_to_unit2_direction_sign = if is_reciprocal { 1 } else { -1 };

        let mut conversion_rate = Ratio::<BigInt>::from_integer(1.into());

        for input_item in input_unit.contained_units.iter() {
            match Self::compute_conversion_term(self, input_item, 1) {
                Some(term) => conversion_rate *= term,
                None => return None,
            }
        }

        for output_item in output_unit.contained_units.iter() {
            match Self::compute_conversion_term(self, output_item, root_to_unit2_direction_sign) {
                Some(term) => conversion_rate *= term,
                None => return None,
            }
        }

        Some(LinearConverter {
            conversion_rate,
            reciprocal: is_reciprocal,
        })
    }
}

impl LinearConverter {
    /// Converts the given value from the input unit to the output unit.
    pub fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        let mut result: Ratio<BigInt> = value * &self.conversion_rate;
        if self.reciprocal {
            result = result.recip();
        }

        result
    }
}
