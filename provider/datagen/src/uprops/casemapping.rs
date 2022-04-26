// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::uprops::uprops_serde;
use crate::{error::DatagenError, SourceData};
use icu_casemapping::provider::{CaseMappingV1, CaseMappingV1Marker};
use icu_casemapping::CaseMappingInternals;
use icu_codepointtrie::CodePointTrieHeader;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::fs;

pub struct CaseMappingDataProvider {
    source: SourceData,
}

/// A data provider reading from .toml files produced by the ICU4C icuwriteuprops tool.
impl From<&SourceData> for CaseMappingDataProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

impl ResourceProvider<CaseMappingV1Marker> for CaseMappingDataProvider {
    fn load_resource(
        &self,
        _req: &DataRequest,
    ) -> Result<DataResponse<CaseMappingV1Marker>, DataError> {
        let path = self.source.get_uprops_root()?.join("ucase.toml");
        let toml_str = fs::read_to_string(&path).map_err(|e| DatagenError::from((e, &path)))?;
        let toml: uprops_serde::case::Main =
            toml::from_str(&toml_str).map_err(|e| DatagenError::from((e, &path)))?;

        let trie_data = &toml.ucase.code_point_trie;
        let trie_header = CodePointTrieHeader::try_from(trie_data)?;
        let trie_index = &trie_data.index;
        let trie_data = &trie_data.data_16.as_ref().ok_or_else(|| {
            DataError::custom("Did not find 16-bit data array for case mapping in TOML")
        })?;
        let exceptions = &toml.ucase.exceptions.exceptions;
        let unfold = &toml.ucase.unfold.unfold;

        let case_mapping = CaseMappingInternals::try_from_icu(
            trie_header,
            trie_index,
            trie_data,
            exceptions,
            unfold,
        )
        .map_err(|e| {
            DataError::custom("Could not create CaseMappingInternals").with_display_context(&e)
        })?;
        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(CaseMappingV1 {
                casemap: case_mapping,
            })),
        })
    }
}

impl icu_provider::datagen::IterableResourceProvider<CaseMappingV1Marker>
    for CaseMappingDataProvider
{
    fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        Ok(Box::new(core::iter::once(Default::default())))
    }
}

