// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    #[cfg(feature = "buffer_provider")]
    use crate::unstable::errors::ffi::DataError;
    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;

    #[diplomat::opaque]
    /// An ICU4X Erased Measurement Unit object which represents a single unit of measurement
    /// such as `meter`, `second`, `kilometer-per-hour`, `square-meter`, etc.
    ///
    /// You can create an instance of this object using [`ErasedMeasureUnitParser`] by calling the `parse` method.
    #[diplomat::rust_link(icu::experimental::measure::measureunit::ErasedMeasureUnit, Struct)]
    pub struct ErasedMeasureUnit(pub icu_experimental::measure::measureunit::ErasedMeasureUnit);

    #[diplomat::opaque]
    /// An ICU4X Erased Measure Unit Parser object, capable of parsing the CLDR unit identifier (e.g. `meter-per-square-second`) and get the [`ErasedMeasureUnit`].
    #[diplomat::rust_link(icu::experimental::measure::parser::ErasedMeasureUnitParser, Struct)]
    pub struct ErasedMeasureUnitParser(
        pub icu_experimental::measure::parser::ErasedMeasureUnitParser,
    );

    impl ErasedMeasureUnitParser {
        /// Construct a new [`ErasedMeasureUnitParser`] instance using compiled data.
        #[diplomat::rust_link(
            icu::experimental::measure::parser::ErasedMeasureUnitParser::new,
            FnInStruct
        )]
        #[diplomat::attr(auto, constructor)]
        #[cfg(feature = "compiled_data")]
        pub fn create() -> Box<ErasedMeasureUnitParser> {
            Box::new(ErasedMeasureUnitParser(
                icu_experimental::measure::parser::ErasedMeasureUnitParser::default(),
            ))
        }
        /// Construct a new [`ErasedMeasureUnitParser`] instance using a particular data source.
        #[diplomat::rust_link(
            icu::experimental::measure::parser::ErasedMeasureUnitParser::new,
            FnInStruct
        )]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_with_provider(
            provider: &DataProvider,
        ) -> Result<Box<ErasedMeasureUnitParser>, DataError> {
            Ok(Box::new(ErasedMeasureUnitParser(
                icu_experimental::measure::parser::ErasedMeasureUnitParser::try_new_with_buffer_provider(
                    provider.get()?,
                )?,
            )))
        }

        #[diplomat::rust_link(
            icu::experimental::measure::parser::ErasedMeasureUnitParser::parse,
            FnInStruct
        )]
        pub fn parse(&self, unit_id: &DiplomatStr) -> Option<Box<ErasedMeasureUnit>> {
            self.0
                .try_from_utf8(unit_id)
                .ok()
                .map(ErasedMeasureUnit)
                .map(Box::new)
        }
    }
}
