// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by built-in segmentation data.

#![cfg_attr(
    not(any(feature = "use_wasm", feature = "use_icu4c")),
    allow(dead_code, unused_imports)
)]

#[cfg(feature = "unstable")]
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
#[cfg(feature = "unstable")]
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use crate::source::AbstractFs;
#[cfg(feature = "unstable")]
use crate::source::Cache;
use crate::source::{UnicodeCache, include_files};
#[cfg(feature = "unstable")]
use icu::collections::codepointinvlist::CodePointInversionList;
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
use icu::properties::{
    CodePointMapData, CodePointMapDataBorrowed, CodePointSetData,
    props::{
        EastAsianWidth, GeneralCategory, GraphemeClusterBreak, IndicConjunctBreak, LineBreak,
        Script, SentenceBreak, WordBreak,
    },
};
use icu::segmenter::options::WordType;
use icu::segmenter::provider::*;
use icu_provider::prelude::*;
#[cfg(feature = "unstable")]
use std::borrow::Cow;
use std::collections::HashSet;
#[cfg(feature = "unstable")]
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::sync::OnceLock;

mod dictionary;
mod lstm;
#[cfg(feature = "unstable")]
mod unihan;

// state machine name define by builtin name
// [[tables]]
// name = "Double_Quote"
//
// state machine name define by combined state and as simple property
// This doesn't break between properties even if combined rules are not matched.
// [[tables]]
// name = "ALetter_ZWJ"
// as_simple_property = true
//
// state machine define for combined state
// [[tables]]
// name = "Double_Quote_ALetter"
// left = "Double_Quote"
// right = "ALetter"
#[derive(serde::Deserialize, Debug)]
struct SegmenterProperty {
    name: String,
    // If left and right are defined, this define is combined state.
    left: Option<String>,
    right: Option<String>,
    // This combine state is an intermediate match rule.
    interm_break_state: Option<bool>,
    // Defiened as single property to move marker even if not matched.
    as_simple_property: Option<bool>,
}

