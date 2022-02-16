// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::properties::{
    maps, sets, EastAsianWidth, GeneralCategory, GraphemeClusterBreak, LineBreak, SentenceBreak,
    WordBreak,
};
use icu_codepointtrie::CodePointTrie;
use icu_provider_fs::FsDataProvider;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::thread;

// state machine name define by builtin name
// [[tables]]
// name = "Double_Quote"
//
// state machine define for combined state
// [[tables]]
// name = "Double_Quote_ALetter"
// left = "Double_Quote"
// right = "ALetter"
//
// state machine define using code point
// [[tables]]
// name = "ABC"
// codepoint = [32, 33, ...]
#[derive(Deserialize, Debug)]
struct SegmenterProperty {
    name: String,
    // If codepoint is defined, this is custom define, not builtin define.
    codepoint: Option<Vec<u32>>,
    // If left and right are defined, this define is combined state.
    left: Option<String>,
    right: Option<String>,
    // This combine state is an intermediate match rule.
    interm_break_state: Option<bool>,
}

// state machine break result define
// The follow is "Double_Quote x Double_Quote".
// [[tables]]
// left = [ "Double_Qoute" ]
// right = [ "Double_Qoute" ]
// break_state = true # true if break opportunity.
#[derive(Deserialize, Debug)]
struct SegmenterState {
    left: Vec<String>,
    right: Vec<String>,
    break_state: Option<bool>,
}

// rule based segmenter define
//
// segmenter_type: builtin type. word, sentence or grapheme.
// tables: state machine name defines.
// rules: state machine rules.
//
// segmenter_type = "word"
// [[tables]]
// ...
// [[rules]]
// ...
#[derive(Deserialize, Debug)]
struct SegmenterRuleTable {
    segmenter_type: String,
    tables: Vec<SegmenterProperty>,
    rules: Vec<SegmenterState>,
}

#[allow(dead_code)]
const BREAK_RULE: i8 = -128;
const UNKNOWN_RULE: i8 = -127;
const NOT_MATCH_RULE: i8 = -2;
const KEEP_RULE: i8 = -1;
// This is a mask bit chosen sufficiently large than all other concrete states.
// If a break state contains this bit, we have to look ahead one more character.
const INTERMEDIATE_MATCH_RULE: i8 = 64;

// UAX29 defines break property until U+0xE01EF
const CODEPOINT_TABLE_LEN: usize = 0xe0400;

fn set_break_state(
    break_state_table: &mut [i8],
    property_length: usize,
    left_index: usize,
    right_index: usize,
    break_state: i8,
) {
    let index = left_index * property_length + right_index;
    if break_state_table[index] == UNKNOWN_RULE || break_state_table[index] == NOT_MATCH_RULE {
        break_state_table[index] = break_state;
    }
}

fn get_index_from_name(properties_names: &[String], s: &str) -> Option<usize> {
    properties_names.iter().position(|n| n.eq(s))
}

fn get_word_segmenter_value_from_name(name: &str) -> WordBreak {
    match name {
        "ALetter" => WordBreak::ALetter,
        "CR" => WordBreak::CR,
        "Double_Quote" => WordBreak::DoubleQuote,
        "Extend" => WordBreak::Extend,
        "ExtendNumLet" => WordBreak::ExtendNumLet,
        "Format" => WordBreak::Format,
        "Katakana" => WordBreak::Katakana,
        "Hebrew_Letter" => WordBreak::HebrewLetter,
        "LF" => WordBreak::LF,
        "MidLetter" => WordBreak::MidLetter,
        "MidNum" => WordBreak::MidNum,
        "MidNumLet" => WordBreak::MidNumLet,
        "Newline" => WordBreak::Newline,
        "Numeric" => WordBreak::Numeric,
        "Regional_Indicator" => WordBreak::RegionalIndicator,
        "Single_Quote" => WordBreak::SingleQuote,
        "WSegSpace" => WordBreak::WSegSpace,
        "ZWJ" => WordBreak::ZWJ,
        _ => {
            panic!("Invalid property name")
        }
    }
}

