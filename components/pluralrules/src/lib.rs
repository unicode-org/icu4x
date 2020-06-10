//! `icu-pluralrules` is one of the [`ICU4X`] components.
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
//! use icu_locale::LanguageIdentifier;
//! use icu_pluralrules::{PluralRules, PluralRuleType, PluralCategory};
//! use icu_pluralrules::data::provider::DummyDataProvider;
//!
//! let lang: LanguageIdentifier = "en".parse()
//!     .expect("Failed to parse a language identifier.");
//!
//! let dp = DummyDataProvider::default();
//!
//! let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp)
//!     .expect("Failed to construct a PluralRules struct.");
//!
//! assert_eq!(pr.select(1_usize), PluralCategory::One);
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
//! * [`One`](./enum.PluralCategory.html#variant.One): `1 miesiąc`
//! * [`Few`](./enum.PluralCategory.html#variant.Few): `2 miesiące`
//! * [`Many`](./enum.PluralCategory.html#variant.Many): `5 miesięcy`
//! * [`Other`](./enum.PluralCategory.html#variant.Other): `1.5 miesiąca`
//!
//! ## Plural Rule Type
//!
//! Plural rules depend on the use case. This crate supports two types of plural rules:
//!
//! * [`Cardinal`](./enum.PluralRuleType.html#variant.Cardinal): `3 doors`, `1 month`, `10 dollars`
//! * [`Ordinal`](./enum.PluralRuleType.html#variant.Ordinal): `1st place`, `10th day`, `11th floor`
//!
//! ## Data Provider
//!
//! In order to function, the API requires data from [`CLDR`].
//!
//! [`ICU4X`] is going to use a special API for handling data management called `DataProvider`.
//! Until that happens, this crate will provide a simple `JSON` and `bincode` providers behind a
//! flag.
//! For tests and documentation examples, there is also a `DummyDataProvider`.
//!
//! All of the content of the [`data`] module is heavily experimental and subject to change.
//!
//! [`ICU4X`]: https://github.com/unicode-org/icu4x
//! [`PluralRules`]: ./struct.PluralRules.html
//! [`PluralRules`]: ./struct.PluralRules.html
//! [`Plural Type`]: ./enum.PluralRuleType.html
//! [`Plural Category`]: ./enum.PluralCategory.html
//! [`Language Plural Rules`]: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
//! [`CLDR`]: http://cldr.unicode.org/
//! [`data`]: ./data/index.html
pub mod data;
mod operands;
pub mod rules;

use icu_locale::LanguageIdentifier;
pub use operands::PluralOperands;

