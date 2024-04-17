// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_experimental::units::converter::UnitsConverter;
use icu_experimental::units::converter_factory::ConverterFactory;
use icu_experimental::units::measureunit::MeasureUnit;
use icu_experimental::units::measureunit::MeasureUnitParser;

#[diplomat::bridge]
pub mod ffi {
    use crate::{errors::ffi::ICU4XError, provider::ffi::ICU4XDataProvider};
    use alloc::boxed::Box;
    use core::str;

    #[diplomat::opaque]
    /// An ICU4X Units Converter Factory object, capable of creating converters a [`ICU4XUnitsConverter`]
    /// from [`ICU4XMeasureUnit`]s.
    /// Also, it can parse the CLDR unit identifier (e.g. `meter-per-square-second`) and get the [`ICU4XMeasureUnit`].
    #[diplomat::rust_link(icu::experimental::units::converter_factory::ConverterFactory, Struct)]
    #[diplomat::rust_link(
        icu::experimental::units::converter_factory::ConverterFactory,
        Struct,
        hidden
    )]
    pub struct ICU4XUnitsConverterFactory(pub ConverterFactory);

    impl ICU4XUnitsConverterFactory {
        /// Construct a new [`ICU4XUnitsConverterFactory`] instance.
        #[diplomat::rust_link(
            icu::experimental::units::converter_factory::ConverterFactory::new,
            FnInStruct
        )]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create(
            provider: &ICU4XDataProvider,
        ) -> Result<Box<ICU4XUnitsConverterFactory>, ICU4XError> {
            Ok(Box::new(ICU4XUnitsConverterFactory(call_constructor!(
                ConverterFactory::new [r => Ok(r)],
                ConverterFactory::try_new_with_any_provider,
                ConverterFactory::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Creates a new [`ICU4XUnitsConverter`] from the input and output `ICU4XMeasureUnit`s.
        /// Returns `None` if the conversion between the two units is not possible.
        /// For example, conversion between `meter` and `second` is not possible.
        pub fn converter(
            &self,
            from: &ICU4XMeasureUnit,
            to: &ICU4XMeasureUnit,
        ) -> Option<Box<ICU4XUnitsConverter>> {
            self.0.converter(input_unit.0, output_unit.0)
        }

        /// Parses the CLDR unit identifier (e.g. `meter-per-square-second`) and returns the corresponding [`ICU4XMeasureUnit`].
        pub fn parser(&self) -> Result<Box<ICU4XMeasureUnitParser>, ICU4XError> {
            self.0.parser().map(Box::new)
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Measurement Unit parser object which is capable of parsing the CLDR unit identifier
    /// (e.g. `meter-per-square-second`) and get the [`ICU4XMeasureUnit`].
    #[diplomat::rust_link(icu::experimental::units::measureunit::MeasureUnitParser, Struct)]
    #[diplomat::rust_link(
        icu::experimental::units::measureunit::MeasureUnitParser,
        Struct,
        hidden
    )]
    pub struct ICU4XMeasureUnitParser(pub MeasureUnitParser);

    impl ICU4XMeasureUnitParser {
        /// Parses the CLDR unit identifier (e.g. `meter-per-square-second`) and returns the corresponding [`ICU4XMeasureUnit`].
        /// Returns an error if the unit identifier is not valid.
        pub fn parse_measure_unit(
            &self,
            unit_id: &str,
        ) -> Result<Box<ICU4XMeasureUnit>, ICU4XError> {
            self.0.try_from_identifier(unit_id).map(Box::new)
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Measurement Unit object which represents a single unit of measurement such as `meter`, `second`, `kilometer-per-hour`, `square-meter`, etc.
    #[diplomat::rust_link(icu::experimental::units::measureunit::MeasureUnit, Struct)]
    #[diplomat::rust_link(icu::experimental::units::measureunit::MeasureUnit, Struct, hidden)]
    pub struct ICU4XMeasureUnit(pub MeasureUnit);

    #[diplomat::opaque]
    /// An ICU4X Units Converter object, capable of converting between two [`ICU4XMeasureUnit`]s.
    #[diplomat::rust_link(icu::experimental::units::converter::UnitsConverter, Struct)]
    #[diplomat::rust_link(icu::experimental::units::converter::UnitsConverter, Struct, hidden)]
    pub struct ICU4XUnitsConverter(pub UnitsConverter);

    impl ICU4XUnitsConverter {
        /// Converts the input value from the input unit to the output unit.
        /// NOTE:
        ///   The conversion using float is not as accurate as the conversion using ratios.
        pub fn convert_f64(&self, input: f64) -> f64 {
            self.0.convert_f64(input)
        }
    }
}
