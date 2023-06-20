// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by TOML files
//! exported from ICU.

use icu_casemapping::provider::{CaseMappingV1, CaseMappingV1Marker};
use icu_collections::codepointtrie::toml::CodePointDataSlice;
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_provider::prelude::*;
use std::convert::TryFrom;

mod ucase_serde;

impl DataProvider<CaseMappingV1Marker> for crate::DatagenProvider {
    fn load(&self, _req: DataRequest) -> Result<DataResponse<CaseMappingV1Marker>, DataError> {
        let toml = &self
            .source
            .icuexport()?
            .read_and_parse_toml::<ucase_serde::Main>(&format!(
                "ucase/{}/ucase.toml",
                self.source.options.trie_type
            ))?
            .ucase;

        let trie_data = &toml.code_point_trie;
        let trie_header = CodePointTrieHeader::try_from(trie_data).map_err(|e| {
            DataError::custom("Could not parse CodePointTrie TOML").with_display_context(&e)
        })?;
        let trie_index = trie_data.index_slice();
        let trie_data = if let Ok(CodePointDataSlice::U16(s)) = trie_data.data_slice() {
            s
        } else {
            return Err(DataError::custom(
                "Did not find 16-bit data array for case mapping in TOML",
            ));
        };
        let exceptions = &toml.exceptions.exceptions;
        let unfold = &toml.unfold.unfold;

        let case_mapping =
            CaseMappingV1::try_from_icu(trie_header, trie_index, trie_data, exceptions, unfold)
                .map_err(|e| {
                    DataError::custom("Could not create CaseMappingInternals")
                        .with_display_context(&e)
                })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(case_mapping)),
        })
    }
}

impl icu_provider::datagen::IterableDataProvider<CaseMappingV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

#[cfg(test)]
mod tests {
    use core::str::FromStr;
    use icu_casemapping::CaseMapping;
    use icu_locid::Locale;

