// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    #[diplomat::opaque]
    /// An ICU4X Units Converter Factory object, capable of creating converters a [`ICU4XUnitsConverter`]
    /// from [`ICU4XMeasureUnit`]s.
    /// Also, it can parse the CLDR unit identifier (e.g. `meter-per-square-second`) and get the [`ICU4XMeasureUnit`].
    #[diplomat::rust_link(icu::experimental::units::converter_factory::ConverterFactory, Struct)]
    #[diplomat::rust_link(icu::experimental::units::converter_factory::ConverterFactory, Struct, hidden)]
    pub struct ICU4XUnitsConverterFactory(pub ConverterFactory);

    impl ICU4XUnitsConverterFactory {
        /// Creates a new [`ICU4XUnitsConverterFactory`] from supplemental data
        // (TODO: is it locale data? or something else?)
        #[diplomat::rust_link(
            icu::experimental::units::converter_factory::ConverterFactory::new,
            FnInStruct
        )]
        pub fn create() -> Result<Box<ICU4XUnitsConverterFactory>, ICU4XError> {
            Ok(Box::new(ICU4XUnitsConverterFactory(call_constructor!(
                ConverterFactory::new,
                ConverterFactory::try_new_with_any_provider,
                ConverterFactory::try_new_with_buffer_provider,
            )?)))
        }

        /// Creates a new [`ICU4XUnitsConverter`] from the input and output `ICU4XMeasureUnit`s.
        pub fn converter(
            &self,
            input_unit: &ICU4XMeasureUnit,
            output_unit: &ICU4XMeasureUnit,
        ) -> Option<Box<ICU4XUnitsConverter>> {
            self.0.converter(input_unit.0, output_unit.0)
        }

        /// Parses the CLDR unit identifier (e.g. `meter-per-square-second`) and returns the corresponding [`ICU4XMeasureUnit`].
        pub fn parse(&self, unit_id: &str) -> Result<ICU4XMeasureUnit, ICU4XError> {
            Ok(ICU4XMeasureUnit(call_method!(self.0, parse, unit_id)?))
        }
    }

    #[diplomat::opaque]
    /// An ICU4X Measurement Unit object which represents a single unit of measurement such as `meter`, `second`, `kilometer-per-hour`, `square-meter`, etc.
    #[diplomat::rust_link(icu::experimental::units::measureunit::MeasureUnit, Struct)]
    #[diplomat::rust_link(icu::experimental::units::measureunit::MeasureUnit, Struct, hidden)]
    pub struct ICU4XMeasureUnit(pub ConverterFactory);

    #[diplomat::opaque]
    /// An ICU4X Units Converter object, capable of converting between two [`ICU4XMeasureUnit`]s.
    #[diplomat::rust_link(icu::experimental::units::converter::UnitsConverter, Struct)]
    #[diplomat::rust_link(icu::experimental::units::converter::UnitsConverter, Struct, hidden)]
    pub struct ICU4XUnitsConverter(pub UnitsConverter);
}
