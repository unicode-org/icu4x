use super::ast;
use crate::operands::PluralOperands;

/// Function used to test [`Condition`] against [`PluralOperands`] to identify
/// the appropriate [`PluralCategory`].
///
/// # Examples
///
/// ```
/// use icu_pluralrules::PluralOperands;
/// use icu_pluralrules::rules::{parse, test_condition};
///
/// let operands = PluralOperands::from(5_usize);
/// let condition = parse(b"i = 4..6")
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
    let exp = calculate_expression(&relation.expression, operands);
    test_range(&relation.range_list, exp, &relation.operator)
}

fn calculate_expression(expression: &ast::Expression, operands: &PluralOperands) -> u64 {
    match expression.operand {
        ast::Operand::N => operands.n as u64,
        ast::Operand::I => operands.i,
        ast::Operand::F => operands.f,
        ast::Operand::V => operands.v as u64,
        ast::Operand::W => operands.w as u64,
        ast::Operand::T => operands.t,
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
