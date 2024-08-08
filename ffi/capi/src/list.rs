// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::{DiplomatStr16Slice, DiplomatStrSlice};

    use crate::{errors::ffi::DataError, locale_core::ffi::Locale, provider::ffi::DataProvider};

    use writeable::Writeable;

    #[diplomat::rust_link(icu::list::ListLength, Enum)]
    #[diplomat::enum_convert(icu_list::ListLength, needs_wildcard)]
    pub enum ListLength {
        Wide,
        Short,
        Narrow,
    }
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::list::ListFormatter, Struct)]
    pub struct ListFormatter(pub icu_list::ListFormatter);

    impl ListFormatter {
        /// Construct a new ListFormatter instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_and_with_length, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "and_with_length")]
        #[diplomat::demo(default_constructor)]
        pub fn create_and_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ListFormatter(call_constructor!(
                icu_list::ListFormatter::try_new_and_with_length,
                icu_list::ListFormatter::try_new_and_with_length_with_any_provider,
                icu_list::ListFormatter::try_new_and_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }
        /// Construct a new ListFormatter instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_or_with_length, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "or_with_length")]
        pub fn create_or_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ListFormatter(call_constructor!(
                icu_list::ListFormatter::try_new_or_with_length,
                icu_list::ListFormatter::try_new_or_with_length_with_any_provider,
                icu_list::ListFormatter::try_new_or_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }
        /// Construct a new ListFormatter instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_unit_with_length, FnInStruct)]
        #[diplomat::attr(supports = fallible_constructors, named_constructor = "unit_with_length")]
        pub fn create_unit_with_length(
            provider: &DataProvider,
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let locale = locale.to_datalocale();
            Ok(Box::new(ListFormatter(call_constructor!(
                icu_list::ListFormatter::try_new_unit_with_length,
                icu_list::ListFormatter::try_new_unit_with_length_with_any_provider,
                icu_list::ListFormatter::try_new_unit_with_length_with_buffer_provider,
                provider,
                &locale,
                length.into()
            )?)))
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::ListFormatter::format_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(*, rename = "format")]
        pub fn format_utf8(&self, list: &[DiplomatStrSlice], write: &mut DiplomatWrite) {
            let _infallible = self
                .0
                .format(
                    list.iter()
                        .copied()
                        .map(crate::utf::PotentiallyInvalidUtf8)
                        .map(crate::utf::LossyWrap),
                )
                .write_to(write);
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::ListFormatter::format_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::attr(not(supports = utf8_strings), rename = "format")]
        #[diplomat::attr(supports = utf8_strings, rename = "format16")]
        pub fn format_utf16(&self, list: &[DiplomatStr16Slice], write: &mut DiplomatWrite) {
            let _infallible = self
                .0
                .format(
                    list.iter()
                        .copied()
                        .map(crate::utf::PotentiallyInvalidUtf16)
                        .map(crate::utf::LossyWrap),
                )
                .write_to(write);
        }
    }
}
