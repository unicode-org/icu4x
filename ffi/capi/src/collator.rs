// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{
        common::ffi::ICU4XOrdering, errors::ffi::ICU4XDataError, locale_core::ffi::ICU4XLocale,
        provider::ffi::ICU4XDataProvider,
    };

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::collator::Collator, Struct)]
    pub struct ICU4XCollator(pub icu_collator::Collator);

    #[diplomat::rust_link(icu::collator::CollatorOptions, Struct)]
    #[diplomat::rust_link(icu::collator::CollatorOptions::new, FnInStruct, hidden)]
    #[diplomat::attr(dart, rename = "CollatorOptions")]
    pub struct ICU4XCollatorOptionsV1 {
        pub strength: ICU4XCollatorStrength,
        pub alternate_handling: ICU4XCollatorAlternateHandling,
        pub case_first: ICU4XCollatorCaseFirst,
        pub max_variable: ICU4XCollatorMaxVariable,
        pub case_level: ICU4XCollatorCaseLevel,
        pub numeric: ICU4XCollatorNumeric,
        pub backward_second_level: ICU4XCollatorBackwardSecondLevel,
    }

    // Note the flipped order of the words `Collator` and `Resolved`, because
    // in FFI `Collator` is part of the `ICU4XCollator` prefix, but in Rust,
    // `ResolvedCollatorOptions` makes more sense as English.
    #[diplomat::rust_link(icu::collator::ResolvedCollatorOptions, Struct)]
    #[diplomat::out]
    #[diplomat::attr(dart, rename = "ResolvedCollatorOptions")]
    pub struct ICU4XCollatorResolvedOptionsV1 {
        pub strength: ICU4XCollatorStrength,
        pub alternate_handling: ICU4XCollatorAlternateHandling,
        pub case_first: ICU4XCollatorCaseFirst,
        pub max_variable: ICU4XCollatorMaxVariable,
        pub case_level: ICU4XCollatorCaseLevel,
        pub numeric: ICU4XCollatorNumeric,
        pub backward_second_level: ICU4XCollatorBackwardSecondLevel,
    }

    #[diplomat::rust_link(icu::collator::Strength, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum ICU4XCollatorStrength {
        Auto = 0,
        Primary = 1,
        Secondary = 2,
        Tertiary = 3,
        Quaternary = 4,
        Identical = 5,
    }

    #[diplomat::rust_link(icu::collator::AlternateHandling, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum ICU4XCollatorAlternateHandling {
        Auto = 0,
        NonIgnorable = 1,
        Shifted = 2,
    }

    #[diplomat::rust_link(icu::collator::CaseFirst, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum ICU4XCollatorCaseFirst {
        Auto = 0,
        Off = 1,
        LowerFirst = 2,
        UpperFirst = 3,
    }

    #[diplomat::rust_link(icu::collator::MaxVariable, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum ICU4XCollatorMaxVariable {
        Auto = 0,
        Space = 1,
        Punctuation = 2,
        Symbol = 3,
        Currency = 4,
    }

    #[diplomat::rust_link(icu::collator::CaseLevel, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum ICU4XCollatorCaseLevel {
        Auto = 0,
        Off = 1,
        On = 2,
    }

    #[diplomat::rust_link(icu::collator::Numeric, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum ICU4XCollatorNumeric {
        Auto = 0,
        Off = 1,
        On = 2,
    }

    #[diplomat::rust_link(icu::collator::BackwardSecondLevel, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum ICU4XCollatorBackwardSecondLevel {
        Auto = 0,
        Off = 1,
        On = 2,
    }

    impl ICU4XCollator {
        /// Construct a new Collator instance.
        #[diplomat::rust_link(icu::collator::Collator::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        pub fn create_v1(
            provider: &ICU4XDataProvider,
            locale: &ICU4XLocale,
            options: ICU4XCollatorOptionsV1,
        ) -> Result<Box<ICU4XCollator>, ICU4XDataError> {
            let locale = locale.to_datalocale();
            let options = icu_collator::CollatorOptions::from(options);

            Ok(Box::new(ICU4XCollator(call_constructor!(
                icu_collator::Collator::try_new,
                icu_collator::Collator::try_new_with_any_provider,
                icu_collator::Collator::try_new_with_buffer_provider,
                provider,
                &locale,
                options,
            )?)))
        }

        /// Compare two strings.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::collator::Collator::compare_utf8, FnInStruct)]
        #[diplomat::attr(*, disable)]
        pub fn compare(&self, left: &DiplomatStr, right: &DiplomatStr) -> ICU4XOrdering {
            self.0.compare_utf8(left, right).into()
        }

        /// Compare two strings.
        #[diplomat::rust_link(icu::collator::Collator::compare, FnInStruct)]
        #[diplomat::attr(*, disable)]
        pub fn compare_valid_utf8(&self, left: &str, right: &str) -> ICU4XOrdering {
            self.0.compare(left, right).into()
        }

        /// Compare two strings.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::collator::Collator::compare_utf16, FnInStruct)]
        #[diplomat::attr(*, disable)]
        pub fn compare_utf16(&self, left: &DiplomatStr16, right: &DiplomatStr16) -> ICU4XOrdering {
            self.0.compare_utf16(left, right).into()
        }

        /// Compare two strings.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::collator::Collator::compare_utf16, FnInStruct)]
        #[diplomat::skip_if_ast]
        #[diplomat::attr(dart, rename = "compare")]
        #[diplomat::attr(cpp, rename = "compare16")]
        pub fn compare_utf16_(
            &self,
            left: &DiplomatStr16,
            right: &DiplomatStr16,
        ) -> core::cmp::Ordering {
            self.0.compare_utf16(left, right)
        }

        /// Compare two strings.
        ///
        /// Ill-formed input is treated as if errors had been replaced with REPLACEMENT CHARACTERs according
        /// to the WHATWG Encoding Standard.
        #[diplomat::rust_link(icu::collator::Collator::compare_utf16, FnInStruct)]
        #[diplomat::skip_if_ast]
        #[diplomat::attr(dart, disable)]
        #[diplomat::attr(cpp, rename = "compare")]
        pub fn compare_(&self, left: &DiplomatStr, right: &DiplomatStr) -> core::cmp::Ordering {
            self.0.compare_utf8(left, right)
        }

        /// The resolved options showing how the default options, the requested options,
        /// and the options from locale data were combined. None of the struct fields
        /// will have `Auto` as the value.
        #[diplomat::rust_link(icu::collator::Collator::resolved_options, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn resolved_options(&self) -> ICU4XCollatorResolvedOptionsV1 {
            self.0.resolved_options().into()
        }
    }
}

