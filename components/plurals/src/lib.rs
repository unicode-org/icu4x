// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! `icu-pluralrules` is one of the [`ICU4X`] components.
//! `icu-plurals` is one of the [`ICU4X`] components.
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
//! use icu_locale_macros::langid;
//! use icu_plurals::{PluralRules, PluralRuleType, PluralCategory};
//! use icu_data_provider::InvariantDataProvider;
//!
//! let lid = langid!("en");
//!
//! let dp = InvariantDataProvider;
//!
//! let pr = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal)
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
mod error;
mod operands;
pub mod rules;

pub use error::PluralRulesError;
use icu_data_provider::{icu_data_key, structs, DataEntry, DataProvider, DataRequest};
use icu_locale::LanguageIdentifier;
pub use operands::PluralOperands;
use std::borrow::Cow;
use std::convert::TryInto;

/// A type of a plural rule which can be associated with the [`PluralRules`] struct.
///
/// [`PluralRules`]: ./struct.PluralRules.html
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
    /// [`One`]: ./enum.PluralCategory.html#variant.One
    /// [`Other`]: ./enum.PluralCategory.html#variant.Other
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
    /// [`One`]: ./enum.PluralCategory.html#variant.One
    /// [`Two`]: ./enum.PluralCategory.html#variant.Two
    /// [`Few`]: ./enum.PluralCategory.html#variant.Few
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
/// use icu_locale_macros::langid;
/// use icu_plurals::{PluralRules, PluralRuleType, PluralCategory};
/// use icu_data_provider::InvariantDataProvider;
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
    /// Returns an ordered iterator over variants of `Plural Categories`.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_plurals::PluralCategory;
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

/// `PluralRules` is a struct which provides an ability to retrieve an appropriate
/// [`Plural Category`] for a given number.
///
/// # Examples
///
/// ```
/// use icu_locale_macros::langid;
/// use icu_plurals::{PluralRules, PluralRuleType, PluralCategory};
/// use icu_data_provider::InvariantDataProvider;
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
/// [`ICU4X`]: https://github.com/unicode-org/icu4x
/// [`PluralRules`]: ./struct.PluralRules.html
/// [`Plural Type`]: ./enum.PluralRuleType.html
/// [`Plural Category`]: ./enum.PluralCategory.html
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
    /// use icu_locale_macros::langid;
    /// use icu_plurals::{PluralRules, PluralRuleType};
    /// use icu_data_provider::InvariantDataProvider;
    ///
    /// let lid = langid!("en");
    ///
    /// let dp = InvariantDataProvider;
    ///
    /// let _ = PluralRules::try_new(lid, &dp, PluralRuleType::Cardinal);
    /// ```
    ///
    /// [`type`]: ./enum.PluralRuleType.html
    /// [`data provider`]: ./data/trait.DataProviderType.html
    pub fn try_new<'d, D: DataProvider<'d>>(
        langid: LanguageIdentifier,
        data_provider: &D,
        type_: PluralRuleType,
    ) -> Result<Self, PluralRulesError> {
        let data_key = match type_ {
            PluralRuleType::Cardinal => icu_data_key!(plurals: cardinal@1),
            PluralRuleType::Ordinal => icu_data_key!(plurals: ordinal@1),
        };
        let response = data_provider.load(&DataRequest {
            data_key,
            data_entry: DataEntry {
                variant: None,
                langid: langid.clone(),
            },
        })?;
        let plurals_data: Cow<structs::plurals::PluralRuleStringsV1> = response.take_payload()?;

        let list: data::PluralRuleList = (&*plurals_data).try_into()?;

        Ok(Self {
            _langid: langid,
            selector: list.into(),
        })
    }

    /// Returns the [`Plural Category`] appropriate for the given number.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locale_macros::langid;
    /// use icu_plurals::{PluralRules, PluralRuleType, PluralCategory};
    /// use icu_data_provider::InvariantDataProvider;
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
    /// For signed numbers and strings, [`Plural Operands`] implement `TryFrom` and `FromStr`,
    /// which should be used before passing the result to `select`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::convert::TryFrom;
    /// # use icu_locale_macros::langid;
    /// # use icu_plurals::{PluralRules, PluralRuleType};
    /// use icu_plurals::{PluralCategory, PluralOperands};
    /// # use icu_data_provider::InvariantDataProvider;
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
    /// [`Plural Category`]: ./enum.PluralCategory.html
    /// [`Plural Operands`]: ./operands/struct.PluralOperands.html
    pub fn select<I: Into<PluralOperands>>(&self, input: I) -> PluralCategory {
        self.selector.select(&input.into())
    }
}
