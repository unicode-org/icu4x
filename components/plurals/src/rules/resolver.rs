// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use super::ast;
use crate::operands::PluralOperands;

/// Function used to test [`Condition`] against [`PluralOperands`] to identify
/// the appropriate [`PluralCategory`].
///
/// # Examples
///
/// ```
/// use icu_plurals::PluralOperands;
/// use icu_plurals::rules::{parse_condition, test_condition};
///
/// let operands = PluralOperands::from(5_usize);
/// let condition = parse_condition(b"i = 4..6")
///     .expect("Failde to parse a rule.");
///
/// assert_eq!(test_condition(&condition, &operands), true);
/// ```
///
/// [`PluralCategory`]: ../enum.PluralCategory.html
/// [`PluralOperands`]: ../struct.PluralOperands.html
/// [`Condition`]: ../rules/ast/struct.Condition.html
pub fn test_condition(condition: &ast::Condition, operands: &PluralOperands) -> bool {
    condition.0.is_empty() || condition.0.iter().any(|c| test_and_condition(c, operands))
}

fn test_and_condition(condition: &ast::AndCondition, operands: &PluralOperands) -> bool {
    condition.0.iter().all(|r| test_relation(r, operands))
}

fn test_relation(relation: &ast::Relation, operands: &PluralOperands) -> bool {
    calculate_expression(&relation.expression, operands)
        .map(|exp| test_range(&relation.range_list, exp, &relation.operator))
        .unwrap_or(false)
}

// UTS 35 Part 2 Section 5.1 specifies that CLDR rules contain only integer values.
// See: https://unicode.org/reports/tr35/tr35-numbers.html#Plural_rules_syntax
//
// If a rule is asking for `n` it will compare it to a list of integers,
// and expect it to not match if there is any fractional part.
//
// That allows us to play a trick - we already have an integer portion of the number as operand `i`.
//
// In result, if we are asked to calculate an operand `n` and it contains a fractional part,
// we know that it will not match the value, which must be an integer without a fractional part.
//
// If that happens, we'll return `None`, and the matching will return `false`.
fn calculate_expression(expression: &ast::Expression, operands: &PluralOperands) -> Option<u64> {
    let value = match expression.operand {
        ast::Operand::N => {
            if operands.w == 0 {
                operands.i
            } else {
                return None;
            }
        }
        ast::Operand::I => operands.i,
        ast::Operand::F => operands.f,
        ast::Operand::V => operands.v as u64,
        ast::Operand::W => operands.w as u64,
        ast::Operand::T => operands.t,
    };
    if let Some(modulus) = &expression.modulus {
        value.checked_rem_euclid(modulus.0)
    } else {
        Some(value)
    }
}

fn test_range(range: &ast::RangeList, value: u64, operator: &ast::Operator) -> bool {
    range
        .0
        .iter()
        .any(|item| test_range_item(item, value, operator))
}

fn test_range_item(item: &ast::RangeListItem, value: u64, operator: &ast::Operator) -> bool {
    let value = match item {
        ast::RangeListItem::Value(n) => n.0 == value,
        ast::RangeListItem::Range(range) => range.contains(&ast::Value(value)),
    };
    match operator {
        ast::Operator::Eq => value,
        ast::Operator::NotEq => !value,
    }
}