impl From<ffi::ICU4XCollatorStrength> for Option<icu_collator::Strength> {
    fn from(strength: ffi::ICU4XCollatorStrength) -> Option<icu_collator::Strength> {
        match strength {
            ffi::ICU4XCollatorStrength::Auto => None,
            ffi::ICU4XCollatorStrength::Primary => Some(icu_collator::Strength::Primary),
            ffi::ICU4XCollatorStrength::Secondary => Some(icu_collator::Strength::Secondary),
            ffi::ICU4XCollatorStrength::Tertiary => Some(icu_collator::Strength::Tertiary),
            ffi::ICU4XCollatorStrength::Quaternary => Some(icu_collator::Strength::Quaternary),
            ffi::ICU4XCollatorStrength::Identical => Some(icu_collator::Strength::Identical),
        }
    }
}

impl From<ffi::ICU4XCollatorAlternateHandling> for Option<icu_collator::AlternateHandling> {
    fn from(
        alternate_handling: ffi::ICU4XCollatorAlternateHandling,
    ) -> Option<icu_collator::AlternateHandling> {
        match alternate_handling {
            ffi::ICU4XCollatorAlternateHandling::Auto => None,
            ffi::ICU4XCollatorAlternateHandling::NonIgnorable => {
                Some(icu_collator::AlternateHandling::NonIgnorable)
            }
            ffi::ICU4XCollatorAlternateHandling::Shifted => {
                Some(icu_collator::AlternateHandling::Shifted)
            }
        }
    }
}

impl From<ffi::ICU4XCollatorCaseFirst> for Option<icu_collator::CaseFirst> {
    fn from(case_first: ffi::ICU4XCollatorCaseFirst) -> Option<icu_collator::CaseFirst> {
        match case_first {
            ffi::ICU4XCollatorCaseFirst::Auto => None,
            ffi::ICU4XCollatorCaseFirst::Off => Some(icu_collator::CaseFirst::Off),
            ffi::ICU4XCollatorCaseFirst::LowerFirst => Some(icu_collator::CaseFirst::LowerFirst),
            ffi::ICU4XCollatorCaseFirst::UpperFirst => Some(icu_collator::CaseFirst::UpperFirst),
        }
    }
}

