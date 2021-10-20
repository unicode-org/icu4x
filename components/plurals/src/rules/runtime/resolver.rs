// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::operands::PluralOperands;
use crate::rules::runtime::ast;
use zerovec::ZeroVec;

pub fn test_rule(rule: &ast::Rule, operands: &PluralOperands) -> bool {
    let mut left_and = true;

    for relation in rule.0.iter() {
        let relation = relation.as_relation();
        if relation.and_or == ast::AndOr::Or {
            if left_and {
                return true;
            } else {
                left_and = test_value(get_value(&relation, operands), &relation.range_list);
            }
        } else if left_and && !test_value(get_value(&relation, operands), &relation.range_list) {
            left_and = false;
        }
    }
    left_and
}

fn test_value(value: Option<u64>, range_list: &ZeroVec<ast::RangeOrValue>) -> bool {
    range_list.iter().any(|range| {
        if let Some(value) = value {
            match range {
                ast::RangeOrValue::Range(min, max) => {
                    value >= u64::from(min) && value <= u64::from(max)
                }
                ast::RangeOrValue::Value(v) => u64::from(v) == value,
            }
        } else {
            false
        }
    })
}

fn get_value(relation: &ast::Relation, operands: &PluralOperands) -> Option<u64> {
    let value = match relation.operand {
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
        ast::Operand::C | ast::Operand::E => operands.c as u64,
    };
    if relation.modulo > 0 {
        value.checked_rem_euclid(relation.modulo.into())
    } else {
        Some(value)
    }
}
