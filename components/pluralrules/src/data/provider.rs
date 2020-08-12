//! Traits for Data Provider and a dummy Data Provider
//!
//! Every data provider must implement the [`DataProviderType`].
//!
//! For the example below, we're going to implement a simple
//! data provider for an imaginary language which has three
//! [`Plural Categories`]: `One`, `Few`, and `Other`.
//!
//! The provider takes a syntax described in [`rules`] module
//! and parses it into a vector of category-condition tuples.
//!
//! Such provider can be then passed to the [`PluralRules`] struct.
//!
//! # Examples
//!
//! ```
//! use icu_pluralrules::{PluralCategory, PluralRuleType};
//! use icu_pluralrules::data::RulesSelector;
//! use icu_pluralrules::data::provider::{DataProviderType, DataProviderError};
//! use icu_pluralrules::rules::ast;
//! use icu_locale::LanguageIdentifier;
//!
//! use icu_pluralrules::rules::parse;
//!
//! struct MyDataProvider {}
//!
//! impl DataProviderType for MyDataProvider {
//!     fn get_selector(
//!         &self,
//!         locale: &LanguageIdentifier,
//!         type_: PluralRuleType,
//!     ) -> Result<Option<RulesSelector>, DataProviderError> {
//!
//!         let sources = vec![
//!             (PluralCategory::One, "i = 1 and v = 0"),
//!             (PluralCategory::Few, "i = 2..4 and v != 0"),
//!             (PluralCategory::Other, "")
//!         ];
//!
//!         let conditions: Vec<(PluralCategory, ast::Condition)> =
//!             sources.iter().map(|(category, rule_str)| {
//!                 let condition = parse(rule_str.as_bytes())
//!                     .expect("Failed to parse the plural rule.");
//!                 (*category, condition)
//!             }).collect();
//!
//!
//!         Ok(
//!             Some(
//!                 RulesSelector::Conditions(
//!                     conditions.into_boxed_slice()
//!                 )
//!             )
//!         )
//!     }
//! }
//!
//! ```
//!
//! [`DataProviderType`]: ./trait.DataProviderType.html
//! [`Plural Categories`]: ../../enum.PluralCategory.html
//! [`rules`]: ../../rules/index.html
//! [`PluralRules`]: ../../struct.PluralRules.html
use crate::data::RulesSelector;
use crate::rules::parser::ParserError;
use crate::{PluralCategory, PluralOperands, PluralRuleType};
use icu_locale::LanguageIdentifier;

/// A list of possible error outcomes for the [`DataProviderType`] trait.
///
/// [`DataProviderType`]: ./trait.DataProviderType.html
#[derive(Debug, PartialEq, Eq)]
pub enum DataProviderError {
    /// An error coming from a rule [`Parser`].
    ///
    /// [`Parser`]: ../../rules/struct.Parser.html
    Parser(ParserError),
    /// An error coming from a failed I/O.
    IO,
    /// An error coming from a failed data deserialization.
    Deserialization,
}

impl From<DataProviderError> for crate::PluralRulesError {
    fn from(input: DataProviderError) -> Self {
        Self::DataProvider(input)
    }
}

impl From<ParserError> for DataProviderError {
    fn from(input: ParserError) -> Self {
        Self::Parser(input)
    }
}

/// Trait required to be implemented by any Data Provider.
///
/// See [`provider`] module docs for example on how to implement the trait.
///
/// # Examples
///
/// ```
/// use icu_locale::LanguageIdentifier;
/// use icu_pluralrules::{PluralCategory, PluralRuleType, PluralOperands};
///
/// use icu_pluralrules::data::provider::{DummyDataProvider, DataProviderType};
///
/// let dp = DummyDataProvider::default();
///
/// // Dummy provider always returns data for "en"
/// let lang: LanguageIdentifier = "en".parse()
///     .expect("Failed to parse a language identifier.");
///
/// // Dummy provider always returns data for cardinal
/// let cardinal_type = PluralRuleType::Cardinal;
///
/// let selector = dp.get_selector(&lang, cardinal_type)
///     .expect("Failed to retrieve a rule.")
///     .expect("Requested rule set does not exist.");
///
/// let operands: PluralOperands = 1_usize.into();
///
/// assert_eq!(selector.select(&operands), PluralCategory::One);
/// ```
///
/// [`provider`]: ../provider/index.html
pub trait DataProviderType {
    /// Returns a `RulesSelector` variant for a requested locale
    /// and type.
    fn get_selector(
        &self,
        locale: &LanguageIdentifier,
        type_: PluralRuleType,
    ) -> Result<Option<RulesSelector>, DataProviderError>;
}

/// A dummy implementation of the [`DataProviderType`] for
/// use in tests and examples.
///
/// The provider implements hardcoded English rule for
/// [`Cardinal`] type.
///
/// # Examples
///
/// ```
/// use icu_locale::LanguageIdentifier;
/// use icu_pluralrules::{PluralCategory, PluralRuleType, PluralOperands};
///
/// use icu_pluralrules::data::provider::{DummyDataProvider, DataProviderType};
///
/// let dp = DummyDataProvider::default();
///
/// // Dummy provider always returns data for "en"
/// let lang: LanguageIdentifier = "en".parse()
///     .expect("Failed to parse a language identifier.");
///
/// // Dummy provider always returns data for cardinal
/// let cardinal_type = PluralRuleType::Cardinal;
///
/// let selector = dp.get_selector(&lang, cardinal_type)
///     .expect("Failed to retrieve a rule.")
///     .expect("Requested rule does not exist.");
///
/// let operands: PluralOperands = 1_usize.into();
///
/// assert_eq!(selector.select(&operands), PluralCategory::One);
/// ```
///
/// [`DataProviderType`]: ./trait.DataProviderType.html
/// [`Cardinal`]: ../../enum.PluralRuleType.html#variant.Cardinal
#[derive(Default)]
pub struct DummyDataProvider {}

impl DataProviderType for DummyDataProvider {
    fn get_selector(
        &self,
        locale: &LanguageIdentifier,
        _type_: PluralRuleType,
    ) -> Result<Option<RulesSelector>, DataProviderError> {
        if locale.language == "xx" {
            return Ok(None);
        }
        Ok(Some(RulesSelector::Function(|pr: &PluralOperands| {
            if pr.i == 1 {
                PluralCategory::One
            } else {
                PluralCategory::Other
            }
        })))
    }
}
