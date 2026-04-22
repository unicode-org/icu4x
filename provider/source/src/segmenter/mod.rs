// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by built-in segmentation data.

#![cfg_attr(
    not(any(feature = "use_wasm", feature = "use_icu4c")),
    allow(dead_code, unused_imports)
)]

use crate::source::{include_files, SerdeCache, UnicodeCache};
#[cfg(feature = "unstable")]
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::properties::{
    props::{
        EastAsianWidth, GeneralCategory, GraphemeClusterBreak, IndicConjunctBreak, LineBreak,
        Script, SentenceBreak, WordBreak,
    },
    CodePointMapData, CodePointMapDataBorrowed, CodePointSetData,
};
use icu::segmenter::options::WordType;
use icu::segmenter::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
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
    use icu::properties::{props::ExtendedPictographic, PropertyParser};
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
                        if prop == WordBreak::MidLetter
                            && (range.contains(&0x003a)
                                || range.contains(&0xfe55)
                                || range.contains(&0xff1a))
                        {
                            // UAX29 defines the colon as MidLetter, but ICU4C's
                            // English data doesn't.
                            // See https://unicode-org.atlassian.net/browse/ICU-22112
                            //
                            // TODO: We have to consider this definition from CLDR instead.
                            for ch in
                                range.filter(|ch| *ch != 0x003a && *ch != 0xfe55 && *ch != 0xff1a)
                            {
                                properties_trie.set_value(ch, property_index);
                            }
                        } else if prop == WordBreak::Extend {
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
        if let Some(left) = &p.left {
            if let Some(right) = &p.right {
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
                // UAX29 defines the colon as MidLetter, but ICU4C's
                // English data doesn't.
                // See https://unicode-org.atlassian.net/browse/ICU-22112
                //
                // TODO: We have to consider this definition from CLDR instead.
                "word" if p.name == "MidLetter" => {
                    properties_trie.set_value(0x003a, property_index);
                    properties_trie.set_value(0xfe55, property_index);
                    properties_trie.set_value(0xff1a, property_index);
                }

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
        provider.icuexport_paths = Some(std::sync::Arc::new(SerdeCache::new(include_files!(
            "../../data/segmenter/icuexportdata74/";
            "uprops/small/ea.toml",
            "uprops/small/gc.toml",
            "uprops/small/gcm.toml",
            "uprops/small/lb.toml",
        ))));
        provider
    })
}

implement!(SegmenterBreakLineV1, "line.toml", |_| unicode_15_1());
implement!(SegmenterBreakGraphemeClusterV1, "grapheme.toml", |s| s);
implement!(SegmenterBreakWordV1, "word.toml", |s| s);
implement!(SegmenterBreakSentenceV1, "sentence.toml", |s| s);
implement_override!(SegmenterBreakWordOverrideV1, "word.toml", ["fi", "sv"]);
implement_override!(SegmenterBreakSentenceOverrideV1, "sentence.toml", ["el"]);

