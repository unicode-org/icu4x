// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! Module with data used by [`PluralRules`].
//!
//! # Examples
//!
//! ```
//! use icu_locale::LanguageIdentifier;
//! use icu_plurals::{PluralCategory, PluralRuleType, PluralOperands};
//! use icu_plurals::data::{PluralRuleList, RulesSelector};
//! use icu_data_provider::prelude::*;
//! use icu_data_provider::{InvariantDataProvider, structs};
//! use std::borrow::Cow;
//! use std::convert::TryInto;
//!
//! // Dummy provider always returns data for "und"
//! let langid = LanguageIdentifier::default();
//!
//! // Dummy provider always returns data for cardinal
//! let type_ = PluralRuleType::Cardinal;
//!
//! let data_provider = InvariantDataProvider;
//! let data_key = match type_ {
//!     PluralRuleType::Cardinal => icu_data_key!(plurals: cardinal@1),
//!     PluralRuleType::Ordinal => icu_data_key!(plurals: ordinal@1),
//! };
//! let response = data_provider
//!     .load(&DataRequest {
//!         data_key,
//!         data_entry: DataEntry {
//!             variant: None,
//!             langid: langid.clone(),
//!         },
//!     })
//!     .expect("Failed to get a response");
//!
//! let plurals_data: Cow<structs::plurals::PluralRuleStringsV1> = response.take_payload()
//!     .expect("Failed to extract payload.");
//! let list: PluralRuleList = (&*plurals_data).try_into()
//!     .expect("Failed to parse plural rules.");
//! let selector: RulesSelector = list.into();
//!
//! let operands: PluralOperands = 3_usize.into();
//!
//! assert_eq!(selector.select(&operands), PluralCategory::Other);
//! ```

use crate::operands::PluralOperands;
use crate::rules;
use crate::rules::ast;
use crate::{PluralCategory, PluralRulesError};
use icu_data_provider::structs::plurals::PluralRuleStringsV1;
use std::borrow::Cow;
use std::convert::TryInto;

/// A raw function pointer to a [`PluralRulesFn`](./type.PluralRulesFn.html)
pub type PluralRulesFn = fn(&PluralOperands) -> PluralCategory;

/// A structure holding a list of [`ast::Condition`] for a given locale and type.
///
/// [`PluralCategory`]: ../enum.PluralCategory.html
/// [`ast::Condition`]: ../rules/ast/struct.Condition.html
#[derive(Default, Debug)]
pub struct PluralRuleList {
    zero: Option<ast::Condition>,
    one: Option<ast::Condition>,
    two: Option<ast::Condition>,
    few: Option<ast::Condition>,
    many: Option<ast::Condition>,
}

impl PluralRuleList {
    fn get(&self, category: &PluralCategory) -> Option<&ast::Condition> {
        match category {
            PluralCategory::Zero => self.zero.as_ref(),
            PluralCategory::One => self.one.as_ref(),
            PluralCategory::Two => self.two.as_ref(),
            PluralCategory::Few => self.few.as_ref(),
            PluralCategory::Many => self.many.as_ref(),
            PluralCategory::Other => None,
        }
    }
}

fn parse_rule<'s>(
    input: &Option<Cow<'s, str>>,
) -> Result<Option<ast::Condition>, PluralRulesError> {
    Ok(if let Some(input) = input {
        Some(rules::parse_condition((&input).as_bytes())?)
    } else {
        None
    })
}

impl TryInto<PluralRuleList> for &PluralRuleStringsV1 {
    type Error = PluralRulesError;
    fn try_into(self) -> Result<PluralRuleList, Self::Error> {
        Ok(PluralRuleList {
            zero: parse_rule(&self.zero)?,
            one: parse_rule(&self.one)?,
            two: parse_rule(&self.two)?,
            few: parse_rule(&self.few)?,
            many: parse_rule(&self.many)?,
        })
    }
}

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
            Self::Conditions(conditions) => PluralCategory::all()
                .find_map(|category| {
                    conditions
                        .get(category)
                        .filter(|cond| rules::test_condition(cond, operands))
                        .map(|_| *category)
                })
                .unwrap_or(PluralCategory::Other),
        }
    }
}

impl From<PluralRuleList> for RulesSelector {
    fn from(input: PluralRuleList) -> Self {
        Self::Conditions(input)
    }
}
