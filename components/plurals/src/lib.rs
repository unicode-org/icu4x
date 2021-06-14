// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`icu_plurals`](crate) is one of the [`ICU4X`] components.
//!
//! This API provides functionality to determine the plural category
//! appropriate for a given number in a given language.
//!
//! For example in English language, when constructing a message
//! such as `{ num } items`, the user has to prepare
//! two variants of the message:
//!
//! * `1 item`
//! * `0 items`, `2 items`, `5 items`, `0.5 items` etc.
//!
//! The former variant is used when the placeholder variable has value `1`,
//! while the latter is used for all other values of the variable.
//!
//! Unicode defines [`Language Plural Rules`] as a mechanism to codify those
//! variants and provides data and algorithms to calculate
//! appropriate [`Plural Category`].
//!
//! # Examples
//!
//! ```
//! use icu::locid::macros::langid;
//! use icu::plurals::{PluralRules, PluralRuleType, PluralCategory};
//!
//! let lid = langid!("en");
//!
//! let provider = icu_testdata::get_provider();
//!
//! let pr = PluralRules::try_new(lid, &provider, PluralRuleType::Cardinal)
//!     .expect("Failed to construct a PluralRules struct.");
//!
//! assert_eq!(pr.select(5_usize), PluralCategory::Other);
//! ```
//!
//! ## Plural Rules
//!
//! The crate provides the main struct [`PluralRules`] which handles selection
//! of the correct [`Plural Category`] for a given language and [`Plural Type`].
//!
//! ## Plural Category
//!
//! Every number in every language belongs to a certain [`Plural Category`].
//! For example, Polish language uses four:
//!
//! * [`One`](PluralCategory::One): `1 miesiąc`
//! * [`Few`](PluralCategory::Few): `2 miesiące`
//! * [`Many`](PluralCategory::Many): `5 miesięcy`
//! * [`Other`](PluralCategory::Other): `1.5 miesiąca`
//!
//! ## Plural Rule Type
//!
//! Plural rules depend on the use case. This crate supports two types of plural rules:
//!
//! * [`Cardinal`](PluralRuleType::Cardinal): `3 doors`, `1 month`, `10 dollars`
//! * [`Ordinal`](PluralRuleType::Ordinal): `1st place`, `10th day`, `11th floor`
//!
//! [`ICU4X`]: ../icu/index.html
//! [`Plural Type`]: PluralRuleType
//! [`Plural Category`]: PluralCategory
//! [`Language Plural Rules`]: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
//! [`CLDR`]: http://cldr.unicode.org/
mod data;
mod error;
mod operands;
pub mod provider;
pub mod rules;

pub use error::PluralRulesError;
use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;
pub use operands::PluralOperands;
use provider::{resolver, PluralRuleStringsV1, PluralRuleStringsV1Marker};
use std::convert::TryInto;

/// A type of a plural rule which can be associated with the [`PluralRules`] struct.
///
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
/// use icu::locid::macros::langid;
/// use icu::plurals::{PluralRules, PluralRuleType, PluralCategory};
/// use icu_provider::inv::InvariantDataProvider;
///
/// let lid = langid!("en");
///
/// let dp = InvariantDataProvider;
///
/// let pr = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal)
///     .expect("Failed to construct a PluralRules struct.");
///
/// assert_eq!(pr.select(5_usize), PluralCategory::Other);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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
    /// assert_eq!(categories.next(), Some(&PluralCategory::Few));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Many));
    /// assert_eq!(categories.next(), Some(&PluralCategory::One));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Other));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Two));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Zero));
    /// assert_eq!(categories.next(), None);
    /// ```
    ///
    /// [`Plural Categories`]: PluralCategory
    pub fn all() -> impl ExactSizeIterator<Item = &'static Self> {
        [
            Self::Few,
            Self::Many,
            Self::One,
            Self::Other,
            Self::Two,
            Self::Zero,
        ]
        .iter()
    }
}

/// A struct which provides an ability to retrieve an appropriate
/// [`Plural Category`] for a given number.
///
/// # Examples
///
/// ```
/// use icu::locid::macros::langid;
/// use icu::plurals::{PluralRules, PluralRuleType, PluralCategory};
/// use icu_provider::inv::InvariantDataProvider;
///
/// let lid = langid!("en");
///
/// let dp = InvariantDataProvider;
///
/// let pr = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal)
///     .expect("Failed to construct a PluralRules struct.");
///
/// assert_eq!(pr.select(5_usize), PluralCategory::Other);
/// ```
///
/// [`ICU4X`]: ../icu/index.html
/// [`Plural Type`]: PluralRuleType
/// [`Plural Category`]: PluralCategory
pub struct PluralRules {
    _langid: LanguageIdentifier,
    selector: data::RulesSelector,
}

