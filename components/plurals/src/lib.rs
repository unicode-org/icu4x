// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Determine the plural category appropriate for a given number in a given language.
//!
//! This module is published as its own crate ([`icu_plurals`](https://docs.rs/icu_plurals/latest/icu_plurals/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! For example in English, when constructing a message
//! such as `{ num } items`, the user has to prepare
//! two variants of the message:
//!
//! * `1 item`
//! * `0 items`, `2 items`, `5 items`, `0.5 items` etc.
//!
//! The former variant is used when the placeholder variable has value `1`,
//! while the latter is used for all other values of the variable.
//!
//! Unicode defines [Language Plural Rules] as a mechanism to codify those
//! variants and provides data and algorithms to calculate
//! appropriate [`PluralCategory`].
//!
//! # Examples
//!
//! ```
//! use icu::locid::locale;
//! use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
//!
//! let pr =
//!     PluralRules::try_new(&locale!("en").into(), PluralRuleType::Cardinal)
//!         .expect("locale should be present");
//!
//! assert_eq!(pr.category_for(5_usize), PluralCategory::Other);
//! ```
//!
//! ## Plural Rules
//!
//! The crate provides the main struct [`PluralRules`] which handles selection
//! of the correct [`PluralCategory`] for a given language and [`PluralRuleType`].
//!
//! ## Plural Category
//!
//! Every number in every language belongs to a certain [`PluralCategory`].
//! For example, the Polish language uses four:
//!
//! * [`One`](PluralCategory::One): `1 miesiąc`
//! * [`Few`](PluralCategory::Few): `2 miesiące`
//! * [`Many`](PluralCategory::Many): `5 miesięcy`
//! * [`Other`](PluralCategory::Other): `1.5 miesiąca`
//!
//! ## `PluralRuleType`
//!
//! Plural rules depend on the use case. This crate supports two types of plural rules:
//!
//! * [`Cardinal`](PluralRuleType::Cardinal): `3 doors`, `1 month`, `10 dollars`
//! * [`Ordinal`](PluralRuleType::Ordinal): `1st place`, `10th day`, `11th floor`
//!
//! [Language Plural Rules]: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

mod error;
mod operands;
pub mod provider;
pub mod rules;

use core::cmp::{Ord, PartialOrd};
pub use error::PluralsError;
use icu_provider::prelude::*;
pub use operands::PluralOperands;
use provider::CardinalV1Marker;
use provider::ErasedPluralRulesV1Marker;
use provider::OrdinalV1Marker;
use rules::runtime::test_rule;

#[cfg(feature = "experimental")]
use provider::PluralRangesV1Marker;
#[cfg(feature = "experimental")]
use provider::UnvalidatedPluralRange;

#[doc(no_inline)]
pub use PluralsError as Error;

/// A type of a plural rule which can be associated with the [`PluralRules`] struct.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[non_exhaustive]
pub enum PluralRuleType {
    /// Cardinal plural forms express quantities of units such as time, currency or distance,
    /// used in conjunction with a number expressed in decimal digits (i.e. "2", not "two").
    ///
    /// For example, English has two forms for cardinals:
    ///
    /// * [`One`]: `1 day`
    /// * [`Other`]: `0 days`, `2 days`, `10 days`, `0.3 days`
    ///
    /// [`One`]: PluralCategory::One
    /// [`Other`]: PluralCategory::Other
    Cardinal,
    /// Ordinal plural forms denote the order of items in a set and are always integers.
    ///
    /// For example, English has four forms for ordinals:
    ///
    /// * [`One`]: `1st floor`, `21st floor`, `101st floor`
    /// * [`Two`]: `2nd floor`, `22nd floor`, `102nd floor`
    /// * [`Few`]: `3rd floor`, `23rd floor`, `103rd floor`
    /// * [`Other`]: `4th floor`, `11th floor`, `96th floor`
    ///
    /// [`One`]: PluralCategory::One
    /// [`Two`]: PluralCategory::Two
    /// [`Few`]: PluralCategory::Few
    /// [`Other`]: PluralCategory::Other
    Ordinal,
}

