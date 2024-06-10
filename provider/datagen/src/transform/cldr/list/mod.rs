// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::transform::cldr::cldr_serde;
use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderCached;
use icu_list::provider::*;
use icu_locale_core::subtags::language;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::OnceLock;

fn load<M: DataMarker<Yokeable = ListFormatterPatternsV1<'static>>>(
    selff: &DatagenProvider,
    req: DataRequest,
) -> Result<DataResponse<M>, DataError> {
    let langid = req.locale.get_langid();

    let resource: &cldr_serde::list_patterns::Resource = selff
        .cldr()?
        .misc()
        .read_and_parse(&langid, "listPatterns.json")?;

    let data = &resource.main.value.list_patterns;

    let (wide, short, narrow) = if M::INFO == AndListV1Marker::INFO {
        (&data.standard, &data.standard_short, &data.standard_narrow)
    } else if M::INFO == OrListV1Marker::INFO {
        (&data.or, &data.or_short, &data.or_narrow)
    } else if M::INFO == UnitListV1Marker::INFO {
        (&data.unit, &data.unit_short, &data.unit_narrow)
    } else {
        return Err(DataError::custom(
            "Unknown marker for ListFormatterPatternsV1",
        ));
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
        if M::INFO == AndListV1Marker::INFO || M::INFO == UnitListV1Marker::INFO {
            static I_SOUND: OnceLock<SerdeDFA<'static>> = OnceLock::new();

            // Starts with i or (hi but not hia/hie)
            let i_sound = I_SOUND.get_or_init(|| {
                SerdeDFA::new(Cow::Borrowed("i|hi([^ae]|$)")).expect("Valid regex")
            });

            // Replace " y " with " e " before /i/ sounds.
            // https://unicode.org/reports/tr35/tr35-general.html#:~:text=important.%20For%20example%3A-,Spanish,AND,-Use%20%E2%80%98e%E2%80%99%20instead
            patterns
                .make_conditional("{0} y {1}", i_sound, "{0} e {1}")
                .expect("valid pattern");
        } else if M::INFO == OrListV1Marker::INFO {
            static O_SOUND: OnceLock<SerdeDFA<'static>> = OnceLock::new();
            // Starts with o, ho, 8 (including 80, 800, ...), or 11 either alone or followed
            // by thousand groups and/or decimals (excluding e.g. 110, 1100, ...)
            let o_sound = O_SOUND.get_or_init(|| {
                SerdeDFA::new(Cow::Borrowed(
                    r"o|ho|8|(11([\.  ]?[0-9]{3})*(,[0-9]*)?([^\.,[0-9]]|$))",
                ))
                .expect("Valid regex")
            });

            // Replace " o " with " u " before /o/ sound.
            // https://unicode.org/reports/tr35/tr35-general.html#:~:text=agua%20e%20hielo-,OR,-Use%20%E2%80%98u%E2%80%99%20instead
            patterns
                .make_conditional("{0} o {1}", o_sound, "{0} u {1}")
                .expect("valid pattern");
        }
    }

    if langid.language == language!("he") {
        // Cannot cache this because it depends on `selff`. However we don't expect many Hebrew locales.
        let non_hebrew = SerdeDFA::new(Cow::Owned(format!(
            "[^{}]",
            icu_properties::maps::load_script(selff)
                .map_err(|e| DataError::custom("data for CodePointTrie of Script")
                    .with_display_context(&e))?
                .as_borrowed()
                .get_set_for_value(icu_properties::Script::Hebrew)
                .as_borrowed()
                .iter_ranges()
                .map(|range| format!(r#"\u{:04x}-\u{:04x}"#, range.start(), range.end()))
                .fold(String::new(), |a, b| a + &b)
        )))
        .expect("valid regex");

        // Add dashes between ו and non-Hebrew characters
        // https://unicode.org/reports/tr35/tr35-general.html#:~:text=is%20not%20mute.-,Hebrew,AND,-Use%20%E2%80%98%2D%D7%95%E2%80%99%20instead
        patterns
            .make_conditional(
                "{0} \u{05D5}{1}", // ״{0} ו{1}״
                // Starts with a non-Hebrew letter
                &non_hebrew,
                "{0} \u{05D5}‑{1}", // ״{0} ו‑{1}״
            )
            .expect("valid pattern");
    }

    let metadata = DataResponseMetadata::default();
    Ok(DataResponse {
        metadata,
        payload: Some(DataPayload::from_owned(patterns)),
    })
}

macro_rules! implement {
    ($marker:ident) => {
        impl DataProvider<$marker> for DatagenProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                load(self, req)
            }
        }

        impl IterableDataProviderCached<$marker> for DatagenProvider {
            fn supported_requests_cached(
                &self,
            ) -> Result<HashSet<(DataLocale, DataMarkerAttributes)>, DataError> {
                Ok(self
                    .cldr()?
                    .misc()
                    .list_langs()?
                    .map(|l| (DataLocale::from(l), Default::default()))
                    .collect())
            }
        }
    };
}

implement!(AndListV1Marker);
implement!(OrListV1Marker);
implement!(UnitListV1Marker);
