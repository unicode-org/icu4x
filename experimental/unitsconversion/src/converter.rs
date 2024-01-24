// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    measureunit::{MeasureUnit, MeasureUnitParser},
    provider::{Base, MeasureUnitItem, SiPrefix, Sign, SignULE, UnitsInfoV1Marker},
    ConversionError,
};
use icu_locid::locale;
use icu_provider::DataError;
use icu_provider::DataPayload;
use icu_provider::DataProvider;
use icu_provider::DataRequest;
use litemap::LiteMap;
use num::{rational::Ratio, BigInt, Zero};
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::{ule::AsULE, ZeroSlice};

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
    /// The conversion rate between the input and output units.
    conversion_rate: Ratio<BigInt>,

    /// The offset between the input and output measurements.
    /// For example, converting between `celsius` and `fahrenheit` requires an offset of 32.
    offset: Ratio<BigInt>,

    /// Determines if the units are reciprocal or not.
    /// For example, `meter-per-second` and `second-per-meter` are reciprocal.
    reciprocal: bool,
}

/// A factory for creating a converter.
/// Also, it can check the convertibility between two units.
pub struct ConverterFactory {
    // TODO(#4522): Make the converter factory owns the data.
    /// Contains the necessary data for the conversion factory.
    /// DataPayload<ErasedPluralRulesV1Marker>
    data: DataPayload<UnitsInfoV1Marker>,
    // payload_store: ZeroTrieSimpleAscii<ZeroVec<'static, u8>>,
}

impl ConverterFactory {
    #[cfg(feature = "datagen")]
    pub fn new(data: &(impl DataProvider<UnitsInfoV1Marker> + ?Sized)) -> Result<Self, DataError> {
        let data = data.load(DataRequest::default())?.take_payload()?.cast();

        // let store: &UnitsInfoV1<'static> = data.get();
        // let store = store.units_conversion_trie.clone().take_store();
        Ok(Self { data })
    }

    pub fn parser(&self) -> MeasureUnitParser<'_> {
        let trie = self.data.get().units_conversion_trie.clone(); // cheap since store is a borrowed ZeroVec

