// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[diplomat::bridge]
#[diplomat::abi_rename = "ICU4X{0}"]
pub mod ffi {
    use alloc::boxed::Box;

    use crate::{errors::ffi::DataError, locale_core::ffi::Locale, provider::ffi::DataProvider};

    #[diplomat::opaque]
    #[diplomat::rust_link(icu::collator::Collator, Struct)]
    pub struct Collator(pub icu_collator::Collator);

    #[diplomat::rust_link(icu::collator::CollatorOptions, Struct)]
    #[diplomat::rust_link(icu::collator::CollatorOptions::new, FnInStruct, hidden)]
    #[diplomat::attr(any(dart, js), rename = "CollatorOptions")]
    pub struct CollatorOptionsV1 {
        pub strength: CollatorStrength,
        pub alternate_handling: CollatorAlternateHandling,
        pub case_first: CollatorCaseFirst,
        pub max_variable: CollatorMaxVariable,
        pub case_level: CollatorCaseLevel,
        pub numeric: CollatorNumeric,
        pub backward_second_level: CollatorBackwardSecondLevel,
    }

    // Note the flipped order of the words `Collator` and `Resolved`, because
    // in FFI `Collator` is part of the `Collator` prefix, but in Rust,
    // `ResolvedCollatorOptions` makes more sense as English.
    #[diplomat::rust_link(icu::collator::ResolvedCollatorOptions, Struct)]
    #[diplomat::out]
    #[diplomat::attr(any(dart, js), rename = "ResolvedCollatorOptions")]
    pub struct CollatorResolvedOptionsV1 {
        pub strength: CollatorStrength,
        pub alternate_handling: CollatorAlternateHandling,
        pub case_first: CollatorCaseFirst,
        pub max_variable: CollatorMaxVariable,
        pub case_level: CollatorCaseLevel,
        pub numeric: CollatorNumeric,
        pub backward_second_level: CollatorBackwardSecondLevel,
    }

