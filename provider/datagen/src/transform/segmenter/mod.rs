// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains implementations of the [`ICU4X`] [data provider] interface
//! based on Unicode properties and TOML files implementing [Unicode Standard Annex #14][UAX14] and
//! [Unicode Standard Annex #29][UAX29] breaking rules.
//!
//! This module exports feature-specific providers. Use [`crate::create_datagen_provider`]
//! for an all-inclusive provider.
//!
//! **Important:** This data provider implementation is not optimized
//! for production use. Read more in the [data provider] docs.
//!
//! [`ICU4X`]: ../icu/index.html
//! [data provider]: icu_provider
//! [UAX14]: https://www.unicode.org/reports/tr14/
//! [UAX29]: https://www.unicode.org/reports/tr29/

use crate::transform::uprops::{
    BinaryPropertyUnicodeSetDataProvider, EnumeratedPropertyCodePointTrieProvider,
};
use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use icu_codepointtrie_builder::{CodePointTrieBuilder, CodePointTrieBuilderData};
use icu_properties::{
    maps, sets, EastAsianWidth, GeneralCategory, GraphemeClusterBreak, LineBreak, SentenceBreak,
    WordBreak,
};
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use icu_segmenter::symbols::*;
use icu_segmenter::*;
use std::fmt::Debug;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use zerovec::ZeroVec;

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
// [[tables]]
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

fn is_cjk_fullwidth(eaw: &CodePointTrie<EastAsianWidth>, codepoint: u32) -> bool {
    matches!(
        eaw.get(codepoint),
        EastAsianWidth::Ambiguous | EastAsianWidth::Fullwidth | EastAsianWidth::Wide
    )
}

/// A data provider reading from segmenter rule files.
#[derive(Debug)]
pub struct SegmenterRuleProvider {
    source: SourceData,
}

impl From<&SourceData> for SegmenterRuleProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

impl SegmenterRuleProvider {
    fn build_rule_data_path(&self, key: ResourceKey) -> Result<PathBuf, DataError> {
        let file_name = key
            .get_path()
            .split(&['/', '@'])
            .nth(1)
            .expect("ResourceKey format should be valid!");
        let mut data_path = self.source.get_segmenter_data_root()?.join(file_name);
        data_path.set_extension("toml");
        Ok(data_path)
    }

