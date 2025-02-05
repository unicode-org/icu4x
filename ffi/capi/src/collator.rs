// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "icu4x_{0}_mv1"]
#[diplomat::attr(auto, namespace = "icu4x")]
pub mod ffi {
    use alloc::boxed::Box;

    #[cfg(feature = "buffer_provider")]
    use crate::provider::ffi::DataProvider;
    #[cfg(any(feature = "compiled_data", feature = "buffer_provider"))]
    use crate::{errors::ffi::DataError, locale_core::ffi::Locale};
    use diplomat_runtime::DiplomatOption;

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::collator::Collator, Struct)]
    #[diplomat::rust_link(icu::collator::CollatorBorrowed, Struct, hidden)]
    pub struct Collator(pub icu_collator::Collator);

    #[diplomat::rust_link(icu::collator::CollatorOptions, Struct)]
    #[diplomat::rust_link(icu::collator::CollatorOptions::default, FnInStruct, hidden)]
    #[diplomat::attr(supports = non_exhaustive_structs, rename = "CollatorOptions")]
    pub struct CollatorOptions {
        pub strength: DiplomatOption<CollatorStrength>,
        pub alternate_handling: DiplomatOption<CollatorAlternateHandling>,
        pub max_variable: DiplomatOption<CollatorMaxVariable>,
        pub case_level: DiplomatOption<CollatorCaseLevel>,
        pub backward_second_level: DiplomatOption<CollatorBackwardSecondLevel>,
    }

    // Note the flipped order of the words `Collator` and `Resolved`, because
    // in FFI `Collator` is part of the `Collator` prefix, but in Rust,
    // `ResolvedCollatorOptions` makes more sense as English.
    #[diplomat::rust_link(icu::collator::ResolvedCollatorOptions, Struct)]
    #[diplomat::out]
    #[diplomat::attr(supports = non_exhaustive_structs, rename = "CollatorResolvedOptions")]
    pub struct CollatorResolvedOptions {
        pub strength: CollatorStrength,
        pub alternate_handling: CollatorAlternateHandling,
        pub case_first: CollatorCaseFirst,
        pub max_variable: CollatorMaxVariable,
        pub case_level: CollatorCaseLevel,
        pub numeric: CollatorNumericOrdering,
        pub backward_second_level: CollatorBackwardSecondLevel,
    }

    #[diplomat::rust_link(icu::collator::Strength, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    #[diplomat::enum_convert(icu_collator::Strength, needs_wildcard)]
    pub enum CollatorStrength {
        Primary = 0,
        Secondary = 1,
        Tertiary = 2,
        Quaternary = 3,
        Identical = 4,
    }

    #[diplomat::rust_link(icu::collator::AlternateHandling, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    #[diplomat::enum_convert(icu_collator::AlternateHandling, needs_wildcard)]
    pub enum CollatorAlternateHandling {
        NonIgnorable = 0,
        Shifted = 1,
    }

    #[diplomat::rust_link(icu::collator::CaseFirst, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorCaseFirst {
        Off = 0,
        Lower = 1,
        Upper = 2,
    }

    #[diplomat::rust_link(icu::collator::MaxVariable, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    #[diplomat::enum_convert(icu_collator::MaxVariable, needs_wildcard)]
    pub enum CollatorMaxVariable {
        Space = 0,
        Punctuation = 1,
        Symbol = 2,
        Currency = 3,
    }

    #[diplomat::rust_link(icu::collator::CaseLevel, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    #[diplomat::enum_convert(icu_collator::CaseLevel, needs_wildcard)]
    pub enum CollatorCaseLevel {
        Off = 0,
        On = 1,
    }

    #[diplomat::rust_link(icu::collator::NumericOrdering, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorNumericOrdering {
        Off = 0,
        On = 1,
    }

    #[diplomat::rust_link(icu::collator::BackwardSecondLevel, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    #[diplomat::enum_convert(icu_collator::BackwardSecondLevel, needs_wildcard)]
    pub enum CollatorBackwardSecondLevel {
        Off = 0,
        On = 1,
    }

    impl Collator {
        /// Construct a new Collator instance using compiled data.
        #[diplomat::rust_link(icu::collator::Collator::try_new, FnInStruct)]
        #[diplomat::rust_link(icu::collator::CollatorBorrowed::try_new, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::collator::CollatorPreferences, Struct, hidden)]
        #[diplomat::attr(all(supports = fallible_constructors, supports = named_constructors), named_constructor = "with_provider")]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "create")]
        #[cfg(feature = "compiled_data")]
        pub fn create_v1(
            locale: &Locale,
            options: CollatorOptions,
        ) -> Result<Box<Collator>, DataError> {
            Ok(Box::new(Collator(
                icu_collator::Collator::try_new((&locale.0).into(), options.into())?
                    .static_to_owned(),
            )))
        }

        /// Construct a new Collator instance using a particular data source.
        #[diplomat::rust_link(icu::collator::Collator::try_new, FnInStruct)]
        #[diplomat::rust_link(icu::collator::CollatorBorrowed::try_new, FnInStruct, hidden)]
        #[diplomat::rust_link(icu::collator::CollatorPreferences, Struct, hidden)]
        #[diplomat::attr(supports = fallible_constructors, constructor)]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "create_with_provider")]
        #[cfg(feature = "buffer_provider")]
        pub fn create_v1_with_provider(
            provider: &DataProvider,
            locale: &Locale,
            options: CollatorOptions,
        ) -> Result<Box<Collator>, DataError> {
            let options = options.into();
            Ok(Box::new(Collator(
                icu_collator::Collator::try_new_with_buffer_provider(
                    provider.get()?,
                    (&locale.0).into(),
                    options,
                )?,
            )))
        }
        /// Compare two strings.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::collator::CollatorBorrowed::compare_utf8, FnInStruct)]
        #[diplomat::rust_link(icu::collator::CollatorBorrowed::compare, FnInStruct, hidden)]
        #[diplomat::attr(not(supports = utf8_strings), disable)]
        #[diplomat::attr(supports = utf8_strings, rename = "compare")]
        pub fn compare_utf8(&self, left: &DiplomatStr, right: &DiplomatStr) -> core::cmp::Ordering {
            self.0.as_borrowed().compare_utf8(left, right)
        }

        /// Compare two strings.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::collator::CollatorBorrowed::compare_utf16, FnInStruct)]
        #[diplomat::attr(not(supports = utf8_strings), rename = "compare")]
        #[diplomat::attr(supports = utf8_strings, rename = "compare16")]
        pub fn compare_utf16(
            &self,
            left: &DiplomatStr16,
            right: &DiplomatStr16,
        ) -> core::cmp::Ordering {
            self.0.as_borrowed().compare_utf16(left, right)
        }

        /// The resolved options showing how the default options, the requested options,
        /// and the options from locale data were combined. None of the struct fields
        /// will have `Auto` as the value.
        #[diplomat::rust_link(icu::collator::CollatorBorrowed::resolved_options, FnInStruct)]
        #[diplomat::attr(auto, getter)]
        #[diplomat::attr(supports = non_exhaustive_structs, rename = "resolved_options")]
        pub fn resolved_options_v1(&self) -> CollatorResolvedOptions {
            self.0.as_borrowed().resolved_options().into()
        }
    }
}

