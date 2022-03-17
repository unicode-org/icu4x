// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// The bit layout of `CollatorOptions` is adapted from ICU4C and, therefore,
// is subject to the ICU license as described in LICENSE.

//! This module contains the types that are part of the API for setting
//! the options for the collator.

use crate::elements::{CASE_MASK, TERTIARY_MASK};

/// The collation strength that indicates how many levels to compare.
/// If an earlier level isn't equal, the earlier level is decisive.
/// If the result is equal on a level, but the strength is higher,
/// the comparison proceeds to the next level.
#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
#[repr(u8)]
pub enum Strength {
    /// Compare only on the level of base letters. This level
    /// corresponds to the ECMA-402 sensitivity "base".
    ///
    /// ```
    /// use icu_collator::*;
    ///
    /// let data_provider = icu_testdata::get_provider();
    /// let mut options = CollatorOptions::new();
    /// options.set_strength(Some(Strength::Primary));
    /// let collator =
    ///   Collator::try_new(icu_locid::Locale::default(),
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(collator.compare("E", "é"),
    ///            core::cmp::Ordering::Equal);
    /// ```
    Primary = 0,

    /// Compare also on the secondary level, which corresponds
    /// to diacritics in scripts that use them. This level corresponds
    /// to the ECMA-402 sensitivity "accent".
    ///
    /// ```
    /// use icu_collator::*;
    ///
    /// let data_provider = icu_testdata::get_provider();
    /// let mut options = CollatorOptions::new();
    /// options.set_strength(Some(Strength::Secondary));
    /// let collator =
    ///   Collator::try_new(icu_locid::Locale::default(),
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(collator.compare("E", "e"),
    ///            core::cmp::Ordering::Equal);
    /// assert_eq!(collator.compare("e", "é"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(collator.compare("あ", "ア"),
    ///            core::cmp::Ordering::Equal);
    /// assert_eq!(collator.compare("ァ", "ア"),
    ///            core::cmp::Ordering::Equal);
    /// assert_eq!(collator.compare("ア", "ｱ"),
    ///            core::cmp::Ordering::Equal);
    /// ```
    Secondary = 1,

    /// Compare also on the tertiary level. By default, if the separate
    /// case level is disabled, this corresponds to case for bicameral
    /// scripts. This level distinguishes Hiragana and Katakana. This
    /// also captures other minor differences, such as half-width vs.
    /// full-width when the Japanese tailoring isn't in use.
    ///
    /// This is the default comparison level and appropriate for
    /// most scripts. This level corresponds to the ECMA-402
    /// sensitivity "variant".
    ///
    /// ```
    /// use icu_collator::*;
    ///
    /// let data_provider = icu_testdata::get_provider();
    /// let mut options = CollatorOptions::new();
    /// options.set_strength(Some(Strength::Tertiary));
    /// let collator =
    ///   Collator::try_new(icu_locid::Locale::default(),
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(collator.compare("E", "e"),
    ///            core::cmp::Ordering::Greater);
    /// assert_eq!(collator.compare("e", "é"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(collator.compare("あ", "ア"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(collator.compare("ァ", "ア"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(collator.compare("ア", "ｱ"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(collator.compare("e", "ｅ"), // Full-width e
    ///            core::cmp::Ordering::Less);
    ///
    /// let locale: icu_locid::Locale = icu_locid::langid!("ja").into();
    /// let ja_collator =
    ///   Collator::try_new(locale,
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(ja_collator.compare("E", "e"),
    ///            core::cmp::Ordering::Greater);
    /// assert_eq!(ja_collator.compare("e", "é"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(ja_collator.compare("あ", "ア"),
    ///            core::cmp::Ordering::Equal); // Unlike root!
    /// assert_eq!(ja_collator.compare("ァ", "ア"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(ja_collator.compare("ア", "ｱ"),
    ///            core::cmp::Ordering::Equal); // Unlike root!
    /// assert_eq!(ja_collator.compare("e", "ｅ"), // Full-width e
    ///            core::cmp::Ordering::Equal); // Unlike root!
    /// ```
    Tertiary = 2,

    /// Compare also on the quaternary level. For Japanese, Higana
    /// and Katakana are distinguished at the quaternary level. Also,
    /// if `AlternateHandling::Shifted` is used, the collation
    /// elements whose level gets shifted are shifted to this
    /// level.
    ///
    /// ```
    /// use icu_collator::*;
    ///
    /// let data_provider = icu_testdata::get_provider();
    /// let mut options = CollatorOptions::new();
    /// options.set_strength(Some(Strength::Quaternary));
    ///
    /// let ja_locale: icu_locid::Locale = icu_locid::langid!("ja").into();
    /// let ja_collator =
    ///   Collator::try_new(ja_locale,
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(ja_collator.compare("あ", "ア"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(ja_collator.compare("ア", "ｱ"),
    ///            core::cmp::Ordering::Equal);
    /// assert_eq!(ja_collator.compare("e", "ｅ"), // Full-width e
    ///            core::cmp::Ordering::Equal);
    ///
    /// // Even this level doesn't distinguish everything,
    /// // e.g. Hebrew cantillation marks are still ignored.
    /// let collator =
    ///   Collator::try_new(icu_locid::Locale::default(),
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(collator.compare("דחי", "דחי֭"),
    ///            core::cmp::Ordering::Equal);
    /// ```
    /// TODO: Thai example.
    Quaternary = 3,

    /// Compare the NFD form by code point order as the quinary
    /// level. This level makes the comparison slower and should
    /// not be used in the general case. However, it can be used
    /// to distinguish full-width and half-width forms when the
    /// Japanese tailoring is in use and to distinguish e.g.
    /// Hebrew cantillation markse. Use this level if you need
    /// JIS X 4061-1996 compliance for Japanese on the level of
    /// distinguishing full-width and half-width forms.
    ///
    /// ```
    /// use icu_collator::*;
    ///
    /// let data_provider = icu_testdata::get_provider();
    /// let mut options = CollatorOptions::new();
    /// options.set_strength(Some(Strength::Identical));
    ///
    /// let ja_locale: icu_locid::Locale = icu_locid::langid!("ja").into();
    /// let ja_collator =
    ///   Collator::try_new(ja_locale,
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(ja_collator.compare("ア", "ｱ"),
    ///            core::cmp::Ordering::Less);
    /// assert_eq!(ja_collator.compare("e", "ｅ"), // Full-width e
    ///            core::cmp::Ordering::Less);
    ///
    /// let collator =
    ///   Collator::try_new(icu_locid::Locale::default(),
    ///                     &data_provider,
    ///                     options).unwrap();
    /// assert_eq!(collator.compare("דחי", "דחי֭"),
    ///            core::cmp::Ordering::Less);
    /// ```
    Identical = 4,
}

/// What to do about characters whose comparison level can be
/// varied dynamically.
#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
#[repr(u8)]
pub enum AlternateHandling {
    /// Keep the characters whose level can be varied on the
    /// primary level.
    NonIgnorable = 0,
    /// Shift the characters at or below `MaxVariable` to the
    /// quaternary level.
    Shifted = 1,
    // Possible future values: ShiftTrimmed, Blanked
}

/// Treatment of case. (Large and small kana
/// differences are treated as case differences.)
#[derive(Eq, PartialEq, Debug, PartialOrd, Ord)]
#[repr(u8)]
pub enum CaseFirst {
    /// Use the default tertiary weights.
    Off = 0,
    /// Lower case first.
    LowerFirst = 1,
    /// Upper case first.
    UpperFirst = 2,
}

/// What characters get shifted to the quaternary level
/// with `AlternateHandling::Shifted`.
#[derive(Eq, PartialEq)]
#[repr(u8)]
pub enum MaxVariable {
    Space = 0,
    Punctuation = 1,
    Symbol = 2,
    Currency = 3,
}

/// Options settable by the user of the API.
///
/// See https://www.unicode.org/reports/tr35/tr35-collation.html#Setting_Options
///
/// The setters take an `Option` so that `None` can be used to go back to default.
///
/// # Options
///
/// ## Strength
///
/// This is the BCP47 key `ks`. The default is `Strength::Tertiary`.
///
/// ## Alternate Handling
///
/// This is the BCP47 key `ka`. Note that `ShiftTrimmed` and `Blanked` are
/// unimplemented. The default is `AlternateHandling::NonIgnorable`, except
/// for Thai, whose default is `AlternateHandling::Shifted`.
///
/// ## Case Level
///
/// See https://www.unicode.org/reports/tr35/tr35-collation.html#Case_Parameters
/// This is the BCP47 key `kc`. The default is `false` (off).
///
/// ## Case First
///
/// See https://www.unicode.org/reports/tr35/tr35-collation.html#Case_Parameters
/// This is the BCP47 key `kf`. Three possibilities: `CaseFirst::Off` (default,
/// except for Danish and Maltese), `CaseFirst::Lower`, and `CaseFirst::Upper`
/// (default for Danish and Maltese).
///
/// ## Backward second level
///
/// Compare the second level in backward order. This is the BCP47 key `kb`. `kb`
/// is prohibited by ECMA 402. The default is `false` (off), except for Canadian
/// French.
///
/// # Numeric
///
/// This is the BCP47 key `kn`. When set to `true` (on), any sequence of decimal
/// digits (General_Category = Nd) is sorted at a primary level accoding to the
/// numeric value. The default is `false` (off).
///
/// # Unsupported BCP47 options
///
/// Reordering (BCP47 `kr`) currently cannot be set via the API and is implied
/// by the locale of the collation. `kr` is probihibited by ECMA 402.
///
/// Normalization is always enabled and cannot be turned off. Therefore, there
/// is no option corresponding to BCP47 `kk`. `kk` is prohibited by ECMA 402.
///
/// Hiragana quaternary handling is part of the strength for the Japanese
/// tailoring. The BCP47 key `kh` is unsupported. `kh` is deprecated and
/// prohibited by ECMA 402.
///
/// Variable top (BCP47 `vt`) is unsupported (use Max Variable instead). `vt`
/// is deprecated and prohibited by ECMA 402.
#[derive(Copy, Clone, Debug)]
pub struct CollatorOptions(u32);

impl CollatorOptions {
    /// Bits 0..2 : Strength
    const STRENGTH_MASK: u32 = 0b111;
    /// Bits 3..4 : Alternate handling: 00 non-ignorable, 01 shifted,
    ///             10 reserved for shift-trimmed, 11 reserved for blanked.
    ///             In other words, bit 4 is currently always 0.
    const ALTERNATE_HANDLING_MASK: u32 = 1 << 3;
    /// Bits 5..6 : 2-bit max variable value to be shifted by `MAX_VARIABLE_SHIFT`.
    const MAX_VARIABLE_MASK: u32 = 0b01100000;
    const MAX_VARIABLE_SHIFT: u32 = 5;
    /// Bit     7 : Reserved for extending max variable.
    /// Bit     8 : Sort uppercase first if case level or case first is on.
    const UPPER_FIRST_MASK: u32 = 1 << 8;
    /// Bit     9 : Keep the case bits in the tertiary weight (they trump
    ///             other tertiary values)
    ///             unless case level is on (when they are *moved* into the separate case level).
    ///             By default, the case bits are removed from the tertiary weight (ignored).
    ///             When CASE_FIRST is off, UPPER_FIRST must be off too, corresponding to
    ///             the tri-value UCOL_CASE_FIRST attribute: UCOL_OFF vs. UCOL_LOWER_FIRST vs.
    ///             UCOL_UPPER_FIRST.
    const CASE_FIRST_MASK: u32 = 1 << 9;
    /// Bit    10 : Insert the case level between the secondary and tertiary levels.
    const CASE_LEVEL_MASK: u32 = 1 << 10;
    /// Bit    11 : Backward secondary level
    const BACKWARD_SECOND_LEVEL_MASK: u32 = 1 << 11;
    /// Bit    12 : Numeric
    const NUMERIC_MASK: u32 = 1 << 12;

    /// Whether strength is explicitly set.
    const EXPLICIT_STRENGTH_MASK: u32 = 1 << 31;
    /// Whether max variable is explicitly set.
    const EXPLICIT_MAX_VARIABLE_MASK: u32 = 1 << 30;
    /// Whether alternate handling is explicitly set.
    const EXPLICIT_ALTERNATE_HANDLING_MASK: u32 = 1 << 29;
    /// Whether case level is explicitly set.
    const EXPLICIT_CASE_LEVEL_MASK: u32 = 1 << 28;
    /// Whether case first is explicitly set.
    const EXPLICIT_CASE_FIRST_MASK: u32 = 1 << 27;
    /// Whether backward secondary is explicitly set.
    const EXPLICIT_BACKWARD_SECOND_LEVEL_MASK: u32 = 1 << 26;
    /// Whether numeric is explicitly set.
    const EXPLICIT_NUMERIC_MASK: u32 = 1 << 25;

    pub const fn new() -> Self {
        Self(Strength::Tertiary as u32)
    }

    /// This is the BCP47 key `ks`.
    pub fn strength(&self) -> Strength {
        let mut bits = self.0 & CollatorOptions::STRENGTH_MASK;
        if bits > 4 {
            debug_assert!(false, "Bad value for strength");
            bits = 4;
        }
        // By construction in range and, therefore,
        // never UB.
        unsafe { core::mem::transmute(bits as u8) }
    }

    /// This is the BCP47 key `ks`. See the enum for examples.
    pub fn set_strength(&mut self, strength: Option<Strength>) {
        self.0 &= !CollatorOptions::STRENGTH_MASK;
        if let Some(strength) = strength {
            self.0 |= CollatorOptions::EXPLICIT_STRENGTH_MASK;
            self.0 |= strength as u32;
        } else {
            self.0 &= !CollatorOptions::EXPLICIT_STRENGTH_MASK;
        }
    }

    /// The maximum character class that `AlternateHandling::Shifted`
    /// applies to.
    pub fn max_variable(&self) -> MaxVariable {
        unsafe {
            core::mem::transmute(
                ((self.0 & CollatorOptions::MAX_VARIABLE_MASK)
                    >> CollatorOptions::MAX_VARIABLE_SHIFT) as u8,
            )
        }
    }

    /// The maximum character class that `AlternateHandling::Shifted`
    /// applies to. See the enum for examples.
    pub fn set_max_variable(&mut self, max_variable: Option<MaxVariable>) {
        self.0 &= !CollatorOptions::MAX_VARIABLE_MASK;
        if let Some(max_variable) = max_variable {
            self.0 |= CollatorOptions::EXPLICIT_MAX_VARIABLE_MASK;
            self.0 |= (max_variable as u32) << CollatorOptions::MAX_VARIABLE_SHIFT;
        } else {
            self.0 &= !CollatorOptions::EXPLICIT_MAX_VARIABLE_MASK;
        }
    }

    /// Whether certain characters are moved from the primary level to
    /// the quaternary level.
    pub fn alternate_handling(&self) -> AlternateHandling {
        if (self.0 & CollatorOptions::ALTERNATE_HANDLING_MASK) != 0 {
            AlternateHandling::Shifted
        } else {
            AlternateHandling::NonIgnorable
        }
    }

    /// Whether certain characters are moved from the primary level to
    /// the quaternary level. See the enum for examples.
    pub fn set_alternate_handling(&mut self, alternate_handling: Option<AlternateHandling>) {
        self.0 &= !CollatorOptions::ALTERNATE_HANDLING_MASK;
        if let Some(alternate_handling) = alternate_handling {
            self.0 |= CollatorOptions::EXPLICIT_ALTERNATE_HANDLING_MASK;
            if alternate_handling == AlternateHandling::Shifted {
                self.0 |= CollatorOptions::ALTERNATE_HANDLING_MASK;
            }
        } else {
            self.0 &= !CollatorOptions::EXPLICIT_ALTERNATE_HANDLING_MASK;
        }
    }

    /// Whether there's a dedicated case level.
    pub fn case_level(&self) -> bool {
        (self.0 & CollatorOptions::CASE_LEVEL_MASK) != 0
    }

    /// Whether there's a dedicated case level. If `true`, detaches
    /// the case aspect of the tertiary level and inserts it between
    /// the secondary and tertiary levels. Can be combined with the
    /// primary-only strength. Setting this to `true` with
    /// `Strength::Primary` corresponds to the ECMA-402 sensitivity
    /// "case".
    ///
    /// See https://unicode-org.github.io/icu/userguide/collation/concepts.html#caselevel
    ///
    /// ```
    /// use icu_collator::*;
    ///
    /// let data_provider = icu_testdata::get_provider();
    /// let mut options = CollatorOptions::new();
    /// options.set_strength(Some(Strength::Tertiary));
    /// let tertiary =
    ///   Collator::try_new(icu_locid::Locale::default(),
    ///                     &data_provider,
    ///                     options).unwrap();
    /// // The first string starts with full-width a
    /// assert_eq!(tertiary.compare("ａa", "aA"),
    ///            core::cmp::Ordering::Greater);
    ///
    /// options.set_case_level(Some(true));
    /// let tertiary_and_case =
    ///   Collator::try_new(icu_locid::Locale::default(),
    ///                     &data_provider,
    ///                     options).unwrap();
    /// // The first string starts with full-width a
    /// // TODO!!!!
    /// ```
    pub fn set_case_level(&mut self, case_level: Option<bool>) {
        self.0 &= !CollatorOptions::CASE_LEVEL_MASK;
        if let Some(case_level) = case_level {
            self.0 |= CollatorOptions::EXPLICIT_CASE_LEVEL_MASK;
            if case_level {
                self.0 |= CollatorOptions::ALTERNATE_HANDLING_MASK;
            }
        } else {
            self.0 &= !CollatorOptions::EXPLICIT_CASE_LEVEL_MASK;
        }
    }

    pub fn case_first(&self) -> CaseFirst {
        if (self.0 & CollatorOptions::CASE_FIRST_MASK) != 0 {
            if (self.0 & CollatorOptions::UPPER_FIRST_MASK) != 0 {
                CaseFirst::UpperFirst
            } else {
                CaseFirst::LowerFirst
            }
        } else {
            CaseFirst::Off
        }
    }

    pub fn set_case_first(&mut self, case_first: Option<CaseFirst>) {
        self.0 &= !(CollatorOptions::CASE_FIRST_MASK | CollatorOptions::UPPER_FIRST_MASK);
        if let Some(case_first) = case_first {
            self.0 |= CollatorOptions::EXPLICIT_CASE_FIRST_MASK;
            match case_first {
                CaseFirst::Off => {}
                CaseFirst::LowerFirst => {
                    self.0 |= CollatorOptions::CASE_FIRST_MASK;
                }
                CaseFirst::UpperFirst => {
                    self.0 |= CollatorOptions::CASE_FIRST_MASK;
                    self.0 |= CollatorOptions::UPPER_FIRST_MASK;
                }
            }
        } else {
            self.0 &= !CollatorOptions::EXPLICIT_CASE_FIRST_MASK;
        }
    }

    pub fn backward_second_level(&self) -> bool {
        (self.0 & CollatorOptions::BACKWARD_SECOND_LEVEL_MASK) != 0
    }

    pub fn set_backward_second_level(&mut self, backward_second_level: Option<bool>) {
        self.0 &= !CollatorOptions::BACKWARD_SECOND_LEVEL_MASK;
        if let Some(backward_second_level) = backward_second_level {
            self.0 |= CollatorOptions::EXPLICIT_BACKWARD_SECOND_LEVEL_MASK;
            if backward_second_level {
                self.0 |= CollatorOptions::BACKWARD_SECOND_LEVEL_MASK;
            }
        } else {
            self.0 &= !CollatorOptions::EXPLICIT_BACKWARD_SECOND_LEVEL_MASK;
        }
    }

    pub fn numeric(&self) -> bool {
        (self.0 & CollatorOptions::NUMERIC_MASK) != 0
    }

    pub fn set_numeric(&mut self, numeric: Option<bool>) {
        self.0 &= !CollatorOptions::NUMERIC_MASK;
        if let Some(numeric) = numeric {
            self.0 |= CollatorOptions::EXPLICIT_NUMERIC_MASK;
            if numeric {
                self.0 |= CollatorOptions::NUMERIC_MASK;
            }
        } else {
            self.0 &= !CollatorOptions::EXPLICIT_NUMERIC_MASK;
        }
    }

    // If strength is <= secondary, returns `None`.
    // Otherwise, returns the appropriate mask.
    pub(crate) fn tertiary_mask(&self) -> Option<u16> {
        if self.strength() <= Strength::Secondary {
            None
        } else if (self.0 & (CollatorOptions::CASE_FIRST_MASK | CollatorOptions::CASE_LEVEL_MASK))
            == CollatorOptions::CASE_FIRST_MASK
        {
            Some(CASE_MASK | TERTIARY_MASK)
        } else {
            Some(TERTIARY_MASK)
        }
    }

    pub(crate) fn upper_first(&self) -> bool {
        (self.0 & CollatorOptions::UPPER_FIRST_MASK) != 0
    }

    pub fn set_defaults(&mut self, other: CollatorOptions) {
        if self.0 & CollatorOptions::EXPLICIT_STRENGTH_MASK == 0 {
            self.0 &= !CollatorOptions::STRENGTH_MASK;
            self.0 |= other.0 & CollatorOptions::STRENGTH_MASK;
            self.0 |= other.0 & CollatorOptions::EXPLICIT_STRENGTH_MASK;
        }
        if self.0 & CollatorOptions::EXPLICIT_MAX_VARIABLE_MASK == 0 {
            self.0 &= !CollatorOptions::MAX_VARIABLE_MASK;
            self.0 |= other.0 & CollatorOptions::MAX_VARIABLE_MASK;
            self.0 |= other.0 & CollatorOptions::EXPLICIT_MAX_VARIABLE_MASK;
        }
        if self.0 & CollatorOptions::EXPLICIT_ALTERNATE_HANDLING_MASK == 0 {
            self.0 &= !CollatorOptions::ALTERNATE_HANDLING_MASK;
            self.0 |= other.0 & CollatorOptions::ALTERNATE_HANDLING_MASK;
            self.0 |= other.0 & CollatorOptions::EXPLICIT_ALTERNATE_HANDLING_MASK;
        }
        if self.0 & CollatorOptions::EXPLICIT_CASE_LEVEL_MASK == 0 {
            self.0 &= !CollatorOptions::CASE_LEVEL_MASK;
            self.0 |= other.0 & CollatorOptions::CASE_LEVEL_MASK;
            self.0 |= other.0 & CollatorOptions::EXPLICIT_CASE_LEVEL_MASK;
        }
        if self.0 & CollatorOptions::EXPLICIT_CASE_FIRST_MASK == 0 {
            self.0 &= !(CollatorOptions::CASE_FIRST_MASK | CollatorOptions::UPPER_FIRST_MASK);
            self.0 |=
                other.0 & (CollatorOptions::CASE_FIRST_MASK | CollatorOptions::UPPER_FIRST_MASK);
            self.0 |= other.0 & CollatorOptions::EXPLICIT_CASE_FIRST_MASK;
        }
        if self.0 & CollatorOptions::EXPLICIT_BACKWARD_SECOND_LEVEL_MASK == 0 {
            self.0 &= !CollatorOptions::BACKWARD_SECOND_LEVEL_MASK;
            self.0 |= other.0 & CollatorOptions::BACKWARD_SECOND_LEVEL_MASK;
            self.0 |= other.0 & CollatorOptions::EXPLICIT_BACKWARD_SECOND_LEVEL_MASK;
        }
        if self.0 & CollatorOptions::EXPLICIT_NUMERIC_MASK == 0 {
            self.0 &= !CollatorOptions::NUMERIC_MASK;
            self.0 |= other.0 & CollatorOptions::NUMERIC_MASK;
            self.0 |= other.0 & CollatorOptions::EXPLICIT_NUMERIC_MASK;
        }
    }
}
