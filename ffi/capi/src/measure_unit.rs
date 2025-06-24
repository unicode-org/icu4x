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
    #[diplomat::rust_link(icu::experimental::measure::measureunit::MeasureUnit, Struct)]
    pub struct MeasureUnit(pub icu_experimental::measure::measureunit::MeasureUnit);

    impl MeasureUnit {
        #[diplomat::rust_link(
            icu::experimental::measure::measureunit::MeasureUnit::try_from_str,
            FnInStruct
        )]
        #[diplomat::attr(dart, rename = "from_str")]
        #[diplomat::attr(not(dart), constructor)]
        pub fn create_from_string(unit_id: &DiplomatStr) -> Option<Box<MeasureUnit>> {
            icu_experimental::measure::measureunit::MeasureUnit::try_from_utf8(unit_id)
                .ok()
                .map(MeasureUnit)
                .map(Box::new)
        }
    }
}
