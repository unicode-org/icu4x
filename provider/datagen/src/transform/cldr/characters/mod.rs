// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_properties::provider::{ExemplarCharactersMainV1Marker, PropertyUnicodeSetV1};
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use itertools::Itertools;

impl DataProvider<ExemplarCharactersMainV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ExemplarCharactersMainV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let data: &cldr_serde::exemplar_chars::Resource = self
            .source
            .cldr()?
            .misc()
            .read_and_parse(&langid, "characters.json")?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(
                PropertyUnicodeSetV1::try_from(data).map_err(|e| {
                    DataError::custom("data for exemplar characters").with_display_context(&e)
                })?,
            )),
        })
    }
}

impl IterableDataProvider<ExemplarCharactersMainV1Marker> for crate::DatagenProvider {
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

fn parse_exemplar_char_string(s: &str) -> Vec<&str> {
    debug_assert!(s.starts_with('['));
    debug_assert!(s.ends_with(']'));
    let without_brackets = s.split_at(1).1.split_at(s.len() - 2).0;

    without_brackets
        .split(' ')
        .map(|ch| {
            if ch.is_empty() {
                // a space (U+0020) belongs in the exemplar character set
                " "
            } else if ch.starts_with('\\') {
                ch.split_at(1).1
            } else {
                ch
            }
        })
        .dedup() // in case of space (U+0020) literal being included in exemplar char set
        .collect()
}

fn string_to_prop_unicodeset(s: &str) -> PropertyUnicodeSetV1<'static> {
    PropertyUnicodeSetV1::CPInversionListStrList(CodePointInversionListAndStringList::from_iter(
        parse_exemplar_char_string(s).iter().copied(),
    ))
}

impl TryFrom<&cldr_serde::exemplar_chars::Resource> for PropertyUnicodeSetV1<'static> {
    type Error = DataError;
    fn try_from(other: &cldr_serde::exemplar_chars::Resource) -> Result<Self, Self::Error> {
        Ok(string_to_prop_unicodeset(other.characters.main.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::parse_exemplar_char_string;

    #[test]
    fn test_parse_exemplar_char_string() {
        let af_numbers = "[  \\- ‑ , % ‰ + 0 1 2 3 4 5 6 7 8 9]";
        let expected = vec![
            " ", "-", "‑", ",", "%", "‰", "+", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
        ];
        let actual = parse_exemplar_char_string(af_numbers);

        assert_eq!(actual, expected);
    }
}
