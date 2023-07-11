// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;
use std::marker::PhantomData;

use crate::transform::cldr::cldr_serde;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_properties::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use itertools::Itertools;

#[derive(Debug)]
struct AnnotatedResource<'a, M: DataMarker>(
    &'a cldr_serde::exemplar_chars::Resource,
    PhantomData<M>,
);

macro_rules! exemplar_chars_impls {
    ($data_marker_name:ident, $cldr_serde_field_name:ident) => {
        impl DataProvider<$data_marker_name> for crate::DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$data_marker_name>, DataError> {
                self.check_req::<$data_marker_name>(req)?;
                let langid = req.locale.get_langid();

                let data: &cldr_serde::exemplar_chars::Resource = self
                    .source
                    .cldr()?
                    .misc()
                    .read_and_parse(&langid, "characters.json")?;

                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: Some(DataPayload::from_owned(
                        PropertyUnicodeSetV1::try_from(AnnotatedResource::<$data_marker_name>(
                            &data,
                            PhantomData,
                        ))
                        .map_err(|e| {
                            DataError::custom("data for exemplar characters")
                                .with_display_context(&e)
                        })?,
                    )),
                })
            }
        }

        impl IterableDataProvider<$data_marker_name> for crate::DatagenProvider {
            fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
                Ok(self.filter_data_locales(
                    self.source
                        .cldr()?
                        .misc()
                        .list_langs()?
                        .map(DataLocale::from)
                        .collect(),
                ))
            }
        }

        impl<'a> TryFrom<AnnotatedResource<'a, $data_marker_name>>
            for PropertyUnicodeSetV1<'static>
        {
            type Error = DataError;
            fn try_from(
                annotated_resource: AnnotatedResource<$data_marker_name>,
            ) -> Result<Self, Self::Error> {
                let source_data_chars: Option<&String> = annotated_resource
                    .0
                    .main
                    .0
                    .iter()
                    .next()
                    .unwrap()
                    .1
                    .characters
                    .$cldr_serde_field_name
                    .as_ref();

                let chars_str = match source_data_chars {
                    Some(chars_str) => chars_str,
                    None => {
                        log::warn!(concat!(
                            "Data missing for ",
                            stringify!($cldr_serde_field_name),
                            " set exemplar characters"
                        ));
                        "[]"
                    }
                };
                Ok(string_to_prop_unicodeset(chars_str))
            }
        }
    };
}

exemplar_chars_impls!(ExemplarCharactersMainV1Marker, main);
exemplar_chars_impls!(ExemplarCharactersAuxiliaryV1Marker, auxiliary);
exemplar_chars_impls!(ExemplarCharactersPunctuationV1Marker, punctuation);
exemplar_chars_impls!(ExemplarCharactersNumbersV1Marker, numbers);
exemplar_chars_impls!(ExemplarCharactersIndexV1Marker, index);

