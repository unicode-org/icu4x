// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use crate::{
        errors::ffi::ICU4XError, locale::ffi::ICU4XLocale, provider::ffi::ICU4XDataProvider,
    };
    use alloc::boxed::Box;
    use diplomat_runtime::DiplomatWriteable;
    use icu_casemap::{CaseMapCloser, CaseMapper};
    use writeable::Writeable;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::casemap::CaseMapper, Struct)]
    pub struct ICU4XCaseMapper(pub CaseMapper);

    impl ICU4XCaseMapper {
        /// Construct a new ICU4XCaseMapper instance for NFC
        #[diplomat::rust_link(icu::casemap::CaseMapper::new, FnInStruct)]
        pub fn create(provider: &ICU4XDataProvider) -> Result<Box<ICU4XCaseMapper>, ICU4XError> {
            Ok(Box::new(ICU4XCaseMapper(call_constructor!(
                CaseMapper::new [r => Ok(r)],
                CaseMapper::try_new_with_any_provider,
                CaseMapper::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Returns the full lowercase mapping of the given string
        #[diplomat::rust_link(icu::casemap::CaseMapper::lowercase, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapper::lowercase_to_string, FnInStruct, hidden)]
        pub fn lowercase(
            &self,
            s: &str,
            locale: &ICU4XLocale,
            write: &mut DiplomatWriteable,
        ) -> Result<(), ICU4XError> {
            // #2520
            // In the future we should be able to make assumptions based on backend
            core::str::from_utf8(s.as_bytes())
                .map_err(|e| ICU4XError::DataIoError.log_original(&e))?;
            self.0.lowercase(s, &locale.0.id).write_to(write)?;

            Ok(())
        }

        /// Returns the full uppercase mapping of the given string
        #[diplomat::rust_link(icu::casemap::CaseMapper::uppercase, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapper::uppercase_to_string, FnInStruct, hidden)]
        pub fn uppercase(
            &self,
            s: &str,
            locale: &ICU4XLocale,
            write: &mut DiplomatWriteable,
        ) -> Result<(), ICU4XError> {
            // #2520
            // In the future we should be able to make assumptions based on backend
            core::str::from_utf8(s.as_bytes())
                .map_err(|e| ICU4XError::DataIoError.log_original(&e))?;
            self.0.uppercase(s, &locale.0.id).write_to(write)?;

            Ok(())
        }

        /// Returns the full titlecase mapping of the given string
        #[diplomat::rust_link(icu::casemap::CaseMapper::titlecase_segment, FnInStruct)]
        #[diplomat::rust_link(
            icu::casemap::CaseMapper::titlecase_segment_to_string,
            FnInStruct,
            hidden
        )]
        pub fn titlecase_segment(
            &self,
            s: &str,
            locale: &ICU4XLocale,
            write: &mut DiplomatWriteable,
        ) -> Result<(), ICU4XError> {
            // #2520
            // In the future we should be able to make assumptions based on backend
            core::str::from_utf8(s.as_bytes())
                .map_err(|e| ICU4XError::DataIoError.log_original(&e))?;
            self.0.titlecase_segment(s, &locale.0.id).write_to(write)?;

            Ok(())
        }

        /// Case-folds the characters in the given string
        #[diplomat::rust_link(icu::casemap::CaseMapper::fold, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapper::fold_string, FnInStruct, hidden)]
        pub fn fold(&self, s: &str, write: &mut DiplomatWriteable) -> Result<(), ICU4XError> {
            // #2520
            // In the future we should be able to make assumptions based on backend
            core::str::from_utf8(s.as_bytes())
                .map_err(|e| ICU4XError::DataIoError.log_original(&e))?;
            self.0.fold(s).write_to(write)?;

            Ok(())
        }
        /// Case-folds the characters in the given string
        /// using Turkic (T) mappings for dotted/dotless I.
        #[diplomat::rust_link(icu::casemap::CaseMapper::fold_turkic, FnInStruct)]
        #[diplomat::rust_link(icu::casemap::CaseMapper::fold_turkic_string, FnInStruct, hidden)]
        pub fn fold_turkic(
            &self,
            s: &str,
            write: &mut DiplomatWriteable,
        ) -> Result<(), ICU4XError> {
            // #2520
            // In the future we should be able to make assumptions based on backend
            core::str::from_utf8(s.as_bytes())
                .map_err(|e| ICU4XError::DataIoError.log_original(&e))?;
            self.0.fold_turkic(s).write_to(write)?;

            Ok(())
        }

        /// Adds all simple case mappings and the full case folding for `c` to `builder`.
        /// Also adds special case closure mappings.
        ///
        /// In other words, this adds all characters that this casemaps to, as
        /// well as all characters that may casemap to this one.
        ///
        /// Note that since ICU4XCodePointSetBuilder does not contain strings, this will
        /// ignore string mappings.
        ///
        /// Identical to the similarly named method on `ICU4XCaseMapCloser`, use that if you
        /// plan on using string case closure mappings too.
        #[cfg(feature = "icu_properties")]
        #[diplomat::rust_link(icu::casemap::CaseMapper::add_case_closure, FnInStruct)]
        pub fn add_case_closure(
            &self,
            c: char,
            builder: &mut crate::collections_sets::ffi::ICU4XCodePointSetBuilder,
        ) {
            self.0.add_case_closure(c, &mut builder.0)
        }

        /// Returns the simple lowercase mapping of the given character.
        ///
        /// This function only implements simple and common mappings.
        /// Full mappings, which can map one char to a string, are not included.
        /// For full mappings, use `ICU4XCaseMapper::lowercase`.
        #[diplomat::rust_link(icu::casemap::CaseMapper::simple_lowercase, FnInStruct)]
        pub fn simple_lowercase(&self, ch: char) -> char {
            self.0.simple_lowercase(ch)
        }

        /// Returns the simple uppercase mapping of the given character.
        ///
        /// This function only implements simple and common mappings.
        /// Full mappings, which can map one char to a string, are not included.
        /// For full mappings, use `ICU4XCaseMapper::uppercase`.
        #[diplomat::rust_link(icu::casemap::CaseMapper::simple_uppercase, FnInStruct)]
        pub fn simple_uppercase(&self, ch: char) -> char {
            self.0.simple_uppercase(ch)
        }

        /// Returns the simple titlecase mapping of the given character.
        ///
        /// This function only implements simple and common mappings.
        /// Full mappings, which can map one char to a string, are not included.
        /// For full mappings, use `ICU4XCaseMapper::titlecase_segment`.
        #[diplomat::rust_link(icu::casemap::CaseMapper::simple_titlecase, FnInStruct)]
        pub fn simple_titlecase(&self, ch: char) -> char {
            self.0.simple_titlecase(ch)
        }

        /// Returns the simple casefolding of the given character.
        ///
        /// This function only implements simple folding.
        /// For full folding, use `ICU4XCaseMapper::fold`.
        #[diplomat::rust_link(icu::casemap::CaseMapper::simple_fold, FnInStruct)]
        pub fn simple_fold(&self, ch: char) -> char {
            self.0.simple_fold(ch)
        }
        /// Returns the simple casefolding of the given character in the Turkic locale
        ///
        /// This function only implements simple folding.
        /// For full folding, use `ICU4XCaseMapper::fold_turkic`.
        #[diplomat::rust_link(icu::casemap::CaseMapper::simple_fold_turkic, FnInStruct)]
        pub fn simple_fold_turkic(&self, ch: char) -> char {
            self.0.simple_fold_turkic(ch)
        }
    }

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::casemap::CaseMapCloser, Struct)]
    pub struct ICU4XCaseMapCloser(pub CaseMapCloser<CaseMapper>);

    impl ICU4XCaseMapCloser {
        /// Construct a new ICU4XCaseMapper instance for NFC
        #[diplomat::rust_link(icu::casemap::CaseMapCloser::new, FnInStruct)]
        pub fn create(provider: &ICU4XDataProvider) -> Result<Box<ICU4XCaseMapCloser>, ICU4XError> {
            Ok(Box::new(ICU4XCaseMapCloser(call_constructor!(
                CaseMapCloser::new [r => Ok(r)],
                CaseMapCloser::try_new_with_any_provider,
                CaseMapCloser::try_new_with_buffer_provider,
                provider,
            )?)))
        }

        /// Adds all simple case mappings and the full case folding for `c` to `builder`.
        /// Also adds special case closure mappings.
        ///
        /// In other words, this adds all characters that this casemaps to, as
        /// well as all characters that may casemap to this one.
        ///
        /// Note that since ICU4XCodePointSetBuilder does not contain strings, this will
        /// ignore string mappings
        ///
        /// Identical to the similarly named method on `ICU4XCaseMapCloser`, use that if you
        /// do not plan on using string case closure mappings to limit the amount of data loaded.
        #[cfg(feature = "icu_properties")]
        #[diplomat::rust_link(icu::casemap::CaseMapCloser::add_case_closure, FnInStruct)]
        pub fn add_case_closure(
            &self,
            c: char,
            builder: &mut crate::collections_sets::ffi::ICU4XCodePointSetBuilder,
        ) {
            self.0.add_case_closure(c, &mut builder.0)
        }

        /// Maps the string to single code points and adds the associated case closure
        /// mappings, if they exist.
        ///
        /// The string is mapped to code points if it is their full case folding string.
        /// In other words, this performs a reverse full case folding and then
        /// adds the case closure items of the resulting code points.
        /// If the string is found and its closure applied, then
        /// the string itself is added as well as part of its code points' closure.
        ///
        /// Returns true if the string was found
        #[cfg(feature = "icu_properties")]
        #[diplomat::rust_link(icu::casemap::CaseMapCloser::add_string_case_closure, FnInStruct)]
        pub fn add_string_case_closure(
            &self,
            s: &str,
            builder: &mut crate::collections_sets::ffi::ICU4XCodePointSetBuilder,
        ) -> bool {
            // #2520
            // In the future we should be able to make assumptions based on backend
            let s = core::str::from_utf8(s.as_bytes()).unwrap_or("");
            self.0.add_string_case_closure(s, &mut builder.0)
        }
    }
}
