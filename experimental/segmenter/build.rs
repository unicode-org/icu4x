// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu::properties::{maps, sets, WordBreak};
use icu_provider_fs::FsDataProvider;
use serde::Deserialize;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

#[derive(Deserialize, Debug)]
struct SegmenterPropertyValueMap {
    codepoint: Option<Vec<u32>>,
    left: Option<String>,
    right: Option<String>,
}

#[derive(Deserialize, Debug)]
struct SegmenterProperty {
    name: String,
    value: Option<SegmenterPropertyValueMap>,
}

#[derive(Deserialize, Debug)]
struct SegmenterState {
    left: Vec<String>,
    right: Vec<String>,
    break_state: Option<bool>,
}

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

fn set_break_state(
    break_state_table: &mut [i8],
    property_length: usize,
    left_index: usize,
    right_index: usize,
    break_state: i8,
) {
    if break_state_table[left_index * property_length + right_index] == UNKNOWN_RULE {
        break_state_table[left_index * property_length + right_index] = break_state;
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

fn generate_rule_segmenter_table(file_name: &str, json_data: &[u8], provider: &FsDataProvider) {
    let mut properties_map: [u8; 0x30000] = [0; 0x30000];
    let mut properties_names = Vec::<String>::new();
    let mut simple_properties_count = 0;

    let payload = maps::get_word_break(provider).expect("The data should be valid!");
    let wb = &payload.get().code_point_trie;

    let payload = sets::get_extended_pictographic(provider).expect("The data should be valid");
    let extended_pictographic = &payload.get().inv_list;

    let segmenter: SegmenterRuleTable =
        serde_json::from_slice(json_data).expect("JSON syntax error");

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

        if p.value.is_none() {
            simple_properties_count += 1;

            match &*segmenter.segmenter_type {
                "word" => {
                    // Extended_Pictographic isn't a part of word break property
                    if p.name == "Extended_Pictographic" {
                        for i in 0..0x30000 {
                            if let Some(c) = char::from_u32(i) {
                                if extended_pictographic.contains(c) {
                                    properties_map[c as usize] = property_index
                                }
                            }
                        }
                        continue;
                    }

                    let prop = get_word_segmenter_value_from_name(&*p.name);
                    for c in 0..0x30000 {
                        if wb.get(c) == prop {
                            properties_map[c as usize] = property_index;
                        }
                    }
                    continue;
                }
                _ => {
                    // TODO
                    // sentence and grapheme
                    panic!("unknown built-in segmenter type");
                }
            }
        }

        if let Some(codepoint) = &p.value.as_ref().unwrap().codepoint {
            simple_properties_count += 1;
            for c in codepoint {
                let c = *c as usize;
                if c >= 0x30000 {
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
    let mut break_state_table = Vec::<i8>::with_capacity(rule_size);
    for _i in 0..rule_size {
        break_state_table.push(UNKNOWN_RULE);
    }

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
                        if break_state == NOT_MATCH_RULE && i == properties_names.len() - 1 {
                            break_state_table[left_index * properties_names.len() + i] =
                                UNKNOWN_RULE;
                        }
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
                // Fill not match
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
        if let Some(value) = &p.value {
            if let Some(left) = &value.left {
                if let Some(right) = &value.right {
                    let right_index = get_index_from_name(&properties_names, right).unwrap();
                    let left_index = get_index_from_name(&properties_names, left).unwrap();

                    let index = properties_names.iter().position(|n| n.eq(&p.name)).unwrap();
                    break_state_table[left_index * property_length + right_index] = index as i8;
                }
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
            if i >= 0x30000 {
                break;
            }
            page += 1;

            // Current codepoint mapping table is filled by 0.
            // Use common table
            if is_same_value && *c == 0 {
                codepoint_table.pop();
                codepoint_table.push("BREAK_PROPERTIES_FILL_BY_0".to_string());
            }

            codepoint_table.push(format!("BREAK_PROPERTIES_{}", page));
            writeln!(out, "#[allow(dead_code)]").ok();
            writeln!(out, "pub const BREAK_PROPERTIES_{}: [u8; 1024] = [", page).ok();
            continue;
        }
    }

    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const BREAK_PROPERTIES_FILL_BY_0: [u8; 1024] = [").ok();
    for i in 0..1024 {
        write!(out, " 0,").ok();
        if ((i + 1) % 16) == 0 {
            writeln!(out).ok();
        }
    }
    writeln!(out, "];").ok();

    writeln!(out, "pub const PROPERTY_TABLE: [&[u8; 1024]; 192] = [").ok();
    for i in codepoint_table.iter() {
        writeln!(out, "    &{},", i).ok();
    }
    writeln!(out, "];").ok();

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
    }

    for (i, p) in properties_names.iter().enumerate() {
        writeln!(out, "// {} = {}", p, i).ok();
    }

    writeln!(out).ok();
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const BREAK_RULE: i8 = -128;").ok();
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const NOT_MATCH_RULE: i8 = -2;").ok();
    writeln!(out, "#[allow(dead_code)]").ok();
    writeln!(out, "pub const KEEP_RULE: i8 = -1;").ok();
}

fn main() {
    const WORD_SEGMENTER_JSON: &[u8] = include_bytes!("data/word.json");

    let provider = icu_testdata::get_provider();
    generate_rule_segmenter_table("generated_word_table.rs", WORD_SEGMENTER_JSON, &provider);
}