    #[diplomat::rust_link(icu::collator::Strength, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorStrength {
        Auto = 0,
        Primary = 1,
        Secondary = 2,
        Tertiary = 3,
        Quaternary = 4,
        Identical = 5,
    }

    #[diplomat::rust_link(icu::collator::AlternateHandling, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorAlternateHandling {
        Auto = 0,
        NonIgnorable = 1,
        Shifted = 2,
    }

    #[diplomat::rust_link(icu::collator::CaseFirst, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorCaseFirst {
        Auto = 0,
        Off = 1,
        LowerFirst = 2,
        UpperFirst = 3,
    }

    #[diplomat::rust_link(icu::collator::MaxVariable, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorMaxVariable {
        Auto = 0,
        Space = 1,
        Punctuation = 2,
        Symbol = 3,
        Currency = 4,
    }

    #[diplomat::rust_link(icu::collator::CaseLevel, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorCaseLevel {
        Auto = 0,
        Off = 1,
        On = 2,
    }

    #[diplomat::rust_link(icu::collator::Numeric, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorNumeric {
        Auto = 0,
        Off = 1,
        On = 2,
    }

    #[diplomat::rust_link(icu::collator::BackwardSecondLevel, Enum)]
    #[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
    pub enum CollatorBackwardSecondLevel {
        Auto = 0,
        Off = 1,
        On = 2,
    }

    impl Collator {
        /// Construct a new Collator instance.
        #[diplomat::rust_link(icu::collator::Collator::try_new, FnInStruct)]
        #[diplomat::attr(all(supports = constructors, supports = fallible_constructors), constructor)]
        #[diplomat::attr(js, rename = "create")]
        pub fn create_v1(
            provider: &DataProvider,
            locale: &Locale,
            options: CollatorOptionsV1,
        ) -> Result<Box<Collator>, DataError> {
            let locale = locale.to_datalocale();
            let options = icu_collator::CollatorOptions::from(options);

            Ok(Box::new(Collator(call_constructor!(
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
        #[diplomat::rust_link(icu::collator::Collator::compare_utf16, FnInStruct)]
        #[diplomat::attr(any(dart, js), rename = "compare")]
        #[diplomat::attr(cpp, rename = "compare16")]
        pub fn compare_utf16(
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
        #[diplomat::rust_link(icu::collator::Collator::compare_utf8, FnInStruct)]
        #[diplomat::rust_link(icu::collator::Collator::compare, FnInStruct, hidden)]
        #[diplomat::attr(any(dart, js), disable)]
        pub fn compare(&self, left: &DiplomatStr, right: &DiplomatStr) -> core::cmp::Ordering {
            self.0.compare_utf8(left, right)
        }

        /// The resolved options showing how the default options, the requested options,
        /// and the options from locale data were combined. None of the struct fields
        /// will have `Auto` as the value.
        #[diplomat::rust_link(icu::collator::Collator::resolved_options, FnInStruct)]
        #[diplomat::attr(supports = accessors, getter)]
        pub fn resolved_options(&self) -> CollatorResolvedOptionsV1 {
            self.0.resolved_options().into()
        }
    }
}

impl From<ffi::CollatorStrength> for Option<icu_collator::Strength> {
    fn from(strength: ffi::CollatorStrength) -> Option<icu_collator::Strength> {
        match strength {
            ffi::CollatorStrength::Auto => None,
            ffi::CollatorStrength::Primary => Some(icu_collator::Strength::Primary),
            ffi::CollatorStrength::Secondary => Some(icu_collator::Strength::Secondary),
            ffi::CollatorStrength::Tertiary => Some(icu_collator::Strength::Tertiary),
            ffi::CollatorStrength::Quaternary => Some(icu_collator::Strength::Quaternary),
            ffi::CollatorStrength::Identical => Some(icu_collator::Strength::Identical),
        }
    }
}

impl From<ffi::CollatorAlternateHandling> for Option<icu_collator::AlternateHandling> {
    fn from(
        alternate_handling: ffi::CollatorAlternateHandling,
    ) -> Option<icu_collator::AlternateHandling> {
        match alternate_handling {
            ffi::CollatorAlternateHandling::Auto => None,
            ffi::CollatorAlternateHandling::NonIgnorable => {
                Some(icu_collator::AlternateHandling::NonIgnorable)
            }
            ffi::CollatorAlternateHandling::Shifted => {
                Some(icu_collator::AlternateHandling::Shifted)
            }
        }
    }
}

impl From<ffi::CollatorCaseFirst> for Option<icu_collator::CaseFirst> {
    fn from(case_first: ffi::CollatorCaseFirst) -> Option<icu_collator::CaseFirst> {
        match case_first {
            ffi::CollatorCaseFirst::Auto => None,
            ffi::CollatorCaseFirst::Off => Some(icu_collator::CaseFirst::Off),
            ffi::CollatorCaseFirst::LowerFirst => Some(icu_collator::CaseFirst::LowerFirst),
            ffi::CollatorCaseFirst::UpperFirst => Some(icu_collator::CaseFirst::UpperFirst),
        }
    }
}

impl From<ffi::CollatorMaxVariable> for Option<icu_collator::MaxVariable> {
    fn from(max_variable: ffi::CollatorMaxVariable) -> Option<icu_collator::MaxVariable> {
        match max_variable {
            ffi::CollatorMaxVariable::Auto => None,
            ffi::CollatorMaxVariable::Space => Some(icu_collator::MaxVariable::Space),
            ffi::CollatorMaxVariable::Punctuation => Some(icu_collator::MaxVariable::Punctuation),
            ffi::CollatorMaxVariable::Symbol => Some(icu_collator::MaxVariable::Symbol),
            ffi::CollatorMaxVariable::Currency => Some(icu_collator::MaxVariable::Currency),
        }
    }
}

impl From<ffi::CollatorCaseLevel> for Option<icu_collator::CaseLevel> {
    fn from(case_level: ffi::CollatorCaseLevel) -> Option<icu_collator::CaseLevel> {
        match case_level {
            ffi::CollatorCaseLevel::Auto => None,
            ffi::CollatorCaseLevel::Off => Some(icu_collator::CaseLevel::Off),
            ffi::CollatorCaseLevel::On => Some(icu_collator::CaseLevel::On),
        }
    }
}

impl From<ffi::CollatorNumeric> for Option<icu_collator::Numeric> {
    fn from(numeric: ffi::CollatorNumeric) -> Option<icu_collator::Numeric> {
        match numeric {
            ffi::CollatorNumeric::Auto => None,
            ffi::CollatorNumeric::Off => Some(icu_collator::Numeric::Off),
            ffi::CollatorNumeric::On => Some(icu_collator::Numeric::On),
        }
    }
}

impl From<ffi::CollatorBackwardSecondLevel> for Option<icu_collator::BackwardSecondLevel> {
    fn from(
        backward_second_level: ffi::CollatorBackwardSecondLevel,
    ) -> Option<icu_collator::BackwardSecondLevel> {
        match backward_second_level {
            ffi::CollatorBackwardSecondLevel::Auto => None,
            ffi::CollatorBackwardSecondLevel::Off => Some(icu_collator::BackwardSecondLevel::Off),
            ffi::CollatorBackwardSecondLevel::On => Some(icu_collator::BackwardSecondLevel::On),
        }
    }
}

impl From<icu_collator::Strength> for ffi::CollatorStrength {
    fn from(strength: icu_collator::Strength) -> ffi::CollatorStrength {
        match strength {
            icu_collator::Strength::Primary => ffi::CollatorStrength::Primary,
            icu_collator::Strength::Secondary => ffi::CollatorStrength::Secondary,
            icu_collator::Strength::Tertiary => ffi::CollatorStrength::Tertiary,
            icu_collator::Strength::Quaternary => ffi::CollatorStrength::Quaternary,
            icu_collator::Strength::Identical => ffi::CollatorStrength::Identical,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::CollatorStrength::Identical // Highest we know of
            }
        }
    }
}

impl From<icu_collator::AlternateHandling> for ffi::CollatorAlternateHandling {
    fn from(alternate_handling: icu_collator::AlternateHandling) -> ffi::CollatorAlternateHandling {
        match alternate_handling {
            icu_collator::AlternateHandling::NonIgnorable => {
                ffi::CollatorAlternateHandling::NonIgnorable
            }
            icu_collator::AlternateHandling::Shifted => ffi::CollatorAlternateHandling::Shifted,
            _ => {
                debug_assert!(false, "FFI out of sync");
                // Possible future values: ShiftTrimmed, Blanked
                ffi::CollatorAlternateHandling::Shifted // Highest we know of
            }
        }
    }
}

impl From<icu_collator::CaseFirst> for ffi::CollatorCaseFirst {
    fn from(case_first: icu_collator::CaseFirst) -> ffi::CollatorCaseFirst {
        match case_first {
            icu_collator::CaseFirst::Off => ffi::CollatorCaseFirst::Off,
            icu_collator::CaseFirst::LowerFirst => ffi::CollatorCaseFirst::LowerFirst,
            icu_collator::CaseFirst::UpperFirst => ffi::CollatorCaseFirst::UpperFirst,
            _ => {
                debug_assert!(false, "FFI out of sync");
                // Does it even make sense that `CaseFirst` is non-exhaustive?
                ffi::CollatorCaseFirst::Off // The most neutral value
            }
        }
    }
}

impl From<icu_collator::MaxVariable> for ffi::CollatorMaxVariable {
    fn from(max_variable: icu_collator::MaxVariable) -> ffi::CollatorMaxVariable {
        match max_variable {
            icu_collator::MaxVariable::Space => ffi::CollatorMaxVariable::Space,
            icu_collator::MaxVariable::Punctuation => ffi::CollatorMaxVariable::Punctuation,
            icu_collator::MaxVariable::Symbol => ffi::CollatorMaxVariable::Symbol,
            icu_collator::MaxVariable::Currency => ffi::CollatorMaxVariable::Currency,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::CollatorMaxVariable::Currency // Highest we know of
            }
        }
    }
}

impl From<icu_collator::CaseLevel> for ffi::CollatorCaseLevel {
    fn from(case_level: icu_collator::CaseLevel) -> ffi::CollatorCaseLevel {
        match case_level {
            icu_collator::CaseLevel::Off => ffi::CollatorCaseLevel::Off,
            icu_collator::CaseLevel::On => ffi::CollatorCaseLevel::On,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::CollatorCaseLevel::On // The most enabled that we know of
            }
        }
    }
}

impl From<icu_collator::Numeric> for ffi::CollatorNumeric {
    fn from(numeric: icu_collator::Numeric) -> ffi::CollatorNumeric {
        match numeric {
            icu_collator::Numeric::Off => ffi::CollatorNumeric::Off,
            icu_collator::Numeric::On => ffi::CollatorNumeric::On,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::CollatorNumeric::On // The most enabled that we know of
            }
        }
    }
}

impl From<icu_collator::BackwardSecondLevel> for ffi::CollatorBackwardSecondLevel {
    fn from(
        backward_second_level: icu_collator::BackwardSecondLevel,
    ) -> ffi::CollatorBackwardSecondLevel {
        match backward_second_level {
            icu_collator::BackwardSecondLevel::Off => ffi::CollatorBackwardSecondLevel::Off,
            icu_collator::BackwardSecondLevel::On => ffi::CollatorBackwardSecondLevel::On,
            _ => {
                debug_assert!(false, "FFI out of sync");
                ffi::CollatorBackwardSecondLevel::On // The most enabled that we know of
            }
        }
    }
}

impl From<ffi::CollatorOptionsV1> for icu_collator::CollatorOptions {
    fn from(options: ffi::CollatorOptionsV1) -> icu_collator::CollatorOptions {
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

impl From<icu_collator::ResolvedCollatorOptions> for ffi::CollatorResolvedOptionsV1 {
    fn from(options: icu_collator::ResolvedCollatorOptions) -> ffi::CollatorResolvedOptionsV1 {
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