    #[test]
    fn test_simple_mappings() {
        let case_mapping = CaseMapping::try_new_unstable(&crate::DatagenProvider::for_test())
            .expect("Loading was successful");

        // Basic case mapping
        assert_eq!(case_mapping.to_uppercase('a'), 'A');
        assert_eq!(case_mapping.to_lowercase('a'), 'a');
        assert_eq!(case_mapping.to_titlecase('a'), 'A');
        assert_eq!(case_mapping.fold('a'), 'a');
        assert_eq!(case_mapping.to_uppercase('A'), 'A');
        assert_eq!(case_mapping.to_lowercase('A'), 'a');
        assert_eq!(case_mapping.to_titlecase('A'), 'A');
        assert_eq!(case_mapping.fold('A'), 'a');

        // Case mapping of titlecase character
        assert_eq!(case_mapping.to_uppercase('\u{1c4}'), '\u{1c4}');
        assert_eq!(case_mapping.to_titlecase('\u{1c4}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lowercase('\u{1c4}'), '\u{1c6}');
        assert_eq!(case_mapping.to_uppercase('\u{1c5}'), '\u{1c4}');
        assert_eq!(case_mapping.to_titlecase('\u{1c5}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lowercase('\u{1c5}'), '\u{1c6}');
        assert_eq!(case_mapping.to_uppercase('\u{1c6}'), '\u{1c4}');
        assert_eq!(case_mapping.to_titlecase('\u{1c6}'), '\u{1c5}');
        assert_eq!(case_mapping.to_lowercase('\u{1c6}'), '\u{1c6}');

        // Turkic case folding
        assert_eq!(case_mapping.fold('I'), 'i');
        assert_eq!(case_mapping.fold_turkic('I'), 'ı');
        assert_eq!(case_mapping.fold('İ'), 'İ');
        assert_eq!(case_mapping.fold_turkic('İ'), 'i');

        // Supplementary code points (Deseret)
        assert_eq!(case_mapping.to_uppercase('\u{1043c}'), '\u{10414}');
        assert_eq!(case_mapping.to_lowercase('\u{1043c}'), '\u{1043c}');
        assert_eq!(case_mapping.to_titlecase('\u{1043c}'), '\u{10414}');
        assert_eq!(case_mapping.fold('\u{1043c}'), '\u{1043c}');
        assert_eq!(case_mapping.to_uppercase('\u{10414}'), '\u{10414}');
        assert_eq!(case_mapping.to_lowercase('\u{10414}'), '\u{1043c}');
        assert_eq!(case_mapping.to_titlecase('\u{10414}'), '\u{10414}');
        assert_eq!(case_mapping.fold('\u{10414}'), '\u{1043c}');
    }

    // These tests are taken from StringCaseTest::TestCaseConversion in ICU4C.
    #[test]
    fn test_full_mappings() {
        let case_mapping = CaseMapping::try_new_unstable(&crate::DatagenProvider::for_test())
            .expect("Loading was successful");

        let turkish_locale = Locale::from_str("tr").expect("Parsing was successful");
        let turkish_case_mapping = CaseMapping::try_new_with_locale_unstable(
            &crate::DatagenProvider::for_test(),
            &turkish_locale,
        )
        .expect("Loading was successful");

        let lithuanian_locale = Locale::from_str("lt").expect("Parsing was successful");
        let lithuanian_case_mapping = CaseMapping::try_new_with_locale_unstable(
            &crate::DatagenProvider::for_test(),
            &lithuanian_locale,
        )
        .expect("Loading was successful");

        let uppercase_greek = "ΙΕΣΥΣ ΧΡΙΣΤΟΣ"; // "IESUS CHRISTOS"
        let lowercase_greek = "ιεσυς χριστος"; // "IESUS CHRISTOS"
        assert_eq!(
            case_mapping.to_full_uppercase_string(lowercase_greek),
            uppercase_greek
        );
        assert_eq!(
            case_mapping.to_full_lowercase_string(uppercase_greek),
            lowercase_greek
        );
        assert_eq!(
            case_mapping.full_fold_string(uppercase_greek),
            case_mapping.full_fold_string(lowercase_greek)
        );

        let lowercase_turkish_1 = "istanbul, not constantınople";
        let uppercase_turkish_1 = "İSTANBUL, NOT CONSTANTINOPLE";
        assert_eq!(
            case_mapping.to_full_lowercase_string(uppercase_turkish_1),
            "i\u{307}stanbul, not constantinople"
        );
        assert_eq!(
            turkish_case_mapping.to_full_lowercase_string(uppercase_turkish_1),
            lowercase_turkish_1
        );

        let lowercase_turkish_2 = "topkapı palace, istanbul";
        let uppercase_turkish_2 = "TOPKAPI PALACE, İSTANBUL";
        assert_eq!(
            case_mapping.to_full_uppercase_string(lowercase_turkish_2),
            "TOPKAPI PALACE, ISTANBUL"
        );
        assert_eq!(
            turkish_case_mapping.to_full_uppercase_string(lowercase_turkish_2),
            uppercase_turkish_2
        );

        let initial_german = "Süßmayrstraße";
        let uppercase_german = "SÜSSMAYRSTRASSE";
        assert_eq!(
            case_mapping.to_full_uppercase_string(initial_german),
            uppercase_german
        );

        let before = "aBIΣßΣ/\u{5ffff}";
        let after = "abiσßς/\u{5ffff}";
        let after_turkish = "abıσßς/\u{5ffff}";
        assert_eq!(case_mapping.to_full_lowercase_string(before), after);
        assert_eq!(
            turkish_case_mapping.to_full_lowercase_string(before),
            after_turkish
        );

        let before = "aBiςßσ/\u{fb03}\u{fb03}\u{fb03}\u{5ffff}";
        let after = "ABIΣSSΣ/FFIFFIFFI\u{5ffff}";
        let after_turkish = "ABİΣSSΣ/FFIFFIFFI\u{5ffff}";
        assert_eq!(case_mapping.to_full_uppercase_string(before), after);
        assert_eq!(
            turkish_case_mapping.to_full_uppercase_string(before),
            after_turkish
        );

        let before = "ßa";
        let after = "SSA";
        assert_eq!(case_mapping.to_full_uppercase_string(before), after);

        let initial_deseret = "\u{1043c}\u{10414}";
        let upper_deseret = "\u{10414}\u{10414}";
        let lower_deseret = "\u{1043c}\u{1043c}";
        assert_eq!(
            case_mapping.to_full_uppercase_string(initial_deseret),
            upper_deseret
        );
        assert_eq!(
            case_mapping.to_full_lowercase_string(initial_deseret),
            lower_deseret
        );

        // lj ligature
        let initial_ligature = "\u{1c7}\u{1c8}\u{1c9}";
        let lower_ligature = "\u{1c9}\u{1c9}\u{1c9}";
        let upper_ligature = "\u{1c7}\u{1c7}\u{1c7}";
        assert_eq!(
            case_mapping.to_full_uppercase_string(initial_ligature),
            upper_ligature
        );
        assert_eq!(
            case_mapping.to_full_lowercase_string(initial_ligature),
            lower_ligature
        );

        // Sigmas preceded and/or followed by cased letters
        let initial_sigmas = "i\u{307}\u{3a3}\u{308}j \u{307}\u{3a3}\u{308}j i\u{ad}\u{3a3}\u{308} \u{307}\u{3a3}\u{308}";
        let lower_sigmas = "i\u{307}\u{3c3}\u{308}j \u{307}\u{3c3}\u{308}j i\u{ad}\u{3c2}\u{308} \u{307}\u{3c3}\u{308}";
        let upper_sigmas = "I\u{307}\u{3a3}\u{308}J \u{307}\u{3a3}\u{308}J I\u{ad}\u{3a3}\u{308} \u{307}\u{3a3}\u{308}";
        assert_eq!(
            case_mapping.to_full_uppercase_string(initial_sigmas),
            upper_sigmas
        );
        assert_eq!(
            case_mapping.to_full_lowercase_string(initial_sigmas),
            lower_sigmas
        );

        // Turkish & Azerbaijani dotless i & dotted I:
        // Remove dot above if there was a capital I before and there are no more accents above.
        let initial_dots = "I İ I\u{307} I\u{327}\u{307} I\u{301}\u{307} I\u{327}\u{307}\u{301}";
        let after = "i i\u{307} i\u{307} i\u{327}\u{307} i\u{301}\u{307} i\u{327}\u{307}\u{301}";
        let after_turkish = "ı i i i\u{327} ı\u{301}\u{307} i\u{327}\u{301}";
        assert_eq!(case_mapping.to_full_lowercase_string(initial_dots), after);
        assert_eq!(
            turkish_case_mapping.to_full_lowercase_string(initial_dots),
            after_turkish
        );

        // Lithuanian dot above in uppercasing
        let initial_dots = "a\u{307} \u{307} i\u{307} j\u{327}\u{307} j\u{301}\u{307}";
        let after = "A\u{307} \u{307} I\u{307} J\u{327}\u{307} J\u{301}\u{307}";
        let after_lithuanian = "A\u{307} \u{307} I J\u{327} J\u{301}\u{307}";
        assert_eq!(case_mapping.to_full_uppercase_string(initial_dots), after);
        assert_eq!(
            lithuanian_case_mapping.to_full_uppercase_string(initial_dots),
            after_lithuanian
        );

        // Lithuanian adds dot above to i in lowercasing if there are more above accents
        let initial_dots = "I I\u{301} J J\u{301} \u{12e} \u{12e}\u{301} \u{cc}\u{cd}\u{128}";
        let after = "i i\u{301} j j\u{301} \u{12f} \u{12f}\u{301} \u{ec}\u{ed}\u{129}";
        let after_lithuanian = "i i\u{307}\u{301} j j\u{307}\u{301} \u{12f} \u{12f}\u{307}\u{301} i\u{307}\u{300}i\u{307}\u{301}i\u{307}\u{303}";
        assert_eq!(case_mapping.to_full_lowercase_string(initial_dots), after);
        assert_eq!(
            lithuanian_case_mapping.to_full_lowercase_string(initial_dots),
            after_lithuanian
        );

        // Test case folding
        let initial = "Aßµ\u{fb03}\u{1040c}İı";
        let simple = "assμffi\u{10434}i\u{307}ı";
        let turkic = "assμffi\u{10434}iı";
        assert_eq!(case_mapping.full_fold_string(initial), simple);
        assert_eq!(case_mapping.full_fold_turkic_string(initial), turkic);
    }
}
