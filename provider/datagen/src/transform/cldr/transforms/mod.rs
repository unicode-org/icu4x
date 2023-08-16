// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_locid::locale;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_transliteration::provider::*;

impl DataProvider<TransliteratorRulesV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<TransliteratorRulesV1Marker>, DataError> {
        self.check_req::<TransliteratorRulesV1Marker>(req)?;

        let metadata = self
            .source
            .cldr()?
            .transforms()
            .read_and_parse_metadata("de-ASCII")?;

        let source: String = self.source.cldr()?.transforms().read_source("de-ASCII")?;

        todo!();
    }
}

impl IterableDataProvider<TransliteratorRulesV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .transforms()
            .list_transforms()?
            .map(|transform| {
                // TODO: conversion to DataLocale with aux key
                // use metadata to get the bcp47 ID
                DataLocale::from(locale!("und"))
            })
            .collect())
    }
}

// /// Substring used to denote alternative region names data variants for a given region. For example: "BA-alt-short", "TL-alt-variant".
// const ALT_SUBSTRING: &str = "-alt-";
// /// Substring used to denote short display names data variants for a given language. For example: "az-alt-short".
// const ALT_SHORT_SUBSTRING: &str = "-alt-short";
// /// Substring used to denote long display names data variants for a given language. For example: "az-alt-long".
// const ALT_LONG_SUBSTRING: &str = "-alt-long";
// /// Substring used to denote menu display names data variants for a given language. For example: "az-alt-menu".
// const ALT_MENU_SUBSTRING: &str = "-alt-menu";

// impl From<&cldr_serde::displaynames::language::Resource> for LanguageDisplayNamesV1<'static> {
//     fn from(other: &cldr_serde::displaynames::language::Resource) -> Self {
//         let mut names = BTreeMap::new();
//         let mut short_names = BTreeMap::new();
//         let mut long_names = BTreeMap::new();
//         let mut menu_names = BTreeMap::new();
//         for entry in other.main.value.localedisplaynames.languages.iter() {
//             if let Some(lang) = entry.0.strip_suffix(ALT_SHORT_SUBSTRING) {
//                 if let Ok(lang) = lang.parse::<Language>() {
//                     short_names.insert(lang.into_tinystr(), entry.1.as_ref());
//                 }
//             } else if let Some(lang) = entry.0.strip_suffix(ALT_LONG_SUBSTRING) {
//                 if let Ok(lang) = lang.parse::<Language>() {
//                     long_names.insert(lang.into_tinystr(), entry.1.as_ref());
//                 }
//             } else if let Some(lang) = entry.0.strip_suffix(ALT_MENU_SUBSTRING) {
//                 if let Ok(lang) = lang.parse::<Language>() {
//                     menu_names.insert(lang.into_tinystr(), entry.1.as_ref());
//                 }
//             } else if let Ok(lang) = entry.0.parse::<Language>() {
//                 names.insert(lang.into_tinystr(), entry.1.as_ref());
//             }
//         }
//         Self {
//             // Old CLDR versions may contain trivial entries, so filter
//             names: names
//                 .into_iter()
//                 .filter(|&(k, v)| k != v)
//                 .map(|(k, v)| (k.to_unvalidated(), v))
//                 .collect(),
//             short_names: short_names
//                 .into_iter()
//                 .filter(|&(k, v)| k != v)
//                 .map(|(k, v)| (k.to_unvalidated(), v))
//                 .collect(),
//             long_names: long_names
//                 .into_iter()
//                 .filter(|&(k, v)| k != v)
//                 .map(|(k, v)| (k.to_unvalidated(), v))
//                 .collect(),
//             menu_names: menu_names
//                 .into_iter()
//                 .filter(|&(k, v)| k != v)
//                 .map(|(k, v)| (k.to_unvalidated(), v))
//                 .collect(),
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let provider = crate::DatagenProvider::for_test();

        let data: DataPayload<TransliteratorRulesV1Marker> = provider
            .load(DataRequest {
                locale: &locale!("und").into(),
                metadata: Default::default(),
            })
            .unwrap()
            .take_payload()
            .unwrap();

        eprintln!();
        panic!();
    }
}