/// The plural categories are used to format messages with numeric placeholders, expressed as decimal numbers.
///
/// The fundamental rule for determining plural categories is the existence of minimal pairs: whenever two different
/// numbers may require different versions of the same message, then the numbers have different plural categories.
///
/// All languages supported by `ICU4X` can match any number to one of the categories.
///
/// # Examples
///
/// ```
/// use icu::locid::locale;
/// use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
///
/// let pr =
///     PluralRules::try_new(&locale!("en").into(), PluralRuleType::Cardinal)
///         .expect("locale should be present");
///
/// assert_eq!(pr.category_for(5_usize), PluralCategory::Other);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[allow(clippy::exhaustive_enums)] // this type is mostly stable. new categories may potentially be added in the future,
                                   // but at a cadence slower than the ICU4X release cycle
pub enum PluralCategory {
    /// CLDR "zero" plural category. Used in Arabic and Latvian, among others.
    ///
    /// Examples of numbers having this category:
    ///
    /// - 0 in Arabic (ar), Latvian (lv)
    /// - 10~20, 30, 40, 50, ... in Latvian (lv)
    Zero,
    /// CLDR "one" plural category. Signifies the singular form in many languages.
    ///
    /// Examples of numbers having this category:
    ///
    /// - 0 in French (fr), Portuguese (pt), ...
    /// - 1 in English (en) and most other languages
    /// - 2.1 in Filipino (fil), Croatian (hr), Latvian (lv), Serbian (sr)
    /// - 2, 3, 5, 7, 8, ... in Filipino (fil)
    One,
    /// CLDR "two" plural category. Used in Arabic, Hebrew, and Slovenian, among others.
    ///
    /// Examples of numbers having this category:
    ///
    /// - 2 in Arabic (ar), Hebrew (iw), Slovenian (sl)
    /// - 2.0 in Arabic (ar)
    Two,
    /// CLDR "few" plural category. Used in Romanian, Polish, Russian, and others.
    ///
    /// Examples of numbers having this category:
    ///
    /// - 0 in Romanian (ro)
    /// - 1.2 in Croatian (hr), Romanian (ro), Slovenian (sl), Serbian (sr)
    /// - 2 in Polish (pl), Russian (ru), Czech (cs), ...
    /// - 5 in Arabic (ar), Lithuanian (lt), Romanian (ro)
    Few,
    /// CLDR "many" plural category. Used in Polish, Russian, Ukrainian, and others.
    ///
    /// Examples of numbers having this category:
    ///
    /// - 0 in Polish (pl)
    /// - 1.0 in Czech (cs), Slovak (sk)
    /// - 1.1 in Czech (cs), Lithuanian (lt), Slovak (sk)
    /// - 15 in Arabic (ar), Polish (pl), Russian (ru), Ukrainian (uk)
    Many,
    /// CLDR "other" plural category, used as a catch-all. Each language supports it, and it
    /// is also used as a fail safe result for in case no better match can be identified.
    ///
    /// In some languages, such as Japanese, Chinese, Korean, and Thai, this is the only
    /// plural category.
    ///
    /// Examples of numbers having this category:
    ///
    /// - 0 in English (en), German (de), Spanish (es), ...
    /// - 1 in Japanese (ja), Korean (ko), Chinese (zh), Thai (th), ...
    /// - 2 in English (en), German (de), Spanish (es), ...
    Other,
}

