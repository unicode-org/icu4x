// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use crate::transform::icuexport::uprops::EnumeratedPropertyCodePointTrieProvider;
use crate::SourceData;
use icu_list::provider::*;
use icu_locid::subtags_language as language;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use lazy_static::lazy_static;

/// A data provider reading from CLDR JSON list rule files.
#[derive(Debug)]
pub struct ListProvider {
    source: SourceData,
}

impl From<&SourceData> for ListProvider {
    fn from(source: &SourceData) -> Self {
        ListProvider {
            source: source.clone(),
        }
    }
}

impl<M: KeyedDataMarker<Yokeable = ListFormatterPatternsV1<'static>>> DataProvider<M>
    for ListProvider
{
    fn load(&self, req: &DataRequest) -> Result<DataResponse<M>, DataError> {
        let langid = req.locale.get_langid();

        let resource: &cldr_serde::list_patterns::Resource = self
            .source
            .cldr()?
            .misc()
            .read_and_parse(&langid, "listPatterns.json")?;

        let data = &resource
            .main
            .0
            .get(&langid)
            .expect("CLDR file contains the expected language")
            .list_patterns;

        let (wide, short, narrow) = if M::KEY == AndListV1Marker::KEY {
            (&data.standard, &data.standard_short, &data.standard_narrow)
        } else if M::KEY == OrListV1Marker::KEY {
            (&data.or, &data.or_short, &data.or_narrow)
        } else if M::KEY == UnitListV1Marker::KEY {
            (&data.unit, &data.unit_short, &data.unit_narrow)
        } else {
            return Err(DataError::custom("Unknown key for ListFormatterPatternsV1"));
        };

        let mut patterns = ListFormatterPatternsV1::try_new([
            &wide.start,
            &wide.middle,
            &wide.end,
            &wide.pair,
            &short.start,
            &short.middle,
            &short.end,
            &short.pair,
            &narrow.start,
            &narrow.middle,
            &narrow.end,
            &narrow.pair,
        ])?;

        if langid.language == language!("es") {
            if M::KEY == AndListV1Marker::KEY || M::KEY == UnitListV1Marker::KEY {
                lazy_static! {
                    // Starts with i or (hi but not hia/hie)
                    static ref I_SOUND: StringMatcher<'static> = StringMatcher::new("i|hi([^ae]|$)").expect("Valid regex");
                }
                // Replace " y " with " e " before /i/ sounds.
                // https://unicode.org/reports/tr35/tr35-general.html#:~:text=important.%20For%20example%3A-,Spanish,AND,-Use%20%E2%80%98e%E2%80%99%20instead
                patterns
                    .make_conditional("{0} y {1}", &*I_SOUND, "{0} e {1}")
                    .expect("valid pattern");
            } else if M::KEY == OrListV1Marker::KEY {
                lazy_static! {
                    // Starts with o, ho, 8 (including 80, 800, ...), or 11 either alone or followed
                    // by thousand groups and/or decimals (excluding e.g. 110, 1100, ...)
                    static ref O_SOUND: StringMatcher<'static> = StringMatcher::new(r"o|ho|8|(11(\.?\d\d\d)*(,\d*)?([^\.,\d]|$))").expect("Valid regex");
                }
                // Replace " o " with " u " before /o/ sound.
                // https://unicode.org/reports/tr35/tr35-general.html#:~:text=agua%20e%20hielo-,OR,-Use%20%E2%80%98u%E2%80%99%20instead
                patterns
                    .make_conditional("{0} o {1}", &*O_SOUND, "{0} u {1}")
                    .expect("valid pattern");
            }
        }

        if langid.language == language!("he") {
            // Cannot cache this because it depends on `self`. However we don't expect many Hebrew locales.
            let non_hebrew = StringMatcher::new(&format!(
                "[^{}]",
                icu_properties::maps::get_script(&EnumeratedPropertyCodePointTrieProvider::from(
                    &self.source
                ))
                .map_err(|e| DataError::custom("data for CodePointTrie of Script")
                    .with_display_context(&e))?
                .get_set_for_value(icu_properties::Script::Hebrew)
                .to_unicode_set()
                .iter_ranges()
                .map(|range| format!(r#"\u{:04x}-\u{:04x}"#, range.start(), range.end()))
                .fold(String::new(), |a, b| a + &b)
            ))
            .expect("valid regex");

            // Add dashes between ו and non-Hebrew characters
            // https://unicode.org/reports/tr35/tr35-general.html#:~:text=is%20not%20mute.-,Hebrew,AND,-Use%20%E2%80%98%2D%D7%95%E2%80%99%20instead
            patterns
                .make_conditional(
                    "{0} \u{05D5}{1}", // ״{0} ו {1}״
                    // Starts with a non-Hebrew letter
                    &non_hebrew,
                    "{0} \u{05D5}-{1}", // ״{0} ו- {1}״
                )
                .expect("valid pattern");
        }

        let metadata = DataResponseMetadata::default();
        Ok(DataResponse {
            metadata,
            payload: Some(DataPayload::from_owned(patterns)),
        })
    }
}

icu_provider::make_exportable_provider!(
    ListProvider,
    [AndListV1Marker, OrListV1Marker, UnitListV1Marker,]
);

impl<M: KeyedDataMarker<Yokeable = ListFormatterPatternsV1<'static>>> IterableDataProvider<M>
    for ListProvider
{
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

#[cfg(test)]
mod tests {
    use super::*;
    use icu_list::{ListFormatter, ListStyle};
    use icu_locid::locale;
    use writeable::assert_writeable_eq;

    macro_rules! test {
        ($locale:literal, $type:ident, $(($input:expr, $output:literal),)+) => {
            let provider = ListProvider::from(&SourceData::for_test());
            let f = ListFormatter::$type(locale!($locale), &provider, ListStyle::Wide).unwrap();
            $(
                assert_writeable_eq!(f.format($input.iter()), $output);
            )+
        };
    }

    #[test]
    fn test_basic() {
        test!("fr", try_new_or, (["A", "B"], "A ou B"),);
    }

    #[test]
    fn test_spanish() {
        test!(
            "es",
            try_new_and,
            (["x", "Mallorca"], "x y Mallorca"),
            (["x", "Ibiza"], "x e Ibiza"),
            (["x", "Hidalgo"], "x e Hidalgo"),
            (["x", "Hierva"], "x y Hierva"),
        );

        test!(
            "es",
            try_new_or,
            (["x", "Ibiza"], "x o Ibiza"),
            (["x", "Okinawa"], "x u Okinawa"),
            (["x", "8 más"], "x u 8 más"),
            (["x", "8"], "x u 8"),
            (["x", "87 más"], "x u 87 más"),
            (["x", "87"], "x u 87"),
            (["x", "11 más"], "x u 11 más"),
            (["x", "11"], "x u 11"),
            (["x", "110 más"], "x o 110 más"),
            (["x", "110"], "x o 110"),
            (["x", "11.000 más"], "x u 11.000 más"),
            (["x", "11.000"], "x u 11.000"),
            (["x", "11.000,92 más"], "x u 11.000,92 más"),
            (["x", "11.000,92"], "x u 11.000,92"),
        );

        test!("es-AR", try_new_and, (["x", "Ibiza"], "x e Ibiza"),);
    }

    #[test]
    fn test_hebrew() {
        test!(
            "he",
            try_new_and,
            (["x", "יפו"], "x ויפו"),
            (["x", "Ibiza"], "x ו-Ibiza"),
        );
    }
}
