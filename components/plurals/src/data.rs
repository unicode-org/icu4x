// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::operands::PluralOperands;
use crate::provider::PluralRuleStringsV1;
use crate::rules;
use crate::rules::ast;
use crate::{PluralCategory, PluralRulesError};
use std::borrow::Cow;
use std::convert::TryInto;

/// A raw function pointer to a `PluralRulesFn`
// pub type PluralRulesFn = fn(&PluralOperands) -> PluralCategory;

/// A structure holding a list of [`ast::Condition`] for a given locale and type.
///
/// [`PluralCategory`]: super::PluralCategory
/// [`ast::Condition`]: super::rules::ast::Condition
#[derive(Default, Debug)]
pub struct PluralRuleList {
    zero: Option<ast::Condition>,
    one: Option<ast::Condition>,
    two: Option<ast::Condition>,
    few: Option<ast::Condition>,
    many: Option<ast::Condition>,
}

impl PluralRuleList {
    fn has_rules_for(&self, category: PluralCategory) -> bool {
        // There is implicitly always a rule for "Other" as the fallback.
        match category {
            PluralCategory::Zero => self.zero.is_some(),
            PluralCategory::One => self.one.is_some(),
            PluralCategory::Two => self.two.is_some(),
            PluralCategory::Few => self.few.is_some(),
            PluralCategory::Many => self.many.is_some(),
            PluralCategory::Other => true,
        }
    }

    fn get(&self, category: PluralCategory) -> Option<&ast::Condition> {
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

fn parse_rule(input: &Option<Cow<str>>) -> Result<Option<ast::Condition>, PluralRulesError> {
    Ok(if let Some(input) = input {
        Some(rules::parse_condition((input).as_bytes())?)
    } else {
        None
    })
}

impl<'s> TryInto<PluralRuleList> for &PluralRuleStringsV1<'s> {
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
    /// A raw function pointer to a `PluralRulesFn`
    ///
    /// This variant is used by providers which store rules as native Rust functions.
    // Function(PluralRulesFn),
    /// A list of tuples of ([`PluralCategory`]-[`ast::Condition`]) pairs.
    ///
    /// This variant is used by providers which parse the list of conditions out
    /// of source strings.
    ///
    /// [`PluralCategory`]: super::PluralCategory
    /// [`ast::Condition`]: super::rules::ast::Condition
    Conditions(PluralRuleList),
}

impl RulesSelector {
    pub fn select(&self, operands: &PluralOperands) -> PluralCategory {
        match self {
            // Self::Function(ptr) => ptr(operands),
            Self::Conditions(conditions) => PluralCategory::all()
                .find_map(|category| {
                    conditions
                        .get(*category)
                        .filter(|cond| rules::test_condition(cond, operands))
                        .map(|_| *category)
                })
                .unwrap_or(PluralCategory::Other),
        }
    }

    /// Returns an iterator over each [`PluralCategory`] for which this [`RulesSelector`] has rules.
    ///
    /// The category [`PluralCategory::Other`] is always included.
    pub fn categories(&self) -> impl Iterator<Item = &'static PluralCategory> + '_ {
        match self {
            Self::Conditions(conditions) => {
                PluralCategory::all().filter(move |&category| conditions.has_rules_for(*category))
            }
        }
    }
}

impl From<PluralRuleList> for RulesSelector {
    fn from(input: PluralRuleList) -> Self {
        Self::Conditions(input)
    }
}