icu_provider::impl_dyn_provider!(
    CaseMappingDataProvider,
    [CaseMappingV1Marker,],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

#[cfg(test)]
mod tests {
    use super::*;
    use core::str::FromStr;
    use icu_casemapping::CaseMapping;
    use icu_locid::Locale;

    #[test]
    fn test_simple_mappings() {
        let provider = CaseMappingDataProvider::from(&SourceData::for_test());
        let case_mapping = CaseMapping::try_new(&provider).expect("Loading was successful");

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
        let provider = CaseMappingDataProvider::from(&SourceData::for_test());

        let case_mapping = CaseMapping::try_new(&provider).expect("Loading was successful");

        let turkish_locale = Locale::from_str("tr").expect("Parsing was successful");
        let turkish_case_mapping = CaseMapping::try_new_with_locale(&provider, &turkish_locale)
            .expect("Loading was successful");

        let lithuanian_locale = Locale::from_str("lt").expect("Parsing was successful");
        let lithuanian_case_mapping =
            CaseMapping::try_new_with_locale(&provider, &lithuanian_locale)
                .expect("Loading was successful");

        let uppercase_greek = "ΙΕΣΥΣ ΧΡΙΣΤΟΣ"; // "IESUS CHRISTOS"
        let lowercase_greek = "ιεσυς χριστος"; // "IESUS CHRISTOS"
        assert_eq!(
            case_mapping.to_full_uppercase(lowercase_greek),
            uppercase_greek
        );
        assert_eq!(
            case_mapping.to_full_lowercase(uppercase_greek),
            lowercase_greek
        );
        assert_eq!(
            case_mapping.full_fold(uppercase_greek),
            case_mapping.full_fold(lowercase_greek)
        );

        let lowercase_turkish_1 = "istanbul, not constantınople";
        let uppercase_turkish_1 = "İSTANBUL, NOT CONSTANTINOPLE";
        assert_eq!(
            case_mapping.to_full_lowercase(uppercase_turkish_1),
            "i\u{307}stanbul, not constantinople"
        );
        assert_eq!(
            turkish_case_mapping.to_full_lowercase(uppercase_turkish_1),
            lowercase_turkish_1
        );

        let lowercase_turkish_2 = "topkapı palace, istanbul";
        let uppercase_turkish_2 = "TOPKAPI PALACE, İSTANBUL";
        assert_eq!(
            case_mapping.to_full_uppercase(lowercase_turkish_2),
            "TOPKAPI PALACE, ISTANBUL"
        );
        assert_eq!(
            turkish_case_mapping.to_full_uppercase(lowercase_turkish_2),
            uppercase_turkish_2
        );

        let initial_german = "Süßmayrstraße";
        let uppercase_german = "SÜSSMAYRSTRASSE";
        assert_eq!(
            case_mapping.to_full_uppercase(initial_german),
            uppercase_german
        );

        let before = "aBIΣßΣ/\u{5ffff}";
        let after = "abiσßς/\u{5ffff}";
        let after_turkish = "abıσßς/\u{5ffff}";
        assert_eq!(case_mapping.to_full_lowercase(before), after);
        assert_eq!(
            turkish_case_mapping.to_full_lowercase(before),
            after_turkish
        );

        let before = "aBiςßσ/\u{fb03}\u{fb03}\u{fb03}\u{5ffff}";
        let after = "ABIΣSSΣ/FFIFFIFFI\u{5ffff}";
        let after_turkish = "ABİΣSSΣ/FFIFFIFFI\u{5ffff}";
        assert_eq!(case_mapping.to_full_uppercase(before), after);
        assert_eq!(
            turkish_case_mapping.to_full_uppercase(before),
            after_turkish
        );

        let before = "ßa";
        let after = "SSA";
        assert_eq!(case_mapping.to_full_uppercase(before), after);

        let initial_deseret = "\u{1043c}\u{10414}";
        let upper_deseret = "\u{10414}\u{10414}";
        let lower_deseret = "\u{1043c}\u{1043c}";
        assert_eq!(
            case_mapping.to_full_uppercase(initial_deseret),
            upper_deseret
        );
        assert_eq!(
            case_mapping.to_full_lowercase(initial_deseret),
            lower_deseret
        );

        // lj ligature
        let initial_ligature = "\u{1c7}\u{1c8}\u{1c9}";
        let lower_ligature = "\u{1c9}\u{1c9}\u{1c9}";
        let upper_ligature = "\u{1c7}\u{1c7}\u{1c7}";
        assert_eq!(
            case_mapping.to_full_uppercase(initial_ligature),
            upper_ligature
        );
        assert_eq!(
            case_mapping.to_full_lowercase(initial_ligature),
            lower_ligature
        );

        // Sigmas preceded and/or followed by cased letters
        let initial_sigmas = "i\u{307}\u{3a3}\u{308}j \u{307}\u{3a3}\u{308}j i\u{ad}\u{3a3}\u{308} \u{307}\u{3a3}\u{308}";
        let lower_sigmas = "i\u{307}\u{3c3}\u{308}j \u{307}\u{3c3}\u{308}j i\u{ad}\u{3c2}\u{308} \u{307}\u{3c3}\u{308}";
        let upper_sigmas = "I\u{307}\u{3a3}\u{308}J \u{307}\u{3a3}\u{308}J I\u{ad}\u{3a3}\u{308} \u{307}\u{3a3}\u{308}";
        assert_eq!(case_mapping.to_full_uppercase(initial_sigmas), upper_sigmas);
        assert_eq!(case_mapping.to_full_lowercase(initial_sigmas), lower_sigmas);

        // Turkish & Azerbaijani dotless i & dotted I:
        // Remove dot above if there was a capital I before and there are no more accents above.
        let initial_dots = "I İ I\u{307} I\u{327}\u{307} I\u{301}\u{307} I\u{327}\u{307}\u{301}";
        let after = "i i\u{307} i\u{307} i\u{327}\u{307} i\u{301}\u{307} i\u{327}\u{307}\u{301}";
        let after_turkish = "ı i i i\u{327} ı\u{301}\u{307} i\u{327}\u{301}";
        assert_eq!(case_mapping.to_full_lowercase(initial_dots), after);
        assert_eq!(
            turkish_case_mapping.to_full_lowercase(initial_dots),
            after_turkish
        );

        // Lithuanian dot above in uppercasing
        let initial_dots = "a\u{307} \u{307} i\u{307} j\u{327}\u{307} j\u{301}\u{307}";
        let after = "A\u{307} \u{307} I\u{307} J\u{327}\u{307} J\u{301}\u{307}";
        let after_lithuanian = "A\u{307} \u{307} I J\u{327} J\u{301}\u{307}";
        assert_eq!(case_mapping.to_full_uppercase(initial_dots), after);
        assert_eq!(
            lithuanian_case_mapping.to_full_uppercase(initial_dots),
            after_lithuanian
        );

        // Lithuanian adds dot above to i in lowercasing if there are more above accents
        let initial_dots = "I I\u{301} J J\u{301} \u{12e} \u{12e}\u{301} \u{cc}\u{cd}\u{128}";
        let after = "i i\u{301} j j\u{301} \u{12f} \u{12f}\u{301} \u{ec}\u{ed}\u{129}";
        let after_lithuanian = "i i\u{307}\u{301} j j\u{307}\u{301} \u{12f} \u{12f}\u{307}\u{301} i\u{307}\u{300}i\u{307}\u{301}i\u{307}\u{303}";
        assert_eq!(case_mapping.to_full_lowercase(initial_dots), after);
        assert_eq!(
            lithuanian_case_mapping.to_full_lowercase(initial_dots),
            after_lithuanian
        );

        // Test case folding
        let initial = "Aßµ\u{fb03}\u{1040c}İı";
        let simple = "assμffi\u{10434}i\u{307}ı";
        let turkic = "assμffi\u{10434}iı";
        assert_eq!(case_mapping.full_fold(initial), simple);
        assert_eq!(case_mapping.full_fold_turkic(initial), turkic);
    }
}
