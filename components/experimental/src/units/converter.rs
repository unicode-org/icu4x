// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::units::convertible::Convertible;

/// A converter for converting between two single or compound units.
/// For example:
///     1 - `meter` to `foot`
///     2 - `mile-per-gallon` to `liter-per-100-kilometer`.
///     3 - `celsius` to `fahrenheit`.
///
/// NOTE:
///     This converter does not support conversions between mixed units,
///     for example, from "meter" to "foot-and-inch".
///
/// # Examples
///
/// ```
/// use icu_experimental::units::converter_factory::ConverterFactory;
/// use icu_experimental::measure::measureunit::MeasureUnit;
///
/// let factory = ConverterFactory::new();
/// let meter = MeasureUnit::try_from_str("meter").unwrap();
/// let foot = MeasureUnit::try_from_str("foot").unwrap();
///
/// // 10 meters is approximately 32.8084 feet.
/// // We use approximate comparison due to floating-point precision.
/// let converter = factory.converter::<f64>(&meter, &foot).unwrap();
/// assert!((converter.convert(&10.0) - 32.8084).abs() < 1e-4);
/// ```
///
/// High-precision conversion using `Ratio<BigInt>`:
///
/// ```
/// use icu_experimental::units::converter_factory::ConverterFactory;
/// use icu_experimental::measure::measureunit::MeasureUnit;
/// use num_bigint::BigInt;
/// use num_rational::Ratio;
///
/// let factory = ConverterFactory::new();
/// let meter = MeasureUnit::try_from_str("meter").unwrap();
/// let foot = MeasureUnit::try_from_str("foot").unwrap();
///
/// // 1 foot is exactly 0.3048 meters.
/// // 1 meter is 10000 / 3048 feet = 1250 / 381 feet.
/// let converter = factory.converter::<Ratio<BigInt>>(&meter, &foot).unwrap();
///
/// // 1 meter should be exactly 1250/381 feet.
/// let one = Ratio::from(BigInt::from(1));
/// let expected = Ratio::new(BigInt::from(1250), BigInt::from(381));
/// assert_eq!(converter.convert(&one), expected);
/// ```
#[derive(Debug, Clone)]
pub struct UnitsConverter<N>(pub(crate) UnitsConverterInner<N>)
where
    N: Convertible;

impl<N> UnitsConverter<N>
where
    N: Convertible,
{
    /// Converts the given value from the input unit to the output unit.
    ///
    ///
    /// See [`UnitsConverter`] for examples.
    pub fn convert(&self, value: &N) -> N {
        self.0.convert(value)
    }
}

/// Enum containing all the of converters: Proportional, Reciprocal, and Offset converters as follows:
///    1 - Proportional: Converts between two units that are proportionally related (e.g. `meter` to `foot`).
///    2 - Reciprocal: Converts between two units that are reciprocal (e.g. `mile-per-gallon` to `liter-per-100-kilometer`).
///    3 - Offset: Converts between two units that require an offset (e.g. `celsius` to `fahrenheit`).
#[derive(Debug, Clone)]
pub(crate) enum UnitsConverterInner<N>
where
    N: Convertible,
{
    Proportional(ProportionalConverter<N>),
    Reciprocal(ReciprocalConverter<N>),
    Offset(OffsetConverter<N>),
}

impl<N> UnitsConverterInner<N>
where
    N: Convertible,
{
    /// Converts the given value from the input unit to the output unit based on the inner converter type.
    fn convert(&self, value: &N) -> N {
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
#[derive(Debug, Clone)]
pub(crate) struct ReciprocalConverter<N>
where
    N: Convertible,
{
    pub(crate) proportional: ProportionalConverter<N>,
}

impl<N> ReciprocalConverter<N>
where
    N: Convertible,
{
    /// Converts the given value from the input unit to the output unit.
    pub(crate) fn convert(&self, value: &N) -> N {
        self.proportional.convert(value).reciprocal()
    }
}

/// A converter for converting between two units that require an offset.
#[derive(Debug, Clone)]
pub(crate) struct OffsetConverter<N>
where
    N: Convertible,
{
    /// The proportional converter.
    pub(crate) proportional: ProportionalConverter<N>,

    /// The offset value to be added to the result of the proportional converter.
    pub(crate) offset: N,
}

impl<N> OffsetConverter<N>
where
    N: Convertible,
{
    /// Converts the given value from the input unit to the output unit.
    pub(crate) fn convert(&self, value: &N) -> N {
        self.proportional.convert(value).add_refs(&self.offset)
    }
}

/// `ProportionalConverter` is responsible for converting between two units that are proportionally related.
/// For example: 1- `meter` to `foot`.
///              2- `square-meter` to `square-foot`.
///
/// However, it cannot convert between two units that are not proportionally related,
/// such as `celsius` to `fahrenheit` and `mile-per-gallon` to `liter-per-100-kilometer`.
///
/// Also, it cannot convert between two units that are not single, such as `meter` to `foot-and-inch`.
#[derive(Debug, Clone)]
pub(crate) struct ProportionalConverter<N>
where
    N: Convertible,
{
    /// The conversion rate between the input and output units.
    pub(crate) conversion_rate: N,
}

impl<N> ProportionalConverter<N>
where
    N: Convertible,
{
    /// Converts the given value from the input unit to the output unit.
    pub fn convert(&self, value: &N) -> N {
        value.mul_refs(&self.conversion_rate)
    }
}
