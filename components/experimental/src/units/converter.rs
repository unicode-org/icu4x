// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::units::ratio::IcuRatio;

/// A converter for converting between two single or compound units.
/// For example:
///     1 - `meter` to `foot`
///     2 - `mile-per-gallon` to `liter-per-100-kilometer`.
///     3 - `celsius` to `fahrenheit`.
///
/// NOTE:
///     This converter does not support conversions between mixed units,
///     for example, from "meter" to "foot-and-inch".
#[derive(Debug, Clone)]
pub struct UnitsConverter(pub(crate) UnitsConverterInner);

impl UnitsConverter {
    /// Converts the given value from the input unit to the output unit.
    pub fn convert(&self, value: &IcuRatio) -> IcuRatio {
        self.0.convert(value)
    }

    /// Converts the given value in [`f64`] from the input unit to the output unit.
    /// NOTE:
    ///   The conversion using [`f64`] is not as accurate as the conversion using ratios.
    pub fn convert_f64(&self, value: f64) -> f64 {
        self.0.convert_f64(value)
    }
}

/// Enum containing all the of converters: Proportional, Reciprocal, and Offset converters as follows:
///    1 - Proportional: Converts between two units that are proportionally related (e.g. `meter` to `foot`).
///    2 - Reciprocal: Converts between two units that are reciprocal (e.g. `mile-per-gallon` to `liter-per-100-kilometer`).
///    3 - Offset: Converts between two units that require an offset (e.g. `celsius` to `fahrenheit`).
#[derive(Debug, Clone)]
pub(crate) enum UnitsConverterInner {
    Proportional(ProportionalConverter),
    Reciprocal(ReciprocalConverter),
    Offset(OffsetConverter),
}

impl UnitsConverterInner {
    /// Converts the given value from the input unit to the output unit based on the inner converter type.
    fn convert(&self, value: &IcuRatio) -> IcuRatio {
        match self {
            UnitsConverterInner::Proportional(converter) => converter.convert(value),
            UnitsConverterInner::Reciprocal(converter) => converter.convert(value),
            UnitsConverterInner::Offset(converter) => converter.convert(value),
        }
    }

    /// Converts the given value in [`f64`] from the input unit to the output unit based on the inner converter type.
    /// NOTE:
    ///    The conversion using [`f64`] is not as accurate as the conversion using ratios.
    fn convert_f64(&self, value: f64) -> f64 {
        match self {
            UnitsConverterInner::Proportional(converter) => converter.convert_f64(value),
            UnitsConverterInner::Reciprocal(converter) => converter.convert_f64(value),
            UnitsConverterInner::Offset(converter) => converter.convert_f64(value),
        }
    }
}

/// A converter for converting between two units that are reciprocal.
/// For example:
///    1 - `meter-per-second` to `second-per-meter`.
///    2 - `mile-per-gallon` to `liter-per-100-kilometer`.
#[derive(Debug, Clone)]
pub(crate) struct ReciprocalConverter {
    pub(crate) proportional: ProportionalConverter,
}

impl ReciprocalConverter {
    /// Converts the given value from the input unit to the output unit.
    pub(crate) fn convert(&self, value: &IcuRatio) -> IcuRatio {
        self.proportional.convert(value).recip()
    }

    /// Converts the given value in [`f64`] from the input unit to the output unit.
    /// NOTE:
    ///   The conversion using [`f64`] is not as accurate as the conversion using ratios.
    pub fn convert_f64(&self, value: f64) -> f64 {
        // TODO: handle the case when the conversion rate is zero or the value is zero.
        self.proportional.convert_f64(value).recip()
    }
}

/// A converter for converting between two units that require an offset.
#[derive(Debug, Clone)]
pub(crate) struct OffsetConverter {
    /// The proportional converter.
    pub(crate) proportional: ProportionalConverter,

    /// The offset value to be added to the result of the proportional converter.
    pub(crate) offset: IcuRatio,

    /// The offset value to be added to the result of the proportional converter in [`f64`].
    pub(crate) offset_f64: f64,
}

impl OffsetConverter {
    /// Converts the given value from the input unit to the output unit.
    pub(crate) fn convert(&self, value: &IcuRatio) -> IcuRatio {
        &self.proportional.convert(value) + &self.offset
    }

    /// Converts the given value in [`f64`] from the input unit to the output unit.
    /// NOTE:
    ///    The conversion using [`f64`] is not as accurate as the conversion using ratios.
    pub fn convert_f64(&self, value: f64) -> f64 {
        self.proportional.convert_f64(value) + self.offset_f64
    }
}

/// ProportionalConverter is responsible for converting between two units that are proportionally related.
/// For example: 1- `meter` to `foot`.
///              2- `square-meter` to `square-foot`.
///
/// However, it cannot convert between two units that are not proportionally related,
/// such as `celsius` to `fahrenheit` and `mile-per-gallon` to `liter-per-100-kilometer`.
///
/// Also, it cannot convert between two units that are not single, such as `meter` to `foot-and-inch`.
#[derive(Debug, Clone)]
pub(crate) struct ProportionalConverter {
    /// The conversion rate between the input and output units.
    pub(crate) conversion_rate: IcuRatio,

    /// The conversion rate between the input and output units in [`f64`].
    pub(crate) conversion_rate_f64: f64,
}

impl ProportionalConverter {
    /// Converts the given value from the input unit to the output unit.
    pub fn convert(&self, value: &IcuRatio) -> IcuRatio {
        &self.conversion_rate * value
    }

    /// Converts the given value in [`f64`] from the input unit to the output unit.
    /// NOTE:
    ///     The conversion using [`f64`] is not as accurate as the conversion using ratios.
    pub fn convert_f64(&self, value: f64) -> f64 {
        self.conversion_rate_f64 * value
    }
}