        MeasureUnitParser::from_payload(ZeroTrieSimpleAscii::from_store(trie.take_store()))
    }

    // TODO(#4512): the need needs to be bikeshedded.
    /// Extract the convertibility from the given units in the form of CLDR identifiers.
    fn extract_convertibility(&self, unit1: &MeasureUnit, unit2: &MeasureUnit) -> Convertibility {
        /// A struct that contains the sums and subtractions of base unit powers.
        /// For example:
        ///     For the input unit `meter-per-second`, the base units are `meter` (power: 1) and `second` (power: -1).
        ///     For the output unit `foot-per-second`, the base units are `meter` (power: 1) and `second` (power: -1).
        ///     The subtractions are: meter: 1 - 1 = 0, second: -1 - (-1) = 0.
        ///     The sums are: meter: 1 + 1 = 2, second: -1 + (-1) = -2.
        ///     If all the sums are zeros, then the units are reciprocal.
        ///     If all the subtractions are zeros, then the units are convertible.
        ///     This means the result for the example is convertible.
        #[derive(Debug)]
        struct DetermineConvertibility {
            subtractions: i16,
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
            map: &mut LiteMap<u16, DetermineConvertibility>,
        ) -> Result<(), ConversionError> {
            for item in units {
                let items_from_item = factory
                    .data
                    .get()
                    .convert_infos
                    .get(item.unit_id as usize)
                    .ok_or(ConversionError::DataNotFoundError)?;

                insert_base_units(items_from_item.basic_units(), item.power as i16, sign, map)?;
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
        ) -> Result<(), ConversionError> {
            for item in basic_units.iter() {
                let item_power = (item.power as i16) * original_power;
                let signed_item_power = item_power * sign;
                if let Some(determine_convertibility) = map.get_mut(&item.unit_id) {
                    determine_convertibility.subtractions += signed_item_power;
                    determine_convertibility.sums += item_power;
                } else {
                    map.insert(
                        item.unit_id,
                        DetermineConvertibility {
                            subtractions: (signed_item_power),
                            sums: (item_power),
                        },
                    );
                }
            }

            Ok(())
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

        let (sums_are_zeros, subtractions_are_zeros) = map.iter().fold(
            (true, true),
            |(sums, subs), (_, determine_convertibility)| {
                (
                    sums && determine_convertibility.sums == 0,
                    subs && determine_convertibility.subtractions == 0,
                )
            },
        );

        if subtractions_are_zeros {
            Convertibility::Convertible
        } else if sums_are_zeros {
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

    fn apply_power(power: i8, ratio: &mut Ratio<BigInt>) {
        *ratio = ratio.pow(power as i32);
    }

    fn add_term(
        &self,
        unit_item: &MeasureUnitItem,
        sign: i8,
        conversion_rate: &mut Ratio<BigInt>,
    ) -> Result<(), ConversionError> {
        let conversion_info = self
            .data
            .get()
            .convert_infos
            .get(unit_item.unit_id as usize)
            .ok_or(ConversionError::DataNotFoundError)?;

        let mut conversion_info_factor = Self::extract_ratio_from_unaligned(
            &conversion_info.factor_sign,
            conversion_info.factor_num(),
            conversion_info.factor_den(),
        );

        Self::apply_si_prefix(&unit_item.si_prefix, &mut conversion_info_factor);
        Self::apply_power(unit_item.power * sign, &mut conversion_info_factor);

        *conversion_rate *= conversion_info_factor;

        Ok(())
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

    fn get_offset(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Result<Ratio<BigInt>, ConversionError> {
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
            return Ok(Ratio::<BigInt>::from_integer(0.into()));
        }

        let input_conversion_info = self
            .data
            .get()
            .convert_infos
            .get(input_unit.contained_units[0].unit_id as usize)
            .ok_or(ConversionError::DataNotFoundError)?;

        let output_conversion_info = self
            .data
            .get()
            .convert_infos
            .get(output_unit.contained_units[0].unit_id as usize)
            .ok_or(ConversionError::DataNotFoundError)?;

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
            return Ok(Ratio::<BigInt>::from_integer(0.into()));
        }

        let output_conversion_rate = Self::extract_ratio_from_unaligned(
            &output_conversion_info.factor_sign,
            output_conversion_info.factor_num(),
            output_conversion_info.factor_den(),
        );

        Ok((input_offset - output_offset) * output_conversion_rate.recip())
    }

    /// Creates a converter for converting between two units in the form of CLDR identifiers.
    pub fn converter(
        &self,
        input_unit: &MeasureUnit,
        output_unit: &MeasureUnit,
    ) -> Option<Converter> {
        let mut conversion_rate = Ratio::<BigInt>::from_integer(1.into());
        let convertibility = self.extract_convertibility(input_unit, output_unit);

        let calculation_sign = match convertibility {
            Convertibility::Convertible => -1,
            Convertibility::Reciprocal => 1,
            Convertibility::NotConvertible => {
                return None;
            }
        };

        for input_item in input_unit.contained_units.iter() {
            if Self::add_term(self, input_item, 1, &mut conversion_rate).is_err() {
                debug_assert!(false, "Failed to add input_item");
                return None;
            }
        }

        for output_item in output_unit.contained_units.iter() {
            if Self::add_term(self, output_item, calculation_sign, &mut conversion_rate).is_err() {
                debug_assert!(false, "Failed to add output_item");
                return None;
            }
        }

        let offset = match Self::get_offset(self, input_unit, output_unit) {
            Ok(val) => val,
            Err(_) => {
                debug_assert!(false, "Failed to get offset");
                return None;
            }
        };

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
