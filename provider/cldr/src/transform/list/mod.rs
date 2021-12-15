// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::CldrPaths;
use icu_list::provider::*;
use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::convert::TryFrom;

/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 3] = [
    key::LIST_FORMAT_AND_V1,
    key::LIST_FORMAT_OR_V1,
    key::LIST_FORMAT_UNIT_V1,
];

/// A data provider reading from CLDR JSON list rule files.
#[derive(PartialEq, Debug)]
pub struct ListProvider {
    data: LiteMap<LanguageIdentifier, cldr_serde::list_patterns::LangListPatterns>,
    uprops_path: std::path::PathBuf,
}

impl TryFrom<&dyn CldrPaths> for ListProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = LiteMap::new();
        for dir in get_langid_subdirectories(&cldr_paths.cldr_misc()?.join("main"))? {
            let path = dir.join("listPatterns.json");
            let resource: cldr_serde::list_patterns::Resource =
                serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
            data.extend_from_litemap(resource.main.0);
        }
        Ok(Self {
            data,
            uprops_path: cldr_paths.uprops()?,
        })
    }
}

impl KeyedDataProvider for ListProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        key::LIST_FORMAT_AND_V1
            .match_key(*resc_key)
            .or_else(|_| key::LIST_FORMAT_OR_V1.match_key(*resc_key))
            .or_else(|_| key::LIST_FORMAT_UNIT_V1.match_key(*resc_key))
    }
}

impl DataProvider<ListFormatterPatternsV1Marker> for ListProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<ListFormatterPatternsV1Marker>, DataError> {
        Self::supports_key(&req.resource_path.key)?;
        let langid = req.try_langid()?;
        let data = match self.data.get(langid) {
            Some(v) => &v.list_patterns,
            None => return Err(DataError::MissingResourceOptions(req.clone())),
        };

        let mut patterns = match req.resource_path.key {
            key::LIST_FORMAT_AND_V1 => parse_and_patterns(data),
            key::LIST_FORMAT_OR_V1 => parse_or_patterns(data),
            key::LIST_FORMAT_UNIT_V1 => parse_unit_patterns(data),
            _ => unreachable!(),
        }
        .map_err(DataError::new_resc_error)?;

        if langid.language == langid!("es").language {
            match req.resource_path.key {
                // Replace " y " with " e " before /i/ sounds.
                // https://unicode.org/reports/tr35/tr35-general.html#:~:text=important.%20For%20example%3A-,Spanish,AND,-Use%20%E2%80%98e%E2%80%99%20instead
                key::LIST_FORMAT_AND_V1 | key::LIST_FORMAT_UNIT_V1 => patterns
                    .make_conditional(
                        "{0} y {1}",
                        // Starts with i or (hi but not hia/hie)
                        "i|hi([^ae]|$)",
                        "{0} e {1}",
                    )
                    .unwrap(),
                // Replace " o " with " u " before /o/ sound.
                // https://unicode.org/reports/tr35/tr35-general.html#:~:text=agua%20e%20hielo-,OR,-Use%20%E2%80%98u%E2%80%99%20instead
                key::LIST_FORMAT_OR_V1 => patterns
                    .make_conditional(
                        "{0} o {1}",
                        // Starts with o, ho, 8 (including 80, 800, ...), or 11 either alone or followed
                        // by thousand groups and/or decimals (excluding e.g. 110, 1100, ...)
                        r"o|ho|8|(11(\.?\d\d\d)*(,\d*)?([^\.,\d]|$))",
                        "{0} u {1}",
                    )
                    .unwrap(),
                _ => unreachable!(),
            }
        }

        if langid.language == langid!("he").language {
            // Add dashes between ו and non-Hebrew characters
            // https://unicode.org/reports/tr35/tr35-general.html#:~:text=is%20not%20mute.-,Hebrew,AND,-Use%20%E2%80%98%2D%D7%95%E2%80%99%20instead
            patterns
                .make_conditional(
                    "{0} \u{05D5}{1}", // ״{0} ו {1}״
                    // Starts with a non-Hebrew letter
                    &format!(
                        "[^{}]",
                        icu_properties::sets::get_for_script(
                            &icu_provider_uprops::PropertiesDataProvider::try_new(
                                &self.uprops_path
                            )
                            .map_err(DataError::new_resc_error)?,
                            icu_properties::Script::Hebrew,
                        )
                        .map_err(DataError::new_resc_error)?
                        .get()
                        .inv_list
                        .iter_ranges()
                        .map(|range| format!(r#"\u{:04x}-\u{:04x}"#, range.start(), range.end()))
                        .fold(String::new(), |a, b| a + &b)
                    ),
                    "{0} \u{05D5}-{1}", // ״{0} ו- {1}״
                )
                .unwrap();
        }

        let metadata = DataResponseMetadata::default();
        // TODO(#1109): Set metadata.data_langid correctly.
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(patterns)),
        })
    }
}

icu_provider::impl_dyn_provider!(ListProvider, {
    _ => ListFormatterPatternsV1Marker,
}, SERDE_SE);