impl PluralRules {
    /// Constructs a new `PluralRules` for a given locale, [`type`] and [`data provider`].
    ///
    /// This constructor will fail if the [`Data Provider`] does not have the data.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::macros::langid;
    /// use icu::plurals::{PluralRules, PluralRuleType};
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let lid = langid!("en");
    ///
    /// let dp = InvariantDataProvider;
    ///
    /// let _ = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal);
    /// ```
    ///
    /// [`type`]: PluralRuleType
    /// [`data provider`]: icu_provider::DataProvider
    pub fn try_new<'d, 's: 'd, D: DataProvider<'d, 's, PluralRuleStringsV1Marker> + ?Sized>(
        langid: LanguageIdentifier,
        data_provider: &D,
        type_: PluralRuleType,
    ) -> Result<Self, PluralRulesError> {
        let data = resolver::resolve_plural_data(langid.clone(), data_provider, type_)?;
        Self::new_from_data(langid, data.get())
    }

    /// Returns the [`Plural Category`] appropriate for the given number.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::macros::langid;
    /// use icu::plurals::{PluralRules, PluralRuleType, PluralCategory};
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let lid = langid!("en");
    ///
    /// let dp = InvariantDataProvider;
    ///
    /// let pr = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal)
    ///     .expect("Failed to construct a PluralRules struct.");
    ///
    /// match pr.select(1_usize) {
    ///     PluralCategory::One => "One item",
    ///     PluralCategory::Other => "Many items",
    ///     _ => { unreachable!(); }
    /// };
    /// ```
    ///
    /// The method accepts any input that can be calculated into [`Plural Operands`].
    /// All unsigned primitive number types can infallibly be converted so they can be
    /// used as an input.
    ///
    /// For signed numbers and strings, [`Plural Operands`] implement [`TryFrom`](std::convert::TryFrom)
    /// and [`FromStr`](std::str::FromStr), which should be used before passing the result to
    /// [`select()`](PluralRules::select()).
    ///
    /// # Examples
    ///
    /// ```
    /// use std::convert::TryFrom;
    /// use icu::locid::macros::langid;
    /// use icu::plurals::{PluralRules, PluralRuleType};
    /// use icu::plurals::{PluralCategory, PluralOperands};
    /// use icu_provider::inv::InvariantDataProvider;
    /// #
    /// # let lid = langid!("en");
    /// #
    /// # let dp = InvariantDataProvider;
    /// #
    /// # let pr = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal)
    /// #     .expect("Failed to construct a PluralRules struct.");
    ///
    /// let operands = PluralOperands::try_from(-5)
    ///     .expect("Failed to parse to operands.");
    /// let operands2: PluralOperands = "5.10".parse()
    ///     .expect("Failed to parse to operands.");
    ///
    /// assert_eq!(pr.select(operands), PluralCategory::Other);
    /// assert_eq!(pr.select(operands2), PluralCategory::Other);
    /// ```
    ///
    /// [`Plural Category`]: PluralCategory
    /// [`Plural Operands`]: operands::PluralOperands
    pub fn select<I: Into<PluralOperands>>(&self, input: I) -> PluralCategory {
        self.selector.select(&input.into())
    }

    /// Returns all [`Plural Categories`] appropriate for a [`PluralRules`] object
    /// based on the [`LanguageIdentifier`] and [`PluralRuleType`].
    ///
    /// The [`Plural Categories`] are returned in alphabetically sorted order.
    ///
    /// The category [`PluralCategory::Other`] is always included.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locid::macros::langid;
    /// use icu::plurals::{PluralRules, PluralRuleType, PluralCategory};
    /// use icu_provider::inv::InvariantDataProvider;
    ///
    /// let lid = langid!("fr");
    ///
    /// let dp = icu_testdata::get_provider();
    ///
    /// let pr = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal)
    ///     .expect("Failed to construct a PluralRules struct.");
    ///
    /// let mut categories = pr.categories();
    /// assert_eq!(categories.next(), Some(&PluralCategory::Many));
    /// assert_eq!(categories.next(), Some(&PluralCategory::One));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Other));
    /// assert_eq!(categories.next(), None);
    /// ```
    ///
    /// [`Plural Categories`]: PluralCategory
    pub fn categories(&self) -> impl Iterator<Item = &'static PluralCategory> + '_ {
        self.selector.categories()
    }

    /// Lower-level constructor that allows constructing a [`PluralRules`] directly from
    /// data obtained from a provider.
    pub fn new_from_data(
        langid: LanguageIdentifier,
        data: &PluralRuleStringsV1,
    ) -> Result<Self, PluralRulesError> {
        let data: data::PluralRuleList = data.try_into()?;
        Ok(Self {
            _langid: langid,
            selector: data.into(),
        })
    }
}
