// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by built-in segmentation data.

#![allow(dead_code)]
#![allow(unused_imports)]

use crate::SourceDataProvider;
use icu::collections::codepointtrie;
use icu::properties::{
    maps, sets, CanonicalCombiningClass, EastAsianWidth, GeneralCategory, GraphemeClusterBreak,
    IndicSyllabicCategory, LineBreak, Script, SentenceBreak, WordBreak,
};
use icu::segmenter::provider::*;
use icu::segmenter::WordType;
use icu_codepointtrie_builder::{CodePointTrieBuilder, CodePointTrieBuilderData};
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::OnceLock;
use zerovec::ZeroVec;

mod dictionary;
mod lstm;

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
#[derive(serde::Deserialize, Debug)]
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
) -> RuleBreakDataV2<'static> {
    let segmenter = provider
        .icuexport()
        .unwrap()
        .read_and_parse_toml::<SegmenterRuleTable>(rules_file)
        .expect("The data should be valid!");

    let data = maps::load_word_break(provider).expect("The data should be valid!");
    let wb = data.as_borrowed();

    let data = maps::load_grapheme_cluster_break(provider).expect("The data should be valid!");
    let gb = data.as_borrowed();

    let data = maps::load_sentence_break(provider).expect("The data should be valid!");
    let sb = data.as_borrowed();

    let data = maps::load_line_break(provider).expect("The data should be valid!");
    let lb = data.as_borrowed();

    let data = maps::load_east_asian_width(provider).expect("The data should be valid!");
    let eaw = data.as_borrowed();

    let data = maps::load_general_category(provider).expect("The data should be valid!");
    let gc = data.as_borrowed();

    let data = maps::load_script(provider).expect("The data should be valid");
    let script = data.as_borrowed();

    let data = sets::load_extended_pictographic(provider).expect("The data should be valid!");
    let extended_pictographic = data.as_borrowed();

    let data = maps::load_indic_syllabic_category(provider).expect("The data should be valid!");
    let insc = data.as_borrowed();

    let data = maps::load_canonical_combining_class(provider).expect("The data should be valid!");
    let ccc = data.as_borrowed();

    let data =
        GraphemeClusterBreak::get_name_to_enum_mapper(provider).expect("The data should be vaild!");
    let gcb_name_to_enum = data.as_borrowed();

    let data = LineBreak::get_name_to_enum_mapper(provider).expect("The data should be vaild!");
    let lb_name_to_enum = data.as_borrowed();

    let data = SentenceBreak::get_name_to_enum_mapper(provider).expect("The data should be vaild!");
    let sb_name_to_enum = data.as_borrowed();

    let data = WordBreak::get_name_to_enum_mapper(provider).expect("The data should be vaild!");
    let wb_name_to_enum = data.as_borrowed();

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

    fn is_cjk_fullwidth(
        eaw: maps::CodePointMapDataBorrowed<EastAsianWidth>,
        codepoint: u32,
    ) -> bool {
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

    let mut properties_map = vec![0; CODEPOINT_TABLE_LEN];
    let mut properties_names = Vec::<String>::new();
    let mut simple_properties_count = 0;

    properties_names.push("Unknown".to_string());
    simple_properties_count += 1;

    for p in &segmenter.tables {
        let property_index = if !properties_names.contains(&p.name) {
            properties_names.push(p.name.clone());
            (properties_names.len() - 1).try_into().unwrap()
        } else {
            continue;
        };

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
                        // SA/CJ property is within 0..U+0x40000
                        for c in 0..0x40000 {
                            if lb.get32(c) == LineBreak::ComplexContext {
                                properties_map[c as usize] = property_index
                            } else if let Some(c) = char::from_u32(c) {
                                match script.get(c) {
                                    Script::Han | Script::Hiragana => {
                                        properties_map[c as usize] = property_index;
                                    }

                                    _ => {}
                                }
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
                    for c in 0..(CODEPOINT_TABLE_LEN as u32) {
                        if wb.get32(c) == prop {
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

                    // The Indic_Conjunct_Break property is separate from the Grapheme_Cluster_Break property.
                    // See https://unicode.org/reports/tr44/#Indic_Conjunct_Break
                    if p.name == "InCBConsonant" || p.name == "InCBLinker" || p.name == "InCBExtend"
                    {
                        let gcb_extend = gcb_name_to_enum
                            .get_loose("Extend")
                            .expect("property name should be valid!");

                        for i in 0..(CODEPOINT_TABLE_LEN as u32) {
                            if let Some(c) = char::from_u32(i) {
                                let insc_value = insc.get(c);
                                let sc = script.get(c);
                                let is_gb9c_script = sc == Script::Bengali
                                    || sc == Script::Devanagari
                                    || sc == Script::Gujarati
                                    || sc == Script::Malayalam
                                    || sc == Script::Oriya
                                    || sc == Script::Telugu;
                                let is_incb_consonant = insc_value
                                    == IndicSyllabicCategory::Consonant
                                    && is_gb9c_script;
                                let is_incb_linker =
                                    insc_value == IndicSyllabicCategory::Virama && is_gb9c_script;
                                // InCB = Linker or InCB = Consonant
                                if (p.name == "InCBConsonant" && is_incb_consonant)
                                    || (p.name == "InCBLinker" && is_incb_linker)
                                    // ZWJ is InCB=Extend, but is in a different GCB class anyway so
                                    // it needs to be special-cased in the tables.
                                    // NOTE(eggrobin): UAX #44, Version 15.1, instead excludes based
                                    // on InSC.
                                    // I believe that to be a defect in that version of Unicode.
                                    // This has been brought to the attention of the Properties and
                                    // Algorithms Group.
                                    || (p.name == "InCBExtend"
                                        && (gb.get32(i) == gcb_extend
                                            && ccc.get32(i) != CanonicalCombiningClass::NotReordered
                                            && !is_incb_consonant
                                            && !is_incb_linker))
                                {
                                    properties_map[c as usize] = property_index;
                                }
                            }
                        }
                        continue;
                    }

                    let prop = gcb_name_to_enum
                        .get_loose(&p.name)
                        .expect("property name should be valid!");
                    for c in 0..(CODEPOINT_TABLE_LEN as u32) {
                        if gb.get32(c) == prop {
                            properties_map[c as usize] = property_index;
                        }
                    }
                    continue;
                }

                "sentence" => {
                    let prop = sb_name_to_enum
                        .get_loose(&p.name)
                        .expect("property name should be valid!");
                    for c in 0..(CODEPOINT_TABLE_LEN as u32) {
                        if sb.get32(c) == prop {
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
                        for i in 0..(CODEPOINT_TABLE_LEN as u32) {
                            match lb.get32(i) {
                                LineBreak::OpenPunctuation => {
                                    if (p.name == "OP_OP30"
                                        && (eaw.get32(i) != EastAsianWidth::Fullwidth
                                            && eaw.get32(i) != EastAsianWidth::Halfwidth
                                            && eaw.get32(i) != EastAsianWidth::Wide))
                                        || (p.name == "OP_EA"
                                            && (eaw.get32(i) == EastAsianWidth::Fullwidth
                                                || eaw.get32(i) == EastAsianWidth::Halfwidth
                                                || eaw.get32(i) == EastAsianWidth::Wide))
                                    {
                                        properties_map[i as usize] = property_index;
                                    }
                                }

                                LineBreak::CloseParenthesis => {
                                    // CP_EA is unused on the latest spec.
                                    if p.name == "CP_EA"
                                        && (eaw.get32(i) == EastAsianWidth::Fullwidth
                                            || eaw.get32(i) == EastAsianWidth::Halfwidth
                                            || eaw.get32(i) == EastAsianWidth::Wide)
                                    {
                                        properties_map[i as usize] = property_index;
                                    }
                                }

                                LineBreak::Ideographic => {
                                    if p.name == "ID_CN"
                                        && gc.get32(i) == GeneralCategory::Unassigned
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

                    let prop = lb_name_to_enum
                        .get_loose(&p.name)
                        .expect("property name should be valid!");
                    for c in 0..(CODEPOINT_TABLE_LEN as u32) {
                        if lb.get32(c) == prop {
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

    RuleBreakDataV2 {
        property_table: CodePointTrieBuilder {
            data: CodePointTrieBuilderData::ValuesByCodePoint(&properties_map),
            default_value: 0,
            error_value: 0,
            trie_type: match trie_type {
                crate::TrieType::Fast => codepointtrie::TrieType::Fast,
                crate::TrieType::Small => codepointtrie::TrieType::Small,
            },
        }
        .build(),
        break_state_table: break_state_table
            .into_iter()
            // All states are initialized
            .map(|o| o.unwrap())
            .collect(),
        word_type_table: if segmenter.segmenter_type == "word" {
            segmenter
                .tables
                .iter()
                .map(|p| match &*p.name {
                    "Numeric" => WordType::Number,
                    "ALetter" | "Hebrew_Letter" | "ExtendNumLet" | "Katakana" | "SA" => {
                        WordType::Letter
                    }
                    _ => WordType::None,
                })
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
    }
}

macro_rules! implement {
    ($marker:ident, $rules:literal) => {
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
                        &hardcoded_segmenter_provider(),
                        $rules,
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
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    }
}

fn hardcoded_segmenter_provider() -> SourceDataProvider {
    use crate::{
        source::{AbstractFs, SerdeCache},
        SourceDataProvider,
    };
    // Singleton so that all instantiations share the same cache.
    static SINGLETON: OnceLock<SourceDataProvider> = OnceLock::new();
    SINGLETON
        .get_or_init(|| {
            let mut provider = SourceDataProvider::new_custom();
            provider.icuexport_paths =
                Some(std::sync::Arc::new(SerdeCache::new(AbstractFs::Memory(
                    [
                        (
                            "uprops/small/ccc.toml",
                            include_bytes!("../../data/segmenter/uprops/small/ccc.toml").as_slice(),
                        ),
                        (
                            "uprops/small/ea.toml",
                            include_bytes!("../../data/segmenter/uprops/small/ea.toml").as_slice(),
                        ),
                        (
                            "uprops/small/ExtPict.toml",
                            include_bytes!("../../data/segmenter/uprops/small/ExtPict.toml")
                                .as_slice(),
                        ),
                        (
                            "uprops/small/gc.toml",
                            include_bytes!("../../data/segmenter/uprops/small/gc.toml").as_slice(),
                        ),
                        (
                            "uprops/small/GCB.toml",
                            include_bytes!("../../data/segmenter/uprops/small/GCB.toml").as_slice(),
                        ),
                        (
                            "uprops/small/InSC.toml",
                            include_bytes!("../../data/segmenter/uprops/small/InSC.toml")
                                .as_slice(),
                        ),
                        (
                            "uprops/small/lb.toml",
                            include_bytes!("../../data/segmenter/uprops/small/lb.toml").as_slice(),
                        ),
                        (
                            "uprops/small/SB.toml",
                            include_bytes!("../../data/segmenter/uprops/small/SB.toml").as_slice(),
                        ),
                        (
                            "uprops/small/sc.toml",
                            include_bytes!("../../data/segmenter/uprops/small/sc.toml").as_slice(),
                        ),
                        (
                            "uprops/small/WB.toml",
                            include_bytes!("../../data/segmenter/uprops/small/WB.toml").as_slice(),
                        ),
                        (
                            "segmenter/grapheme.toml",
                            include_bytes!("../../data/segmenter/grapheme.toml").as_slice(),
                        ),
                        (
                            "segmenter/line.toml",
                            include_bytes!("../../data/segmenter/line.toml").as_slice(),
                        ),
                        (
                            "segmenter/sentence.toml",
                            include_bytes!("../../data/segmenter/sentence.toml").as_slice(),
                        ),
                        (
                            "segmenter/word.toml",
                            include_bytes!("../../data/segmenter/word.toml").as_slice(),
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ))));
            provider
        })
        .clone()
}

implement!(LineBreakDataV2Marker, "segmenter/line.toml");
implement!(GraphemeClusterBreakDataV2Marker, "segmenter/grapheme.toml");
implement!(WordBreakDataV2Marker, "segmenter/word.toml");
implement!(SentenceBreakDataV2Marker, "segmenter/sentence.toml");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_grapheme_cluster_data() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<GraphemeClusterBreakDataV2Marker> = provider
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
        let response: DataResponse<LineBreakDataV2Marker> = provider
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

        const CM: u8 = 10;
        const XX: u8 = 45;
        const ID: u8 = 21;

        assert_eq!(data.property_table.get32(0x20000), ID);
        assert_eq!(data.property_table.get32(0x3fffd), ID);
        assert_eq!(data.property_table.get32(0xd0000), XX);
        assert_eq!(data.property_table.get32(0xe0001), CM);
        assert_eq!(data.property_table.get32(0xe0020), CM);
    }
}
