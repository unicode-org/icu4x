// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::operands::PluralOperands;
use crate::rules::runtime::ast;

pub fn test_rule(rule: &ast::Rule, operands: &PluralOperands) -> bool {
    let mut left = true;

    for condition in rule.0.iter() {
        let condition = condition.as_relation();
        if condition.conjunction {
        }
    }
    true
}