// state machine break result define
// The follow is "Double_Quote x Double_Quote".
// [[rules]]
// left = [ "Double_Qoute" ]
// right = [ "Double_Qoute" ]
// break_state = true # true if break opportunity.
#[derive(serde::Deserialize, Debug)]
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
#[derive(serde::Deserialize, Debug)]
struct SegmenterRuleTable {
    segmenter_type: String,
    tables: Vec<SegmenterProperty>,
    rules: Vec<SegmenterState>,
}

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
fn generate_rule_break_data(
    provider: &SourceDataProvider,
    rules_file: &str,
    trie_type: crate::TrieType,
) -> Result<RuleBreakData<'static>, DataError> {
    use icu::properties::{PropertyParser, props::ExtendedPictographic};
    use icu_codepointtrie_builder::CodePointTrieBuilder;

    let segmenter =
        toml::from_str::<SegmenterRuleTable>(rules_file).expect("The data should be valid!");

    fn set_break_state(
        break_state_table: &mut [Option<BreakState>],
        property_length: usize,
        left_index: usize,
        right_index: usize,
        break_state: BreakState,
    ) {
        let index = left_index * property_length + right_index;
        if break_state_table[index].is_none()
            || break_state_table[index] == Some(BreakState::NoMatch)
        {
            break_state_table[index] = Some(break_state);
        }
    }

    fn get_index_from_name(properties_names: &[String], s: &str) -> Option<usize> {
        properties_names.iter().position(|n| n.eq(s))
    }

    fn is_cjk_fullwidth(eaw: CodePointMapDataBorrowed<EastAsianWidth>, codepoint: u32) -> bool {
        matches!(
            eaw.get32(codepoint),
            EastAsianWidth::Ambiguous | EastAsianWidth::Fullwidth | EastAsianWidth::Wide
        )
    }

    // As of Unicode 14.0.0, the break property and the largest codepoint defined in UCD are
    // summarized in the following list. See details in the property txt in
    // https://www.unicode.org/Public/14.0.0/ucd/
    //
    // Line Break Property: U+E01EF ; CM [1]
    // Grapheme Break Property: U+E0FFF ; Control
    // Sentence Break Property: U+E01EF ; Extend
    // Word Break Property: U+E01EF ; Extend
    //
    // The table length should be large enough to contain all codepoints.
    //
    // [1] In LineBreak.txt, it defines F0000..FFFFD and 100000..10FFFD to be "XX", which are
    // the default unassigned values, so it's ok to omit them in the table.
    const CODEPOINT_TABLE_LEN: usize = 0xE1000;

    let mut properties_trie = CodePointTrieBuilder::new(0u8, 0, trie_type.into());
    let mut properties_names = Vec::<String>::new();
    let mut simple_properties_count = 0;

    properties_names.push("Unknown".to_string());
    simple_properties_count += 1;

    match &*segmenter.segmenter_type {
        "word" => {
            let wb = CodePointMapData::<WordBreak>::try_new_unstable(provider)?;
            let wb = wb.as_borrowed();
            let extended_pictographic =
                CodePointSetData::try_new_unstable::<ExtendedPictographic>(provider)?;
            let extended_pictographic = extended_pictographic.as_borrowed();
            let script = CodePointMapData::<Script>::try_new_unstable(provider)?;
            let script = script.as_borrowed();
            let lb = CodePointMapData::<LineBreak>::try_new_unstable(provider)?;
            let lb = lb.as_borrowed();
            let wb_name_to_enum = PropertyParser::<WordBreak>::try_new_unstable(provider)?;
            let wb_name_to_enum = wb_name_to_enum.as_borrowed();

            for p in &segmenter.tables {
                let property_index = if !properties_names.contains(&p.name) {
                    properties_names.push(p.name.clone());
                    (properties_names.len() - 1).try_into().unwrap()
                } else {
                    continue;
                };

                if p.left.is_none() && p.right.is_none() {
                    // If any values aren't set, this is builtin type.
                    simple_properties_count += 1;

                    if p.as_simple_property.is_some() {
                        // defined as simple property. It means that we move the marker to the next property.
                        continue;
                    }
                    if p.name == "Extended_Pictographic" {
                        // :Word_Break=ALetter: includes Extended_Pictographic. So we want to
                        // exlude ALetter.
                        // [[:Extended_Pictographic:] - [:Word_Break=ALetter:]]
                        for range in extended_pictographic.iter_ranges() {
                            for ch in range.filter(|ch| wb.get32(*ch) != WordBreak::ALetter) {
                                properties_trie.set_value(ch, property_index);
                            }
                        }
                        continue;
                    }

                    if p.name == "ALetter_Extended_Pictographic" {
                        // [[:Extended_Pictographic:] & [:Word_Break=ALetter:]]
                        for range in wb.iter_ranges_for_value(WordBreak::ALetter) {
                            for ch in range.filter(|ch| extended_pictographic.contains32(*ch)) {
                                properties_trie.set_value(ch, property_index);
                            }
                        }
                        continue;
                    }

                    if p.name == "SA" {
                        // Word break property doesn't define SA, but we will use non-UAX29 rules.
                        for range in script.iter_ranges_for_value(Script::Han) {
                            properties_trie.set_range_value(range, property_index);
                        }
                        for range in script.iter_ranges_for_value(Script::Hiragana) {
                            properties_trie.set_range_value(range, property_index);
                        }
                        for range in lb.iter_ranges_for_value(LineBreak::ComplexContext) {
                            // Unicode 16.0 changes some Complex properties to others such as U+19DA.
                            // Excluding Numriec should be removed after line break is 16.0
                            for ch in range.filter(|ch| *ch != 0x19da) {
                                properties_trie.set_value(ch, property_index);
                            }
                        }
                        continue;
                    }

                    // TODO(#2239):
                    // How to handle Katakana in UAX29? UAX29 defines Katakana rule, but CJ dictionary has another rules.
                    // Katakana will use UAX#29 rules instead of dictionary.

                    let prop = wb_name_to_enum
                        .get_loose(&p.name)
                        .expect("property name should be valid!");
                    for range in wb.iter_ranges_for_value(prop) {
                        if prop == WordBreak::Extend {
                            // [[:Word_Break=Extend:] - [[:Hani:] [:Line_Break=Complex_Context:]]]
                            for ch in range.filter(|ch| {
                                script.get32(*ch) != Script::Han
                                    && lb.get32(*ch) != LineBreak::ComplexContext
                            }) {
                                properties_trie.set_value(ch, property_index);
                            }
                        } else if prop == WordBreak::ALetter {
                            // :Word_Break=ALetter: includes Extended_Pictographic. So we want to
                            // exlude it.
                            // "[[:Word_Break=ALetter:] - [:Extended_Pictographic:]]"
                            for ch in range.filter(|ch| !extended_pictographic.contains32(*ch)) {
                                properties_trie.set_value(ch, property_index);
                            }
                        } else {
                            properties_trie.set_range_value(range, property_index);
                        }
                    }

                    continue;
                }
            }
        }

        "grapheme" => {
            let extended_pictographic =
                CodePointSetData::try_new_unstable::<ExtendedPictographic>(provider)?;
            let extended_pictographic = extended_pictographic.as_borrowed();
            let incb = CodePointMapData::<IndicConjunctBreak>::try_new_unstable(provider)?;
            let incb = incb.as_borrowed();
            let gcb_name_to_enum =
                PropertyParser::<GraphemeClusterBreak>::try_new_unstable(provider)?;
            let gcb_name_to_enum = gcb_name_to_enum.as_borrowed();
            let gb = CodePointMapData::<GraphemeClusterBreak>::try_new_unstable(provider)?;
            let gb = gb.as_borrowed();

            for p in &segmenter.tables {
                let property_index = if !properties_names.contains(&p.name) {
                    properties_names.push(p.name.clone());
                    (properties_names.len() - 1).try_into().unwrap()
                } else {
                    continue;
                };

                if p.left.is_none() && p.right.is_none() {
                    // If any values aren't set, this is builtin type.
                    simple_properties_count += 1;

                    if p.as_simple_property.is_some() {
                        // defined as simple property. It means that we move the marker to the next property.
                        continue;
                    }
                    // Extended_Pictographic isn't a part of grapheme break property
                    if p.name == "Extended_Pictographic" {
                        for range in extended_pictographic.iter_ranges() {
                            properties_trie.set_range_value(range, property_index);
                        }
                        continue;
                    }

                    let relevant_incb = match &*p.name {
                        "InCBConsonant" => Some(IndicConjunctBreak::Consonant),
                        "InCBLinker" => Some(IndicConjunctBreak::Linker),
                        "InCBExtend" => Some(IndicConjunctBreak::Extend),
                        _ => None,
                    };

                    if let Some(relevant_incb) = relevant_incb {
                        for range in incb.iter_ranges_for_value(relevant_incb) {
                            if range.contains(&0x200D) {
                                // ZWJ is handled as a separate rule
                                for ch in range.filter(|ch| *ch != 0x200D) {
                                    properties_trie.set_value(ch, property_index);
                                }
                            } else {
                                properties_trie.set_range_value(range, property_index);
                            }
                        }

                        continue;
                    }

                    let prop = gcb_name_to_enum
                        .get_loose(&p.name)
                        .expect("property name should be valid!");

                    for range in gb.iter_ranges_for_value(prop) {
                        properties_trie.set_range_value(range, property_index);
                    }
                    continue;
                }
            }
        }

        "sentence" => {
            let sb = CodePointMapData::<SentenceBreak>::try_new_unstable(provider)?;
            let sb = sb.as_borrowed();
            let sb_name_to_enum = PropertyParser::<SentenceBreak>::try_new_unstable(provider)?;
            let sb_name_to_enum = sb_name_to_enum.as_borrowed();

            for p in &segmenter.tables {
                let property_index = if !properties_names.contains(&p.name) {
                    properties_names.push(p.name.clone());
                    (properties_names.len() - 1).try_into().unwrap()
                } else {
                    continue;
                };

                if p.left.is_none() && p.right.is_none() {
                    // If any values aren't set, this is builtin type.
                    simple_properties_count += 1;

                    if p.as_simple_property.is_some() {
                        // defined as simple property. It means that we move the marker to the next property.
                        continue;
                    }
                    let prop = sb_name_to_enum
                        .get_loose(&p.name)
                        .expect("property name should be valid!");
                    for range in sb.iter_ranges_for_value(prop) {
                        properties_trie.set_range_value(range, property_index);
                    }
                    continue;
                }
            }
        }

        "line" => {
            let lb = CodePointMapData::<LineBreak>::try_new_unstable(provider)?;
            let lb = lb.as_borrowed();
            let eaw = CodePointMapData::<EastAsianWidth>::try_new_unstable(provider)?;
            let eaw = eaw.as_borrowed();
            let gc = CodePointMapData::<GeneralCategory>::try_new_unstable(provider)?;
            let gc = gc.as_borrowed();
            let extended_pictographic =
                CodePointSetData::try_new_unstable::<ExtendedPictographic>(provider)?;
            let extended_pictographic = extended_pictographic.as_borrowed();
            let lb_name_to_enum = PropertyParser::<LineBreak>::try_new_unstable(provider)?;
            let lb_name_to_enum = lb_name_to_enum.as_borrowed();

            for p in &segmenter.tables {
                let property_index = if !properties_names.contains(&p.name) {
                    properties_names.push(p.name.clone());
                    (properties_names.len() - 1).try_into().unwrap()
                } else {
                    continue;
                };

                if p.left.is_none() && p.right.is_none() {
                    // If any values aren't set, this is builtin type.
                    simple_properties_count += 1;

                    if p.as_simple_property.is_some() {
                        // defined as simple property. It means that we move the marker to the next property.
                        continue;
                    }
                    if p.name == "CP_EA"
                        || p.name == "OP_OP30"
                        || p.name == "OP_EA"
                        || p.name == "ID_CN"
                        || p.name == "PO_EAW"
                        || p.name == "PR_EAW"
                        || p.name == "AL_DOTTED_CIRCLE"
                        || p.name == "QU_PI"
                        || p.name == "QU_PF"
                    {
                        for cp in 0..(CODEPOINT_TABLE_LEN as u32) {
                            match lb.get32(cp) {
                                LineBreak::OpenPunctuation
                                    if ((p.name == "OP_OP30"
                                        && (eaw.get32(cp) != EastAsianWidth::Fullwidth
                                            && eaw.get32(cp) != EastAsianWidth::Halfwidth
                                            && eaw.get32(cp) != EastAsianWidth::Wide))
                                        || (p.name == "OP_EA"
                                            && (eaw.get32(cp) == EastAsianWidth::Fullwidth
                                                || eaw.get32(cp) == EastAsianWidth::Halfwidth
                                                || eaw.get32(cp) == EastAsianWidth::Wide)))
                                    => {
                                        properties_trie.set_value(cp, property_index);
                                    }

                                LineBreak::CloseParenthesis
                                    // CP_EA is unused on the latest spec.
                                    if p.name == "CP_EA"
                                        && (eaw.get32(cp) == EastAsianWidth::Fullwidth
                                            || eaw.get32(cp) == EastAsianWidth::Halfwidth
                                            || eaw.get32(cp) == EastAsianWidth::Wide)
                                    => {
                                        properties_trie.set_value(cp, property_index);
                                    }

                                LineBreak::Ideographic
                                    if p.name == "ID_CN"
                                        && gc.get32(cp) == GeneralCategory::Unassigned
                                    => {
                                        if let Some(c) = char::from_u32(cp) {
                                            if extended_pictographic.contains(c) {
                                                properties_trie.set_value(cp, property_index);
                                            } else {
                                                // Line segmenter doesn't use Unicode 17's data,
                                                // but extended_pictographic is 17.
                                                // So this is a hack to use old Unicode rules with
                                                // newer Unicode data.
                                                // This should be removed when line segmenter uses
                                                // Unicode 17.
                                                // (https://github.com/unicode-org/icu4x/issues/7134)
                                                match cp {
                                                    0x1f774..=0x1f77f => properties_trie
                                                        .set_value(cp, property_index),
                                                    0x1f8ae..=0x1f8ff => properties_trie
                                                        .set_value(cp, property_index),
                                                    0x1f947..=0x1faff => properties_trie
                                                        .set_value(cp, property_index),
                                                    _ => {}
                                                };
                                            }
                                        }
                                    }

                                LineBreak::PostfixNumeric
                                    if p.name == "PO_EAW" && is_cjk_fullwidth(eaw, cp) => {
                                        properties_trie.set_value(cp, property_index);
                                    }

                                LineBreak::PrefixNumeric
                                    if p.name == "PR_EAW" && is_cjk_fullwidth(eaw, cp) => {
                                        properties_trie.set_value(cp, property_index);
                                    }

                                LineBreak::Alphabetic
                                    if p.name == "AL_DOTTED_CIRCLE" && cp == 0x25CC => {
                                        properties_trie.set_value(cp, property_index);
                                    }

                                LineBreak::Quotation => {
                                    if p.name == "QU_PI"
                                        && gc.get32(cp) == GeneralCategory::InitialPunctuation
                                    {
                                        properties_trie.set_value(cp, property_index);
                                    }

                                    if p.name == "QU_PF"
                                        && gc.get32(cp) == GeneralCategory::FinalPunctuation
                                    {
                                        properties_trie.set_value(cp, property_index);
                                    }
                                }

                                _ => {}
                            }
                        }
                        continue;
                    }

                    let prop = lb_name_to_enum
                        .get_loose(&p.name)
                        .expect("property name should be valid!");
                    for range in lb.iter_ranges_for_value(prop) {
                        properties_trie.set_range_value(range, property_index);
                    }

                    if p.name == "AL" {
                        // LB1: SG has no special rules.
                        let prop = lb_name_to_enum
                            .get_loose("SG")
                            .expect("property name should be valid!");
                        for range in lb.iter_ranges_for_value(prop) {
                            properties_trie.set_range_value(range, property_index);
                        }
                    }
                    continue;
                }
            }

            for (name, value) in [
                ("AI", RuleBreakData::LINE_PROPERTY_AI),
                ("AK", RuleBreakData::LINE_PROPERTY_AK),
                (
                    "AL_DOTTED_CIRCLE",
                    RuleBreakData::LINE_PROPERTY_AL_DOTTED_CIRCLE,
                ),
                ("AL", RuleBreakData::LINE_PROPERTY_AL),
                ("AP", RuleBreakData::LINE_PROPERTY_AP),
                ("AS", RuleBreakData::LINE_PROPERTY_AS),
                ("B2", RuleBreakData::LINE_PROPERTY_B2),
                ("BA", RuleBreakData::LINE_PROPERTY_BA),
                ("BB", RuleBreakData::LINE_PROPERTY_BB),
                ("BK", RuleBreakData::LINE_PROPERTY_BK),
                ("CB", RuleBreakData::LINE_PROPERTY_CB),
                ("CJ", RuleBreakData::LINE_PROPERTY_CJ),
                ("CL", RuleBreakData::LINE_PROPERTY_CL),
                ("CM", RuleBreakData::LINE_PROPERTY_CM),
                ("CP", RuleBreakData::LINE_PROPERTY_CP),
                ("CR", RuleBreakData::LINE_PROPERTY_CR),
                ("EB", RuleBreakData::LINE_PROPERTY_EB),
                ("EM", RuleBreakData::LINE_PROPERTY_EM),
                ("EX", RuleBreakData::LINE_PROPERTY_EX),
                ("GL", RuleBreakData::LINE_PROPERTY_GL),
                ("H2", RuleBreakData::LINE_PROPERTY_H2),
                ("H3", RuleBreakData::LINE_PROPERTY_H3),
                ("HL", RuleBreakData::LINE_PROPERTY_HL),
                ("HY", RuleBreakData::LINE_PROPERTY_HY),
                ("ID_CN", RuleBreakData::LINE_PROPERTY_ID_CN),
                ("ID", RuleBreakData::LINE_PROPERTY_ID),
                ("IN", RuleBreakData::LINE_PROPERTY_IN),
                ("IS", RuleBreakData::LINE_PROPERTY_IS),
                ("JL", RuleBreakData::LINE_PROPERTY_JL),
                ("JT", RuleBreakData::LINE_PROPERTY_JT),
                ("JV", RuleBreakData::LINE_PROPERTY_JV),
                ("LF", RuleBreakData::LINE_PROPERTY_LF),
                ("NL", RuleBreakData::LINE_PROPERTY_NL),
                ("NS", RuleBreakData::LINE_PROPERTY_NS),
                ("NU", RuleBreakData::LINE_PROPERTY_NU),
                ("OP_EA", RuleBreakData::LINE_PROPERTY_OP_EA),
                ("OP_OP30", RuleBreakData::LINE_PROPERTY_OP_OP30),
                ("PO_EAW", RuleBreakData::LINE_PROPERTY_PO_EAW),
                ("PO", RuleBreakData::LINE_PROPERTY_PO),
                ("PR_EAW", RuleBreakData::LINE_PROPERTY_PR_EAW),
                ("PR", RuleBreakData::LINE_PROPERTY_PR),
                ("QU_PF", RuleBreakData::LINE_PROPERTY_QU_PF),
                ("QU_PI", RuleBreakData::LINE_PROPERTY_QU_PI),
                ("QU", RuleBreakData::LINE_PROPERTY_QU),
                ("RI", RuleBreakData::LINE_PROPERTY_RI),
                ("SP", RuleBreakData::LINE_PROPERTY_SP),
                ("SY", RuleBreakData::LINE_PROPERTY_SY),
                ("VF", RuleBreakData::LINE_PROPERTY_VF),
                ("VI", RuleBreakData::LINE_PROPERTY_VI),
                ("WJ", RuleBreakData::LINE_PROPERTY_WJ),
                ("XX", RuleBreakData::LINE_PROPERTY_XX),
                ("ZW", RuleBreakData::LINE_PROPERTY_ZW),
                ("ZWJ", RuleBreakData::LINE_PROPERTY_ZWJ),
            ] {
                assert_eq!(
                    get_index_from_name(&properties_names, name),
                    Some(value as usize),
                    "{name} {properties_names:?}"
                );
            }
        }

        _ => {
            panic!("unknown built-in segmenter type");
        }
    }

    // sot and eot
    properties_names.push("sot".to_string());
    properties_names.push("eot".to_string());

    let rule_size = properties_names.len() * properties_names.len();
    let mut break_state_table = vec![None; rule_size];

    for rule in &segmenter.rules {
        let break_state = if let Some(state) = rule.break_state {
            if state {
                BreakState::Break
            } else {
                BreakState::Keep
            }
        } else {
            BreakState::NoMatch
        };

        for l in &rule.left {
            if l == "Any" {
                // Special case: left is Any
                for r in &rule.right {
                    if r == "Any" {
                        // Fill all unknown state.
                        for item in break_state_table.iter_mut().take(rule_size) {
                            if item.is_none() {
                                *item = Some(break_state);
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
                        == Some(BreakState::NoMatch)
                {
                    break_state_table[left_index * properties_names.len() + right_index] = None;
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
                            BreakState::NoMatch,
                        );
                    }
                }
            }
        }
    }

    // State machine alias
    for p in &segmenter.tables {
        if let Some(left) = &p.left
            && let Some(right) = &p.right
        {
            let right_index = get_index_from_name(&properties_names, right).unwrap();
            let left_index = get_index_from_name(&properties_names, left).unwrap();

            let index = properties_names.iter().position(|n| n.eq(&p.name)).unwrap();
            break_state_table[left_index * properties_names.len() + right_index] =
                Some(if p.interm_break_state.is_some() {
                    BreakState::Intermediate(index.try_into().unwrap())
                } else {
                    BreakState::Index(index.try_into().unwrap())
                })
        }
    }

    Ok(RuleBreakData {
        property_table: properties_trie.build(),
        break_state_table: break_state_table
            .into_iter()
            // All states are initialized
            .map(|o| o.unwrap())
            .collect(),
        rule_status_table: if segmenter.segmenter_type == "word" {
            segmenter
                .tables
                .iter()
                .map(|p| match &*p.name {
                    "Numeric" => WordType::Number,
                    "ALetter" | "Hebrew_Letter" | "ExtendNumLet" | "Katakana" | "SA" => {
                        WordType::Letter
                    }
                    _ => WordType::None,
                } as u8)
                .collect()
        } else {
            Default::default()
        },
        property_count: properties_names.len().try_into().unwrap(),
        last_codepoint_property: (simple_properties_count - 1).try_into().unwrap(),
        sot_property: (properties_names.len() - 2).try_into().unwrap(),
        eot_property: (properties_names.len() - 1).try_into().unwrap(),
        // Return 127 if the complex language isn't handled.
        complex_property: get_index_from_name(&properties_names, "SA")
            .unwrap_or(127)
            .try_into()
            .unwrap(),
    })
}

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
fn generate_rule_break_data_override(
    _provider: &SourceDataProvider,
    rules_file: &str,
    trie_type: crate::TrieType,
) -> RuleBreakDataOverride<'static> {
    use icu_codepointtrie_builder::CodePointTrieBuilder;

    let segmenter =
        toml::from_str::<SegmenterRuleTable>(rules_file).expect("The data should be valid!");

    let mut properties_trie = CodePointTrieBuilder::new(0u8, 0, trie_type.into());
    let mut properties_names = Vec::<String>::new();

    properties_names.push("Unknown".to_string());

    for p in &segmenter.tables {
        let property_index = if !properties_names.contains(&p.name) {
            properties_names.push(p.name.clone());
            (properties_names.len() - 1).try_into().unwrap()
        } else {
            continue;
        };

        if p.left.is_none() && p.right.is_none() {
            // If any values aren't set, this is builtin type.
            match &*segmenter.segmenter_type {
                // UAX#29 doesn't define the 2 characters as STerm, but ICU4C's
                // Greek data does.
                //
                // TODO: We have to consider this definition from CLDR instead.
                "sentence" if p.name == "STerm" => {
                    properties_trie.set_value(0x003b, property_index);
                    properties_trie.set_value(0x037e, property_index);
                }
                _ => {}
            }
        }
    }

    RuleBreakDataOverride {
        property_table_override: properties_trie.build(),
    }
}

