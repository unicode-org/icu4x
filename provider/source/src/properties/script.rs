// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::SourceDataProvider;
use icu::collections::codepointtrie::TrieValue;
use icu::properties::props::EnumeratedProperty;
use icu::properties::props::Script;
use icu::properties::provider::{PropertyScriptWithExtensionsV1, ScriptWithExtensionsProperty};
use icu::properties::script::ScriptWithExt;
use icu::properties::{CodePointMapData, PropertyParser};
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use zerovec::{VarZeroVec, ZeroSlice, ZeroVec};

// implement data provider
impl DataProvider<PropertyScriptWithExtensionsV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<PropertyScriptWithExtensionsV1>, DataError> {
        self.check_req::<PropertyScriptWithExtensionsV1>(req)?;

        self.validate_property_name(
            core::str::from_utf8(Script::NAME).unwrap(),
            core::str::from_utf8(Script::SHORT_NAME).unwrap(),
        )?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build properties data",
        )
        .with_req(PropertyScriptWithExtensionsV1::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        {
            let data = if let Some(t) = self
                .rscd()?
                .cpt_cache
                .get(core::str::from_utf8(Script::SHORT_NAME).unwrap())
                .and_then(|t| t.downcast_ref::<ScriptWithExtensionsProperty>().cloned())
            {
                t
            } else {
                let script_parser = PropertyParser::<Script>::try_new_unstable(&self)?;
                let script = CodePointMapData::try_new_unstable(self)?;

                let mut script_sets = vec![];
                let mut script_sets_lookup = HashMap::new();

                let mut char_with_extensions = HashMap::new();

                for line in self.rscd()?.parse_ucd_lines("ucd/ScriptExtensions.txt")? {
                    let Some(line) = line.skip_missing_rule() else {
                        continue;
                    };
                    let mut fields = line.fields();
                    let cp_range = fields.next().unwrap();
                    let values = fields.next().unwrap();
                    let mut value = values
                        .split_ascii_whitespace()
                        .filter_map(|s| script_parser.as_borrowed().get_strict(s))
                        .collect::<Vec<_>>();
                    // Sort in stable order
                    value.sort_by_key(|s| s.to_u32());

                    let cp_range = super::ucd_helpers::parse_range(cp_range);

                    for cp in cp_range {
                        let mut value = value.clone();

                        let script = script.as_borrowed().get32(cp);
                        if !matches!(script, Script::Inherited | Script::Common) {
                            value.insert(0, script);
                        }

                        if !script_sets_lookup.contains_key(&value) {
                            script_sets_lookup.insert(value.clone(), script_sets.len());
                            script_sets.push(value.clone());
                        }

                        char_with_extensions.insert(
                            cp,
                            ScriptWithExt::new(script, script_sets_lookup[&value] as u16),
                        );
                    }
                }

                let mut builder = icu_codepointtrie_builder::CodePointTrieBuilder::new(
                    ScriptWithExt::single(Script::Unknown),
                    ScriptWithExt::single(Script::Unknown),
                    icu::collections::codepointtrie::TrieType::Small,
                );

                for cp in 0..(char::MAX as u32) {
                    builder.set_value(
                        cp,
                        char_with_extensions.get(&cp).copied().unwrap_or_else(|| {
                            ScriptWithExt::single(script.as_borrowed().get32(cp))
                        }),
                    );
                }

                let extensions: VarZeroVec<ZeroSlice<Script>> = VarZeroVec::from(
                    script_sets
                        .into_iter()
                        .map(|v| v.into_iter().collect::<ZeroVec<_>>())
                        .collect::<Vec<ZeroVec<_>>>()
                        .as_slice(),
                );

                let trie = builder.build();

                let data = ScriptWithExtensionsProperty { trie, extensions };

                self.rscd()?.cpt_cache.insert(
                    core::str::from_utf8(Script::SHORT_NAME).unwrap(),
                    Box::new(data.clone()),
                );

                data
            };

            Ok(DataResponse {
                metadata: Default::default(),
                payload: DataPayload::from_owned(data),
            })
        }
    }
}

impl crate::IterableDataProviderCached<PropertyScriptWithExtensionsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_val_from_script_extensions() {
        let provider = SourceDataProvider::new_testing();

        let swe =
            icu::properties::script::ScriptWithExtensions::try_new_unstable(&provider).unwrap();
        let swe = swe.as_borrowed();