    fn load_rule_data(&self, key: ResourceKey) -> Result<SegmenterRuleTable, DataError> {
        let path = self.build_rule_data_path(key)?;
        let mut file = File::open(&path).map_err(|e| {
            DataErrorKind::Io(e.kind())
                .with_key(key)
                .with_str_context("Failed to open rule data file!")
        })?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).map_err(|e| {
            DataErrorKind::Io(e.kind())
                .with_key(key)
                .with_str_context("Failed to read rule data file!")
        })?;
        toml::from_slice(&buffer).map_err(|_| {
            DataErrorKind::MissingPayload
                .with_key(key)
                .with_str_context("Failed to parse rule data file!")
        })
    }

    fn generate_rule_break_data(
        &self,
        key: ResourceKey,
    ) -> Result<RuleBreakDataV1<'static>, DataError> {
        let segmenter = self.load_rule_data(key)?;

        // Load enumerate Unicode property dependencies.
        let cp_map_provider = EnumeratedPropertyCodePointTrieProvider::from(&self.source);

        let payload = maps::get_word_break(&cp_map_provider).expect("The data should be valid!");
        let wb = &payload.get().code_point_trie;

        let payload =
            maps::get_grapheme_cluster_break(&cp_map_provider).expect("The data should be valid!");
        let gb = &payload.get().code_point_trie;

        let payload =
            maps::get_sentence_break(&cp_map_provider).expect("The data should be valid!");
        let sb = &payload.get().code_point_trie;

        let payload = maps::get_line_break(&cp_map_provider).expect("The data should be valid!");
        let lb = &payload.get().code_point_trie;

        let payload =
            maps::get_east_asian_width(&cp_map_provider).expect("The data should be valid!");
        let eaw = &payload.get().code_point_trie;

        let payload =
            maps::get_general_category(&cp_map_provider).expect("The data should be valid!");
        let gc = &payload.get().code_point_trie;

        // Load binary Unicode property dependencies.
        let uniset_provider = BinaryPropertyUnicodeSetDataProvider::from(&self.source);

        let payload =
            sets::get_extended_pictographic(&uniset_provider).expect("The data should be valid!");
        let extended_pictographic = &payload.get().inv_list;

        // As of Unicode 14.0.0, the break property and the largest codepoint defined in UCD are
        // summarized in the following list. See details in the property txt in
        // https://www.unicode.org/Public/14.0.0/ucd/auxiliary/.
        //
        // Grapheme Break Property: U+E0FFF ; Control
        // Sentence Break Property: U+E01EF ; Extend
        // Word Break Property: U+E01EF ; Extend
        //
        // The table length should be large enough to contain all codepoints.
        const UAX29_CODEPOINT_TABLE_LEN: usize = 0xE1000;

        // The property values of codepoints >= U+0x20000 are built into the line segmenter.
        const UAX14_CODEPOINT_TABLE_LEN: usize = 0x20000;

        let mut properties_map = if segmenter.segmenter_type == "line" {
            vec![0; UAX14_CODEPOINT_TABLE_LEN]
        } else {
            vec![0; UAX29_CODEPOINT_TABLE_LEN]
        };
        let mut properties_names = Vec::<String>::new();
        let mut simple_properties_count = 0;

        properties_names.push("Unknown".to_string());
        simple_properties_count += 1;

        for p in &segmenter.tables {
            let property_index = if !properties_names.contains(&p.name) {
                properties_names.push(p.name.clone());
                (properties_names.len() - 1) as u8
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
                            // SA property is within 0..U+0x20000
                            for c in 0..0x20000 {
                                if lb.get(c) == LineBreak::ComplexContext {
                                    properties_map[c as usize] = property_index
                                }
                            }
                            continue;
                        }

                        let prop = get_word_segmenter_value_from_name(&*p.name);
                        for c in 0..(UAX29_CODEPOINT_TABLE_LEN as u32) {
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
                        for c in 0..(UAX29_CODEPOINT_TABLE_LEN as u32) {
                            if gb.get(c) == prop {
                                properties_map[c as usize] = property_index;
                            }
                        }
                        continue;
                    }

                    "sentence" => {
                        let prop = get_sentence_segmenter_value_from_name(&*p.name);
                        for c in 0..(UAX29_CODEPOINT_TABLE_LEN as u32) {
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
                                        if p.name == "ID_CN"
                                            && gc.get(i) == GeneralCategory::Unassigned
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
                    if c > UAX29_CODEPOINT_TABLE_LEN {
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

        for rule in &segmenter.rules {
            let break_state = if let Some(state) = rule.break_state {
                if state {
                    BREAK_RULE
                } else {
                    KEEP_RULE
                }
            } else {
                NOT_MATCH_RULE
            };

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

        // Return 127 if the complex language isn't handled.
        let complex_property = get_index_from_name(&properties_names, "SA").unwrap_or(127);

        // Generate a CodePointTrie from properties_map
        let property_trie: CodePointTrie<u8> = CodePointTrieBuilder {
            data: CodePointTrieBuilderData::ValuesByCodePoint(&properties_map),
            default_value: 0,
            error_value: 0,
            trie_type: self.source.trie_type(),
        }
        .build();

        Ok(RuleBreakDataV1 {
            property_table: RuleBreakPropertyTable(property_trie),
            break_state_table: RuleBreakStateTable(ZeroVec::Owned(break_state_table)),
            property_count: property_length as u8,
            last_codepoint_property: (simple_properties_count - 1) as i8,
            sot_property: (property_length - 2) as u8,
            eot_property: (property_length - 1) as u8,
            complex_property: complex_property as u8,
        })
    }
}

impl ResourceProvider<LineBreakDataV1Marker> for SegmenterRuleProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<LineBreakDataV1Marker>, DataError> {
        let break_data = self.generate_rule_break_data(LineBreakDataV1Marker::KEY)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(break_data)),
        })
    }
}

impl ResourceProvider<GraphemeClusterBreakDataV1Marker> for SegmenterRuleProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<GraphemeClusterBreakDataV1Marker>, DataError> {
        let break_data = self.generate_rule_break_data(GraphemeClusterBreakDataV1Marker::KEY)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(break_data)),
        })
    }
}

impl ResourceProvider<WordBreakDataV1Marker> for SegmenterRuleProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<WordBreakDataV1Marker>, DataError> {
        let break_data = self.generate_rule_break_data(WordBreakDataV1Marker::KEY)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(break_data)),
        })
    }
}

impl ResourceProvider<SentenceBreakDataV1Marker> for SegmenterRuleProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<SentenceBreakDataV1Marker>, DataError> {
        let break_data = self.generate_rule_break_data(SentenceBreakDataV1Marker::KEY)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(break_data)),
        })
    }
}

icu_provider::impl_dyn_provider!(
    SegmenterRuleProvider,
    [
        LineBreakDataV1Marker,
        GraphemeClusterBreakDataV1Marker,
        WordBreakDataV1Marker,
        SentenceBreakDataV1Marker,
    ],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

impl IterableResourceProvider<LineBreakDataV1Marker> for SegmenterRuleProvider {
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

impl IterableResourceProvider<GraphemeClusterBreakDataV1Marker> for SegmenterRuleProvider {
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

impl IterableResourceProvider<WordBreakDataV1Marker> for SegmenterRuleProvider {
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

impl IterableResourceProvider<SentenceBreakDataV1Marker> for SegmenterRuleProvider {
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_grapheme_cluster_data() {
        let provider = SegmenterRuleProvider::from(&SourceData::for_test());
        let payload: DataPayload<GraphemeClusterBreakDataV1Marker> = provider
            .load_resource(&DataRequest::default())
            .expect("Loading should succeed!")
            .take_payload()
            .expect("Data should be present!");
        let data: &RuleBreakDataV1 = payload.get();
        assert_eq!(
            data.complex_property, 127,
            "Grapheme cluster data doesn't handle SA"
        );
    }
}