macro_rules! implement {
    ($marker:ident, $rules:literal, $provider:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
                return Err(DataError::custom(
                    "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
                )
                .with_req($marker::INFO, req));
                #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
                return {
                    self.check_req::<$marker>(req)?;
                    let data = generate_rule_break_data(
                        ($provider)(self),
                        include_str!(concat!("../../data/segmenter/", $rules)),
                        self.trie_type(),
                    )?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(data),
                    })
                };
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    }
}

macro_rules! implement_override {
    ($marker:ident, $rules:literal, [$($supported:expr),*]) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
                return Err(DataError::custom(
                    "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
                )
                .with_req($marker::INFO, req));
                #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
                return {
                    self.check_req::<$marker>(req)?;
                    let data = generate_rule_break_data_override(
                        self,
                        include_str!(concat!("../../data/segmenter/", $rules)),
                        self.trie_type(),
                    );

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(data),
                    })
                };
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                const SUPPORTED: &[&str] = &[$($supported),*];
                Ok(SUPPORTED
                   .iter()
                   .map(|l|DataIdentifierCow::from_locale(DataLocale::try_from_str(l).unwrap()))
                   .collect())
            }
        }
    }
}

fn unicode_15_1() -> &'static SourceDataProvider {
    // Singleton so that all instantiations share the same cache.
    static SINGLETON: OnceLock<SourceDataProvider> = OnceLock::new();
    SINGLETON.get_or_init(|| {
        let mut provider = SourceDataProvider::new_custom();
        provider.unicode_paths = Some(std::sync::Arc::new(UnicodeCache::new_local(
            include_files!(
                "../../data/segmenter/unicode15/";
                "ucd/DerivedCoreProperties.txt",
                "ucd/emoji/emoji-data.txt",
                "ucd/extracted/DerivedEastAsianWidth.txt",
                "ucd/extracted/DerivedGeneralCategory.txt",
                "ucd/LineBreak.txt",
                "ucd/PropertyAliases.txt",
                "ucd/PropertyValueAliases.txt",
                "ucd/PropList.txt",
            ),
        )));
        provider
    })
}