        assert_eq!(swe.get_script_val('𐓐'), Script::Osage); // U+104D0 OSAGE CAPITAL LETTER KHA
        assert_eq!(swe.get_script_val('🥳'), Script::Common); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
        assert_eq!(swe.get_script_val32(0x200D), Script::Inherited); // ZERO WIDTH JOINER
        assert_eq!(swe.get_script_val('௫'), Script::Tamil); // U+0BEB TAMIL DIGIT FIVE
        assert_eq!(swe.get_script_val32(0x11303), Script::Grantha); // GRANTHA SIGN VISARGA
        assert_eq!(swe.get_script_val32(0x30A0), Script::Common); // U+30A0 KATAKANA-HIRAGANA DOUBLE HYPHEN
    }

    #[test]
    fn test_scx_array_from_script_extensions() {
        let provider = SourceDataProvider::new_testing();

        let swe =
            icu::properties::script::ScriptWithExtensions::try_new_unstable(&provider).unwrap();
        let swe = swe.as_borrowed();

        assert_eq!(
            swe.get_script_extensions_val('𐓐') /* U+104D0 OSAGE CAPITAL LETTER KHA */
                .iter()
                .collect::<Vec<_>>(),
            [Script::Osage]
        );
        assert_eq!(
            swe.get_script_extensions_val('🥳') /* U+1F973 FACE WITH PARTY HORN AND PARTY HAT */
                .iter()
                .collect::<Vec<_>>(),
            [Script::Common]
        );
        assert_eq!(
            swe.get_script_extensions_val32(0x200D) // ZERO WIDTH JOINER
                .iter()
                .collect::<Vec<_>>(),
            [Script::Inherited]
        );
        assert_eq!(
            swe.get_script_extensions_val('௫') // U+0BEB TAMIL DIGIT FIVE
                .iter()
                .collect::<Vec<_>>(),
            [Script::Tamil, Script::Grantha]
        );
        assert_eq!(
            swe.get_script_extensions_val32(0x11303) // GRANTHA SIGN VISARGA
                .iter()
                .collect::<Vec<_>>(),
            [Script::Tamil, Script::Grantha]
        );
        assert_eq!(
            swe.get_script_extensions_val32(0x30A0) // KATAKANA-HIRAGANA DOUBLE HYPHEN
                .iter()
                .collect::<Vec<_>>(),
            [Script::Hiragana, Script::Katakana]
        );

        assert_eq!(
            swe.get_script_extensions_val32(0x200D) // ZERO WIDTH JOINER
                .iter()
                .next(),
            Some(Script::Inherited)
        );

        assert!(
            swe.get_script_extensions_val32(0x11303) // GRANTHA SIGN VISARGA
                .contains(&Script::Grantha)
        );

        assert!(
            !swe.get_script_extensions_val32(0x11303) // GRANTHA SIGN VISARGA
                .contains(&Script::Common)
        );

        // // Invalid code point
        assert_eq!(
            swe.get_script_extensions_val32(0x11_0000) // CODE_POINT_MAX + 1 is invalid
                .iter()
                .collect::<Vec<_>>(),
            [Script::Unknown]
        );
    }

    #[test]
    fn test_has_script() {
        let provider = SourceDataProvider::new_testing();

        let swe =
            icu::properties::script::ScriptWithExtensions::try_new_unstable(&provider).unwrap();
        let swe = swe.as_borrowed();

        assert!(swe.has_script('𐓐', Script::Osage));
        assert!(!swe.has_script('𐓐', Script::Common));
        assert!(!swe.has_script('𐓐', Script::Inherited));

        assert!(swe.has_script('🥳', Script::Common));
        assert!(!swe.has_script('🥳', Script::Inherited));

        assert!(!swe.has_script32(0x200D, Script::Common));
        assert!(swe.has_script32(0x200D, Script::Inherited));

        assert!(swe.has_script('௫', Script::Tamil));
        assert!(swe.has_script('௫', Script::Grantha));
        assert!(!swe.has_script('௫', Script::Common));
        assert!(!swe.has_script('௫', Script::Inherited));

        assert!(swe.has_script32(0x11303, Script::Tamil));
        assert!(swe.has_script32(0x11303, Script::Grantha));
        assert!(!swe.has_script32(0x11303, Script::Common));
        assert!(!swe.has_script32(0x11303, Script::Inherited));

        assert!(swe.has_script32(0x30A0, Script::Hiragana));
        assert!(swe.has_script32(0x30A0, Script::Katakana));
        assert!(!swe.has_script32(0x30A0, Script::Common));
        assert!(!swe.has_script32(0x30A0, Script::Inherited));

        // U+0964 DEVANAGARI DANDA
        assert!(!swe.has_script32(0x0964, Script::Common));
        assert!(swe.has_script32(0x0964, Script::Devanagari));
        assert!(swe.has_script32(0x0964, Script::Bengali));

        // TestHasScript() test cases from ICU4J

        // U+063F ARABIC LETTER FARSI YEH WITH THREE DOTS ABOVE
        assert!(!swe.has_script32(0x063F, Script::Common));
        assert!(swe.has_script32(0x063F, Script::Arabic)); // main Script value
        assert!(!swe.has_script32(0x063F, Script::Syriac));
        assert!(!swe.has_script32(0x063F, Script::Thaana));

        // U+0640 ARABIC TATWEEL
        assert!(!swe.has_script32(0x0640, Script::Common)); // main Script value
        assert!(swe.has_script32(0x0640, Script::Arabic));
        assert!(swe.has_script32(0x0640, Script::Syriac));
        assert!(!swe.has_script32(0x0640, Script::Thaana));

        // U+0650 ARABIC KASRA
        assert!(!swe.has_script32(0x0650, Script::Inherited)); // main Script value
        assert!(swe.has_script32(0x0650, Script::Arabic));
        assert!(swe.has_script32(0x0650, Script::Syriac));
        assert!(!swe.has_script32(0x0650, Script::Thaana));

        // U+0660 ARABIC-INDIC DIGIT ZERO
        assert!(!swe.has_script32(0x0660, Script::Common));
        assert!(swe.has_script32(0x0660, Script::Arabic)); // main Script value
        assert!(!swe.has_script32(0x0660, Script::Syriac));
        assert!(swe.has_script32(0x0660, Script::Thaana));

        // U+FDF2 ARABIC LIGATURE ALLAH ISOLATED FORM
        assert!(!swe.has_script32(0xFDF2, Script::Common));
        assert!(swe.has_script32(0xFDF2, Script::Arabic)); // main Script value
        assert!(!swe.has_script32(0xFDF2, Script::Syriac));
        assert!(swe.has_script32(0xFDF2, Script::Thaana));
    }

    #[test]
    fn test_get_script_extensions_set() {
        let provider = SourceDataProvider::new_testing();

        let swe =
            icu::properties::script::ScriptWithExtensions::try_new_unstable(&provider).unwrap();
        let swe = swe.as_borrowed();

        let grantha = swe.get_script_extensions_set(Script::Grantha);
        assert!(!grantha.contains32(0x0BE5)); // unknown with unknown script in Tamil block
        assert!(grantha.contains32(0x0BE6)); // TAMIL DIGIT ZERO
        assert!(grantha.contains32(0x0BEB)); // TAMIL DIGIT FIVE
        assert!(grantha.contains32(0x0BEF)); // TAMIL DIGIT NINE
        assert!(grantha.contains32(0x0BF2)); // TAMIL NUMBER ONE THOUSAND
        assert!(grantha.contains32(0x0BF3)); // TAMIL DAY SIGN
        assert!(!grantha.contains32(0x0BF4)); // TAMIL MONTH SIGN
        assert!(grantha.contains32(0x11300)); // GRANTHA SIGN COMBINING ANUSVARA ABOVE
        assert!(grantha.contains32(0x11301)); // GRANTHA SIGN CANDRABINDU
        assert!(grantha.contains32(0x11302)); // GRANTHA SIGN ANUSVARA
        assert!(grantha.contains32(0x11303)); // GRANTHA SIGN VISARGA
        assert!(!grantha.contains32(0x11304)); // unknown with unknown script in Grantha block
        assert!(grantha.contains32(0x11305)); // GRANTHA LETTER A

        let tamil = swe.get_script_extensions_set(Script::Tamil);
        assert!(!tamil.contains32(0x0BE5)); // unknown with unknown script in Tamil block
        assert!(tamil.contains32(0x0BE6)); // TAMIL DIGIT ZERO
        assert!(tamil.contains32(0x0BEB)); // TAMIL DIGIT FIVE
        assert!(tamil.contains32(0x0BEF)); // TAMIL DIGIT NINE
        assert!(tamil.contains32(0x0BF2)); // TAMIL NUMBER ONE THOUSAND
        assert!(tamil.contains32(0x0BF3)); // TAMIL DAY SIGN
        assert!(tamil.contains32(0x0BF4)); // TAMIL MONTH SIGN
        assert!(!tamil.contains32(0x11300)); // GRANTHA SIGN COMBINING ANUSVARA ABOVE
        assert!(tamil.contains32(0x11301)); // GRANTHA SIGN CANDRABINDU
        assert!(!tamil.contains32(0x11302)); // GRANTHA SIGN ANUSVARA
        assert!(tamil.contains32(0x11303)); // GRANTHA SIGN VISARGA
        assert!(!tamil.contains32(0x11304)); // unknown with unknown script in Grantha block
        assert!(!tamil.contains32(0x11305)); // GRANTHA LETTER A

        let hiragana = swe.get_script_extensions_set(Script::Hiragana);
        assert!(hiragana.contains32(0x3046)); // HIRAGANA LETTER U
        assert!(hiragana.contains32(0x309F)); // HIRAGANA DIGRAPH YORI
        assert!(hiragana.contains32(0x30A0)); // KATAKANA-HIRAGANA DOUBLE HYPHEN
        assert!(!hiragana.contains32(0x30A1)); // KATAKANA LETTER SMALL A
        assert!(hiragana.contains32(0x30FB)); // KATAKANA MIDDLE DOT
        assert!(hiragana.contains32(0x30FC)); // KATAKANA-HIRAGANA PROLONGED SOUND MARK
        assert!(!hiragana.contains32(0x30FD)); // KATAKANA ITERATION MARK

        let katakana = swe.get_script_extensions_set(Script::Katakana);
        assert!(!katakana.contains32(0x3046)); // HIRAGANA LETTER U
        assert!(!katakana.contains32(0x309F)); // HIRAGANA DIGRAPH YORI
        assert!(katakana.contains32(0x30A0)); // KATAKANA-HIRAGANA DOUBLE HYPHEN
        assert!(katakana.contains32(0x30A1)); // KATAKANA LETTER SMALL A
        assert!(katakana.contains32(0x30FB)); // KATAKANA MIDDLE DOT
        assert!(katakana.contains32(0x30FC)); // KATAKANA-HIRAGANA PROLONGED SOUND MARK
        assert!(katakana.contains32(0x30FD)); // KATAKANA ITERATION MARK

        let common = swe.get_script_extensions_set(Script::Common);
        assert!(common.contains('🥳'));
        assert!(!common.contains32(0x200D));
        assert!(!common.contains32(0x30A0));

        let inherited = swe.get_script_extensions_set(Script::Inherited);
        assert!(!inherited.contains('🥳'));
        assert!(inherited.contains32(0x200D));
        assert!(!inherited.contains32(0x30A0));

        // inspired by https://github.com/unicode-org/unicodetools/issues/192

        let bangla = swe.get_script_extensions_set(Script::Bengali);
        assert!(bangla.contains32(0x09E7)); // BENGALI DIGIT ONE
        assert!(!bangla.contains32(0x0963)); // DEVANAGARI VOWEL SIGN VOCALIC LL
        assert!(bangla.contains32(0x0964)); // DEVANAGARI DANDA
        assert!(bangla.contains32(0x0965)); // DEVANAGARI DOUBLE DANDA
        assert!(!bangla.contains32(0x0966)); // DEVANAGARI DIGIT ZERO

        let devanagari = swe.get_script_extensions_set(Script::Devanagari);
        assert!(!devanagari.contains32(0x09E7)); // BENGALI DIGIT ONE
        assert!(devanagari.contains32(0x0963)); // DEVANAGARI VOWEL SIGN VOCALIC LL
        assert!(devanagari.contains32(0x0964)); // DEVANAGARI DANDA
        assert!(devanagari.contains32(0x0965)); // DEVANAGARI DOUBLE DANDA
        assert!(devanagari.contains32(0x0966)); // DEVANAGARI DIGIT ZERO

        assert!(!common.contains32(0x0964)); // DEVANAGARI DANDA
        assert!(!common.contains32(0x0965)); // DEVANAGARI DOUBLE DANDA
    }
}
