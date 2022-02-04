// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::support::KeyedDataProvider;
use crate::CldrPaths;
use icu_list::provider::*;
use icu_locid::LanguageIdentifier;
use icu_locid_macros::langid;
use icu_provider::iter::IterableProvider;
use icu_provider::prelude::*;
use litemap::LiteMap;
use std::convert::TryFrom;

/// A data provider reading from CLDR JSON list rule files.
#[derive(PartialEq, Debug)]
pub struct ListProvider {
    data: LiteMap<LanguageIdentifier, cldr_serde::list_patterns::LangListPatterns>,
    uprops_path: std::path::PathBuf,
}

impl TryFrom<&dyn CldrPaths> for ListProvider {
    type Error = crate::error::Error;
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
    fn supported_keys() -> Vec<ResourceKey> {
        vec![
            key::LIST_FORMAT_AND_V1,
            key::LIST_FORMAT_OR_V1,
            key::LIST_FORMAT_UNIT_V1,
        ]
    }
}

impl DynProvider<ListFormatterPatternsV1Marker> for ListProvider {
    fn load_payload(
        &self,
        key: ResourceKey,
        req: &DataRequest,
    ) -> Result<DataResponse<ListFormatterPatternsV1Marker>, DataError> {
        let langid = req
            .get_langid()
            .ok_or_else(|| DataErrorKind::NeedsLocale.with_req(key, req))?;
        let data = match self.data.get(langid) {
            Some(v) => &v.list_patterns,
            None => return Err(DataErrorKind::MissingLocale.with_req(key, req)),
        };

        let mut patterns = match key {
            key::LIST_FORMAT_AND_V1 => parse_and_patterns(data),
            key::LIST_FORMAT_OR_V1 => parse_or_patterns(data),
            key::LIST_FORMAT_UNIT_V1 => parse_unit_patterns(data),
            _ => return Err(DataErrorKind::MissingResourceKey.with_key(key)),
        }
        .map_err(|e| e.with_req(key, req))?;

        if langid.language == langid!("es").language {
            match key {
                // Replace " y " with " e " before /i/ sounds.
                // https://unicode.org/reports/tr35/tr35-general.html#:~:text=important.%20For%20example%3A-,Spanish,AND,-Use%20%E2%80%98e%E2%80%99%20instead
                key::LIST_FORMAT_AND_V1 | key::LIST_FORMAT_UNIT_V1 => patterns
                    .make_conditional(
                        "{0} y {1}",
                        // Starts with i or (hi but not hia/hie)
                        "i|hi([^ae]|$)",
                        "{0} e {1}",
                    )
                    .expect("Valid regex and pattern"),
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
                    .expect("Valid regex and pattern"),
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
                            .map_err(|e| DataError::custom("Properties data provider error")
                                .with_display_context(&e))?,
                            icu_properties::Script::Hebrew,
                        )
                        .map_err(|e| DataError::custom("Could not find Hebrew script set")
                            .with_display_context(&e))?
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

impl IterableProvider for ListProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        Ok(Box::new(
            self.data
                .iter_keys()
                // TODO(#568): Avoid the clone
                .cloned()
                // ur-IN has a buggy pattern ("{1}, {0}") which violates
                // our invariant that {0} is at index 0 (and rotates the output).
                // ml has middle and start patterns with suffixes.
                // See https://github.com/unicode-org/icu4x/issues/1282
                .filter(|l| l != &langid!("ur-IN") && l != &langid!("ml"))
                .map(Into::<ResourceOptions>::into),
        ))
    }
}

fn parse_and_patterns<'a>(
    raw: &cldr_serde::list_patterns::ListPatterns,
) -> Result<ListFormatterPatternsV1<'a>, DataError> {
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
) -> Result<ListFormatterPatternsV1<'a>, DataError> {
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
) -> Result<ListFormatterPatternsV1<'a>, DataError> {
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
    use icu_list::{ListFormatter, ListStyle, ListType};
    use icu_locid_macros::langid;
    use writeable::assert_writeable_eq;

    macro_rules! formatter {
        ($name:ident, $langid:expr, $type:expr, $width:expr) => {
            let cldr_paths = crate::cldr_paths::for_test();
            let provider = ListProvider::try_from(&cldr_paths as &dyn CldrPaths).unwrap();
            let $name = ListFormatter::try_new($langid, &provider, $type, $width).unwrap();
        };
    }

    #[test]
    fn test_basic() {
        formatter!(f, langid!("fr"), ListType::Or, ListStyle::Wide);
        assert_writeable_eq!(f.format(["A", "B"].iter()), "A ou B");
    }

    #[test]
    fn test_spanish() {
        formatter!(and, langid!("es"), ListType::And, ListStyle::Wide);
        assert_writeable_eq!(and.format(["", "Mallorca"].iter()), " y Mallorca");
        assert_writeable_eq!(and.format(["", "Ibiza"].iter()), " e Ibiza");
        assert_writeable_eq!(and.format(["", "Hidalgo"].iter()), " e Hidalgo");
        assert_writeable_eq!(and.format(["", "Hierva"].iter()), " y Hierva");

        formatter!(or, langid!("es"), ListType::Or, ListStyle::Wide);
        assert_writeable_eq!(or.format(["", "Ibiza"].iter()), " o Ibiza");
        assert_writeable_eq!(or.format(["", "Okinawa"].iter()), " u Okinawa");
        assert_writeable_eq!(or.format(["", "8 más"].iter()), " u 8 más");
        assert_writeable_eq!(or.format(["", "8"].iter()), " u 8");
        assert_writeable_eq!(or.format(["", "87 más"].iter()), " u 87 más");
        assert_writeable_eq!(or.format(["", "87"].iter()), " u 87");
        assert_writeable_eq!(or.format(["", "11 más"].iter()), " u 11 más");
        assert_writeable_eq!(or.format(["", "11"].iter()), " u 11");
        assert_writeable_eq!(or.format(["", "110 más"].iter()), " o 110 más");
        assert_writeable_eq!(or.format(["", "110"].iter()), " o 110");
        assert_writeable_eq!(or.format(["", "11.000 más"].iter()), " u 11.000 más");
        assert_writeable_eq!(or.format(["", "11.000"].iter()), " u 11.000");
        assert_writeable_eq!(or.format(["", "11.000,92 más"].iter()), " u 11.000,92 más");
        assert_writeable_eq!(or.format(["", "11.000,92"].iter()), " u 11.000,92");

        formatter!(and, langid!("es-AR"), ListType::And, ListStyle::Wide);
        assert_writeable_eq!(and.format(["", "Ibiza"].iter()), " e Ibiza");
    }

    #[test]
    fn test_hebrew() {
        formatter!(and, langid!("he"), ListType::And, ListStyle::Wide);

        assert_writeable_eq!(and.format(["", "יפו"].iter()), " ויפו");
        assert_writeable_eq!(and.format(["", "Ibiza"].iter()), " ו-Ibiza");
    }
}