impl PluralCategory {
    /// Returns an ordered iterator over variants of [`Plural Categories`].
    ///
    /// Categories are returned in alphabetical order.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::plurals::PluralCategory;
    ///
    /// let mut categories = PluralCategory::all();
    ///
    /// assert_eq!(categories.next(), Some(PluralCategory::Few));
    /// assert_eq!(categories.next(), Some(PluralCategory::Many));
    /// assert_eq!(categories.next(), Some(PluralCategory::One));
    /// assert_eq!(categories.next(), Some(PluralCategory::Other));
    /// assert_eq!(categories.next(), Some(PluralCategory::Two));
    /// assert_eq!(categories.next(), Some(PluralCategory::Zero));
    /// assert_eq!(categories.next(), None);
    /// ```
    ///
    /// [`Plural Categories`]: PluralCategory
    pub fn all() -> impl ExactSizeIterator<Item = Self> {
        [
            Self::Few,
            Self::Many,
            Self::One,
            Self::Other,
            Self::Two,
            Self::Zero,
        ]
        .iter()
        .copied()
    }

    /// Returns the PluralCategory corresponding to given TR35 string.
    pub fn get_for_cldr_string(category: &str) -> Option<PluralCategory> {
        Self::get_for_cldr_bytes(category.as_bytes())
    }
    /// Returns the PluralCategory corresponding to given TR35 string as bytes
    pub fn get_for_cldr_bytes(category: &[u8]) -> Option<PluralCategory> {
        match category {
            b"zero" => Some(PluralCategory::Zero),
            b"one" => Some(PluralCategory::One),
            b"two" => Some(PluralCategory::Two),
            b"few" => Some(PluralCategory::Few),
            b"many" => Some(PluralCategory::Many),
            b"other" => Some(PluralCategory::Other),
            _ => None,
        }
    }
}

/// A struct which provides an ability to retrieve an appropriate
/// [`Plural Category`] for a given number.
///
/// # Examples
///
/// ```
/// use icu::locid::locale;
/// use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
///
/// let pr =
///     PluralRules::try_new(&locale!("en").into(), PluralRuleType::Cardinal)
///         .expect("locale should be present");
///
/// assert_eq!(pr.category_for(5_usize), PluralCategory::Other);
/// ```
///
/// [`ICU4X`]: ../icu/index.html
/// [`Plural Type`]: PluralRuleType
/// [`Plural Category`]: PluralCategory
#[derive(Debug)]
pub struct PluralRules(DataPayload<ErasedPluralRulesV1Marker>);

impl AsRef<PluralRules> for PluralRules {
    fn as_ref(&self) -> &PluralRules {
        self
    }
}

