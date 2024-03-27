// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_provider::prelude::*;
use icu_provider::DataError;
use litemap::LiteMap;
use num_bigint::BigInt;
use num_rational::Ratio;
use num_traits::{One, Zero};
use zerovec::ule::AsULE;
use zerovec::ZeroSlice;

use crate::units::provider::{MeasureUnitItem, SignULE};

use super::provider;
use super::{
    converter::{
        OffsetConverter, ProportionalConverter, ReciprocalConverter, UnitsConverter,
        UnitsConverterInner,
    },
    measureunit::{MeasureUnit, MeasureUnitParser},
    provider::{Base, SiPrefix, Sign},
};

/// ConverterFactory is a factory for creating a converter.
pub struct ConverterFactory {
    /// Contains the necessary data for the conversion factory.
    payload: DataPayload<provider::UnitsInfoV1Marker>,
}

impl From<Sign> for num_bigint::Sign {
    fn from(val: Sign) -> Self {
        match val {
            Sign::Positive => num_bigint::Sign::Plus,
            Sign::Negative => num_bigint::Sign::Minus,
        }
    }
}

impl ConverterFactory {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: skip,
        options: skip,
        error: DataError,
        #[cfg(skip)]
        functions: [
            new,
            try_new_with_any_provider,
            try_new_with_buffer_provider,
            try_new_unstable,
            Self,
        ]
    );

    /// Creates a new [`ConverterFactory`] from compiled data.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self {
            payload: DataPayload::from_static_ref(crate::provider::Baked::SINGLETON_UNITS_INFO_V1),
        }
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, DataError>
    where
        D: ?Sized + DataProvider<provider::UnitsInfoV1Marker>,
    {
        let payload = provider.load(DataRequest::default())?.take_payload()?;

        Ok(Self { payload })
    }

    pub fn parser(&self) -> MeasureUnitParser<'_> {
        MeasureUnitParser::from_payload(self.payload.get().units_conversion_trie.as_borrowed())
    }

    /// Calculates the offset between two units by performing the following steps:
    /// 1. Identify the conversion rate from the first unit to the base unit as ConversionRate1: N1/D1 with an Offset1: OffsetN1/OffsetD1.
    /// 2. Identify the conversion rate from the second unit to the base unit as ConversionRate2: N2/D2 with an Offset2: OffsetN2/OffsetD2.
    /// 3. The conversion from the base unit to the second unit is represented by ConversionRateBaseToUnit2: (D2/N2) and an OffsetBaseToUnit2: - (OffsetN2/OffsetD2) * (D2/N2).
    /// 4. To convert a value V from the first unit to the second unit, first convert V to the base unit using ConversionRate1:
    ///    (V * N1/D1) + OffsetN1/OffsetD1, referred to as V_TO_Base.
    /// 5. Then, convert V_TO_Base to the second unit using the formula: CR: (D2/N2) and Offset: - (OffsetN2/OffsetD2) * (D2/N2).
    ///    The result is: (V_TO_Base * (D2/N2)) - (OffsetN2/OffsetD2) * (D2/N2).
    /// 6. By inserting V_TO_Base from step 4 into step 5, the equation becomes:
    ///    (((V * N1/D1) + OffsetN1/OffsetD1) * D2/N2) - (OffsetN2/OffsetD2) * (D2/N2).
    /// 7. Simplifying the equation gives:
    ///    (V * (N1/D1) * (D2/N2)) + (OffsetN1/OffsetD1 * (D2/N2)) - (OffsetN2/OffsetD2) * (D2/N2).
    /// 8. Focusing on the constants to find the offset formula, we get:
    ///    Offset = ((OffsetN1/OffsetD1) - (OffsetN2/OffsetD2)) * (D2/N2),
    ///    which simplifies to: Offset = (Offset1 - Offset2) * (1/ConversionRate2).
    ///
    /// NOTE:
    ///   An offset can be calculated if both the input and output units are simple.
    ///   A unit is considered simple if it is made up of a single unit item, with a power of 1 and an SI prefix of 0.
    ///   
    ///   For example:
    ///     `meter` and `foot` are simple units.
    ///     `meter-per-second` and `foot-per-second` are not simple units.
    fn compute_offset(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Option<Ratio<BigInt>> {
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
            .get()
            .convert_infos
            .get(input_unit.contained_units[0].unit_id as usize);
        debug_assert!(
            input_conversion_info.is_some(),
            "Failed to get input conversion info"
        );
        let input_conversion_info = input_conversion_info?;

        let output_conversion_info = self
            .payload
            .get()
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

        let output_conversion_rate_recip = Self::extract_ratio_from_unaligned(
            &output_conversion_info.factor_sign,
            // Because we are computing the reciprocal, the numerator and denominator are swapped.
            output_conversion_info.factor_den(),
            output_conversion_info.factor_num(),
        );

        Some((input_offset - output_offset) * output_conversion_rate_recip)
    }

    /// Checks whether the given units are reciprocal.
    /// If they are not reciprocal, it implies that the units are proportional.
    /// NOTE:
    ///   If the units are neither proportional nor reciprocal, the function will return `None`,
    ///   indicating that the units are incompatible.
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
        ) -> Option<()> {
            for item in units {
                let items_from_item = factory
                    .payload
                    .get()
                    .convert_infos
                    .get(item.unit_id as usize);

                debug_assert!(items_from_item.is_some(), "Failed to get convert info");

                insert_base_units(items_from_item?.basic_units(), item.power as i16, sign, map);
            }

            Some(())
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
        insert_non_basic_units(self, unit1, 1, &mut map)?;
        insert_non_basic_units(self, unit2, -1, &mut map)?;

        let (power_sums_are_zero, power_diffs_are_zero) =
            map.iter_values()
                .fold((true, true), |(sums, diffs), powers_info| {
                    (
                        sums && powers_info.sums == 0,
                        diffs && powers_info.diffs == 0,
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
        let conversion_info = self
            .payload
            .get()
            .convert_infos
            .get(unit_item.unit_id as usize);
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
        let sign = Sign::from_unaligned(*sign_ule).into();
        Ratio::<BigInt>::new(
            BigInt::from_bytes_le(sign, num_ule.as_ule_slice()),
            BigInt::from_bytes_le(num_bigint::Sign::Plus, den_ule.as_ule_slice()),
        )
    }

    /// Creates a converter for converting between two single or compound units.
    /// For example:
    ///    1 - `meter` to `foot` --> Simple
    ///    2 - `kilometer-per-hour` to `mile-per-hour` --> Compound
    ///    3 - `mile-per-gallon` to `liter-per-100-kilometer` --> Reciprocal
    ///    4 - `celsius` to `fahrenheit` --> Needs an offset
    ///
    /// NOTE:
    ///    This converter does not support conversions between mixed units,
    ///    such as, from "meter" to "foot-and-inch".
    pub fn converter(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Option<UnitsConverter> {
        let is_reciprocal = self.is_reciprocal(input_unit, output_unit)?;

        // Determine the sign of the powers of the units from root to unit2.
        let root_to_unit2_direction_sign = if is_reciprocal { 1 } else { -1 };

        let mut conversion_rate = Ratio::one();
        for input_item in input_unit.contained_units.iter() {
            conversion_rate *= Self::compute_conversion_term(self, input_item, 1)?;
        }

        for output_item in output_unit.contained_units.iter() {
            conversion_rate *=
                Self::compute_conversion_term(self, output_item, root_to_unit2_direction_sign)?;
        }

        let offset = self.compute_offset(input_unit, output_unit)?;

        if is_reciprocal && !offset.is_zero() {
            debug_assert!(
                false,
                "The units are reciprocal, but the offset is not zero. This is should not happen!.",
            );
            return None;
        }

        if is_reciprocal {
            Some(UnitsConverter(UnitsConverterInner::Reciprocal(
                ReciprocalConverter {
                    proportional: ProportionalConverter { conversion_rate },
                },
            )))
        } else if offset.is_zero() {
            Some(UnitsConverter(UnitsConverterInner::Proportional(
                ProportionalConverter { conversion_rate },
            )))
        } else {
            Some(UnitsConverter(UnitsConverterInner::Offset(
                OffsetConverter {
                    proportional: ProportionalConverter { conversion_rate },
                    offset,
                },
            )))
        }
    }
}