/// In the occurrence of subsequences that are used to represent character literals,
/// like "\\\\:" or "\\\\\\\\[", excise the subsequence from the input string
/// and prepopulate the set with the corresponding characters like ":" and "[".
/// But since Unicode code point escape sequences, like "\\\\\\\\U00011000" can & should
/// be handled in a later step by the TOML parser, leave those subsequences alone.
fn preprocess_char_literal_notation(set: &mut HashSet<String>, input: &mut String) {
    let mut result = input.to_string();

    // These are backslash substrings sometimes used to escape character literals like punctuation.
    let possible_slash_strs = ["\\\\\\\\", "\\\\\\", "\\\\"];

    // Iterate in order of largest to smallest. Guarantee this with `.sorted().rev()`.
    for slash_str in possible_slash_strs.iter().sorted().rev() {
        let mut slash_result = result.clone();

        for match_tuple in result.rmatch_indices(slash_str) {
            let slash_idx = match_tuple.0;

            // find returns a byte index, so temporarily use a byte index just for size check
            let maybe_next_char_idx = slash_idx + slash_str.len();
            if maybe_next_char_idx < slash_result.len() {
                let char_literal = slash_result[maybe_next_char_idx..].chars().next().unwrap();
                let char_literal_str = char_literal.to_string();

                // Skip if we're looking at a Unicode code point escape sequence (ex: "\\\\Uxxxxxxxx")
                // rather than a Unix/bash-style escaped character literal (ex: "\\\\:", "\\\\-").
                // Also skip if we're seeing a suprious result, ex: we are looking for a double backslash
                // (ex: "\\\\" in the presence of quad backslashes like "\\\\\\\\Uxxxxxxxx") that should
                // be left alone.
                // Also skip if there is whitespace after the backslahses. Let's assume that this
                // is part of a token of all backslashes. Allow that to be fully parsed and
                // handled later in `unescape_exemplar_chars()`.
                if char_literal_str == "U"
                    || char_literal_str == "u"
                    || char_literal_str == "\\"
                    || char_literal.is_whitespace()
                {
                    continue;
                }

                let char_literal_byte_len = char_literal_str.len();
                set.insert(char_literal_str);

                // Remove the slash and the char literal following it from the original string.
                let mut new_slash_result = slash_result[..slash_idx].to_string();
                new_slash_result
                    .push_str(&slash_result[(maybe_next_char_idx + char_literal_byte_len)..]);
                slash_result = new_slash_result;
            }
        }
        result.clear();
        result.push_str(&slash_result);
    }
    input.clear();
    input.push_str(&result);
}

/// Predicate fn that returns whether a character should be used in `.split()` to tokenize
/// the exemplar characters JSON string.
fn is_exemplar_string_split_char(c: char) -> bool {
    // Don't include the close brace in the split criteria so that, after we split,
    // we know where the `{...}` sequence ends.
    c.is_whitespace() || c == '{'
}

/// Unescape a (sub-)string of exemplar character data
fn unescape_exemplar_chars(char_block: &str) -> String {
    // Exit early with degenerate case that interferes with TOML parser workaround.
    // Also handle a char block solely consisting of all backslashes (ex: "\\\\\\\\") as a backslash literal.
    if char_block.chars().all(|ch| ch == '\\') {
        return "\\".to_string();
    } else if char_block
        .chars()
        .all(|ch| ch == '\"' || ch == '＂' || ch == '\\')
    {
        return char_block.replace('\\', "");
    }

    // Unescape the escape sequences like \uXXXX and \UXXXXXXXX into the proper code points.
    // Also, workaround errant extra backslash escaping.
    // Because JSON does not support \UXXXXXXXX Unicode code point escaping, use the TOML parser.
    let ch_for_json = format!("x=\"{char_block}\"");

    // Workaround for literal values like "\\-"" that cause problems for the TOML parser.
    // In such cases, remove the '\\' character preceding the non-Unicode-escape-sequence character.
    let mut ch_vec = ch_for_json.chars().collect::<Vec<char>>();
    let mut ch_indices_to_remove: Vec<usize> = vec![];
    for (idx, ch) in ch_vec.iter().enumerate().rev() {
        if ch == &'\\' {
            let ch_after_slash = ch_vec.get(idx + 1).unwrap();
            if ch_after_slash != &'u' && ch_after_slash != &'U' {
                ch_indices_to_remove.push(idx);
            }
        }
    }
    for idx in ch_indices_to_remove {
        ch_vec.remove(idx);
    }
    let ch_for_json = ch_vec.iter().collect::<String>();

    let ch_lite_t_val: toml::Value =
        toml::from_str(&ch_for_json).unwrap_or_else(|_| panic!("{char_block:?}"));
    let ch_lite = if let toml::Value::Table(t) = ch_lite_t_val {
        if let Some(toml::Value::String(s)) = t.get("x") {
            s.to_owned()
        } else {
            panic!();
        }
    } else {
        panic!();
    };

    let result = ch_lite.trim().to_string();

    result
}