impl From<ffi::CollatorOptions> for icu_collator::CollatorOptions {
    fn from(options: ffi::CollatorOptions) -> icu_collator::CollatorOptions {
        let mut result = icu_collator::CollatorOptions::default();
        result.strength = options.strength.into_converted_option();
        result.alternate_handling = options.alternate_handling.into_converted_option();
        result.max_variable = options.max_variable.into_converted_option();
        result.case_level = options.case_level.into_converted_option();
        result.backward_second_level = options.backward_second_level.into_converted_option();

        result
    }
}

impl From<icu_collator::ResolvedCollatorOptions> for ffi::CollatorResolvedOptions {
    fn from(options: icu_collator::ResolvedCollatorOptions) -> ffi::CollatorResolvedOptions {
        Self {
            strength: options.strength.into(),
            alternate_handling: options.alternate_handling.into(),
            case_first: options.case_first.into(),
            max_variable: options.max_variable.into(),
            case_level: options.case_level.into(),
            numeric: options.numeric.into(),
            backward_second_level: options.backward_second_level.into(),
        }
    }
}

impl From<icu_collator::CaseFirst> for ffi::CollatorCaseFirst {
    fn from(other: icu_collator::CaseFirst) -> Self {
        match other {
            icu_collator::CaseFirst::Upper => ffi::CollatorCaseFirst::Upper,
            icu_collator::CaseFirst::Lower => ffi::CollatorCaseFirst::Lower,
            icu_collator::CaseFirst::False => ffi::CollatorCaseFirst::Off,
            _ => {
                debug_assert!(false, "unknown variant for icu_collator::CaseFirst");
                ffi::CollatorCaseFirst::Off
            }
        }
    }
}
impl From<ffi::CollatorCaseFirst> for icu_collator::CaseFirst {
    fn from(this: ffi::CollatorCaseFirst) -> Self {
        match this {
            ffi::CollatorCaseFirst::Off => icu_collator::CaseFirst::False,
            ffi::CollatorCaseFirst::Lower => icu_collator::CaseFirst::Lower,
            ffi::CollatorCaseFirst::Upper => icu_collator::CaseFirst::Upper,
        }
    }
}

impl From<icu_collator::NumericOrdering> for ffi::CollatorNumericOrdering {
    fn from(other: icu_collator::NumericOrdering) -> Self {
        match other {
            icu_collator::NumericOrdering::True => ffi::CollatorNumericOrdering::On,
            icu_collator::NumericOrdering::False => ffi::CollatorNumericOrdering::Off,
            _ => {
                debug_assert!(false, "unknown variant for icu_collator::NumericOrdering");
                ffi::CollatorNumericOrdering::Off
            }
        }
    }
}
impl From<ffi::CollatorNumericOrdering> for icu_collator::NumericOrdering {
    fn from(this: ffi::CollatorNumericOrdering) -> Self {
        match this {
            ffi::CollatorNumericOrdering::Off => icu_collator::NumericOrdering::False,
            ffi::CollatorNumericOrdering::On => icu_collator::NumericOrdering::True,
        }
    }
}