impl IterableDataProviderCore for ListProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .data
            .iter()
            // ur-IN has a buggy pattern ("{1}, {0}") which violates
            // our invariant that {0} is at index 0 (and rotates the output).
            // See https://github.com/unicode-org/icu4x/issues/1282
            .filter(|(l, _)| *l != &icu_locid_macros::langid!("ur-IN"))
            .map(|(l, _)| ResourceOptions {
                variant: None,
                langid: Some(l.clone()),
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}

fn parse_and_patterns<'a>(
    raw: &cldr_serde::list_patterns::ListPatterns,
) -> Result<ListFormatterPatternsV1<'a>, icu_list::error::Error> {
    ListFormatterPatternsV1::try_new([
        &raw.standard.start,
        &raw.standard.middle,
        &raw.standard.end,
        &raw.standard.pair,
        &raw.standard_short.start,
        &raw.standard_short.middle,
        &raw.standard_short.end,
        &raw.standard_short.pair,
        &raw.standard_narrow.start,
        &raw.standard_narrow.middle,
        &raw.standard_narrow.end,
        &raw.standard_narrow.pair,
    ])
}

fn parse_or_patterns<'a>(
    raw: &cldr_serde::list_patterns::ListPatterns,
) -> Result<ListFormatterPatternsV1<'a>, icu_list::error::Error> {
    ListFormatterPatternsV1::try_new([
        &raw.or.start,
        &raw.or.middle,
        &raw.or.end,
        &raw.or.pair,
        &raw.or_short.start,
        &raw.or_short.middle,
        &raw.or_short.end,
        &raw.or_short.pair,
        &raw.or_narrow.start,
        &raw.or_narrow.middle,
        &raw.or_narrow.end,
        &raw.or_narrow.pair,
    ])
}

fn parse_unit_patterns<'a>(
    raw: &cldr_serde::list_patterns::ListPatterns,
) -> Result<ListFormatterPatternsV1<'a>, icu_list::error::Error> {
    ListFormatterPatternsV1::try_new([
        &raw.unit.start,
        &raw.unit.middle,
        &raw.unit.end,
        &raw.unit.pair,
        &raw.unit_short.start,
        &raw.unit_short.middle,
        &raw.unit_short.end,
        &raw.unit_short.pair,
        &raw.unit_narrow.start,
        &raw.unit_narrow.middle,
        &raw.unit_narrow.end,
        &raw.unit_narrow.pair,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu_list::options::Width;
    use icu_locid::LanguageIdentifier;
    use icu_locid_macros::langid;

    fn provide(
        lang: LanguageIdentifier,
        key: ResourceKey,
    ) -> DataPayload<ListFormatterPatternsV1Marker> {
        let cldr_paths = crate::cldr_paths::for_test();
        let provider = ListProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();
        provider
            .load_payload(&DataRequest {
                resource_path: ResourcePath {
                    key,
                    options: ResourceOptions {
                        variant: None,
                        langid: Some(lang),
                    },
                },
            })
            .unwrap()
            .take_payload()
            .unwrap()
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            provide(langid!("fr"), key::LIST_FORMAT_OR_V1)
                .get()
                .end(Width::Wide)
                .parts(""),
            (" ou ", "")
        );
    }

    #[test]
    fn test_spanish() {
        let y_parts = (" y ", "");
        let e_parts = (" e ", "");
        let o_parts = (" o ", "");
        let u_parts = (" u ", "");

        let payload_and = provide(langid!("es"), key::LIST_FORMAT_AND_V1);
        let and = &payload_and.get().end(Width::Wide);
        let payload_or = provide(langid!("es"), key::LIST_FORMAT_OR_V1);
        let or = &payload_or.get().end(Width::Wide);

        // ... y Mallorca
        assert_eq!(and.parts("Mallorca"), y_parts);
        // ... e Ibiza
        assert_eq!(and.parts("Ibiza"), e_parts);
        // ... e Hidalgo
        assert_eq!(and.parts("Hidalgo"), e_parts);
        // ... y Hierva
        assert_eq!(and.parts("Hierva"), y_parts);

        // ... o Ibiza
        assert_eq!(or.parts("Ibiza"), o_parts);
        // ... u Okinawa
        assert_eq!(or.parts("Okinawa"), u_parts);
        // ... u 8 más
        assert_eq!(or.parts("8 más"), u_parts);
        // ... u 8
        assert_eq!(or.parts("8"), u_parts);
        // ... u 87 más
        assert_eq!(or.parts("87 más"), u_parts);
        // ... u 87
        assert_eq!(or.parts("87"), u_parts);
        // ... u 11 más
        assert_eq!(or.parts("11 más"), u_parts);
        // ... u 11
        assert_eq!(or.parts("11"), u_parts);
        // ... o 110 más
        assert_eq!(or.parts("110 más"), o_parts);
        // ... o 110
        assert_eq!(or.parts("110"), o_parts);
        // ... o 11.000 más
        assert_eq!(or.parts("11.000 más"), u_parts);
        // ... o 11.000
        assert_eq!(or.parts("11.000"), u_parts);
        // ... o 11.000,92 más
        assert_eq!(or.parts("11.000,92 más"), u_parts);
        // ... o 11.000,92
        assert_eq!(or.parts("11.000,92"), u_parts);

        // Works for all es-* locales
        assert_eq!(
            provide(langid!("es-AR"), key::LIST_FORMAT_AND_V1)
                .get()
                .end(Width::Wide)
                .parts("Ibiza"),
            e_parts
        );
    }

    #[test]
    fn test_hebrew() {
        let vav_parts = (" ו", "");
        let vav_dash_parts = (" ו-", "");

        assert_eq!(
            provide(langid!("he"), key::LIST_FORMAT_AND_V1)
                .get()
                .end(Width::Wide)
                .parts("יפו"),
            vav_parts
        );

        assert_eq!(
            provide(langid!("he"), key::LIST_FORMAT_AND_V1)
                .get()
                .end(Width::Wide)
                .parts("Ibiza"),
            vav_dash_parts
        );
    }
}
