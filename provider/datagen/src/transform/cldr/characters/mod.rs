// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;
use std::collections::HashSet;
use std::marker::PhantomData;

use crate::transform::cldr::cldr_serde;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_properties::provider::*;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;

struct AnnotatedResource<'a, M: DataMarker>(
    &'a cldr_serde::exemplar_chars::Resource,
    PhantomData<M>,
);

macro_rules! exemplar_chars_impls {
    ($data_marker_name:ident, $cldr_serde_field_name:ident) => {
        impl DataProvider<$data_marker_name> for crate::DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$data_marker_name>, DataError> {
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
                Ok(self
                    .source
                    .cldr()?
                    .misc()
                    .list_langs()?
                    .map(DataLocale::from)
                    .collect())
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

fn is_exemplar_string_split_char(c: char) -> bool {
    // don't include the close brace in the split criteria so that, after we split,
    // we know where the `{...}` sequence ends
    c.is_whitespace() || c == '{'
}

// unescape a (sub-)string of exemplar character data
fn unescape_exemplar_chars(char_block: &str) -> String {
    // Unescape the escape sequences like \uXXXX and \UXXXXXXXX into the proper code points.
    // Also, workaround errant extra backslash escaping.
    // Because JSON does not support \UXXXXXXXX Unicode code point escaping, use the TOML parser
    let ch_for_json = format!(
        "x=\"{}\"",
        char_block.replace("\\\\", "\\").replace("\\\\", "\\")
    );
    let ch_lite_t_val: toml::Value =
        toml::from_str(&ch_for_json).unwrap_or_else(|_| panic!("{:?}", char_block));
    let ch_lite = if let toml::Value::Table(t) = ch_lite_t_val {
        if let Some(toml::Value::String(s)) = t.get("x") {
            s.to_owned()
        } else {
            panic!();
        }
    } else {
        panic!();
    };

    ch_lite.trim().to_string()
}

fn insert_chars_from_string(set: &mut HashSet<Cow<str>>, input: &str) {
    let s = if input.chars().count() > 1 && input.starts_with('\\') {
        input
            .chars()
            .skip_while(|ch| ch == &'\\')
            .collect::<String>()
    } else {
        input.to_string()
    };
    if s.contains('-') && s.find('-').unwrap() > 0 {
        let (begin, end) = s.split_once('-').unwrap();
        let begin_char = begin.chars().rev().next().unwrap();
        let end_char = end.chars().next().unwrap();

        for code_point in (begin_char as u32)..=(end_char as u32) {
            let char_str = char::from_u32(code_point)
                .expect("Character range should not span non-Unicode-scalar-value code points")
                .to_string();
            set.insert(Cow::Owned(char_str));
        }

        // after handling the range substring, recursively handle any chars/ranges in the remaining
        // parts of the string
        let rem_begin_str = &begin[..(begin.len() - begin_char.len_utf8())];
        let rem_end_str = &end[end_char.len_utf8()..];
        insert_chars_from_string(set, rem_begin_str);
        insert_chars_from_string(set, rem_end_str);
    } else {
        for ch in s.chars().filter(|c| !c.is_whitespace()) {
            set.insert(Cow::Owned(ch.to_string()));
        }
    }
}

// helper function for parsing CLDR data string
fn parse_exemplar_char_string(s: &str) -> HashSet<Cow<str>> {
    debug_assert!(s.starts_with('['));
    debug_assert!(s.ends_with(']'));
    let without_brackets = s.split_at(1).1.split_at(s.len() - 2).0;

    if without_brackets.is_empty() {
        return HashSet::new();
    }

    // We want to use the hashset to dedup in case of space (U+0020) literal being included in exemplar char set
    let mut dedup_chars = HashSet::<Cow<str>>::new();

    without_brackets
        .split(is_exemplar_string_split_char)
        .filter(|t| !t.is_empty())
        .for_each(|token| {
            let mut string_and_chars = token.split('}');

            if let Some(maybe_char_string) = string_and_chars.next() {
                if !maybe_char_string.is_empty() {
                    if token.contains('}') {
                        dedup_chars.insert(Cow::Borrowed(maybe_char_string));
                    } else {
                        insert_chars_from_string(&mut dedup_chars, maybe_char_string);
                    }
                }

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
        parse_exemplar_char_string(s).iter().map(|s| &**s),
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
        let expected: HashSet<Cow<str>> = [
            "-", "‑", ",", "%", "‰", "+", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ]
        .iter()
        .copied()
        .map(Cow::Borrowed)
        .collect();
        let actual = parse_exemplar_char_string(af_numbers);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_exemplar_char_sequences() {
        let sr_main = "[a b c č ć d {dž} đ e f g h i j k l {lj} m n {nj} o p r s š t u v z ž]";
        let expected: HashSet<Cow<str>> = [
            "a", "b", "c", "č", "ć", "d", "dž", "đ", "e", "f", "g", "h", "i", "j", "k", "l", "lj",
            "m", "n", "nj", "o", "p", "r", "s", "š", "t", "u", "v", "z", "ž",
        ]
        .iter()
        .copied()
        .map(Cow::Borrowed)
        .collect();
        let actual = parse_exemplar_char_string(sr_main);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_exemplar_char_ranges() {
        let ja_main_subset_range = "[万-下]";
        let expected: HashSet<Cow<str>> = ["万", "丈", "三", "上", "下"]
            .iter()
            .copied()
            .map(Cow::Borrowed)
            .collect();
        let actual = parse_exemplar_char_string(ja_main_subset_range);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_exemplar_char_ranges_no_whitespace() {
        let ja_main_subset_range = "[a万-下z]";
        let expected: HashSet<Cow<str>> = ["万", "丈", "三", "上", "下", "a", "z"]
            .iter()
            .copied()
            .map(Cow::Borrowed)
            .collect();
        let actual = parse_exemplar_char_string(ja_main_subset_range);

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_parse_splits() {
        let sr_main = "[a b cčć d{dž}đ    e\u{00A0}f  \u{202F}   ghijkl{lj}mn{nj}oprsštuvzž]";
        let expected: HashSet<Cow<str>> = [
            "a", "b", "c", "č", "ć", "d", "dž", "đ", "e", "f", "g", "h", "i", "j", "k", "l", "lj",
            "m", "n", "nj", "o", "p", "r", "s", "š", "t", "u", "v", "z", "ž",
        ]
        .iter()
        .copied()
        .map(Cow::Borrowed)
        .collect();
        let actual = parse_exemplar_char_string(sr_main);

        assert_eq!(actual, expected);
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