/// A type of a plural rule which can be associated with the [`PluralRules`] struct.
///
/// [`PluralRules`]: ./struct.PluralRules.html
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PluralRuleType {
    /// Ordinal plural forms denote the order of items in a set and are always integers.
    ///
    /// For example, English has four forms for ordinals:
    ///
    /// * [`One`]: `1st floor`, `21st floor`, `101st floor`
    /// * [`Two`]: `2nd floor`, `22nd floor`, `102nd floor`
    /// * [`Few`]: `3rd floor`, `23rd floor`, `103rd floor`
    /// * [`Other`]: `4th floor`, `11th floor`, `96th floor`
    ///
    /// [`One`]: ./enum.PluralCategory.html#variant.One
    /// [`Two`]: ./enum.PluralCategory.html#variant.Two
    /// [`Few`]: ./enum.PluralCategory.html#variant.Few
    /// [`Other`]: ./enum.PluralCategory.html#variant.Other
    Cardinal,
    /// Cardinal plural forms express quantities of units such as time, currency or distance,
    /// used in conjunction with a number expressed in decimal digits (i.e. "2", not "two").
    ///
    /// For example, English has two forms for cardinals:
    ///
    /// * [`One`]: `1 day`
    /// * [`Other`]: `0 days`, `2 days`, `10 days`, `0.3 days`
    ///
    /// [`One`]: ./enum.PluralCategory.html#variant.One
    /// [`Other`]: ./enum.PluralCategory.html#variant.Other
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
/// use icu_locale::LanguageIdentifier;
/// use icu_pluralrules::{PluralRules, PluralRuleType, PluralCategory};
/// use icu_pluralrules::data::provider::DummyDataProvider;
///
/// let lang: LanguageIdentifier = "en".parse()
///     .expect("Failed to parse a language identifier.");
///
/// let dp = DummyDataProvider::default();
///
/// let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp)
///     .expect("Failed to construct a PluralRules struct.");
///
/// assert_eq!(pr.select(1_usize), PluralCategory::One);
/// assert_eq!(pr.select(5_usize), PluralCategory::Other);
/// ```
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PluralCategory {
    /// This category is used for message forms that are specific to cases
    /// where the number is `0`.
    ///
    /// For example, English does not distinguish between `0 items` and `5 items`,
    /// so both will fall into the `Other` category.
    ///
    /// On the other hand Welsh does distinguish - `0 cwn`, `1 ci` and `6 chi`, so
    /// the `Zero` category is being used separately from `Other`.
    Zero,
    /// This category is used for message forms that are specific to cases
    /// where the number is `1`.
    ///
    /// In English `1 table`, `1 window` etc.
    ///
    /// *Note*: It is important to recognize, that in some languages this category
    /// is used for numbers different from `1`.
    /// For example, in Russian, `1`, `21`, `101` and `1001` all use this category.
    One,
    /// This category is used for message forms that are specific to cases
    /// where the number is `2`.
    ///
    /// For example, in Breton `2 gi`, `22 gi`, `102 gi` all use this category.
    Two,
    /// This category is used for message forms that are specific to cases
    /// where the number is more than `1 but less than `many`.
    ///
    /// If there is a specific `Two` category in a given language, this category will
    /// follow that.
    ///
    /// For example, in Serbian it covers `2-4`, `22-24`, `62`, `102` etc.
    Few,
    /// This category is used for message forms that are specific to cases
    /// where the number is higher than the previous category.
    ///
    /// For example, in Arabic it covers `11-26`, `111`, `1011`, `12.0` etc.
    Many,
    /// This category is a catch-all used to handle all numbers not covered by any previous
    /// category.
    ///
    /// Each language supports it and it is also used as a fail safe result for in case
    /// no better match can be identified.
    Other,
}

impl PluralCategory {
    /// Returns an ordered iterator over variants of `Plural Categories`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_pluralrules::PluralCategory;
    ///
    /// let mut categories = PluralCategory::all();
    ///
    /// assert_eq!(categories.next(), Some(&PluralCategory::Zero));
    /// assert_eq!(categories.next(), Some(&PluralCategory::One));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Two));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Few));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Many));
    /// assert_eq!(categories.next(), Some(&PluralCategory::Other));
    /// assert_eq!(categories.next(), None);
    /// ```
    pub fn all() -> impl ExactSizeIterator<Item = &'static Self> {
        [
            Self::Zero,
            Self::One,
            Self::Two,
            Self::Few,
            Self::Many,
            Self::Other,
        ]
        .iter()
    }
}

/// A list of possible error outcomes for the [`PluralRules`] struct.
///
/// [`PluralRules`]: ./struct.PluralRules.html
#[derive(Debug, PartialEq, Eq)]
pub enum PluralRulesError {
    /// The data for a requested language and type could not be found.
    MissingData,
    /// An error originating inside of the DataProvider
    DataProvider(data::provider::DataProviderError),
}

