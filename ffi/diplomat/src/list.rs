// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::{
        errors::ffi::ICU4XError, locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider,
    };
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec::Vec;
    use diplomat_runtime::{DiplomatResult, DiplomatWriteable};
    use icu_list::{ListFormatter, ListStyle};
    use writeable::Writeable;

    /// A list of strings
    #[diplomat::opaque]
    pub struct ICU4XList(pub Vec<String>);

    impl ICU4XList {
        /// Create a new list of strings
        pub fn new() -> Box<ICU4XList> {
            Box::new(ICU4XList(Vec::new()))
        }

        /// Create a new list of strings with preallocated space to hold
        /// at least `capacity` elements
        pub fn with_capacity(capacity: usize) -> Box<ICU4XList> {
            Box::new(ICU4XList(Vec::with_capacity(capacity)))
        }

        /// Push a string to the list
        ///
        /// For C++ users, potentially invalid UTF8 will be handled via
        /// REPLACEMENT CHARACTERs
        pub fn push(&mut self, val: &str) {
            let val = val.as_bytes(); // #2520
            self.0.push(String::from_utf8_lossy(val).into_owned());
        }

        /// The number of elements in this list
        #[allow(clippy::len_without_is_empty)] // don't need to follow Rust conventions over FFI
        pub fn len(&self) -> usize {
            self.0.len()
        }
    }

    #[diplomat::rust_link(icu::list::ListStyle, Enum)]
    #[diplomat::enum_convert(ListStyle, needs_wildcard)]
    pub enum ICU4XListStyle {
        Wide,
        Short,
        Narrow,
    }
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::list::ListFormatter, Struct)]
    pub struct ICU4XListFormatter(pub ListFormatter);

    impl ICU4XListFormatter {
        /// Construct a new ICU4XListFormatter instance for And patterns
        #[diplomat::rust_link(icu::normalizer::ListFormatter::try_new_and_unstable, FnInStruct)]
        pub fn try_new_and(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            style: ICU4XListStyle,
        ) -> DiplomatResult<Box<ICU4XListFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();
            ListFormatter::try_new_and_unstable(&provider.0, &locale, style.into())
                .map(|o| Box::new(ICU4XListFormatter(o)))
                .map_err(Into::into)
                .into()
        }
        /// Construct a new ICU4XListFormatter instance for And patterns
        #[diplomat::rust_link(icu::normalizer::ListFormatter::try_new_or_unstable, FnInStruct)]
        pub fn try_new_or(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            style: ICU4XListStyle,
        ) -> DiplomatResult<Box<ICU4XListFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();
            ListFormatter::try_new_or_unstable(&provider.0, &locale, style.into())
                .map(|o| Box::new(ICU4XListFormatter(o)))
                .map_err(Into::into)
                .into()
        }
        /// Construct a new ICU4XListFormatter instance for And patterns
        #[diplomat::rust_link(icu::normalizer::ListFormatter::try_new_unit_unstable, FnInStruct)]
        pub fn try_new_unit(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            style: ICU4XListStyle,
        ) -> DiplomatResult<Box<ICU4XListFormatter>, ICU4XError> {
            let locale = locale.to_datalocale();
            ListFormatter::try_new_unit_unstable(&provider.0, &locale, style.into())
                .map(|o| Box::new(ICU4XListFormatter(o)))
                .map_err(Into::into)
                .into()
        }

        #[diplomat::rust_link(icu::normalizer::ListFormatter::format, FnInStruct)]
        pub fn format(
            &self,
            list: &ICU4XList,
            write: &mut DiplomatWriteable,
        ) -> DiplomatResult<(), ICU4XError> {
            let formatted = self.0.format(list.0.iter());
            let result = formatted.write_to(write).map_err(Into::into);
            write.flush();
            result.into()
        }
    }
}