implement!(SegmenterBreakLineV1, "line.toml", |_| unicode_15_1());
implement!(SegmenterBreakGraphemeClusterV1, "grapheme.toml", |s| s);
implement!(SegmenterBreakWordV1, "word.toml", |s| s);
implement!(SegmenterBreakSentenceV1, "sentence.toml", |s| s);
implement_override!(SegmenterBreakWordOverrideV1, "word.toml", []);
implement_override!(SegmenterBreakSentenceOverrideV1, "sentence.toml", ["el"]);

#[cfg(feature = "unstable")]
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
fn neo_sources() -> AbstractFs {
    include_files!(
        "../../data/segmenter/neo/";
        "GraphemeClusterBreakStates.txt",
        "GraphemeClusterBreakSymbols.txt",
        "GraphemeClusterBreakTransitions.txt",
        "LineBreakStates.txt",
        "LineBreakSymbols.txt",
        "LineBreakTailoring_cj.txt",
        "LineBreakTailoring_loose_cj.txt",
        "LineBreakTailoring_loose.txt",
        "LineBreakTailoring_normal_cj.txt",
        "LineBreakTailoring_normal.txt",
        "LineBreakTailoring_word_breakall.txt",
        "LineBreakTailoring_word_keepall.txt",
        "LineBreakTransitions.txt",
        "SentenceBreakStates.txt",
        "SentenceBreakSymbols.txt",
        "SentenceBreakTailoring_el.txt",
        "SentenceBreakTransitions.txt",
        "WordBreakStates.txt",
        "WordBreakSymbols.txt",
        "WordBreakTransitions.txt",
    )
}