/// `PluralRules` is a struct which provides an ability to retrieve an appropriate
/// [`Plural Category`] for a given number.
///
/// # Examples
///
/// ```
/// use icu_locale::LanguageIdentifier;
/// use icu_pluralrules::{PluralRules, PluralRuleType, PluralCategory};
/// use icu_pluralrules::data::provider::DummyDataProvider;
///
/// let lang: LanguageIdentifier = "en".parse()
///     .expect("Failed to parse a language identifier.");
///
/// let dp = DummyDataProvider::default();
///
/// let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp)
///     .expect("Failed to construct a PluralRules struct.");
///
/// assert_eq!(pr.select(1_usize), PluralCategory::One);
/// assert_eq!(pr.select(5_usize), PluralCategory::Other);
/// ```
///
/// [`ICU4X`]: https://github.com/unicode-org/icu4x
/// [`PluralRules`]: ./struct.PluralRules.html
/// [`Plural Type`]: ./enum.PluralRuleType.html
/// [`Plural Category`]: ./enum.PluralCategory.html
pub struct PluralRules {
    locale: LanguageIdentifier,
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
    /// use icu_locale::LanguageIdentifier;
    /// use icu_pluralrules::{PluralRules, PluralRuleType};
    /// use icu_pluralrules::data::provider::DummyDataProvider;
    ///
    /// let lang: LanguageIdentifier = "en".parse()
    ///     .expect("Failed to parse a language identifier.");
    ///
    /// let dp = DummyDataProvider::default();
    ///
    /// let _ = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp);
    /// ```
    ///
    /// [`type`]: ./enum.PluralRuleType.html
    /// [`data provider`]: ./data/trait.DataProviderType.html
    pub fn try_new<D: data::provider::DataProviderType>(
        locale: LanguageIdentifier,
        type_: PluralRuleType,
        data_provider: &D,
    ) -> Result<Self, PluralRulesError> {
        let selector = data_provider
            .get_selector(&locale, type_)?
            .ok_or(PluralRulesError::MissingData)?;
        Ok(Self { locale, selector })
    }

    /// Returns the [`Plural Category`] appropriate for the given number.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::LanguageIdentifier;
    /// use icu_pluralrules::{PluralRules, PluralRuleType, PluralCategory};
    /// use icu_pluralrules::data::provider::DummyDataProvider;
    ///
    /// let lang: LanguageIdentifier = "en".parse()
    ///     .expect("Failed to parse a language identifier.");
    ///
    /// let dp = DummyDataProvider::default();
    ///
    /// let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp)
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
    /// For signed numbers and strings, [`Plural Operands`] implement `TryFrom` and `FromStr`,
    /// which should be used before passing the result to `select`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use icu_locale::LanguageIdentifier;
    /// # use icu_pluralrules::{PluralRules, PluralRuleType};
    /// use icu_pluralrules::{PluralCategory, PluralOperands};
    /// # use icu_pluralrules::data::provider::DummyDataProvider;
    /// #
    /// # let lang: LanguageIdentifier = "en".parse()
    /// #     .expect("Failed to parse a language identifier.");
    /// #
    /// # let dp = DummyDataProvider::default();
    /// #
    /// # let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp)
    /// #     .expect("Failed to construct a PluralRules struct.");
    ///
    /// let operands = PluralOperands::try_from(-5.2)
    ///     .expect("Failed to parse to operands.");
    /// let operands2: PluralOperands = "5.10".parse()
    ///     .expect("Failed to parse to operands.");
    ///
    /// assert_eq!(pr.select(operands), PluralCategory::Other);
    /// assert_eq!(pr.select(operands2), PluralCategory::Other);
    /// ```
    ///
    /// [`Plural Category`]: ./enum.PluralCategory.html
    /// [`Plural Operands`]: ./operands/struct.PluralOperands.html
    pub fn select<I: Into<PluralOperands>>(&self, input: I) -> PluralCategory {
        self.selector.select(&input.into())
    }

    /// Returns the LanguageIdentifier associated with this struct.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale::LanguageIdentifier;
    /// use icu_pluralrules::{PluralRules, PluralRuleType, PluralCategory};
    /// use icu_pluralrules::data::provider::DummyDataProvider;
    ///
    /// let lang: LanguageIdentifier = "en".parse()
    ///     .expect("Failed to parse a language identifier.");
    ///
    /// let dp = DummyDataProvider::default();
    ///
    /// let pr = PluralRules::try_new(lang, PluralRuleType::Cardinal, &dp)
    ///     .expect("Failed to construct a PluralRules struct.");
    ///
    /// assert_eq!(&pr.locale().to_string(), "en");
    /// ```
    pub fn locale(&self) -> &LanguageIdentifier {
        &self.locale
    }
}
