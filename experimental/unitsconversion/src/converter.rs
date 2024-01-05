// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    measureunit::{MeasureUnit, MeasureUnitParser},
    provider::{MeasureUnitItem, UnitsInfoV1},
    ConversionError,
};
use litemap::LiteMap;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::{ZeroSlice, ZeroVec};

/// Represents the possible cases for the convertibility between two units.
pub enum Convertibility {
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

/// A factory for creating a converter.
/// Also, it can check the convertibility between two units.
pub struct ConverterFactory<'data> {
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
        MeasureUnitParser::from_payload(&self.payload_store)
    }

    // TODO(#4512): the need needs to be bikeshedded.
    /// Extract the convertibility from the given units in the form of CLDR identifiers.
    pub fn extract_convertibility(
        &self,
        unit1: &MeasureUnit,
        unit2: &MeasureUnit,
    ) -> Convertibility {
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
                    .payload
                    .convert_infos
                    .get(item.unit_id as usize)
                    .ok_or(ConversionError::DataNotFoundError)?;

                insert_units_powers(items_from_item.basic_units(), item.power as i16, sign, map)?;
            }

            Ok(())
        }

        /// Inserting the basic units into the map.
        /// NOTE:   
        ///     The basic units should be multiplied by the original power.
        ///     For example, `square-foot` , the base unit is `meter` with power 1.
        ///     Thus, the inserted power should be `1 * 2 = 2`.
        fn insert_units_powers(
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
}