impl From<ffi::ICU4XCollatorMaxVariable> for Option<icu_collator::MaxVariable> {
    fn from(max_variable: ffi::ICU4XCollatorMaxVariable) -> Option<icu_collator::MaxVariable> {
        match max_variable {
            ffi::ICU4XCollatorMaxVariable::Auto => None,
            ffi::ICU4XCollatorMaxVariable::Space => Some(icu_collator::MaxVariable::Space),
            ffi::ICU4XCollatorMaxVariable::Punctuation => {
                Some(icu_collator::MaxVariable::Punctuation)
            }
            ffi::ICU4XCollatorMaxVariable::Symbol => Some(icu_collator::MaxVariable::Symbol),
            ffi::ICU4XCollatorMaxVariable::Currency => Some(icu_collator::MaxVariable::Currency),
        }
    }
}

impl From<ffi::ICU4XCollatorCaseLevel> for Option<icu_collator::CaseLevel> {
    fn from(case_level: ffi::ICU4XCollatorCaseLevel) -> Option<icu_collator::CaseLevel> {
        match case_level {
            ffi::ICU4XCollatorCaseLevel::Auto => None,
            ffi::ICU4XCollatorCaseLevel::Off => Some(icu_collator::CaseLevel::Off),
            ffi::ICU4XCollatorCaseLevel::On => Some(icu_collator::CaseLevel::On),
        }
    }
}

impl From<ffi::ICU4XCollatorNumeric> for Option<icu_collator::Numeric> {
    fn from(numeric: ffi::ICU4XCollatorNumeric) -> Option<icu_collator::Numeric> {
        match numeric {
            ffi::ICU4XCollatorNumeric::Auto => None,
            ffi::ICU4XCollatorNumeric::Off => Some(icu_collator::Numeric::Off),
            ffi::ICU4XCollatorNumeric::On => Some(icu_collator::Numeric::On),
        }
    }
}

impl From<ffi::ICU4XCollatorBackwardSecondLevel> for Option<icu_collator::BackwardSecondLevel> {
    fn from(
        backward_second_level: ffi::ICU4XCollatorBackwardSecondLevel,
    ) -> Option<icu_collator::BackwardSecondLevel> {
        match backward_second_level {
            ffi::ICU4XCollatorBackwardSecondLevel::Auto => None,
            ffi::ICU4XCollatorBackwardSecondLevel::Off => {
                Some(icu_collator::BackwardSecondLevel::Off)
            }
            ffi::ICU4XCollatorBackwardSecondLevel::On => {
                Some(icu_collator::BackwardSecondLevel::On)
            }
        }
    }
}

impl From<icu_collator::Strength> for ffi::ICU4XCollatorStrength {
    fn from(strength: icu_collator::Strength) -> ffi::ICU4XCollatorStrength {
        match strength {
            icu_collator::Strength::Primary => ffi::ICU4XCollatorStrength::Primary,
            icu_collator::Strength::Secondary => ffi::ICU4XCollatorStrength::Secondary,
            icu_collator::Strength::Tertiary => ffi::ICU4XCollatorStrength::Tertiary,
            icu_collator::Strength::Quaternary => ffi::ICU4XCollatorStrength::Quaternary,
            icu_collator::Strength::Identical => ffi::ICU4XCollatorStrength::Identical,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::ICU4XCollatorStrength::Identical // Highest we know of
            }
        }
    }
}

impl From<icu_collator::AlternateHandling> for ffi::ICU4XCollatorAlternateHandling {
    fn from(
        alternate_handling: icu_collator::AlternateHandling,
    ) -> ffi::ICU4XCollatorAlternateHandling {
        match alternate_handling {
            icu_collator::AlternateHandling::NonIgnorable => {
                ffi::ICU4XCollatorAlternateHandling::NonIgnorable
            }
            icu_collator::AlternateHandling::Shifted => {
                ffi::ICU4XCollatorAlternateHandling::Shifted
            }
            _ => {
                debug_assert!(false, "FFI out of sync");
                // Possible future values: ShiftTrimmed, Blanked
                ffi::ICU4XCollatorAlternateHandling::Shifted // Highest we know of
            }
        }
    }
}