impl PluralRules {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        rule_type: PluralRuleType,
        error: PluralsError,
        /// Constructs a new `PluralRules` for a given locale and type using compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locid::locale;
        /// use icu::plurals::{PluralRuleType, PluralRules};
        ///
        /// let _ = PluralRules::try_new(
        ///     &locale!("en").into(),
        ///     PluralRuleType::Cardinal,
        /// ).expect("locale should be present");
        /// ```
        ///
        /// [`type`]: PluralRuleType
        /// [`data provider`]: icu_provider
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<CardinalV1Marker> + DataProvider<OrdinalV1Marker> + ?Sized),
        locale: &DataLocale,
        rule_type: PluralRuleType,
    ) -> Result<Self, PluralsError> {
        match rule_type {
            PluralRuleType::Cardinal => Self::try_new_cardinal_unstable(provider, locale),
            PluralRuleType::Ordinal => Self::try_new_ordinal_unstable(provider, locale),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: skip,
        error: PluralsError,
        /// Constructs a new `PluralRules` for a given locale for cardinal numbers using compiled data.
        ///
        /// Cardinal plural forms express quantities of units such as time, currency or distance,
        /// used in conjunction with a number expressed in decimal digits (i.e. "2", not "two").
        ///
        /// For example, English has two forms for cardinals:
        ///
        /// * [`One`]: `1 day`
        /// * [`Other`]: `0 days`, `2 days`, `10 days`, `0.3 days`
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locid::locale;
        /// use icu::plurals::{PluralCategory, PluralRules};
        ///
        /// let rules = PluralRules::try_new_cardinal(&locale!("ru").into()).expect("locale should be present");
        ///
        /// assert_eq!(rules.category_for(2_usize), PluralCategory::Few);
        /// ```
        ///
        /// [`One`]: PluralCategory::One
        /// [`Other`]: PluralCategory::Other
        functions: [
            try_new_cardinal,
            try_new_cardinal_with_any_provider,
            try_new_cardinal_with_buffer_provider,
            try_new_cardinal_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_cardinal)]
    pub fn try_new_cardinal_unstable(
        provider: &(impl DataProvider<CardinalV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<Self, PluralsError> {
        Ok(Self(
            provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?
                .cast(),
        ))
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: skip,
        error: PluralsError,
        /// Constructs a new `PluralRules` for a given locale for ordinal numbers using compiled data.
        ///
        /// Ordinal plural forms denote the order of items in a set and are always integers.
        ///
        /// For example, English has four forms for ordinals:
        ///
        /// * [`One`]: `1st floor`, `21st floor`, `101st floor`
        /// * [`Two`]: `2nd floor`, `22nd floor`, `102nd floor`
        /// * [`Few`]: `3rd floor`, `23rd floor`, `103rd floor`
        /// * [`Other`]: `4th floor`, `11th floor`, `96th floor`
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locid::locale;
        /// use icu::plurals::{PluralCategory, PluralRules};
        ///
        /// let rules = PluralRules::try_new_ordinal(
        ///     &locale!("ru").into(),
        /// )
        /// .expect("locale should be present");
        ///
        /// assert_eq!(rules.category_for(2_usize), PluralCategory::Other);
        /// ```
        ///
        /// [`One`]: PluralCategory::One
        /// [`Two`]: PluralCategory::Two
        /// [`Few`]: PluralCategory::Few
        /// [`Other`]: PluralCategory::Other
        functions: [
            try_new_ordinal,
            try_new_ordinal_with_any_provider,
            try_new_ordinal_with_buffer_provider,
            try_new_ordinal_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_ordinal)]
    pub fn try_new_ordinal_unstable(
        provider: &(impl DataProvider<OrdinalV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<Self, PluralsError> {
        Ok(Self(
            provider
                .load(DataRequest {
                    locale,
                    metadata: Default::default(),
                })?
                .take_payload()?
                .cast(),
        ))
    }

    /// Returns the [`Plural Category`] appropriate for the given number.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
    ///
    /// let pr =
    ///     PluralRules::try_new(&locale!("en").into(), PluralRuleType::Cardinal)
    ///         .expect("locale should be present");
    ///
    /// match pr.category_for(1_usize) {
    ///     PluralCategory::One => "One item",
    ///     PluralCategory::Other => "Many items",
    ///     _ => unreachable!(),
    /// };
    /// ```
    ///
    /// The method accepts any input that can be calculated into [`Plural Operands`].
    /// All unsigned primitive number types can infallibly be converted so they can be
    /// used as an input.
    ///
    /// For signed numbers and strings, [`Plural Operands`] implement [`TryFrom`]
    /// and [`FromStr`](std::str::FromStr), which should be used before passing the result to
    /// [`category_for()`](PluralRules::category_for()).
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::plurals::{PluralCategory, PluralOperands};
    /// use icu::plurals::{PluralRuleType, PluralRules};
    /// use std::convert::TryFrom;
    /// #
    /// # let pr = PluralRules::try_new(&locale!("en").into(), PluralRuleType::Cardinal)
    /// #     .expect("locale should be present");
    ///
    /// let operands = PluralOperands::try_from(-5).expect("Failed to parse to operands.");
    /// let operands2: PluralOperands = "5.10".parse().expect("Failed to parse to operands.");
    ///
    /// assert_eq!(pr.category_for(operands), PluralCategory::Other);
    /// assert_eq!(pr.category_for(operands2), PluralCategory::Other);
    /// ```
    ///
    /// [`Plural Category`]: PluralCategory
    /// [`Plural Operands`]: operands::PluralOperands
    pub fn category_for<I: Into<PluralOperands>>(&self, input: I) -> PluralCategory {
        let rules = self.0.get();
        let input = input.into();

        macro_rules! test_rule {
            ($rule:ident, $cat:ident) => {
                rules
                    .$rule
                    .as_ref()
                    .and_then(|r| test_rule(r, &input).then(|| PluralCategory::$cat))
            };
        }

        test_rule!(zero, Zero)
            .or_else(|| test_rule!(one, One))
            .or_else(|| test_rule!(two, Two))
            .or_else(|| test_rule!(few, Few))
            .or_else(|| test_rule!(many, Many))
            .unwrap_or(PluralCategory::Other)
    }

    /// Returns all [`Plural Categories`] appropriate for a [`PluralRules`] object
    /// based on the [`LanguageIdentifier`](icu::locid::{LanguageIdentifier}) and [`PluralRuleType`].
    ///
    /// The [`Plural Categories`] are returned in UTS 35 sorted order.
    ///
    /// The category [`PluralCategory::Other`] is always included.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::plurals::{PluralCategory, PluralRuleType, PluralRules};
    ///
    /// let pr =
    ///     PluralRules::try_new(&locale!("fr").into(), PluralRuleType::Cardinal)
    ///         .expect("locale should be present");
    ///
    /// let mut categories = pr.categories();
    /// assert_eq!(categories.next(), Some(PluralCategory::One));
    /// assert_eq!(categories.next(), Some(PluralCategory::Many));
    /// assert_eq!(categories.next(), Some(PluralCategory::Other));
    /// assert_eq!(categories.next(), None);
    /// ```
    ///
    /// [`Plural Categories`]: PluralCategory
    pub fn categories(&self) -> impl Iterator<Item = PluralCategory> + '_ {
        let rules = self.0.get();

        macro_rules! test_rule {
            ($rule:ident, $cat:ident) => {
                rules
                    .$rule
                    .as_ref()
                    .map(|_| PluralCategory::$cat)
                    .into_iter()
            };
        }

        test_rule!(zero, Zero)
            .chain(test_rule!(one, One))
            .chain(test_rule!(two, Two))
            .chain(test_rule!(few, Few))
            .chain(test_rule!(many, Many))
            .chain(Some(PluralCategory::Other))
    }
}

/// A [`PluralRules`] that also has the ability to retrieve an appropriate [`Plural Category`] for a
/// range.
///
/// ✨ *Enabled with the `experimental` Cargo feature.*
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/4140">#4140</a>
/// </div>
///
/// # Examples
///
/// ```
/// use icu::locid::locale;
/// use icu::plurals::{PluralCategory, PluralOperands};
/// use icu::plurals::{PluralRuleType, PluralRulesWithRanges};
/// use std::convert::TryFrom;
///
/// let ranges = PluralRulesWithRanges::try_new(
///     &locale!("ar").into(),
///     PluralRuleType::Cardinal,
/// )
/// .expect("locale should be present");
///
/// let operands = PluralOperands::from(1_usize);
/// let operands2: PluralOperands =
///     "2.0".parse().expect("parsing to operands should succeed");
///
/// assert_eq!(
///     ranges.category_for_range(operands, operands2),
///     PluralCategory::Other
/// );
/// ```
///
/// [`Plural Category`]: PluralCategory
#[cfg(feature = "experimental")]
#[derive(Debug)]
pub struct PluralRulesWithRanges<R> {
    rules: R,
    ranges: DataPayload<PluralRangesV1Marker>,
}

#[cfg(feature = "experimental")]
impl PluralRulesWithRanges<PluralRules> {
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        rule_type: PluralRuleType,
        error: PluralsError,
        /// Constructs a new `PluralRulesWithRanges` for a given locale using compiled data.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locid::locale;
        /// use icu::plurals::{PluralRuleType, PluralRulesWithRanges};
        ///
        /// let _ = PluralRulesWithRanges::try_new(
        ///     &locale!("en").into(),
        ///     PluralRuleType::Cardinal,
        /// ).expect("locale should be present");
        /// ```
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable(
        provider: &(impl DataProvider<PluralRangesV1Marker>
              + DataProvider<CardinalV1Marker>
              + DataProvider<OrdinalV1Marker>
              + ?Sized),
        locale: &DataLocale,
        rule_type: PluralRuleType,
    ) -> Result<Self, PluralsError> {
        match rule_type {
            PluralRuleType::Cardinal => Self::try_new_cardinal_unstable(provider, locale),
            PluralRuleType::Ordinal => Self::try_new_ordinal_unstable(provider, locale),
        }
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: skip,
        error: PluralsError,
        /// Constructs a new `PluralRulesWithRanges` for a given locale for cardinal numbers using
        /// compiled data.
        ///
        /// See [`PluralRules::try_new_cardinal`] for more information.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locid::locale;
        /// use icu::plurals::{PluralCategory, PluralRulesWithRanges};
        ///
        /// let rules = PluralRulesWithRanges::try_new_cardinal(&locale!("ru").into())
        ///     .expect("locale should be present");
        ///
        /// assert_eq!(rules.category_for_range(0_usize, 2_usize), PluralCategory::Few);
        /// ```
        functions: [
            try_new_cardinal,
            try_new_cardinal_with_any_provider,
            try_new_cardinal_with_buffer_provider,
            try_new_cardinal_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_cardinal)]
    pub fn try_new_cardinal_unstable(
        provider: &(impl DataProvider<CardinalV1Marker> + DataProvider<PluralRangesV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<Self, PluralsError> {
        let rules = PluralRules::try_new_cardinal_unstable(provider, locale)?;

        PluralRulesWithRanges::try_new_with_rules_unstable(provider, locale, rules)
    }

    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        options: skip,
        error: PluralsError,
        /// Constructs a new `PluralRulesWithRanges` for a given locale for ordinal numbers using
        /// compiled data.
        ///
        /// See [`PluralRules::try_new_ordinal`] for more information.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locid::locale;
        /// use icu::plurals::{PluralCategory, PluralRulesWithRanges};
        ///
        /// let rules = PluralRulesWithRanges::try_new_ordinal(
        ///     &locale!("ru").into(),
        /// )
        /// .expect("locale should be present");
        ///
        /// assert_eq!(rules.category_for_range(0_usize, 2_usize), PluralCategory::Other);
        /// ```
        functions: [
            try_new_ordinal,
            try_new_ordinal_with_any_provider,
            try_new_ordinal_with_buffer_provider,
            try_new_ordinal_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_ordinal)]
    pub fn try_new_ordinal_unstable(
        provider: &(impl DataProvider<OrdinalV1Marker> + DataProvider<PluralRangesV1Marker> + ?Sized),
        locale: &DataLocale,
    ) -> Result<Self, PluralsError> {
        let rules = PluralRules::try_new_ordinal_unstable(provider, locale)?;

        PluralRulesWithRanges::try_new_with_rules_unstable(provider, locale, rules)
    }
}

#[cfg(feature = "experimental")]
impl<R> PluralRulesWithRanges<R>
where
    R: AsRef<PluralRules>,
{
    icu_provider::gen_any_buffer_data_constructors!(
        locale: include,
        rules: R,
        error: PluralsError,
        /// Constructs a new `PluralRulesWithRanges` for a given locale from an existing
        /// `PluralRules` (either owned or as a reference) and compiled data.
        ///
        /// # ⚠️ Warning
        ///
        /// The provided `locale` **MUST** be the same as the locale provided to the constructor
        /// of `rules`. Otherwise, [`Self::category_for_range`] will return incorrect results.
        ///
        /// ✨ *Enabled with the `compiled_data` Cargo feature.*
        ///
        /// [📚 Help choosing a constructor](icu_provider::constructors)
        ///
        /// # Examples
        ///
        /// ```
        /// use icu::locid::locale;
        /// use icu::plurals::{PluralRuleType, PluralRulesWithRanges, PluralRules};
        ///
        /// let rules = PluralRules::try_new(&locale!("en").into(), PluralRuleType::Cardinal)
        ///     .expect("locale should be present");
        ///
        /// let _ =
        ///     PluralRulesWithRanges::try_new_with_rules(&locale!("en").into(), rules)
        ///         .expect("locale should be present");
        /// ```
        functions: [
            try_new_with_rules,
            try_new_with_rules_with_any_provider,
            try_new_with_rules_with_buffer_provider,
            try_new_with_rules_unstable,
            Self,
        ]
    );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new_with_rules)]
    pub fn try_new_with_rules_unstable(
        provider: &(impl DataProvider<PluralRangesV1Marker> + ?Sized),
        locale: &DataLocale,
        rules: R,
    ) -> Result<Self, PluralsError> {
        let ranges = provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })?
            .take_payload()?;

        Ok(Self { rules, ranges })
    }

    /// Gets a reference to the inner `PluralRules`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::plurals::{PluralCategory, PluralRulesWithRanges};
    ///
    /// let ranges =
    ///     PluralRulesWithRanges::try_new_cardinal(&locale!("en").into())
    ///         .expect("locale should be present");
    ///
    /// let rules = ranges.rules();
    ///
    /// assert_eq!(rules.category_for(1u8), PluralCategory::One);
    /// ```
    pub fn rules(&self) -> &PluralRules {
        self.rules.as_ref()
    }

    /// Returns the [`Plural Category`] appropriate for a range.
    ///
    /// Note that the returned category is correct only if the range fulfills the following requirements:
    /// - The start value is strictly less than the end value.
    /// - Both values are positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::plurals::{
    ///     PluralCategory, PluralOperands, PluralRuleType, PluralRulesWithRanges,
    /// };
    ///
    /// let ranges = PluralRulesWithRanges::try_new(
    ///     &locale!("ro").into(),
    ///     PluralRuleType::Cardinal,
    /// )
    /// .expect("locale should be present");
    /// let operands: PluralOperands =
    ///     "0.5".parse().expect("parsing to operands should succeed");
    /// let operands2 = PluralOperands::from(1_usize);
    ///
    /// assert_eq!(
    ///     ranges.category_for_range(operands, operands2),
    ///     PluralCategory::Few
    /// );
    /// ```
    ///
    /// [`Plural Category`]: PluralCategory
    pub fn category_for_range<S: Into<PluralOperands>, E: Into<PluralOperands>>(
        &self,
        start: S,
        end: E,
    ) -> PluralCategory {
        let rules = self.rules.as_ref();
        let start = rules.category_for(start);
        let end = rules.category_for(end);

        self.resolve_range(start, end)
    }

    /// Returns the [`Plural Category`] appropriate for a range from the categories of its endpoints.
    ///
    /// Note that the returned category is correct only if the original numeric range fulfills the
    /// following requirements:
    /// - The start value is strictly less than the end value.
    /// - Both values are positive.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::locale;
    /// use icu::plurals::{PluralCategory, PluralRuleType, PluralRulesWithRanges};
    ///
    /// let ranges = PluralRulesWithRanges::try_new(
    ///     &locale!("sl").into(),
    ///     PluralRuleType::Ordinal,
    /// )
    /// .expect("locale should be present");
    ///
    /// assert_eq!(
    ///     ranges.resolve_range(PluralCategory::Other, PluralCategory::One),
    ///     PluralCategory::Few
    /// );
    /// ```
    ///
    /// [`Plural Category`]: PluralCategory
    pub fn resolve_range(&self, start: PluralCategory, end: PluralCategory) -> PluralCategory {
        self.ranges
            .get()
            .ranges
            .get_copied(&UnvalidatedPluralRange::from_range(
                start.into(),
                end.into(),
            ))
            .map(PluralCategory::from)
            .unwrap_or(end)
    }
}