fn get_grapheme_segmenter_value_from_name(name: &str) -> GraphemeClusterBreak {
    match name {
        "Control" => GraphemeClusterBreak::Control,
        "CR" => GraphemeClusterBreak::CR,
        "Extend" => GraphemeClusterBreak::Extend,
        "L" => GraphemeClusterBreak::L,
        "LF" => GraphemeClusterBreak::LF,
        "LV" => GraphemeClusterBreak::LV,
        "LVT" => GraphemeClusterBreak::LVT,
        "Prepend" => GraphemeClusterBreak::Prepend,
        "Regional_Indicator" => GraphemeClusterBreak::RegionalIndicator,
        "SpacingMark" => GraphemeClusterBreak::SpacingMark,
        "T" => GraphemeClusterBreak::T,
        "V" => GraphemeClusterBreak::V,
        "ZWJ" => GraphemeClusterBreak::ZWJ,
        _ => {
            panic!("Invalid property name")
        }
    }
}

fn get_sentence_segmenter_value_from_name(name: &str) -> SentenceBreak {
    match name {
        "ATerm" => SentenceBreak::ATerm,
        "Close" => SentenceBreak::Close,
        "CR" => SentenceBreak::CR,
        "Extend" => SentenceBreak::Extend,
        "Format" => SentenceBreak::Format,
        "LF" => SentenceBreak::LF,
        "Lower" => SentenceBreak::Lower,
        "Numeric" => SentenceBreak::Numeric,
        "OLetter" => SentenceBreak::OLetter,
        "SContinue" => SentenceBreak::SContinue,
        "Sep" => SentenceBreak::Sep,
        "Sp" => SentenceBreak::Sp,
        "STerm" => SentenceBreak::STerm,
        "Upper" => SentenceBreak::Upper,
        _ => {
            panic!("Invalid property name")
        }
    }
}

fn get_line_segmenter_value_from_name(name: &str) -> LineBreak {
    match name {
        "AI" => LineBreak::Ambiguous,
        "AL" => LineBreak::Alphabetic,
        "B2" => LineBreak::BreakBoth,
        "BA" => LineBreak::BreakAfter,
        "BB" => LineBreak::BreakBefore,
        "BK" => LineBreak::MandatoryBreak,
        "CB" => LineBreak::ContingentBreak,
        "CJ" => LineBreak::ConditionalJapaneseStarter,
        "CL" => LineBreak::ClosePunctuation,
        "CM" => LineBreak::CombiningMark,
        "CP" => LineBreak::CloseParenthesis,
        "CR" => LineBreak::CarriageReturn,
        "EB" => LineBreak::EBase,
        "EM" => LineBreak::EModifier,
        "EX" => LineBreak::Exclamation,
        "GL" => LineBreak::Glue,
        "H2" => LineBreak::H2,
        "H3" => LineBreak::H3,
        "HL" => LineBreak::HebrewLetter,
        "HY" => LineBreak::Hyphen,
        "ID" => LineBreak::Ideographic,
        "IN" => LineBreak::Inseparable,
        "IS" => LineBreak::InfixNumeric,
        "JL" => LineBreak::JL,
        "JT" => LineBreak::JT,
        "JV" => LineBreak::JV,
        "LF" => LineBreak::LineFeed,
        "NL" => LineBreak::NextLine,
        "NS" => LineBreak::Nonstarter,
        "NU" => LineBreak::Numeric,
        "OP" => LineBreak::OpenPunctuation,
        "PO" => LineBreak::PostfixNumeric,
        "PR" => LineBreak::PrefixNumeric,
        "QU" => LineBreak::Quotation,
        "RI" => LineBreak::RegionalIndicator,
        "SA" => LineBreak::ComplexContext,
        "SG" => LineBreak::Surrogate,
        "SP" => LineBreak::Space,
        "SY" => LineBreak::BreakSymbols,
        "WJ" => LineBreak::WordJoiner,
        "XX" => LineBreak::Unknown,
        "ZW" => LineBreak::ZWSpace,
        "ZWJ" => LineBreak::ZWJ,
        _ => {
            panic!("Invalid property name: {}", name)
        }
    }
}

fn is_cjk_fullwidth(eaw: &CodePointTrie<icu::properties::EastAsianWidth>, codepoint: u32) -> bool {
    match eaw.get(codepoint) {
        EastAsianWidth::Ambiguous | EastAsianWidth::Fullwidth | EastAsianWidth::Wide => true,
        _ => false,
    }
}