#[test]
#[ignore]
#[cfg(feature = "networking")]
fn download() {
    use std::fs::File;
    use std::io::Write;

    let data_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/segmenter/neo");

    for file in neo_sources().list("").unwrap() {
        if matches!(
            file.as_str(),
            "SentenceBreakTailoring_el.txt"
                | "LineBreakTailoring_word_breakall.txt"
                | "LineBreakTailoring_word_keepall.txt"
        ) {
            // ICU4X-custom tailorings
            continue;
        }

        let target = data_root.join(&file);
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        crlify::BufWriterWithLineEndingFix::new(File::create(&target).unwrap())
            .write_all(
                &AbstractFs::new_from_url(
                    concat!(
                        "https://raw.githubusercontent.com/eggrobin/unicodetools/",
                        "refs/heads/RoBertBastIan/"
                    )
                    .into(),
                )
                .read_to_buf(&file)
                .unwrap(),
            )
            .unwrap();
    }
}

#[cfg(feature = "unstable")]
type TailoredSegmenter = (
    SegmenterStateMachine<'static>,
    BTreeMap<String, SegmenterStateMachineOverride<'static>>,
);

#[cfg(feature = "unstable")]
#[derive(Debug, Default)]
pub(crate) struct NeoSegmenters {
    line: Cache<TailoredSegmenter>,
    word: Cache<TailoredSegmenter>,
    sentence: Cache<TailoredSegmenter>,
    grapheme_cluster: Cache<TailoredSegmenter>,
}

