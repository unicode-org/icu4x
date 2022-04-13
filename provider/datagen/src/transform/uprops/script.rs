// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::uprops::uprops_helpers;
use crate::transform::uprops::uprops_serde::script_extensions::ScriptWithExtensionsProperty;
use crate::SourceData;
use icu_codepointtrie::CodePointTrie;
use icu_properties::provider::{
    key, ScriptWithExtensionsPropertyV1, ScriptWithExtensionsPropertyV1Marker,
};
use icu_properties::script::{ScriptWithExt, ScriptWithExtensions};
use icu_properties::Script;
use icu_provider::datagen::IterableDynProvider;
use icu_provider::prelude::*;
use std::convert::TryFrom;
use zerovec::{VarZeroVec, ZeroSlice, ZeroVec};

/// A data provider reading from TOML files produced by the ICU4C icuwriteuprops tool.
///
/// This data provider returns a [`ScriptWithExtensions`] instance,
/// which efficiently represents data for the Script and Script_Extensions
/// properties.
pub struct ScriptWithExtensionsPropertyProvider {
    source: SourceData,
}

impl From<&SourceData> for ScriptWithExtensionsPropertyProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

// source data to ICU4X plain/raw/utility data structure
impl TryFrom<&ScriptWithExtensionsProperty> for ScriptWithExtensions<'static> {
    type Error = DataError;

    fn try_from(
        scx_data: &ScriptWithExtensionsProperty,
    ) -> Result<ScriptWithExtensions<'static>, DataError> {
        let cpt_data = &scx_data.code_point_trie;
        let scx_array_data = &scx_data.script_code_array;

        let trie = CodePointTrie::<ScriptWithExt>::try_from(cpt_data)?;

        // Convert the input from Vec<Vec<u16>> to Vec<ZeroVec<Script>> so that
        // we can go through the VarZeroVec construction process for a desired result
        // type of VZV<ZeroSlice<Script>>
        let ule_scx_array_data: Vec<ZeroVec<Script>> = scx_array_data
            .iter()
            .map(|v| v.iter().map(|i| Script(*i)).collect::<ZeroVec<Script>>())
            .collect::<Vec<ZeroVec<Script>>>();
        let scx_vzv: VarZeroVec<ZeroSlice<Script>> =
            VarZeroVec::from(ule_scx_array_data.as_slice());

        Ok(ScriptWithExtensions::new(trie, scx_vzv))
    }
}

// source data to ICU4X provider data struct conversion
impl TryFrom<&ScriptWithExtensionsProperty> for ScriptWithExtensionsPropertyV1<'static> {
    type Error = DataError;

    fn try_from(
        scx_data: &ScriptWithExtensionsProperty,
    ) -> Result<ScriptWithExtensionsPropertyV1<'static>, DataError> {
        let swe = ScriptWithExtensions::try_from(scx_data);
        swe.map(|s| ScriptWithExtensionsPropertyV1 { data: s })
    }
}

// implement data provider
impl DynProvider<ScriptWithExtensionsPropertyV1Marker> for ScriptWithExtensionsPropertyProvider {
    fn load_payload(
        &self,
        key: ResourceKey,
        _: &DataRequest,
    ) -> Result<DataResponse<ScriptWithExtensionsPropertyV1Marker>, DataError> {
        if uprops_helpers::get_last_component_no_version(key) != "scx" {
            return Err(DataErrorKind::MissingResourceKey.into_error());
        }

        let source_scx_data =
            uprops_helpers::load_script_extensions_from_dir(self.source.get_uprops_root()?)?;

        let data_struct = ScriptWithExtensionsPropertyV1::try_from(&source_scx_data)?;

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(data_struct)),
        })
    }
}

icu_provider::impl_dyn_provider!(ScriptWithExtensionsPropertyProvider, {
    key::SCRIPT_EXTENSIONS_V1 => ScriptWithExtensionsPropertyV1Marker,
}, SERDE_SE, ITERABLE_SERDE_SE, DATA_CONVERTER);

