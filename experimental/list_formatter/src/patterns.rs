// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use std::collections::HashMap;

use crate::list_formatter::{Type, Width};

lazy_static! {
    // This static map should be a compact representation of the CLDR data. Each locale entry is
    // a 3 x 3 array (type x width) of [&str; 4] objects, which contain references to static
    // strings. As lots of patterns are repeated many times, this should be more efficient than
    // deserializing JSON into memory.
    static ref RAW_PATTERNS: HashMap<&'static str, [[[&'static str;4];3];3]> = { 
        let mut m = HashMap::new();       
    
        // m.insert(locale, [
        //     [standard, standard-narrow, standard-short],
        //     [or, or-narrow, or-short],
        //     [unit, unit-narrow, unit-short]);
    
        // ****************************************************************************************************
        // Generated code
        // ****************************************************************************************************
        m.insert("en", [
            [["{0}, {1}", "{0} and {1}", "{0}, {1}", "{0}, and {1}"], ["{0}, {1}", "{0} and {1}", "{0}, {1}", "{0}, {1}"], ["{0}, {1}", "{0} & {1}", "{0}, {1}", "{0}, & {1}"]],
            [["{0}, {1}", "{0} or {1}", "{0}, {1}", "{0}, or {1}"], ["{0}, {1}", "{0} or {1}", "{0}, {1}", ", or "], ["{0}, {1}", "{0} or {1}", "{0}, {1}", "{1}, or {1}"]],
            [["{0}, {1}", "{0}, {1}", "{0}, {1}", "{0}, {1}"], ["{0}, {1}", "{0}, {1}", "{0}, {1}", "{0}, {1}"], ["{0}, {1}", "{0}, {1}", "{0}, {1}", "{0}, {1}"]],
        ]);
        m.insert("es", [
            [["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"], ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"], ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"]],
            [["{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*[^\\d]{then}{0} u {1}{else}{0} o {1}", "{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*[^\\d]{then}{0} u {1}{else}{0} o {1}"], ["{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*[^\\d]{then}{0} u {1}{else}{0} o {1}", "{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*[^\\d]{then}{0} u {1}{else}{0} o {1}"], ["{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*[^\\d]{then}{0} u {1}{else}{0} o {1}", "{0}, {1}", "{cond}^(?i)(o|ho|8).*$|11(\\d\\d\\d)*[^\\d]{then}{0} u {1}{else}{0} o {1}"]],
            [["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}"], ["{0} {1}", "{0} {1}", "{0} {1}", "{0} {1}"], ["{0}, {1}", "{cond}^(?i)i.*|hi|hi[^ae].*${then}{0} e {1}{else}{0} y {1}", "{0}, {1}", "{0}, {1}"]],
        ]);
        // ****************************************************************************************************
        // End generated code
        // ****************************************************************************************************  
        m
    };
}

pub(crate)fn get_patterns(locale: &str, type_: Type, width: Width) -> Option<[&'static str;4]> {
    RAW_PATTERNS.get(locale).map(|patterns| patterns[match type_ {
            Type::And => 0,
            Type::Or => 1,
            Type::Unit => 2,
        }][match width {
            Width::Wide => 0,
            Width::Narrow => 1,
            Width::Short => 2,
        }])
}