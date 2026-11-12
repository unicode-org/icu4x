// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
pub mod ffi {
    use alloc::boxed::Box;
    use diplomat_runtime::{DiplomatStr16Slice, DiplomatStrSlice};
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use icu_list::{ListFormatterPreferences, options::ListFormatterOptions};

    #[cfg(feature = "unstable")]
    use crate::unstable::parts::ffi::Parts;
    #[cfg(feature = "buffer_provider")]
    use crate::unstable::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::unstable::{errors::ffi::DataError, locale_core::ffi::Locale};

    use writeable::Writeable;

    #[diplomat::rust_link(icu::list::options::ListLength, Enum)]
    #[diplomat::enum_convert(icu_list::options::ListLength, needs_wildcard)]
    #[non_exhaustive]
    pub enum ListLength {
        #[diplomat::attr(auto, default)]
        Wide,
        Short,
        Narrow,
    }
    #[diplomat::opaque]
    #[diplomat::rust_link(icu::list::ListFormatter, Struct)]
    pub struct ListFormatter(pub icu_list::ListFormatter);

    impl ListFormatter {
        /// Construct a new `ListFormatter` instance for And patterns from compiled data.
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_and, FnInStruct)]
        #[diplomat::rust_link(icu::list::options::ListFormatterOptions, Struct, hidden)]
        #[diplomat::rust_link(
            icu::list::options::ListFormatterOptions::with_length,
            FnInStruct,
            hidden
        )]
        #[diplomat::rust_link(
            icu::list::options::ListFormatterOptions::default,
            FnInStruct,
            hidden
        )]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "and_with_length")]
        #[cfg(feature = "compiled_data")]
        #[diplomat::demo(default_constructor)]
        pub fn create_and_with_length(
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let prefs = ListFormatterPreferences::from(&locale.0);
            let options = ListFormatterOptions::default().with_length(length.into());
            Ok(Box::new(ListFormatter(
                icu_list::ListFormatter::try_new_and(prefs, options)?,
            )))
        }

        /// Construct a new `ListFormatter` instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_and, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "and_with_length_and_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_and_with_length_and_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let prefs = ListFormatterPreferences::from(&locale.0);
            let options = ListFormatterOptions::default().with_length(length.into());
            Ok(Box::new(ListFormatter(
                icu_list::ListFormatter::try_new_and_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?,
            )))
        }

        /// Construct a new `ListFormatter` instance for And patterns from compiled data.
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_or, FnInStruct)]
        #[diplomat::rust_link(icu::list::options::ListFormatterOptions, Struct, hidden)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "or_with_length")]
        #[cfg(feature = "compiled_data")]
        pub fn create_or_with_length(
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let prefs = ListFormatterPreferences::from(&locale.0);
            let options = ListFormatterOptions::default().with_length(length.into());
            Ok(Box::new(ListFormatter(
                icu_list::ListFormatter::try_new_or(prefs, options)?,
            )))
        }

        /// Construct a new `ListFormatter` instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_or, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "or_with_length_and_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_or_with_length_and_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let prefs = ListFormatterPreferences::from(&locale.0);
            let options = ListFormatterOptions::default().with_length(length.into());
            Ok(Box::new(ListFormatter(
                icu_list::ListFormatter::try_new_or_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?,
            )))
        }

        /// Construct a new `ListFormatter` instance for And patterns from compiled data.
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_unit, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "unit_with_length")]
        #[cfg(feature = "compiled_data")]
        pub fn create_unit_with_length(
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let prefs = ListFormatterPreferences::from(&locale.0);
            let options = ListFormatterOptions::default().with_length(length.into());
            Ok(Box::new(ListFormatter(
                icu_list::ListFormatter::try_new_unit(prefs, options)?,
            )))
        }

        /// Construct a new `ListFormatter` instance for And patterns
        #[diplomat::rust_link(icu::list::ListFormatter::try_new_unit, FnInStruct)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "unit_with_length_and_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_unit_with_length_and_provider(
            provider: &DataProvider,
            locale: &Locale,
            length: ListLength,
        ) -> Result<Box<ListFormatter>, DataError> {
            let prefs = ListFormatterPreferences::from(&locale.0);
            let options = ListFormatterOptions::default().with_length(length.into());
            Ok(Box::new(ListFormatter(
                icu_list::ListFormatter::try_new_unit_with_buffer_provider(
                    provider.get()?,
                    prefs,
                    options,
                )?,
            )))
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
                        .map(|a| potential_utf::PotentialUtf8::from_bytes(a))
                        .map(writeable::adapters::LossyWrap),
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
                        .map(|a| potential_utf::PotentialUtf16::from_slice(a))
                        .map(writeable::adapters::LossyWrap),
                )
                .write_to(write);
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::rust_link(icu::list::parts, Mod, hidden)]
        #[diplomat::rust_link(icu::list::parts::ELEMENT, Constant, hidden)]
        #[diplomat::rust_link(icu::list::parts::LITERAL, Constant, hidden)]
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(*, rename = "format_to_parts")]
        #[cfg(feature = "unstable")]
        pub fn format_utf8_to_parts(
            &self,
            list: &[DiplomatStrSlice],
            parts: &mut Parts,
            write: &mut DiplomatWrite,
        ) {
            let _infallible = crate::unstable::parts::write_to_parts(
                &self.0.format(
                    list.iter()
                        .map(|a| potential_utf::PotentialUtf8::from_bytes(a))
                        .map(writeable::adapters::LossyWrap),
                ),
                parts,
                write,
            );
        }

        #[diplomat::rust_link(icu::list::ListFormatter::format, FnInStruct)]
        #[diplomat::rust_link(icu::list::FormattedList, Struct, hidden)]
        #[diplomat::rust_link(icu::list::parts, Mod, hidden)]
        #[diplomat::rust_link(icu::list::parts::ELEMENT, Constant, hidden)]
        #[diplomat::rust_link(icu::list::parts::LITERAL, Constant, hidden)]
        #[diplomat::attr(not(supports = utf8_strings), rename = "format_to_parts")]
        #[diplomat::attr(supports = utf8_strings, rename = "format_to_parts16")]
        #[diplomat::cfg(supports = utf16_writeables)]
        #[diplomat::attr(demo_gen, disable)]
        #[cfg(feature = "unstable")]
        pub fn format_utf16_to_parts(
            &self,
            list: &[DiplomatStr16Slice],
            parts: &mut Parts,
            write: &mut DiplomatWrite16,
        ) {
            let _infallible = crate::unstable::parts::write_to_parts_utf16(
                &self.0.format(
                    list.iter()
                        .map(|a| potential_utf::PotentialUtf16::from_slice(a))
                        .map(writeable::adapters::LossyWrap),
                ),
                parts,
                write,
            );
        }
    }
}

