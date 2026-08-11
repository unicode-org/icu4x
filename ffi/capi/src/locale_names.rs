#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::unstable::locale_core::ffi::Locale;
    use crate::unstable::errors::ffi::DataError;
    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;

    #[diplomat::opaque]
    pub struct LocaleNamesUnstable;

    impl LocaleNamesUnstable {
        #[cfg(feature = "compiled_data")]
        pub fn for_region_with_compiled_data_light(locale: &Locale, region: &DiplomatStr, write: &mut diplomat_runtime::DiplomatWrite) -> Result<(), DataError> {
            todo!()
        }

        #[cfg(feature = "buffer_provider")]
        #[diplomat::attr(demo_gen, disable)]
        pub fn for_region_with_provider_light(provider: &DataProvider, locale: &Locale, region: &DiplomatStr, write: &mut diplomat_runtime::DiplomatWrite) -> Result<(), DataError> {
            todo!()
        }

        // TODO: Fill this in for script, variant, and language identifier.
    }
}
