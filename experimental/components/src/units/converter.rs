// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).



use num_bigint::BigInt;
use num_rational::Ratio;




/// A converter for converting between two single or compound units.
/// For example:
///     1 - `meter` to `foot`
///     2 - `mile-per-gallon` to `liter-per-100-kilometer`.
///     3 - `celsius` to `fahrenheit`.
///
/// NOTE:
///     This converter does not support conversions between mixed units,
///     for example, from "meter" to "foot-and-inch".
pub struct UnitsConverter(pub(crate) UnitsConverterInner);

impl UnitsConverter {
    /// Converts the given value from the input unit to the output unit.
    pub fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        self.0.convert(value)
    }
}

/// Enum containing different types of converters: Proportional, Reciprocal, and Offset converters.
#[derive(Debug)]
pub(crate) enum UnitsConverterInner {
    Proportional(ProportionalConverter),
    Reciprocal(ReciprocalConverter),
    Offset(OffsetConverter),
}

impl UnitsConverterInner {
    /// Converts the given value from the input unit to the output unit based on the inner converter type.
    fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        match self {
            UnitsConverterInner::Proportional(converter) => converter.convert(value),
            UnitsConverterInner::Reciprocal(converter) => converter.convert(value),
            UnitsConverterInner::Offset(converter) => converter.convert(value),
        }
    }
}

/// A converter for converting between two units that are reciprocal.
/// For example:
///    1 - `meter-per-second` to `second-per-meter`.
///    2 - `mile-per-gallon` to `liter-per-100-kilometer`.
#[derive(Debug)]
pub(crate) struct ReciprocalConverter {
    pub(crate) proportional: ProportionalConverter,
}

impl ReciprocalConverter {
    /// Converts the given value from the input unit to the output unit.
    pub(crate) fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        self.proportional.convert(value).recip()
    }
}

/// A converter for converting between two units that require an offset.
#[derive(Debug)]
pub(crate) struct OffsetConverter {
    /// The proportional converter.
    pub(crate) proportional: ProportionalConverter,

    /// The offset value to be added to the result of the proportional converter.
    pub(crate) offset: Ratio<BigInt>,
}

impl OffsetConverter {
    /// Converts the given value from the input unit to the output unit.
    pub(crate) fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        let result = self.proportional.convert(value);
        result + &self.offset
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
#[derive(Debug)]
pub(crate) struct ProportionalConverter {
    // TODO(#4554): Implement a New Struct `IcuRatio` to Encapsulate `Ratio<BigInt>`.
    /// The conversion rate between the input and output units.
    pub(crate) conversion_rate: Ratio<BigInt>,
}

impl ProportionalConverter {
    /// Converts the given value from the input unit to the output unit.
    pub fn convert(&self, value: &Ratio<BigInt>) -> Ratio<BigInt> {
        value * &self.conversion_rate
    }
}