fn output_propery_plane_with_same_value(out: &mut File, name: &str, value: u8) {
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(
        out,
        "pub const BREAK_PROPERTIES_FILL_BY_{}: [u8; 1024] = [",
        name
    )
    .ok();
    for i in 0..1024 {
        write!(out, " {},", value).ok();
        if ((i + 1) % 16) == 0 {
            writeln!(out).ok();
        }
    }
    writeln!(out, "];").ok();
}

fn generate_rule_segmenter_table(file_name: &str, toml_data: &[u8], provider: &FsDataProvider) {
    let mut properties_map: [u8; CODEPOINT_TABLE_LEN] = [0; CODEPOINT_TABLE_LEN];
    let mut properties_names = Vec::<String>::new();
    let mut simple_properties_count = 0;

    let payload = maps::get_word_break(provider).expect("The data should be valid!");
    let wb = &payload.get().code_point_trie;

    let payload = maps::get_grapheme_cluster_break(provider).expect("The data should be valid!");
    let gb = &payload.get().code_point_trie;

    let payload = maps::get_sentence_break(provider).expect("The data should be valid!");
    let sb = &payload.get().code_point_trie;

    let payload = sets::get_extended_pictographic(provider).expect("The data should be valid!");
    let extended_pictographic = &payload.get().inv_list;

    let payload = maps::get_line_break(provider).expect("The data should be valid!");
    let lb = &payload.get().code_point_trie;

    let payload = maps::get_east_asian_width(provider).expect("The data should be valid!");
    let eaw = &payload.get().code_point_trie;

    let payload = maps::get_general_category(provider).expect("The data should be valid!");
    let gc = &payload.get().code_point_trie;

    let segmenter: SegmenterRuleTable = toml::de::from_slice(toml_data).expect("TOML syntax error");

    properties_names.push("Unknown".to_string());
    simple_properties_count += 1;

    for p in &segmenter.tables {
        let property_index;
        if !properties_names.contains(&p.name) {
            properties_names.push(p.name.clone());
            property_index = (properties_names.len() - 1) as u8;
        } else {
            continue;
        }

        if p.left.is_none() && p.right.is_none() && p.codepoint.is_none() {
            // If any values aren't set, this is builtin type.
            simple_properties_count += 1;

            match &*segmenter.segmenter_type {
                "word" => {
                    // Extended_Pictographic isn't a part of word break property
                    // Extended pictographic property is within 0..U+0x20000
                    if p.name == "Extended_Pictographic" {
                        for i in 0..0x20000 {
                            if let Some(c) = char::from_u32(i) {
                                if extended_pictographic.contains(c) {
                                    properties_map[c as usize] = property_index
                                }
                            }
                        }
                        continue;
                    }

                    if p.name == "SA" {
                        // Word break property doesn't define SA, but we will use non-UAX29 rules.
                        // SA property is within 0..U+0x20000
                        for c in 0..0x20000 {
                            if lb.get(c) == LineBreak::ComplexContext {
                                properties_map[c as usize] = property_index
                            }
                        }
                        continue;
                    }

                    let prop = get_word_segmenter_value_from_name(&*p.name);
                    for c in 0..(CODEPOINT_TABLE_LEN as u32) {
                        if wb.get(c) == prop {
                            properties_map[c as usize] = property_index;
                        }
                    }
                    continue;
                }

                "grapheme" => {
                    // Extended_Pictographic isn't a part of grapheme break property
                    // Extended pictographic property is within 0..U+0x20000
                    if p.name == "Extended_Pictographic" {
                        for i in 0..0x20000 {
                            if let Some(c) = char::from_u32(i) {
                                if extended_pictographic.contains(c) {
                                    properties_map[c as usize] = property_index
                                }
                            }
                        }
                        continue;
                    }

                    let prop = get_grapheme_segmenter_value_from_name(&*p.name);
                    for c in 0..(CODEPOINT_TABLE_LEN as u32) {
                        if gb.get(c) == prop {
                            properties_map[c as usize] = property_index;
                        }
                    }
                    continue;
                }

                "sentence" => {
                    let prop = get_sentence_segmenter_value_from_name(&*p.name);
                    for c in 0..(CODEPOINT_TABLE_LEN as u32) {
                        if sb.get(c) == prop {
                            properties_map[c as usize] = property_index;
                        }
                    }
                    continue;
                }

                "line" => {
                    if p.name == "CP_EA"
                        || p.name == "OP_OP30"
                        || p.name == "OP_EA"
                        || p.name == "ID_CN"
                        || p.name == "PO_EAW"
                        || p.name == "PR_EAW"
                    {
                        for i in 0..0x20000 {
                            match lb.get(i) {
                                LineBreak::OpenPunctuation => {
                                    if (p.name == "OP_OP30"
                                        && (eaw.get(i) != EastAsianWidth::Fullwidth
                                            && eaw.get(i) != EastAsianWidth::Halfwidth
                                            && eaw.get(i) != EastAsianWidth::Wide))
                                        || (p.name == "OP_EA"
                                            && (eaw.get(i) == EastAsianWidth::Fullwidth
                                                || eaw.get(i) == EastAsianWidth::Halfwidth
                                                || eaw.get(i) == EastAsianWidth::Wide))
                                    {
                                        properties_map[i as usize] = property_index;
                                    }
                                }

                                LineBreak::CloseParenthesis => {
                                    // CP_EA is unused on the latest spec.
                                    if p.name == "CP_EA"
                                        && (eaw.get(i) == EastAsianWidth::Fullwidth
                                            || eaw.get(i) == EastAsianWidth::Halfwidth
                                            || eaw.get(i) == EastAsianWidth::Wide)
                                    {
                                        properties_map[i as usize] = property_index;
                                    }
                                }

                                LineBreak::Ideographic => {
                                    if p.name == "ID_CN" && gc.get(i) == GeneralCategory::Unassigned
                                    {
                                        if let Some(c) = char::from_u32(i) {
                                            if extended_pictographic.contains(c) {
                                                properties_map[i as usize] = property_index;
                                            }
                                        }
                                    }
                                }

                                LineBreak::PostfixNumeric => {
                                    if p.name == "PO_EAW" && is_cjk_fullwidth(eaw, i) {
                                        properties_map[i as usize] = property_index;
                                    }
                                }

                                LineBreak::PrefixNumeric => {
                                    if p.name == "PR_EAW" && is_cjk_fullwidth(eaw, i) {
                                        properties_map[i as usize] = property_index;
                                    }
                                }

                                _ => {}
                            }
                        }
                        continue;
                    }

                    let prop = get_line_segmenter_value_from_name(&*p.name);
                    for c in 0..0x20000 {
                        if lb.get(c) == prop {
                            properties_map[c as usize] = property_index;
                        }
                    }
                    continue;
                }

                _ => {
                    panic!("unknown built-in segmenter type");
                }
            }
        }

        if let Some(codepoint) = &p.codepoint {
            simple_properties_count += 1;
            for c in codepoint {
                let c = *c as usize;
                if c > CODEPOINT_TABLE_LEN {
                    continue;
                }
                properties_map[c] = property_index;
            }
        }
    }

    // sot and eot
    properties_names.push("sot".to_string());
    properties_names.push("eot".to_string());

    let rule_size = properties_names.len() * properties_names.len();
    let mut break_state_table = vec![UNKNOWN_RULE; rule_size];

    for rule in segmenter.rules {
        let break_state;
        if let Some(state) = rule.break_state {
            break_state = if state { BREAK_RULE } else { KEEP_RULE };
        } else {
            break_state = NOT_MATCH_RULE;
        }

        for l in &rule.left {
            if l == "Any" {
                // Special case: left is Any
                for r in &rule.right {
                    if r == "Any" {
                        // Fill all unknown state.
                        for item in break_state_table.iter_mut().take(rule_size) {
                            if *item == UNKNOWN_RULE {
                                *item = break_state;
                            }
                        }
                    } else {
                        let right_index = get_index_from_name(&properties_names, r).unwrap();
                        for i in 0..simple_properties_count {
                            set_break_state(
                                &mut break_state_table,
                                properties_names.len(),
                                i,
                                right_index,
                                break_state,
                            );
                        }
                    }
                }
                continue;
            }
            let left_index = get_index_from_name(&properties_names, l).unwrap();
            for r in &rule.right {
                // Special case: right is Any
                if r == "Any" {
                    for i in 0..properties_names.len() {
                        set_break_state(
                            &mut break_state_table,
                            properties_names.len(),
                            left_index,
                            i,
                            break_state,
                        );
                    }
                    continue;
                }
                let right_index = get_index_from_name(&properties_names, r).unwrap();
                if r != "eot"
                    && break_state_table[left_index * properties_names.len() + right_index]
                        == NOT_MATCH_RULE
                {
                    break_state_table[left_index * properties_names.len() + right_index] =
                        UNKNOWN_RULE;
                }
                set_break_state(
                    &mut break_state_table,
                    properties_names.len(),
                    left_index,
                    right_index,
                    break_state,
                );
                // Fill not match for combine state
                for i in 0..properties_names.len() {
                    if left_index >= simple_properties_count {
                        set_break_state(
                            &mut break_state_table,
                            properties_names.len(),
                            left_index,
                            i,
                            NOT_MATCH_RULE,
                        );
                    }
                }
            }
        }
    }

    let property_length = properties_names.len();

    // State machine alias
    for p in &segmenter.tables {
        if let Some(left) = &p.left {
            if let Some(right) = &p.right {
                let right_index = get_index_from_name(&properties_names, right).unwrap();
                let left_index = get_index_from_name(&properties_names, left).unwrap();
                let interm_break_state = if p.interm_break_state.is_some() {
                    INTERMEDIATE_MATCH_RULE
                } else {
                    0
                };

                let index = properties_names.iter().position(|n| n.eq(&p.name)).unwrap() as i8;
                break_state_table[left_index * property_length + right_index] =
                    index | interm_break_state;
            }
        }
    }

    let out_dir = env::var("OUT_DIR").unwrap();
    let out = Path::new(&out_dir).join(file_name);
    let mut out = File::create(&out).unwrap();
    let mut codepoint_table = Vec::<String>::new();

    let mut i = 0;
    let mut page = 0;
    let mut previous = 0;
    let mut is_same_value = true;

    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const BREAK_PROPERTIES_{}: [u8; 1024] = [", page).ok();
    codepoint_table.push(format!("BREAK_PROPERTIES_{}", page));
    for c in properties_map.iter() {
        // Compress table
        if (i % 1024) == 0 {
            // Reset
            is_same_value = true;
        } else if is_same_value {
            is_same_value = previous == *c;
        }
        previous = *c;

        write!(out, "{: >2},", c).ok();
        i += 1;

        if (i % 16) == 0 {
            writeln!(out).ok();
        }

        if i > 1 && (i % 1024) == 0 {
            writeln!(out, "];").ok();
            if i >= 0x20000 && segmenter.segmenter_type == "line" {
                // Unnecessary over U+0x20000 on line segmenter.
                // It is into line_break.rs.
                break;
            }
            if i >= CODEPOINT_TABLE_LEN {
                break;
            }
            page += 1;

            // Current codepoint mapping table is filled by 0.
            // Use common table
            if is_same_value && *c == 0 {
                codepoint_table.pop();
                codepoint_table.push("BREAK_PROPERTIES_FILL_BY_0".to_string());
            }

            if segmenter.segmenter_type == "line" && is_same_value {
                if get_index_from_name(&properties_names, "ID").unwrap() == *c as usize {
                    codepoint_table.pop();
                    codepoint_table.push("BREAK_PROPERTIES_FILL_BY_ID".to_string());
                }
                if get_index_from_name(&properties_names, "XX").unwrap() == *c as usize {
                    codepoint_table.pop();
                    codepoint_table.push("BREAK_PROPERTIES_FILL_BY_XX".to_string());
                }
            }

            codepoint_table.push(format!("BREAK_PROPERTIES_{}", page));
            writeln!(out, "#[allow(dead_code)]").ok();
            writeln!(out, "pub const BREAK_PROPERTIES_{}: [u8; 1024] = [", page).ok();
            continue;
        }
    }

    output_propery_plane_with_same_value(&mut out, "0", 0);

    if segmenter.segmenter_type == "line" {
        output_propery_plane_with_same_value(
            &mut out,
            "ID",
            get_index_from_name(&properties_names, "ID").unwrap() as u8,
        );
        output_propery_plane_with_same_value(
            &mut out,
            "XX",
            get_index_from_name(&properties_names, "XX").unwrap() as u8,
        );
    }

    if segmenter.segmenter_type == "line" {
        writeln!(out, "pub const PROPERTY_TABLE: [[u8; 1024]; 128] = [").ok();
        for i in codepoint_table.iter() {
            writeln!(out, "    {},", i).ok();
        }
        writeln!(out, "];").ok();
    } else {
        writeln!(
            out,
            "pub const PROPERTY_TABLE: [&[u8; 1024]; {}] = [",
            CODEPOINT_TABLE_LEN / 1024
        )
        .ok();
        for i in codepoint_table.iter() {
            writeln!(out, "    &{},", i).ok();
        }
        writeln!(out, "];").ok();
    }

    writeln!(
        out,
        "pub const BREAK_STATE_MACHINE_TABLE: [i8; {}] = [",
        rule_size
    )
    .ok();
    let mut i = 1;
    writeln!(out, "// {}", properties_names[i - 1]).ok();
    for c in break_state_table.iter() {
        write!(out, "{: >4},", c).ok();
        i += 1;
        if ((i - 1) % properties_names.len()) == 0 {
            writeln!(out).ok();
            if (i / properties_names.len()) < properties_names.len() {
                writeln!(out, "// {}", properties_names[i / properties_names.len()]).ok();
            }
        }
    }
    writeln!(out, "];").ok();

    writeln!(
        out,
        "pub const PROPERTY_COUNT: usize = {};",
        properties_names.len()
    )
    .ok();
    writeln!(
        out,
        "pub const LAST_CODEPOINT_PROPERTY: i8 = {};",
        simple_properties_count - 1
    )
    .ok();

    writeln!(
        out,
        "pub const PROP_SOT: usize = {};",
        properties_names.len() - 2
    )
    .ok();
    writeln!(
        out,
        "pub const PROP_EOT: usize = {};",
        properties_names.len() - 1
    )
    .ok();
    if let Some(sa_index) = get_index_from_name(&properties_names, "SA") {
        writeln!(out, "pub const PROP_COMPLEX: usize = {};", sa_index,).ok();
    } else {
        // complex language isn't handled.
        writeln!(out, "pub const PROP_COMPLEX: usize = 127;").ok();
    }

    for (i, p) in properties_names.iter().enumerate() {
        if segmenter.segmenter_type == "line" {
            writeln!(out, "#[allow(dead_code)]").ok();
            writeln!(out, "pub const {}: u8 = {};", p.to_uppercase(), i).ok();
        } else {
            writeln!(out, "// {} = {}", p, i).ok();
        }
    }

    writeln!(out).ok();
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const BREAK_RULE: i8 = {};", BREAK_RULE).ok();
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const NOT_MATCH_RULE: i8 = {};", NOT_MATCH_RULE).ok();
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const KEEP_RULE: i8 = {};", KEEP_RULE).ok();
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(
        out,
        "pub const INTERMEDIATE_MATCH_RULE: i8 = {};",
        INTERMEDIATE_MATCH_RULE
    )
    .ok();
}

fn main() {
    println!("cargo:rerun-if-changed=data");

    const WORD_SEGMENTER_TOML: &[u8] = include_bytes!("data/word.toml");
    const GRAPHEME_SEGMENTER_TOML: &[u8] = include_bytes!("data/grapheme.toml");
    const SENTENCE_SEGMENTER_TOML: &[u8] = include_bytes!("data/sentence.toml");
    const LINE_SEGMENTER_TOML: &[u8] = include_bytes!("data/line.toml");

    let word_thread = thread::spawn(|| {
        let provider = icu_testdata::get_provider();
        generate_rule_segmenter_table("generated_word_table.rs", WORD_SEGMENTER_TOML, &provider);
    });

    let grapheme_thread = thread::spawn(|| {
        let provider = icu_testdata::get_provider();
        generate_rule_segmenter_table(
            "generated_grapheme_table.rs",
            GRAPHEME_SEGMENTER_TOML,
            &provider,
        );
    });

    let sentence_thread = thread::spawn(|| {
        let provider = icu_testdata::get_provider();
        generate_rule_segmenter_table(
            "generated_sentence_table.rs",
            SENTENCE_SEGMENTER_TOML,
            &provider,
        );
    });

    let provider = icu_testdata::get_provider();
    generate_rule_segmenter_table("generated_line_table.rs", LINE_SEGMENTER_TOML, &provider);

    word_thread
        .join()
        .expect("Couldn't join generating word table thread!.");
    grapheme_thread
        .join()
        .expect("Couldn't join generating grapheme table thread!.");
    sentence_thread
        .join()
        .expect("Couldn't join generating sentence table thread!.");
}
