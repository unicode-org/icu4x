//! Temporary module with logic required to provide
//! data to the crate.
//!
//! # Examples
//!
//! ```
//! use icu_locale::LanguageIdentifier;
//! use icu_pluralrules::{PluralCategory, PluralRuleType, PluralOperands};
//!
//! use icu_pluralrules::data::provider::{DummyDataProvider, DataProviderType};
//!
//! let dp = DummyDataProvider::default();
//!
//! // Dummy provider always returns data for "en"
//! let lang: LanguageIdentifier = "en".parse()
//!     .expect("Failed to parse a language identifier.");
//!
//! // Dummy provider always returns data for cardinal
//! let cardinal_type = PluralRuleType::Cardinal;
//!
//! let selector = dp.get_selector(&lang, cardinal_type)
//!     .expect("Failed to retrieve a rule.")
//!     .expect("Requested rule set does not exist.");
//!
//! let operands: PluralOperands = 1_usize.into();
//!
//! assert_eq!(selector.select(&operands), PluralCategory::One);
//! ```
#[cfg(feature = "io")]
pub mod io;

pub mod cldr_resource;

pub mod provider;

use crate::operands::PluralOperands;
use crate::rules;
use crate::rules::ast;
use crate::PluralCategory;

/// A raw function pointer to a [`PluralRulesFn`](./type.PluralRulesFn.html)
pub type PluralRulesFn = fn(&PluralOperands) -> PluralCategory;

/// A list of tuples of ([`PluralCategory`]-[`ast::Condition`]) pairs.
///
/// [`PluralCategory`]: ../enum.PluralCategory.html
/// [`ast::Condition`]: ../rules/ast/struct.Condition.html
pub type PluralRuleList = Box<[(PluralCategory, ast::Condition)]>;

/// An enum storing models of
/// handling plural rules selection.
pub enum RulesSelector {
    /// A raw function pointer to a [`PluralRulesFn`](./type.PluralRulesFn.html)
    ///
    /// This variant is used by providers which store rules as native Rust functions.
    Function(PluralRulesFn),
    /// A list of tuples of ([`PluralCategory`]-[`ast::Condition`]) pairs.
    ///
    /// This variant is used by providers which parse the list of conditions out
    /// of source strings.
    ///
    /// [`PluralCategory`]: ../enum.PluralCategory.html
    /// [`ast::Condition`]: ../rules/ast/struct.Condition.html
    Conditions(PluralRuleList),
}

impl RulesSelector {
    pub fn select(&self, operands: &PluralOperands) -> PluralCategory {
        match self {
            Self::Function(ptr) => ptr(operands),
            Self::Conditions(conditions) => conditions
                .iter()
                .find_map(|(category, rule)| {
                    if rules::test_condition(rule, operands) {
                        Some(*category)
                    } else {
                        None
                    }
                })
                .unwrap_or(PluralCategory::Other),
        }
    }
}