/// Parse the input string, and insert the represented exemplar "characters" (each of
/// which could either be an individual code point or a code point sequence) into the set.
fn insert_chars_from_string(set: &mut HashSet<String>, input: &str) {
    let s = if input.chars().count() > 1 && input.starts_with('\\') {
        input
            .chars()
            .skip_while(|ch| ch == &'\\')
            .collect::<String>()
    } else {
        input.to_string()
    };
    // A range of consecutive code point characters can be represented as <char_start>-<char_end>.
    if s.contains('-') && s.find('-').unwrap() > 0 {
        let (begin, end) = s.split_once('-').unwrap();
        let begin_char = begin.chars().rev().next().unwrap();
        let end_char = end.chars().next().unwrap();

        for code_point in (begin_char as u32)..=(end_char as u32) {
            let char_str = char::from_u32(code_point)
                .expect("Character range should not span non-Unicode-scalar-value code points")
                .to_string();
            set.insert(char_str);
        }

        // After handling the range substring, recursively handle any chars/ranges in the remaining
        // parts of the string.
        let rem_begin_str = &begin[..(begin.len() - begin_char.len_utf8())];
        let rem_end_str = &end[end_char.len_utf8()..];
        insert_chars_from_string(set, rem_begin_str);
        insert_chars_from_string(set, rem_end_str);
    } else {
        for ch in s.chars() {
            set.insert(ch.to_string());
        }
    }
}

/// Parse the input CLDR JSON string representing exemplar character data and return a
/// set of strings representing each code point or string represented by the CLDR JSON
/// serialized form.
fn parse_exemplar_char_string(s: &str) -> HashSet<String> {
    debug_assert!(s.starts_with('['));
    debug_assert!(s.ends_with(']'));
    let mut transformed_input = s.split_at(1).1.split_at(s.len() - 2).0.to_string();

    if transformed_input.is_empty() {
        return HashSet::new();
    }

    // Initialize result collection of parsed element strings of exemplar character data.
    // Note: We want to use the hashset to dedup in case of space (U+0020) literal being included in exemplar char set.
    let mut dedup_chars = HashSet::<String>::new();

    // CLDR JSON uses an "over"-escaped notation to indicate a character literal, including
    // for characters that overlap with notational syntax characters. Since these are special
    // cases, handle them first before proceeding.
    preprocess_char_literal_notation(&mut dedup_chars, &mut transformed_input);

    transformed_input
        .split(is_exemplar_string_split_char)
        .filter(|t| !t.is_empty())
        .for_each(|token| {
            let mut string_and_chars = token.split('}');

            if let Some(maybe_char_string) = string_and_chars.next() {
                if !maybe_char_string.is_empty() {
                    if token.contains('}') {
                        // If we see a '}', then we assume it was the ending of a string
                        // denoted by `{...}` in a well-formed input.
                        // We need to unescape first so that we turn a substring like "...{ɛ\\u0300}..."
                        // into "...ɛ̀..."
                        let unescaped_char_string = unescape_exemplar_chars(maybe_char_string);
                        dedup_chars.insert(unescaped_char_string);
                    } else {
                        // If we don't see '}', it means we have a string that was whitespace delimited
                        let unescaped_char_block = unescape_exemplar_chars(maybe_char_string);
                        insert_chars_from_string(&mut dedup_chars, &unescaped_char_block);
                    }
                }

                // since we already split on '{' in order to create `token`, then only the first
                // subarray split could contain '}'. all other subarray splits should be considered
                // as strings of one or more consecutive characters
                for char_block in string_and_chars.filter(|t| !t.is_empty()) {
                    let unescaped_char_block = unescape_exemplar_chars(char_block);
                    insert_chars_from_string(&mut dedup_chars, &unescaped_char_block);
                }
            }
        });

    dedup_chars
}