#[cfg(feature = "unstable")]
impl DataProvider<SegmenterBreakLineV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBreakLineV2>, DataError> {
        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakLineV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        {
            use icu::collections::codepointtrie::TrieType;
            use icu_codepointtrie_builder::CodePointTrieBuilder;
            use std::collections::{BTreeMap, BTreeSet};

            self.check_req::<SegmenterBreakLineV2>(req)?;

            let classes = include_str!("../../data/segmenter/neo/LineBreakClasses.txt")
                .lines()
                .map(|l| l.split('#').next().unwrap().trim())
                .filter(|l| !l.is_empty())
                .map(|line| {
                    let mut iter = line.split(';');
                    let class = iter.next().unwrap().trim();
                    let unicode_set = iter.next().unwrap().trim();

                    let set = icu::properties::unicodeset_parse::parse_unstable(
                        unicode_set,
                        unicode_15_1(),
                    )
                    .map_err(|e| DataError::custom("unicodeset parse").with_debug_context(&e))?
                    .0;
                    Ok((class, set))
                })
                .collect::<Result<BTreeMap<_, _>, DataError>>()?;
            let states = include_str!("../../data/segmenter/neo/LineBreakStates.txt")
                .lines()
                .map(|l| l.split('#').next().unwrap().trim())
                .filter(|l| !l.is_empty())
                .map(|line| {
                    let mut iter = line.split(';');
                    let state = iter.next().unwrap().trim();
                    let accepting = iter.next().unwrap().trim();
                    let lookahead = iter.next().unwrap().trim();
                    (
                        state,
                        (accepting, Some(lookahead).filter(|s| !s.is_empty())),
                    )
                })
                .collect::<BTreeMap<_, _>>();
            let transitions = include_str!("../../data/segmenter/neo/LineBreakTransitions.txt")
                .lines()
                .map(|l| l.split('#').next().unwrap().trim())
                .filter(|l| !l.is_empty())
                .map(|line| {
                    let mut iter = line.split(';');
                    let state = iter.next().unwrap().trim();
                    let class = iter.next().unwrap().trim();
                    let next_state = iter.next().unwrap().trim();
                    ((state, class), next_state)
                })
                .collect::<BTreeMap<_, _>>();

            let lookaheads = states
                .iter()
                .flat_map(|(_, &(_, lookahead))| lookahead)
                .collect::<BTreeSet<_>>();

            // Reserve one class for EOT
            assert!(classes.len() < usize::from(Class::MAX) - 1);
            // Reserve two states for START and TRASH
            assert!(states.len() < usize::from(State::MAX) - 2);
            // Reserve three values for Acceptance::{Accept, Continue, AcceptMandatory}
            assert!(lookaheads.len() < usize::from(Lookahead::MAX) - 3);
            // Check invariants of the start state
            assert_eq!(states["START"], ("No", None));

            let class_lookup = core::iter::once("eot")
                .chain(classes.keys().filter(|&&s| s != "eot").copied())
                .enumerate()
                .map(|(i, class)| (class, Class::try_from(i).unwrap()))
                .collect::<BTreeMap<_, _>>();

            let state_lookup = core::iter::once("START")
                .chain(states.keys().filter(|&&s| s != "START").copied())
                .enumerate()
                .map(|(i, state)| (state, State::try_from(i).unwrap()))
                .collect::<BTreeMap<_, _>>();

            let lookahead_lookup = lookaheads
                .iter()
                .enumerate()
                .map(|(i, lookahead)| (*lookahead, Lookahead::try_from(i).unwrap()))
                .collect::<BTreeMap<_, _>>();

            let mut builder = CodePointTrieBuilder::new(0, 0, TrieType::Fast);
            for (&class, set) in &classes {
                for range in set.code_points().iter_ranges() {
                    builder.set_range_value(range.clone(), class_lookup[class]);
                }
            }
            let classes = builder.build();

            let lb = CodePointMapData::<LineBreak>::try_new_unstable(unicode_15_1()).unwrap();
            let lb = lb.as_borrowed();

            let mandatory_break_classes = [
                LineBreak::CarriageReturn,
                LineBreak::LineFeed,
                LineBreak::MandatoryBreak,
                LineBreak::NextLine,
            ]
            .into_iter()
            .flat_map(|l| lb.iter_ranges_for_value(l))
            .flatten()
            .map(|c| classes.get32(c))
            .collect::<BTreeSet<_>>();

            let mandatory_break_states = transitions
                .iter()
                .filter_map(|(&(_, class), &right)| {
                    mandatory_break_classes
                        .contains(&class_lookup[class])
                        .then_some(right)
                })
                .inspect(|&state| {
                    // all incoming transitions are mandatory classes
                    assert!(transitions
                        .iter()
                        .all(|(&(_, class), &right)| right != state
                            || mandatory_break_classes.contains(&class_lookup[class])));

                    // the state is unconditionally accepting
                    assert_eq!(states[state].0, "Yes");

                    // the state can't be reached by lookahead
                    assert_eq!(states[state].1, None);
                })
                .collect::<BTreeSet<_>>();

            let states = states
                .iter()
                .map(|(&state, &(accepting, lookahead))| {
                    (
                        state_lookup[state],
                        (
                            match accepting {
                                "Yes" if mandatory_break_states.contains(state) => {
                                    Acceptance::AcceptMandatory
                                }
                                "Yes" => Acceptance::Accept,
                                "No" => Acceptance::Continue,
                                l => Acceptance::Conditional(lookahead_lookup[l]),
                            },
                            lookahead.as_ref().map(|l| lookahead_lookup[l]),
                        ),
                    )
                })
                .collect::<BTreeMap<_, _>>()
                .into_values()
                .collect();

            let transitions = transitions
                .iter()
                .map(|((state, class), next_state)| {
                    (
                        usize::from(state_lookup[state])
                            + state_lookup.len() * usize::from(class_lookup[class]),
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

            Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(SegmenterStateMachine {
                    transitions,
                    classes,
                    states,
                    num_lookaheads: lookahead_lookup.len(),
                }),
            })
        }
    }
}

#[cfg(feature = "unstable")]
impl IterableDataProviderCached<SegmenterBreakLineV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
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