#[cfg(feature = "unstable")]
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
impl SourceDataProvider {
    fn line_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.unicode()?
            .segmenter_cache
            .line
            .get_or_init(|| {
                self.build_segmenter(&neo_sources(), "LineBreak", |s| {
                    if s == "Mandatory" { 1 } else { 0 }
                })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn word_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.unicode()?
            .segmenter_cache
            .word
            .get_or_init(|| {
                self.build_segmenter(&neo_sources(), "WordBreak", |s| match s {
                    "Letter" => WordType::Letter,
                    "Number" => WordType::Number,
                    _ => WordType::None,
                } as u8)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn sentence_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.unicode()?
            .segmenter_cache
            .sentence
            .get_or_init(|| {
                self.build_segmenter(&neo_sources(), "SentenceBreak", |s| {
                    if s == "Nonterminated" { 1 } else { 0 }
                })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn grapheme_cluster_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.unicode()?
            .segmenter_cache
            .grapheme_cluster
            .get_or_init(|| {
                self.build_segmenter(&neo_sources(), "GraphemeClusterBreak", |s| match s {
                    "" => 0,
                    s => unreachable!("{s}"),
                })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn build_segmenter(
        &self,
        sources: &AbstractFs,
        prefix: &str,
        status_lookup: fn(&str) -> u8,
    ) -> Result<
        (
            SegmenterStateMachine<'static>,
            BTreeMap<String, SegmenterStateMachineOverride<'static>>,
        ),
        DataError,
    > {
        let mut magic_symbols = BTreeMap::new();
        let mut fixed_symbol_assignments = BTreeMap::new();
        let mut complex_symbols = BTreeMap::new();
        let symbols = sources.read_to_string(&format!("{prefix}Symbols.txt"))?;
        let mut symbols = symbols
            .lines()
            .map(|l| l.split('#').next().unwrap().trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut iter = line.split(';').map(str::trim);
                let symbol = iter.next().unwrap();
                let unicode_set = iter.next().unwrap();

                if let Some(non_complex_equivalent) = iter.next()
                    && !non_complex_equivalent.is_empty()
                {
                    complex_symbols.insert(symbol, non_complex_equivalent);
                }

                let set = icu::properties::unicodeset_parse::parse_unstable(unicode_set, self)
                    .map_err(|e| {
                        DataError::custom("unicodeset parse")
                            .with_display_context(&e.fmt_with_source(unicode_set))
                    })?
                    .0;
                for string in set.strings().iter() {
                    assert_eq!(magic_symbols.insert(String::from(string), symbol), None);
                }
                let set = set.code_points().clone();
                Ok((Cow::Borrowed(symbol), set))
            })
            .collect::<Result<BTreeMap<_, _>, DataError>>()?;
        fixed_symbol_assignments.insert(
            magic_symbols.remove("eot").unwrap_or("eot").to_string(),
            SegmenterStateMachine::EOT_SYMBOL,
        );

        let states = sources.read_to_string(&format!("{prefix}States.txt"))?;
        let states = states
            .lines()
            .map(|l| l.split('#').next().unwrap().trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut iter = line.split(';');
                let state = iter.next().unwrap().trim();
                let accepting = iter.next().unwrap().trim();
                let lookahead = iter.next().unwrap().trim();
                let status = iter.next().unwrap().trim();
                (
                    state,
                    (accepting, Some(lookahead).filter(|s| !s.is_empty()), status),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let transitions = sources.read_to_string(&format!("{prefix}Transitions.txt"))?;
        let mut transitions = transitions
            .lines()
            .map(|l| l.split('#').next().unwrap().trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut iter = line.split(';');
                let state = iter.next().unwrap().trim();
                let symbol = iter.next().unwrap().trim();
                let next_state = iter.next().unwrap().trim();
                ((state, symbol), next_state)
            })
            .collect::<BTreeMap<_, _>>();

        let lookaheads = states
            .iter()
            .flat_map(|(_, &(_, lookahead, _))| lookahead)
            .collect::<BTreeSet<_>>();

        let mut pseudo_symbol_map = BTreeMap::<String, (String, ComplexScript)>::new();

        // Create pseudo symbols for complex scripts, allowing the state machine to use the correct
        // dictionary without further lookup.

        let complex_languages = match prefix {
            "LineBreak" => [
                (ComplexScript::Myanmar, "[:sc=Myanmar:]&[:lb=SA:]"),
                (ComplexScript::Khmer, "[:sc=Khmer:]&[:lb=SA:]"),
                (ComplexScript::Lao, "[:sc=Lao:]&[:lb=SA:]"),
                (ComplexScript::Thai, "[:sc=Thai:]&[:lb=SA:]"),
            ]
            .as_slice(),
            "WordBreak" => [
                (ComplexScript::Myanmar, "[:sc=Myanmar:]&[:lb=SA:]"),
                (
                    ComplexScript::ChineseOrJapanese,
                    "[[[:sc=Han:] [:sc=Hiragana:] [:wb=Katakana:] 가-힣] - [:lb=SA:]]",
                ),
                (ComplexScript::Khmer, "[:sc=Khmer:]&[:lb=SA:]"),
                (ComplexScript::Lao, "[:sc=Lao:]&[:lb=SA:]"),
                (ComplexScript::Thai, "[:sc=Thai:]&[:lb=SA:]"),
            ]
            .as_slice(),
            _ => &[],
        }
        .iter()
        .map(|&(l, set)| {
            (
                l,
                icu::properties::unicodeset_parse::parse_unstable(set, self)
                    .unwrap()
                    .0
                    .code_points()
                    .clone(),
            )
        })
        .collect::<Vec<_>>();

        for (&symbol, &non_complex_symbol) in &complex_symbols {
            let set = symbols.get(symbol).unwrap().clone();

            let mut set_builder = CodePointInversionListBuilder::new();
            set_builder.add_set(&set);

            for &(language, ref language_set) in &complex_languages {
                if language_set
                    .iter_ranges()
                    .all(|mut range| range.all(|c| !set.contains32(c)))
                {
                    // no overlap
                    continue;
                }

                set_builder.remove_set(language_set);

                let mut intersection = CodePointInversionListBuilder::new();
                intersection.add_set(language_set);
                for r in set.iter_ranges_complemented() {
                    intersection.remove_range32(r);
                }

                let intersection_symbol = format!("{symbol}_{language:?}");

                pseudo_symbol_map.insert(
                    intersection_symbol.clone(),
                    (non_complex_symbol.into(), language),
                );
                symbols.insert(Cow::Owned(intersection_symbol), intersection.build());
            }

            if symbol != non_complex_symbol {
                let symbol_transitions = transitions
                    .iter()
                    .filter(|&(&(_, s), _)| s == symbol)
                    .map(|(&(before, _), &after)| (before, after))
                    .collect::<BTreeSet<_>>();
                let non_complex_symbol_transitions = transitions
                    .iter()
                    .filter(|&(&(_, s), _)| s == non_complex_symbol)
                    .map(|(&(before, _), &after)| (before, after))
                    .collect::<BTreeSet<_>>();

                if symbol_transitions == non_complex_symbol_transitions {
                    let non_complex_set = symbols.get_mut(non_complex_symbol).unwrap();
                    let mut non_complex_set_builder = CodePointInversionListBuilder::new();
                    non_complex_set_builder.add_set(non_complex_set);
                    non_complex_set_builder.add_set(&set_builder.build());
                    *non_complex_set = non_complex_set_builder.build();

                    symbols.remove(symbol);
                    transitions.retain(|&(_, s), _| s != symbol);
                } else {
                    log::warn!(
                        "{symbol}/{non_complex_symbol}: {:?} != {:?}",
                        symbol_transitions
                            .difference(&non_complex_symbol_transitions)
                            .collect::<Vec<_>>(),
                        non_complex_symbol_transitions
                            .difference(&symbol_transitions)
                            .collect::<Vec<_>>()
                    );
                }
            }
        }

        let mut tailorings = BTreeMap::new();

        for tailoring in sources.list(&format!("{prefix}Tailoring_"))? {
            let tailoring = tailoring.strip_suffix(".txt").unwrap();

            let mut overrides = BTreeMap::<Cow<'static, str>, BTreeSet<char>>::new();

            for line in sources
                .read_to_string(&format!("{prefix}Tailoring_{tailoring}.txt"))?
                .lines()
                .map(|l| l.split('#').next().unwrap().trim())
                .filter(|l| !l.is_empty())
            {
                let mut iter = line.split(';');
                let unicode_set = iter.next().unwrap().trim();
                let target = iter.next().unwrap().trim();

                let set = icu::properties::unicodeset_parse::parse_unstable(unicode_set, self)
                    .map_err(|e| {
                        DataError::custom("unicodeset parse")
                            .with_display_context(&e.fmt_with_source(unicode_set))
                    })?
                    .0;

                let target = icu::properties::unicodeset_parse::parse_unstable(target, self)
                    .map_err(|e| {
                        DataError::custom("unicodeset parse")
                            .with_display_context(&e.fmt_with_source(unicode_set))
                    })?
                    .0;

                let target_symbol = if target.has_strings() {
                    let target = target.strings().iter().next().unwrap();
                    let magic = *magic_symbols.get(target).expect(target);
                    &Cow::Borrowed(magic)
                } else {
                    let target = target.code_points().iter_chars().next().unwrap();
                    symbols
                        .iter()
                        .find(|(_, set)| set.contains(target))
                        .unwrap()
                        .0
                };

                for c in set.code_points().iter_chars() {
                    overrides
                        .entry(target_symbol.clone())
                        .or_default()
                        .insert(c);
                }
            }

            tailorings.insert(
                String::from(tailoring),
                overrides
                    .into_iter()
                    .map(|(k, v)| {
                        let mut builder = CodePointInversionListBuilder::new();
                        v.into_iter().for_each(|c| builder.add_char(c));
                        (k, builder.build())
                    })
                    .collect::<BTreeMap<_, CodePointInversionList>>(),
            );
        }

        // Intersect the symbols with all tailorings' overrides.
        for (tailoring, overrides) in tailorings.clone() {
            for (rule, set) in overrides {
                for (symbol, set2) in symbols.clone().into_iter().collect::<Vec<_>>() {
                    if set.iter_chars().any(|c| set2.contains(c)) {
                        // Overlapping sets. We need to create a new pseudo-symbol.
                        let pseudo_symbol = format!("{symbol}_{tailoring}_{rule}");
                        // Add the intersection as a new symbol.
                        symbols.insert(Cow::Owned(pseudo_symbol.clone()), {
                            let mut builder = CodePointInversionListBuilder::new();
                            builder.add_set(&set);
                            for r in set2.iter_ranges_complemented() {
                                builder.remove_range32(r);
                            }
                            builder.build()
                        });
                        pseudo_symbol_map.insert(pseudo_symbol, {
                            let mut s = &*symbol;
                            // Non-pseudo symbols have Language::Other
                            let mut l = ComplexScript::None;
                            while let Some(&(ref x, y)) = pseudo_symbol_map.get(s) {
                                s = x.as_str();
                                l = y;
                            }
                            (s.to_string(), l)
                        });
                        // Remove the intersection from the root symbol.
                        symbols.insert(symbol, {
                            let mut builder = CodePointInversionListBuilder::new();
                            builder.add_set(&set2);
                            builder.remove_set(&set);
                            builder.build()
                        });
                    }
                }
            }
        }

        // Remove unused symbols
        symbols.retain(|n, set| {
            if pseudo_symbol_map.contains_key(n.as_ref()) {
                // Symbol is a pseudo symbol
                return true;
            }

            if !set.is_empty() {
                // Symbol used in root
                return true;
            }

            if pseudo_symbol_map
                .values()
                .any(|(root_symbol, _)| root_symbol == n.as_ref())
                || tailorings
                    .values()
                    .any(|overrides| overrides.contains_key(n))
            {
                // Symbol is a pseudo symbol target
                return true;
            }

            transitions.retain(|&(_, m), _| m != n);

            false
        });

        let highest_fixed_symbol = fixed_symbol_assignments.values().copied().max().unwrap();
        let symbol_lookup = symbols
            .keys()
            .filter(|&s| {
                !fixed_symbol_assignments.contains_key(s.as_ref())
                    && !pseudo_symbol_map.contains_key(s.as_ref())
            })
            .enumerate()
            .map(|(i, symbol)| {
                (
                    symbol.as_ref(),
                    Symbol::try_from(i + highest_fixed_symbol as usize + 1).unwrap(),
                )
            })
            .chain(
                fixed_symbol_assignments
                    .iter()
                    .map(|(k, v)| (k.as_str(), *v)),
            )
            .collect::<BTreeMap<_, _>>();

        let pseudo_symbol_shift = symbol_lookup.values().copied().max().unwrap() + 1;
        let pseudo_symbol_lookup = pseudo_symbol_map
            .keys()
            .enumerate()
            .map(|(i, k)| {
                (
                    k.as_str(),
                    Symbol::try_from(i + usize::from(pseudo_symbol_shift)).unwrap(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        // Reserve two states for START and TRASH
        assert!(states.len() < usize::from(State::MAX) - 2);
        let state_lookup = core::iter::once("START")
            .chain(states.keys().filter(|&&s| s != "START").copied())
            .enumerate()
            .map(|(i, state)| (state, State::try_from(i).unwrap()))
            .collect::<BTreeMap<_, _>>();
        assert!(lookaheads.len() < 0b11111);
        let lookahead_lookup = lookaheads
            .iter()
            .enumerate()
            .map(|(i, lookahead)| (*lookahead, Lookahead::try_from(i).unwrap()))
            .collect::<BTreeMap<_, _>>();

        use icu::collections::codepointinvlist::CodePointInversionListBuilder;
        use icu::collections::codepointtrie::TrieType;
        use icu_codepointtrie_builder::CodePointTrieBuilder;

        let mut builder = CodePointTrieBuilder::new(0, 0, TrieType::Fast);
        let mut missing_codepoints = CodePointInversionListBuilder::new();
        missing_codepoints.add_set(&CodePointInversionList::all());
        for (symbol, set) in &symbols {
            for range in set.iter_ranges() {
                missing_codepoints.remove_range32(range.clone());
                builder.set_range_value(
                    range.clone(),
                    symbol_lookup
                        .get(&**symbol)
                        .or_else(|| pseudo_symbol_lookup.get(&**symbol))
                        .copied()
                        .unwrap(),
                );
            }
        }
        let missing_codepoints = missing_codepoints.build();
        assert!(missing_codepoints.is_empty(), "{missing_codepoints:?}");
        let symbols = builder.build();

        let tailorings = tailorings
            .into_iter()
            .map(|(tailoring, overrides)| {
                let mut tailored_pseudo_symbol_map = BTreeMap::<u8, (u8, ComplexScript)>::new();

                for (target_symbol, set) in overrides {
                    let target_symbol = symbol_lookup[&*target_symbol];
                    // TODO?
                    let target_language = ComplexScript::None;
                    // The set might cover multiple pseudo symbols
                    for c in set.iter_chars() {
                        let pseudo_symbol = symbols.get(c);
                        let prev = tailored_pseudo_symbol_map
                            .insert(pseudo_symbol, (target_symbol, target_language));
                        // we fragmented the symbols sufficiently above
                        assert!(
                            prev.is_none_or(|p| p == (target_symbol, target_language)),
                            "{prev:?} {target_symbol} {tailoring} {pseudo_symbol} {c:?}"
                        );
                    }
                }

                (tailoring, tailored_pseudo_symbol_map)
            })
            .collect::<BTreeMap<_, _>>();

        let states = states
            .iter()
            .map(|(&state, &(accepting, lookahead, status))| {
                let status = status_lookup(status);
                // This bound comes from Acceptance::to_unaligned
                assert!(status < 0b111);

                let acceptance = match accepting {
                    "Yes" => Acceptance::Accept(status),
                    "No" => Acceptance::Continue,
                    l => Acceptance::Conditional(lookahead_lookup[l], status),
                };

                (
                    state_lookup[state],
                    (acceptance, lookahead.as_ref().map(|l| lookahead_lookup[l])),
                )
            })
            .collect::<BTreeMap<_, _>>()
            .into_values()
            .collect();

        let transitions = transitions
            .iter()
            .map(|((state, symbol), next_state)| {
                (
                    usize::from(state_lookup[state])
                        + state_lookup.len() * usize::from(symbol_lookup[symbol]),
                    *state_lookup.get(next_state).expect(next_state),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let transitions = (0..=*transitions.last_key_value().unwrap().0)
            .map(|i| {
                transitions
                    .get(&i)
                    .copied()
                    .unwrap_or(SegmenterStateMachine::TRASH_STATE)
            })
            .collect();

        let pseudo_symbol_map = pseudo_symbol_map
            .iter()
            .map(|(k, &(ref v, l))| {
                (
                    pseudo_symbol_lookup[k.as_str()],
                    (symbol_lookup[v.as_str()], l),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let tailorings = tailorings
            .into_iter()
            .map(|(tailoring, tailored_pseudo_symbol_map)| {
                let pseudo_symbol_map = pseudo_symbol_map
                    .iter()
                    .map(|(&pseudo_symbol, &root_symbol)| {
                        tailored_pseudo_symbol_map
                            .get(&pseudo_symbol)
                            .copied()
                            .unwrap_or(root_symbol)
                    })
                    .collect::<zerovec::ZeroVec<_>>();
                (
                    tailoring,
                    SegmenterStateMachineOverride { pseudo_symbol_map },
                )
            })
            .collect();

        Ok((
            SegmenterStateMachine {
                transitions,
                symbols,
                states,
                num_lookaheads: lookahead_lookup.len(),
                pseudo_symbol_shift,
                pseudo_symbol_map: pseudo_symbol_map.values().copied().collect(),
            },
            tailorings,
        ))
    }
}

#[cfg(feature = "unstable")]
impl DataProvider<SegmenterBreakLineV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBreakLineV2>, DataError> {
        self.check_req::<SegmenterBreakLineV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakLineV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(self.line_segmenter()?.0.clone()),
        })
    }
}

#[cfg(feature = "unstable")]
impl DataProvider<SegmenterBreakWordV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBreakWordV2>, DataError> {
        self.check_req::<SegmenterBreakWordV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakWordV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(self.word_segmenter()?.0.clone()),
        })
    }
}

#[cfg(feature = "unstable")]
impl DataProvider<SegmenterBreakSentenceV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBreakSentenceV2>, DataError> {
        self.check_req::<SegmenterBreakSentenceV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakSentenceV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(self.sentence_segmenter()?.0.clone()),
        })
    }
}

#[cfg(feature = "unstable")]
impl DataProvider<SegmenterBreakGraphemeClusterV2> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterBreakGraphemeClusterV2>, DataError> {
        self.check_req::<SegmenterBreakGraphemeClusterV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakGraphemeClusterV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(self.grapheme_cluster_segmenter()?.0.clone()),
        })
    }
}

#[cfg(feature = "unstable")]
impl IterableDataProviderCached<SegmenterBreakLineV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

#[cfg(feature = "unstable")]
impl IterableDataProviderCached<SegmenterBreakSentenceV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

#[cfg(feature = "unstable")]
impl IterableDataProviderCached<SegmenterBreakWordV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

#[cfg(feature = "unstable")]
impl IterableDataProviderCached<SegmenterBreakGraphemeClusterV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

#[cfg(feature = "unstable")]
impl DataProvider<SegmenterBreakLineOverrideV2> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterBreakLineOverrideV2>, DataError> {
        self.check_req::<SegmenterBreakLineOverrideV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakLineOverrideV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.line_segmenter()?
                    .1
                    .get(req.id.marker_attributes.as_str())
                    .ok_or_else(|| {
                        DataErrorKind::IdentifierNotFound
                            .with_req(SegmenterBreakLineOverrideV2::INFO, req)
                    })?
                    .clone(),
            ),
        })
    }
}

#[cfg(feature = "unstable")]
impl IterableDataProviderCached<SegmenterBreakLineOverrideV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_marker(SegmenterBreakLineOverrideV2::INFO));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(self
            .line_segmenter()?
            .1
            .keys()
            .map(|s| DataMarkerAttributes::try_from_string(s.clone()).unwrap())
            .map(DataIdentifierCow::from_marker_attributes_owned)
            .collect())
    }
}

