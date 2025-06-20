// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::si_prefix::SiPrefix;
use alloc::string::String;
use alloc::string::ToString;

/// Represents a single unit in a measure unit.
/// For example, the MeasureUnit `kilometer-per-square-second` contains two single units:
///    1. `kilometer` with power 1 and prefix 3 with base 10.
///    2. `second` with power -2 and prefix power equal to 0.
#[zerovec::make_ule(SingleUnitULE)]
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Default)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::measure::provider::single_unit))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SingleUnit {
    /// The power of the unit.
    pub power: i8,

    /// The si base of the unit.
    pub si_prefix: SiPrefix,

    /// The id of the unit.
    pub unit_id: u16,
}

impl SingleUnit {
    /// Appends the short representation of the single unit to the given string.
    ///
    /// The format of the short representation is as follows:
    /// 1. If the power is not 1, the power is prefixed with "P" followed by the power value.
    /// 2. If the si prefix power is not 0, the si prefix is represented by its base character ('D' for Decimal, 'B' for Binary) followed by the prefix power value.
    /// 3. The unit ID is prefixed with "I" and appended to the string.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_experimental::measure::provider::single_unit::SingleUnit;
    /// use icu_experimental::measure::provider::si_prefix::SiPrefix;
    /// use icu_experimental::measure::provider::si_prefix::Base;
    ///
    /// let mut short_representation = String::new();
    /// let single_unit = SingleUnit {
    ///     power: 3,
    ///     si_prefix: SiPrefix { power: 2, base: Base::Decimal },
    ///     unit_id: 85,
    /// };
    /// single_unit.append_short_representation(&mut short_representation);
    /// assert_eq!(short_representation, "P3D2I85");
    /// ```
    pub fn append_short_representation(&self, append_to: &mut String) {
        if self.power != 1 {
            append_to.push('P');
            append_to.push_str(&self.power.to_string());
        }

        if self.si_prefix.power != 0 {
            self.si_prefix.append_short_representation(append_to);
        }

        append_to.push('I');
        append_to.push_str(&self.unit_id.to_string());
    }
}
