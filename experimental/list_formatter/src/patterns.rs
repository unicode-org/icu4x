use std::collections::HashMap;

use regex::Regex;

use crate::list_formatter::{ConditionalPattern, Pattern};

// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

lazy_static! {
    // Starts with "hi" or "i" but not with "hie" nor "hia"
    static ref E_BEFORE_I: Regex = Regex::new("^(?i)i.*|hi|hi[^ae].*$").expect("valid");
    // Starts with "o", "ho", "8", or "11" (the number, not the digits)
    // TODO: This doesn't match "11 cosas" yet
    static ref U_BEFORE_O: Regex = Regex::new("^(?i)(o|ho|8).*|11[[\\s.,]?\\d\\d\\d]+$").expect("valid");
}

pub(crate) fn create_formatters(
    cond: fn(&'static Regex, &'static dyn Pattern, &'static dyn Pattern) -> ConditionalPattern<'static>
) -> HashMap<&'static str, [[[&'static dyn Pattern;4];3];3]> { 
    let mut m: HashMap<&'static str, [[[&'static dyn Pattern;4];3];3]> = HashMap::new();       

    // m.insert(locale, [
    //     [standard, standard-narrow, standard-short],
    //     [or, or-narrow, or-short],
    //     [unit, unit-narrow, unit-short]);

    // ****************************************************************************************************
    // Generated code
    // ****************************************************************************************************
    m.insert("en", [
        [[&", ", " and ", ", ", ", and "], [", ", " and ", ", ", ", "], [", ", " & ", ", ", ", & "]],
        [[", ", " or ", ", ", ", or "], [", ", " or ", ", ", ", or "], [", ", " or ", ", ", ", or "]],
        [[", ", ", ", ", ", ", "], [" ", " ", " ", " "], [", ", ", ", ", ", ", "]],
    ]);
    m.insert("es", [
        [[", ", " y ", ", ", " y "], [", ", " y ", ", ", " y "], [", ", " y ", ", ", " y "]],
        [[", ", " o ", ", ", " o "], [", ", " o ", ", ", " o "], [", ", " o ", ", ", " o "]],
        [[", ", " y ", ", ", " y "], [" ", " ", " ", " "], [", ", " y ", ", ", ", "]],
    ]);
    // ****************************************************************************************************
    // End generated code
    // ****************************************************************************************************

    // Replace Spanish by special cases
    for k in m.keys() {
        if *k == "es" || k.starts_with("es-") {
            let patterns = m.get(k).expect("obv");
            for i in 0..patterns.len() {
                for j in 0..patterns[i].len() {
                    for k in 0..patterns[i][j].len() {
                            if patterns[i][j][k] == " y " {
                                patterns[i][j][k] = &cond(&E_BEFORE_I, &" y ", &" e ")
                            } else if patterns[i][j][k] == &" o " {
                                patterns[i][j][k] = &cond(&U_BEFORE_O, &" o ", &" u ")
                            }
                    }
                }
            }
        }
    }    
    m
}