#[cfg(all(test, feature = "compiled_data", feature = "unstable"))]
mod tests {
    use super::ffi::*;
    use crate::unstable::locale_core::ffi::Locale;
    use crate::unstable::parts::ffi::{PartKind, PartSpan, Parts};
    use alloc::vec::Vec;
    use diplomat_runtime::{
        DiplomatStr16Slice, DiplomatStrSlice,
        rust_interop::{RustWriteVec, RustWriteVecGeneric},
    };

    #[test]
    fn test_list_formatter_format_to_parts() {
        let locale = Locale::from_string(b"en").unwrap();
        let formatter = ListFormatter::create_and_with_length(&locale, ListLength::Wide).unwrap();
        let mut parts = Parts::create();
        let mut write = RustWriteVec::with_capacity(0);

        let items = [
            DiplomatStrSlice::from(b"Alice" as &[u8]),
            DiplomatStrSlice::from(b"Bob" as &[u8]),
            DiplomatStrSlice::from("Céline".as_bytes()),
        ];

        formatter.format_utf8_to_parts(&items, &mut parts, unsafe { write.borrow_mut() });
        let out = core::str::from_utf8(write.borrow().as_bytes()).unwrap();

        assert_eq!(out, "Alice, Bob, and Céline");
        let expected = [
            PartSpan::new(0, 5, PartKind::ListElement),
            PartSpan::new(5, 7, PartKind::ListLiteral),
            PartSpan::new(7, 10, PartKind::ListElement),
            PartSpan::new(10, 16, PartKind::ListLiteral),
            PartSpan::new(16, 23, PartKind::ListElement),
        ];
        assert_eq!(parts.len(), expected.len());
        for (i, expected_span) in expected.iter().copied().enumerate() {
            assert_eq!(parts.get(i), Some(expected_span));
        }
        assert_eq!(parts.get(expected.len()), None);
        assert_eq!(&out[0..5], "Alice");
        assert_eq!(&out[5..7], ", ");
        assert_eq!(&out[7..10], "Bob");
        assert_eq!(&out[10..16], ", and ");
        assert_eq!(&out[16..23], "Céline");
    }

    #[test]
    fn test_list_formatter_format_utf16_to_parts() {
        let locale = Locale::from_string(b"en").unwrap();
        let formatter = ListFormatter::create_and_with_length(&locale, ListLength::Wide).unwrap();
        let mut parts = Parts::create();
        let mut write = RustWriteVecGeneric::<u16>::with_capacity(0);

        let item0: Vec<u16> = "Alice".encode_utf16().collect();
        let item1: Vec<u16> = "Bob".encode_utf16().collect();
        let item2: Vec<u16> = "Céline 🥳".encode_utf16().collect();
        let items = [
            DiplomatStr16Slice::from(item0.as_slice()),
            DiplomatStr16Slice::from(item1.as_slice()),
            DiplomatStr16Slice::from(item2.as_slice()),
        ];

        formatter.format_utf16_to_parts(&items, &mut parts, unsafe { write.borrow_mut() });
        let out = write.as_slice();
        let expected_out: Vec<u16> = "Alice, Bob, and Céline 🥳".encode_utf16().collect();
        assert_eq!(out, expected_out.as_slice());

        // "Céline 🥳" is 9 UTF-16 code units (6 for "Céline" + 1 space + 2 for surrogate pair U+1F973)
        let expected = [
            PartSpan::new(0, 5, PartKind::ListElement),
            PartSpan::new(5, 7, PartKind::ListLiteral),
            PartSpan::new(7, 10, PartKind::ListElement),
            PartSpan::new(10, 16, PartKind::ListLiteral),
            PartSpan::new(16, 25, PartKind::ListElement),
        ];
        assert_eq!(parts.len(), expected.len());
        for (i, expected_span) in expected.iter().copied().enumerate() {
            assert_eq!(parts.get(i), Some(expected_span));
        }
        assert_eq!(&out[16..25], item2.as_slice());
    }
}