fn string_to_prop_unicodeset(s: &str) -> PropertyUnicodeSetV1<'static> {
    PropertyUnicodeSetV1::CPInversionListStrList(CodePointInversionListAndStringList::from_iter(
        parse_exemplar_char_string(s).iter().map(|s| &**s).sorted(),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;
    use icu_properties::sets::UnicodeSetData;

    #[test]
    fn test_parse_exemplar_chars() {
        let af_numbers = "[  \\- ‑ , % ‰ + 0 1 2 3 4 5 6 7 8 9]";
        let expected: HashSet<String> = [
            "-", "‑", ",", "%", "‰", "+", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]
        .iter()
        .copied()
        .map(std::string::String::from)
        .collect();
        let actual = parse_exemplar_char_string(af_numbers);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_exemplar_char_sequences() {
        let sr_main = "[a b c č ć d {dž} đ e f g h i j k l {lj} m n {nj} o p r s š t u v z ž]";
        let expected: HashSet<String> = [
            "a", "b", "c", "č", "ć", "d", "dž", "đ", "e", "f", "g", "h", "i", "j", "k", "l", "lj",
            "m", "n", "nj", "o", "p", "r", "s", "š", "t", "u", "v", "z", "ž",
        ]
        .iter()
        .copied()
        .map(std::string::String::from)
        .collect();
        let actual = parse_exemplar_char_string(sr_main);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_exemplar_char_ranges() {
        let ja_main_subset_range = "[万-下]";
        let expected: HashSet<String> = ["万", "丈", "三", "上", "下"]
            .iter()
            .copied()
            .map(std::string::String::from)
            .collect();
        let actual = parse_exemplar_char_string(ja_main_subset_range);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_exemplar_char_ranges_no_whitespace() {
        let range_amid_chars = "[a万-下z]";
        let expected: HashSet<String> = ["万", "丈", "三", "上", "下", "a", "z"]
            .iter()
            .copied()
            .map(std::string::String::from)
            .collect();
        let actual = parse_exemplar_char_string(range_amid_chars);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_splits() {
        let sr_main = "[a b cčć d{dž}đ    e\u{00A0}f  \u{202F}   ghijkl{lj}mn{nj}oprsštuvzž]";
        let expected: HashSet<String> = [
            "a", "b", "c", "č", "ć", "d", "dž", "đ", "e", "f", "g", "h", "i", "j", "k", "l", "lj",
            "m", "n", "nj", "o", "p", "r", "s", "š", "t", "u", "v", "z", "ž",
        ]
        .iter()
        .copied()
        .map(std::string::String::from)
        .collect();
        let actual = parse_exemplar_char_string(sr_main);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_unescape() {
        let ar_eg_auxiliary = "[ـ\\u200C\\u200D\\u200E\\u200F پ چ ژ ڜ ڢ ڤ ڥ ٯ ڧ ڨ ک گ ی]";
        let expected: HashSet<String> = [
            "ـ", "\u{200C}", "\u{200D}", "\u{200E}", "\u{200F}", "پ", "چ", "ژ", "ڜ", "ڢ", "ڤ", "ڥ",
            "ٯ", "ڧ", "ڨ", "ک", "گ", "ی",
        ]
        .iter()
        .copied()
        .map(std::string::String::from)
        .collect();
        let actual = parse_exemplar_char_string(ar_eg_auxiliary);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_quotes() {
        let quotes = "[\"＂]";
        let expected: HashSet<String> = ["\"", "＂"]
            .iter()
            .copied()
            .map(std::string::String::from)
            .collect();
        let actual = parse_exemplar_char_string(quotes);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_escaped_punctuation() {
        let ja_punctuation = "[‾ _＿ \\\\\\\\-－ ‐ ‑ — ― 〜 ・ ･ ,， 、､ ;； \\\\\\\\:： !！ ?？ .． ‥ … 。｡ ＇ ‘ ’ \\\\\\\"＂ “ ” (（ )） \\\\\\\\[［ \\\\\\\\]］ \\\\\\\\{｛ \\\\\\\\}｝ 〈 〉 《 》 「｢ 」｣ 『 』 【 】 〔 〕 ‖ § ¶ @＠ *＊ /／ \\\\\\\\＼ \\\\\\\\&＆ #＃ %％ ‰ † ‡ ′ ″ 〃 ※]";

        let actual = parse_exemplar_char_string(ja_punctuation);

        let any_backslashes = actual.iter().any(|parsed_str| parsed_str.contains('\\'));

        assert!(!any_backslashes);
        assert!(actual.contains("-"));
        assert!(actual.contains(":"));
        assert!(actual.contains("\""));
    }

    #[test]
    fn test_parse_escaped_punctuation_preserve_code_point_notation() {
        let ccp_main = "[\\\\\\\\U00011100 \\\\\\\\U00011101 \\\\\\\\U00011102 𑄃 𑄄 𑄅 𑄆 𑄇 𑄈 𑄉 𑄊 𑄋 𑄌 𑄍 𑄎 𑄏 𑄐 𑄑 𑄒 𑄓 𑄔 𑄕 𑄖 𑄗 𑄘 𑄙 𑄚 𑄛 𑄜 𑄝 𑄞 𑄟 𑄠 𑄡 𑄢 𑄣 𑄤 𑄥 𑄦 \\\\\\\\U00011127 \\\\\\\\U00011128 \\\\\\\\U00011129 \\\\\\\\U0001112A \\\\\\\\U0001112B 𑄬 \\\\\\\\U0001112D \\\\\\\\U0001112E \\\\\\\\U0001112F \\\\\\\\U00011130 \\\\\\\\U00011131 \\\\\\\\U00011132 \\\\\\\\U00011133 \\\\\\\\U00011134]";

        let actual = parse_exemplar_char_string(ccp_main);

        assert!(actual.contains("\u{11100}"));
        assert!(actual.contains("𑄃"));
    }

    #[test]
    fn test_parse_escaped_punctuation_allow_backslash_literal() {
        let es_puncutation = "[\\\\\\\\- ‐ ‑ – — , ; \\\\\\\\: ! ¡ ? ¿ . … ' ‘ ’ \\\\\\\" “ ” « » ( ) \\\\\\\\[ \\\\\\\\] § @ * / \\\\\\\\ \\\\& # † ‡ ′ ″]";

        let actual = parse_exemplar_char_string(es_puncutation);

        assert!(actual.contains("\\"));
        assert!(!actual.contains(" "));
    }

    #[test]
    fn test_parse_unescape_in_strings() {
        let bn_main = "[\\\\u09BC ৺ অ আ ই ঈ উ ঊ ঋ ৠ ঌ ৡ এ ঐ ও ঔ ং ঃ \\\\u0981 ক {ক\\\\u09CDষ} খ গ ঘ ঙ চ ছ জ ঝ ঞ ট ঠ ড {ড\\u09BC} ঢ {ঢ\\\\u09BC} ণ ত ৎ থ দ ধ ন প ফ ব ভ ম য {য\\\\u09BC} র ল শ ষ স হ ঽ া ি ী \\\\u09C1 \\\\u09C2 \\\\u09C3 \\\\u09C4 \\\\u09E2 \\\\u09E3 ে ৈ ো ৌ \\\\u09CD ৗ]";

        let actual = parse_exemplar_char_string(bn_main);

        assert!(actual.contains("\u{0981}"));
        assert!(actual.contains("ক\u{09CD}ষ"));
    }

    #[test]
    fn test_basic() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<ExemplarCharactersMainV1Marker> = provider
            .load(DataRequest {
                locale: &DataLocale::from(locale!("en-001")),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        let exp_chars = vec![
            "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q",
            "r", "s", "t", "u", "v", "w", "x", "y", "z",
        ];
        let exp_chars_cpilsl =
            CodePointInversionListAndStringList::from_iter(exp_chars.iter().cloned());

        let actual = UnicodeSetData::from_data(data);
        let act_chars_cpilsl = actual.to_code_point_inversion_list_string_list();

        assert_eq!(exp_chars_cpilsl, act_chars_cpilsl,);
    }
}