impl IterableDynProvider<ScriptWithExtensionsPropertyV1Marker>
    for ScriptWithExtensionsPropertyProvider
{
    fn supported_options_for_key(
        &self,
        _: ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_properties::provider::key;

    #[test]
    fn test_script_val_from_script_extensions() {
        let provider = ScriptWithExtensionsPropertyProvider::from(&SourceData::for_test());

        let payload: DataPayload<ScriptWithExtensionsPropertyV1Marker> = provider
            .load_payload(
                key::SCRIPT_EXTENSIONS_V1,
                &DataRequest {
                    options: ResourceOptions::default(),
                    metadata: Default::default(),
                },
            )
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let swe: &ScriptWithExtensions = &payload.get().data;

        assert_eq!(swe.get_script_val('𐓐' as u32), Script::Osage); // U+104D0 OSAGE CAPITAL LETTER KHA
        assert_eq!(swe.get_script_val('🥳' as u32), Script::Common); // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
        assert_eq!(swe.get_script_val(0x200D), Script::Inherited); // ZERO WIDTH JOINER
        assert_eq!(swe.get_script_val('௫' as u32), Script::Tamil); // U+0BEB TAMIL DIGIT FIVE
        assert_eq!(swe.get_script_val(0x11303), Script::Grantha); // GRANTHA SIGN VISARGA
        assert_eq!(swe.get_script_val(0x30A0), Script::Common); // U+30A0 KATAKANA-HIRAGANA DOUBLE HYPHEN
    }

    #[test]
    fn test_scx_array_from_script_extensions() {
        use zerovec::ZeroVec;

        let provider = ScriptWithExtensionsPropertyProvider::from(&SourceData::for_test());

        let payload: DataPayload<ScriptWithExtensionsPropertyV1Marker> = provider
            .load_payload(
                key::SCRIPT_EXTENSIONS_V1,
                &DataRequest {
                    options: ResourceOptions::default(),
                    metadata: Default::default(),
                },
            )
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let swe: &ScriptWithExtensions = &payload.get().data;

        assert_eq!(
            swe.get_script_extensions_val('𐓐' as u32).as_zerovec(), // U+104D0 OSAGE CAPITAL LETTER KHA
            ZeroVec::<Script>::alloc_from_slice(&[Script::Osage])
        );
        assert_eq!(
            swe.get_script_extensions_val('🥳' as u32).as_zerovec(), // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
            ZeroVec::<Script>::alloc_from_slice(&[Script::Common])
        );
        assert_eq!(
            swe.get_script_extensions_val(0x200D).as_zerovec(), // ZERO WIDTH JOINER
            ZeroVec::<Script>::alloc_from_slice(&[Script::Inherited])
        );
        assert_eq!(
            swe.get_script_extensions_val('௫' as u32).as_zerovec(), // U+0BEB TAMIL DIGIT FIVE
            ZeroVec::<Script>::alloc_from_slice(&[Script::Tamil, Script::Grantha])
        );
        assert_eq!(
            swe.get_script_extensions_val(0x11303).as_zerovec(), // GRANTHA SIGN VISARGA
            ZeroVec::<Script>::alloc_from_slice(&[Script::Tamil, Script::Grantha])
        );
        assert_eq!(
            swe.get_script_extensions_val(0x30A0).as_zerovec(), // KATAKANA-HIRAGANA DOUBLE HYPHEN
            ZeroVec::<Script>::alloc_from_slice(&[Script::Hiragana, Script::Katakana])
        );

        // Invalid code point
        assert_eq!(
            swe.get_script_extensions_val(0x11_0000).as_zerovec(), // CODE_POINT_MAX + 1 is invalid
            ZeroVec::<Script>::alloc_from_slice(&[Script::Unknown])
        );
    }

    #[test]
    fn test_has_script() {
        let provider = ScriptWithExtensionsPropertyProvider::from(&SourceData::for_test());

        let payload: DataPayload<ScriptWithExtensionsPropertyV1Marker> = provider
            .load_payload(key::SCRIPT_EXTENSIONS_V1, &DataRequest::default())
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let swe: &ScriptWithExtensions = &payload.get().data;

        assert!(swe.has_script('𐓐' as u32, Script::Osage));
        assert!(!swe.has_script('𐓐' as u32, Script::Common));
        assert!(!swe.has_script('𐓐' as u32, Script::Inherited));

        assert!(swe.has_script('🥳' as u32, Script::Common));
        assert!(!swe.has_script('🥳' as u32, Script::Inherited));

        assert!(!swe.has_script(0x200D, Script::Common));
        assert!(swe.has_script(0x200D, Script::Inherited));

        assert!(swe.has_script('௫' as u32, Script::Tamil));
        assert!(swe.has_script('௫' as u32, Script::Grantha));
        assert!(!swe.has_script('௫' as u32, Script::Common));
        assert!(!swe.has_script('௫' as u32, Script::Inherited));

        assert!(swe.has_script(0x11303, Script::Tamil));
        assert!(swe.has_script(0x11303, Script::Grantha));
        assert!(!swe.has_script(0x11303, Script::Common));
        assert!(!swe.has_script(0x11303, Script::Inherited));

        assert!(swe.has_script(0x30A0, Script::Hiragana));
        assert!(swe.has_script(0x30A0, Script::Katakana));
        assert!(!swe.has_script(0x30A0, Script::Common));
        assert!(!swe.has_script(0x30A0, Script::Inherited));

        // U+0964 DEVANAGARI DANDA
        assert!(!swe.has_script(0x0964, Script::Common));
        assert!(swe.has_script(0x0964, Script::Devanagari));
        assert!(swe.has_script(0x0964, Script::Bengali));

        // TestHasScript() test cases from ICU4J

        // U+063F ARABIC LETTER FARSI YEH WITH THREE DOTS ABOVE
        assert!(!swe.has_script(0x063F, Script::Common));
        assert!(swe.has_script(0x063F, Script::Arabic)); // main Script value
        assert!(!swe.has_script(0x063F, Script::Syriac));
        assert!(!swe.has_script(0x063F, Script::Thaana));

        // U+0640 ARABIC TATWEEL
        assert!(!swe.has_script(0x0640, Script::Common)); // main Script value
        assert!(swe.has_script(0x0640, Script::Arabic));
        assert!(swe.has_script(0x0640, Script::Syriac));
        assert!(!swe.has_script(0x0640, Script::Thaana));

        // U+0650 ARABIC KASRA
        assert!(!swe.has_script(0x0650, Script::Inherited)); // main Script value
        assert!(swe.has_script(0x0650, Script::Arabic));
        assert!(swe.has_script(0x0650, Script::Syriac));
        assert!(!swe.has_script(0x0650, Script::Thaana));

        // U+0660 ARABIC-INDIC DIGIT ZERO
        assert!(!swe.has_script(0x0660, Script::Common));
        assert!(swe.has_script(0x0660, Script::Arabic)); // main Script value
        assert!(!swe.has_script(0x0660, Script::Syriac));
        assert!(swe.has_script(0x0660, Script::Thaana));

        // U+FDF2 ARABIC LIGATURE ALLAH ISOLATED FORM
        assert!(!swe.has_script(0xFDF2, Script::Common));
        assert!(swe.has_script(0xFDF2, Script::Arabic)); // main Script value
        assert!(!swe.has_script(0xFDF2, Script::Syriac));
        assert!(swe.has_script(0xFDF2, Script::Thaana));

        // The ICU4J comment for this test says:
        // An unguarded implementation might go into an infinite loop.
        assert!(!swe.has_script(0x0640, Script(0xAFFE)));
    }

    #[test]
    fn test_get_script_extensions_set() {
        let provider = ScriptWithExtensionsPropertyProvider::from(&SourceData::for_test());

        let payload: DataPayload<ScriptWithExtensionsPropertyV1Marker> = provider
            .load_payload(key::SCRIPT_EXTENSIONS_V1, &DataRequest::default())
            .expect("The data should be valid")
            .take_payload()
            .expect("Loading was successful");

        let swe: &ScriptWithExtensions = &payload.get().data;

        let grantha = swe.get_script_extensions_set(Script::Grantha);
        assert!(!grantha.contains_u32(0x0BE5)); // unknown with unknown script in Tamil block
        assert!(grantha.contains_u32(0x0BE6)); // TAMIL DIGIT ZERO
        assert!(grantha.contains_u32(0x0BEB)); // TAMIL DIGIT FIVE
        assert!(grantha.contains_u32(0x0BEF)); // TAMIL DIGIT NINE
        assert!(grantha.contains_u32(0x0BF2)); // TAMIL NUMBER ONE THOUSAND
        assert!(grantha.contains_u32(0x0BF3)); // TAMIL DAY SIGN
        assert!(!grantha.contains_u32(0x0BF4)); // TAMIL MONTH SIGN
        assert!(grantha.contains_u32(0x11300)); // GRANTHA SIGN COMBINING ANUSVARA ABOVE
        assert!(grantha.contains_u32(0x11301)); // GRANTHA SIGN CANDRABINDU
        assert!(grantha.contains_u32(0x11302)); // GRANTHA SIGN ANUSVARA
        assert!(grantha.contains_u32(0x11303)); // GRANTHA SIGN VISARGA
        assert!(!grantha.contains_u32(0x11304)); // unknown with unknown script in Grantha block
        assert!(grantha.contains_u32(0x11305)); // GRANTHA LETTER A

        let tamil = swe.get_script_extensions_set(Script::Tamil);
        assert!(!tamil.contains_u32(0x0BE5)); // unknown with unknown script in Tamil block
        assert!(tamil.contains_u32(0x0BE6)); // TAMIL DIGIT ZERO
        assert!(tamil.contains_u32(0x0BEB)); // TAMIL DIGIT FIVE
        assert!(tamil.contains_u32(0x0BEF)); // TAMIL DIGIT NINE
        assert!(tamil.contains_u32(0x0BF2)); // TAMIL NUMBER ONE THOUSAND
        assert!(tamil.contains_u32(0x0BF3)); // TAMIL DAY SIGN
        assert!(tamil.contains_u32(0x0BF4)); // TAMIL MONTH SIGN
        assert!(!tamil.contains_u32(0x11300)); // GRANTHA SIGN COMBINING ANUSVARA ABOVE
        assert!(tamil.contains_u32(0x11301)); // GRANTHA SIGN CANDRABINDU
        assert!(!tamil.contains_u32(0x11302)); // GRANTHA SIGN ANUSVARA
        assert!(tamil.contains_u32(0x11303)); // GRANTHA SIGN VISARGA
        assert!(!tamil.contains_u32(0x11304)); // unknown with unknown script in Grantha block
        assert!(!tamil.contains_u32(0x11305)); // GRANTHA LETTER A

        let hiragana = swe.get_script_extensions_set(Script::Hiragana);
        assert!(hiragana.contains_u32(0x3046)); // HIRAGANA LETTER U
        assert!(hiragana.contains_u32(0x309F)); // HIRAGANA DIGRAPH YORI
        assert!(hiragana.contains_u32(0x30A0)); // KATAKANA-HIRAGANA DOUBLE HYPHEN
        assert!(!hiragana.contains_u32(0x30A1)); // KATAKANA LETTER SMALL A
        assert!(hiragana.contains_u32(0x30FB)); // KATAKANA MIDDLE DOT
        assert!(hiragana.contains_u32(0x30FC)); // KATAKANA-HIRAGANA PROLONGED SOUND MARK
        assert!(!hiragana.contains_u32(0x30FD)); // KATAKANA ITERATION MARK

        let katakana = swe.get_script_extensions_set(Script::Katakana);
        assert!(!katakana.contains_u32(0x3046)); // HIRAGANA LETTER U
        assert!(!katakana.contains_u32(0x309F)); // HIRAGANA DIGRAPH YORI
        assert!(katakana.contains_u32(0x30A0)); // KATAKANA-HIRAGANA DOUBLE HYPHEN
        assert!(katakana.contains_u32(0x30A1)); // KATAKANA LETTER SMALL A
        assert!(katakana.contains_u32(0x30FB)); // KATAKANA MIDDLE DOT
        assert!(katakana.contains_u32(0x30FC)); // KATAKANA-HIRAGANA PROLONGED SOUND MARK
        assert!(katakana.contains_u32(0x30FD)); // KATAKANA ITERATION MARK

        let common = swe.get_script_extensions_set(Script::Common);
        assert!(common.contains('🥳'));
        assert!(!common.contains_u32(0x200D));
        assert!(!common.contains_u32(0x30A0));

        let inherited = swe.get_script_extensions_set(Script::Inherited);
        assert!(!inherited.contains('🥳'));
        assert!(inherited.contains_u32(0x200D));
        assert!(!inherited.contains_u32(0x30A0));

        // inspired by https://github.com/unicode-org/unicodetools/issues/192

        let bengali = swe.get_script_extensions_set(Script::Bengali);
        assert!(bengali.contains_u32(0x09E7)); // BENGALI DIGIT ONE
        assert!(!bengali.contains_u32(0x0963)); // DEVANAGARI VOWEL SIGN VOCALIC LL
        assert!(bengali.contains_u32(0x0964)); // DEVANAGARI DANDA
        assert!(bengali.contains_u32(0x0965)); // DEVANAGARI DOUBLE DANDA
        assert!(!bengali.contains_u32(0x0966)); // DEVANAGARI DIGIT ZERO

        let devanagari = swe.get_script_extensions_set(Script::Devanagari);
        assert!(!devanagari.contains_u32(0x09E7)); // BENGALI DIGIT ONE
        assert!(devanagari.contains_u32(0x0963)); // DEVANAGARI VOWEL SIGN VOCALIC LL
        assert!(devanagari.contains_u32(0x0964)); // DEVANAGARI DANDA
        assert!(devanagari.contains_u32(0x0965)); // DEVANAGARI DOUBLE DANDA
        assert!(devanagari.contains_u32(0x0966)); // DEVANAGARI DIGIT ZERO

        assert!(!common.contains_u32(0x0964)); // DEVANAGARI DANDA
        assert!(!common.contains_u32(0x0965)); // DEVANAGARI DOUBLE DANDA
    }
}