#[cfg(feature = "unstable")]
impl DataProvider<SegmenterBreakSentenceOverrideV2> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterBreakSentenceOverrideV2>, DataError> {
        self.check_req::<SegmenterBreakSentenceOverrideV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakSentenceOverrideV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(
                self.sentence_segmenter()?
                    .1
                    .get(&req.id.locale.to_string())
                    .ok_or_else(|| {
                        DataErrorKind::IdentifierNotFound
                            .with_req(SegmenterBreakSentenceOverrideV2::INFO, req)
                    })?
                    .clone(),
            ),
        })
    }
}

#[cfg(feature = "unstable")]
impl IterableDataProviderCached<SegmenterBreakSentenceOverrideV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_marker(SegmenterBreakSentenceOverrideV2::INFO));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(self
            .sentence_segmenter()?
            .1
            .keys()
            .map(|s| icu::locale::Locale::try_from_str(s).unwrap().into())
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_grapheme_cluster_data() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<SegmenterBreakGraphemeClusterV1> = provider
            .load(Default::default())
            .expect("Loading should succeed!");
        assert_eq!(
            response.payload.get().complex_property,
            127,
            "Grapheme cluster data doesn't handle SA"
        );
    }

    #[test]
    fn load_line_data() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<SegmenterBreakLineV1> = provider
            .load(Default::default())
            .expect("Loading should succeed!");
        let data = response.payload.get();
        // Note: The following match statement had been used in line.rs:
        //
        // match codepoint {
        //     0x20000..=0x2fffd => ID,
        //     0x30000..=0x3fffd => ID,
        //     0xe0001 => CM,
        //     0xe0020..=0xe007f => CM,
        //     0xe0100..=0xe01ef => CM,
        //     _ => XX,
        // }

        const CM: u8 = 14;
        const XX: u8 = 52;
        const ID: u8 = 25;

        assert_eq!(data.property_table.get32(0x20000), ID);
        assert_eq!(data.property_table.get32(0x3fffd), ID);
        assert_eq!(data.property_table.get32(0xd0000), XX);
        assert_eq!(data.property_table.get32(0xe0001), CM);
        assert_eq!(data.property_table.get32(0xe0020), CM);
    }

    #[test]
    #[should_panic]
    fn missing_locale_data() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<SegmenterBreakSentenceOverrideV1> = provider
            .load(Default::default())
            .expect("Loading should succeed!");
        response.payload.get();
    }
    // TODO: Add loading override table data. But no locales in testdata.
}