impl From<icu_collator::CaseFirst> for ffi::ICU4XCollatorCaseFirst {
    fn from(case_first: icu_collator::CaseFirst) -> ffi::ICU4XCollatorCaseFirst {
        match case_first {
            icu_collator::CaseFirst::Off => ffi::ICU4XCollatorCaseFirst::Off,
            icu_collator::CaseFirst::LowerFirst => ffi::ICU4XCollatorCaseFirst::LowerFirst,
            icu_collator::CaseFirst::UpperFirst => ffi::ICU4XCollatorCaseFirst::UpperFirst,
            _ => {
                debug_assert!(false, "FFI out of sync");
                // Does it even make sense that `CaseFirst` is non-exhaustive?
                ffi::ICU4XCollatorCaseFirst::Off // The most neutral value
            }
        }
    }
}

impl From<icu_collator::MaxVariable> for ffi::ICU4XCollatorMaxVariable {
    fn from(max_variable: icu_collator::MaxVariable) -> ffi::ICU4XCollatorMaxVariable {
        match max_variable {
            icu_collator::MaxVariable::Space => ffi::ICU4XCollatorMaxVariable::Space,
            icu_collator::MaxVariable::Punctuation => ffi::ICU4XCollatorMaxVariable::Punctuation,
            icu_collator::MaxVariable::Symbol => ffi::ICU4XCollatorMaxVariable::Symbol,
            icu_collator::MaxVariable::Currency => ffi::ICU4XCollatorMaxVariable::Currency,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::ICU4XCollatorMaxVariable::Currency // Highest we know of
            }
        }
    }
}

impl From<icu_collator::CaseLevel> for ffi::ICU4XCollatorCaseLevel {
    fn from(case_level: icu_collator::CaseLevel) -> ffi::ICU4XCollatorCaseLevel {
        match case_level {
            icu_collator::CaseLevel::Off => ffi::ICU4XCollatorCaseLevel::Off,
            icu_collator::CaseLevel::On => ffi::ICU4XCollatorCaseLevel::On,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::ICU4XCollatorCaseLevel::On // The most enabled that we know of
            }
        }
    }
}

impl From<icu_collator::Numeric> for ffi::ICU4XCollatorNumeric {
    fn from(numeric: icu_collator::Numeric) -> ffi::ICU4XCollatorNumeric {
        match numeric {
            icu_collator::Numeric::Off => ffi::ICU4XCollatorNumeric::Off,
            icu_collator::Numeric::On => ffi::ICU4XCollatorNumeric::On,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::ICU4XCollatorNumeric::On // The most enabled that we know of
            }
        }
    }
}

impl From<icu_collator::BackwardSecondLevel> for ffi::ICU4XCollatorBackwardSecondLevel {
    fn from(
        backward_second_level: icu_collator::BackwardSecondLevel,
    ) -> ffi::ICU4XCollatorBackwardSecondLevel {
        match backward_second_level {
            icu_collator::BackwardSecondLevel::Off => ffi::ICU4XCollatorBackwardSecondLevel::Off,
            icu_collator::BackwardSecondLevel::On => ffi::ICU4XCollatorBackwardSecondLevel::On,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::ICU4XCollatorBackwardSecondLevel::On // The most enabled that we know of
            }
        }
    }
}

impl From<ffi::ICU4XCollatorOptionsV1> for icu_collator::CollatorOptions {
    fn from(options: ffi::ICU4XCollatorOptionsV1) -> icu_collator::CollatorOptions {
        let mut result = icu_collator::CollatorOptions::new();
        result.strength = options.strength.into();
        result.alternate_handling = options.alternate_handling.into();
        result.case_first = options.case_first.into();
        result.max_variable = options.max_variable.into();
        result.case_level = options.case_level.into();
        result.numeric = options.numeric.into();
        result.backward_second_level = options.backward_second_level.into();

        result
    }
}

impl From<icu_collator::ResolvedCollatorOptions> for ffi::ICU4XCollatorResolvedOptionsV1 {
    fn from(options: icu_collator::ResolvedCollatorOptions) -> ffi::ICU4XCollatorResolvedOptionsV1 {
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
