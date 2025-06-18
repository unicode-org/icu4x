// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    #[diplomat::opaque]
    /// An ICU4X Measurement Unit object which represents a single unit of measurement
    /// such as `meter`, `second`, `kilometer-per-hour`, `square-meter`, etc.
    ///
    /// You can create an instance of this object using [`MeasureUnitParser`] by calling the `parse` method.
    #[diplomat::rust_link(icu::experimental::measure::measureunit::MeasureUnit, Struct)]
    pub struct MeasureUnit(pub icu_experimental::measure::measureunit::MeasureUnit);

    #[diplomat::opaque]
    /// An ICU4X Measure Unit Parser object, capable of parsing the CLDR unit identifier (e.g. `meter-per-square-second`) and get the [`MeasureUnit`].
    #[diplomat::rust_link(icu::experimental::measure::parser::MeasureUnitParser, Struct)]
    pub struct MeasureUnitParser(pub icu_experimental::measure::parser::MeasureUnitParser);

    impl MeasureUnitParser {
        #[diplomat::rust_link(
            icu::experimental::measure::parser::MeasureUnitParser::parse,
            FnInStruct
        )]
        pub fn parse(unit_id: &DiplomatStr) -> Option<Box<MeasureUnit>> {
            icu_experimental::measure::parser::MeasureUnitParser::try_from_utf8(unit_id)
                .ok()
                .map(MeasureUnit)
                .map(Box::new)
        }
    }
}
