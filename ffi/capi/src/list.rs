// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;
    use alloc::string::String;
    use alloc::vec::Vec;

    use crate::{errors::ffi::DataError, locale_core::ffi::Locale, provider::ffi::DataProvider};

    use writeable::Writeable;

    /// A list of strings
    #[diplomat::opaque]
    #[diplomat::attr(*, disable)]
    pub struct List(pub Vec<String>);

    impl List {
        /// Create a new list of strings
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create() -> Box<List> {
            Box::new(List(Vec::new()))
        }

        /// Create a new list of strings with preallocated space to hold
        /// at least `capacity` elements
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "with_capacity")]
        pub fn create_with_capacity(capacity: usize) -> Box<List> {
            Box::new(List(Vec::with_capacity(capacity)))
        }

        /// Push a string to the list
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        pub fn push(&mut self, val: &DiplomatStr) {
            self.0.push(String::from_utf8_lossy(val).into_owned());
        }

        /// The number of elements in this list
        #[diplomat::attr(supports = accessors, getter = "length")]
        pub fn len(&self) -> usize {
            self.0.len()
        }

        /// Whether this list is empty
        #[diplomat::attr(supports = accessors, getter)]
        pub fn is_empty(&self) -> bool {
            self.0.is_empty()
        }
    }

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
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "and_with_length")]
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
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "or_with_length")]
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
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors, supports = named_constructors), named_constructor = "unit_with_length")]
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
        #[diplomat::attr(*, disable)]
        pub fn format(&self, list: &List, write: &mut DiplomatWrite) {
            let _infallible = self.0.format(list.0.iter()).write_to(write);
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::ListFormatter::format_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::attr(dart, disable)]
        #[diplomat::skip_if_ast]
        pub fn format_valid_utf8(&self, list: &[&str], write: &mut DiplomatWrite) {
            let _infallible = self.0.format(list.iter()).write_to(write);
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::ListFormatter::format_to_string, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::attr(dart, disable)]
        #[diplomat::skip_if_ast]
        pub fn format_utf8(&self, list: &[&DiplomatStr], write: &mut DiplomatWrite) {
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
        #[diplomat::attr(dart, rename = "format")]
        #[diplomat::skip_if_ast]
        pub fn format_utf16(&self, list: &[&DiplomatStr16], write: &mut DiplomatWrite) {
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
