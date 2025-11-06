// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::list::provider::*;
use icu::locale::subtags::language;
use icu_provider::prelude::*;
use std::borrow::Cow;
use std::collections::HashSet;
use std::sync::OnceLock;

fn load<M: DataMarker<DataStruct = ListFormatterPatterns<'static>>>(
    selff: &SourceDataProvider,
    req: DataRequest,
) -> Result<DataResponse<M>, DataError> {
    let resource: &cldr_serde::list_patterns::Resource = selff
        .cldr()?
        .misc()
        .read_and_parse(req.id.locale, "listPatterns.json")?;

    let data = &resource.main.value.list_patterns;

    let patterns = if M::INFO == ListAndV1::INFO {
        match req.id.marker_attributes.as_str() {
            ListFormatterPatterns::SHORT_STR => &data.standard_short,
            ListFormatterPatterns::NARROW_STR => &data.standard_narrow,
            ListFormatterPatterns::WIDE_STR => &data.standard,
            _ => return Err(DataErrorKind::IdentifierNotFound.with_req(M::INFO, req)),
        }
    } else if M::INFO == ListOrV1::INFO {
        match req.id.marker_attributes.as_str() {
            ListFormatterPatterns::SHORT_STR => &data.or_short,
            ListFormatterPatterns::NARROW_STR => &data.or_narrow,
            ListFormatterPatterns::WIDE_STR => &data.or,
            _ => return Err(DataErrorKind::IdentifierNotFound.with_req(M::INFO, req)),
        }
    } else if M::INFO == ListUnitV1::INFO {
        match req.id.marker_attributes.as_str() {
            ListFormatterPatterns::SHORT_STR => &data.unit_short,
            ListFormatterPatterns::NARROW_STR => &data.unit_narrow,
            ListFormatterPatterns::WIDE_STR => &data.unit,
            _ => return Err(DataErrorKind::IdentifierNotFound.with_req(M::INFO, req)),
        }
    } else {
        return Err(DataError::custom(
            "Unknown marker for ListFormatterPatterns",
        ));
    };

    let mut patterns = ListFormatterPatterns::try_new(
        &patterns.start,
        &patterns.middle,
        &patterns.end,
        &patterns.pair,
    )?;

    if req.id.locale.language == language!("es") {
        if M::INFO == ListAndV1::INFO || M::INFO == ListUnitV1::INFO {
            // Replace " y " with " e " before /i/ sounds.
            // https://unicode.org/reports/tr35/tr35-general.html#:~:text=important.%20For%20example%3A-,Spanish,AND,-Use%20%E2%80%98e%E2%80%99%20instead

            static I_SOUND_BECOMES_E: OnceLock<SpecialCasePattern<'static>> = OnceLock::new();
            let i_sound_becomes_e = I_SOUND_BECOMES_E.get_or_init(|| {
                SpecialCasePattern {
                    // Starts with i or (hi but not hia/hie)
                    condition: SerdeDFA::new(Cow::Borrowed("^[iI]|(?:[hH][iI](?:[^aeAE]|$))"))
                        .expect("Valid regex"),
                    pattern: ListJoinerPattern::try_from_str("{0} e {1}", false, false)
                        .expect("Valid pattern"),
                }
            });

            let default =
                ListJoinerPattern::try_from_str("{0} y {1}", false, false).expect("valid pattern");

            if patterns.end.default == default {
                patterns.end.special_case = Some(i_sound_becomes_e.clone());
            }
            if let Some(pair) = patterns.pair.as_mut() {
                if pair.default == default {
                    pair.special_case = Some(i_sound_becomes_e.clone());
                }
            }
        } else if M::INFO == ListOrV1::INFO {
            // Replace " o " with " u " before /o/ sound.
            // https://unicode.org/reports/tr35/tr35-general.html#:~:text=agua%20e%20hielo-,OR,-Use%20%E2%80%98u%E2%80%99%20instead

            static O_SOUND_BECOMES_U: OnceLock<SpecialCasePattern<'static>> = OnceLock::new();
            let o_sound_becomes_u = O_SOUND_BECOMES_U.get_or_init(|| {
                SpecialCasePattern {
                    // Starts with o, ho, 8 (including 80, 800, ...), or 11 either alone or followed
                    // by thousand groups and/or decimals (excluding e.g. 110, 1100, ...)
                    condition: SerdeDFA::new(Cow::Borrowed(
                        r"^[oO]|[hH][oO]|8|(?:11(?:[\.  ]?[0-9]{3})*(?:,[0-9]*)?(?:[^\.,[0-9]]|$))",
                    ))
                    .expect("Valid regex"),
                    pattern: ListJoinerPattern::try_from_str("{0} u {1}", false, false)
                        .expect("valid pattern"),
                }
            });

            let default =
                ListJoinerPattern::try_from_str("{0} o {1}", false, false).expect("valid pattern");

            if patterns.end.default == default {
                patterns.end.special_case = Some(o_sound_becomes_u.clone());
            }
            if let Some(pair) = patterns.pair.as_mut() {
                if pair.default == default {
                    pair.special_case = Some(o_sound_becomes_u.clone());
                }
            }
        }
    }

    if req.id.locale.language == language!("he") {
        // Add dashes between ו and non-Hebrew characters
        // https://unicode.org/reports/tr35/tr35-general.html#:~:text=is%20not%20mute.-,Hebrew,AND,-Use%20%E2%80%98%2D%D7%95%E2%80%99%20instead

        // Cannot cache this because it depends on `selff`. However we don't expect many Hebrew locales.
        let dashes_in_front_of_non_hebrew = SpecialCasePattern {
            condition: SerdeDFA::new(Cow::Owned(format!(
                "^[^{}]",
                icu::properties::CodePointMapData::<icu::properties::props::Script>::try_new_unstable(selff)
                    .map_err(|e| DataError::custom("data for CodePointTrie of Script")
                        .with_display_context(&e))?
                    .as_borrowed()
                    .get_set_for_value(icu::properties::props::Script::Hebrew)
                    .as_borrowed()
                    .iter_ranges()
                    .map(|range| format!(r#"\u{:04x}-\u{:04x}"#, range.start(), range.end()))
                    .fold(String::new(), |a, b| a + &b)
            )))
            .expect("valid regex"),
            pattern: ListJoinerPattern::try_from_str("{0} \u{05D5}‑{1}", false, false).unwrap(), // ״{0} ו‑{1}״
        };

        let default = ListJoinerPattern::try_from_str("{0} \u{05D5}{1}", false, false)
            .expect("valid pattern"); // ״{0} ו{1}״

        if patterns.end.default == default {
            patterns.end.special_case = Some(dashes_in_front_of_non_hebrew.clone());
        }
        if let Some(pair) = patterns.pair.as_mut() {
            if pair.default == default {
                pair.special_case = Some(dashes_in_front_of_non_hebrew.clone());
            }
        }
    }

    let metadata = DataResponseMetadata::default();
    Ok(DataResponse {
        metadata,
        payload: DataPayload::from_owned(patterns),
    })
}

macro_rules! implement {
    ($marker:ident) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                load(self, req)
            }
        }

        impl IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(self
                    .cldr()?
                    .misc()
                    .list_locales()?
                    .flat_map(|l| {
                        [
                            ListFormatterPatterns::SHORT,
                            ListFormatterPatterns::NARROW,
                            ListFormatterPatterns::WIDE,
                        ]
                        .into_iter()
                        .map(move |a| DataIdentifierCow::from_borrowed_and_owned(a, l.clone()))
                    })
                    .collect())
            }
        }
    };
}

implement!(ListAndV1);
implement!(ListOrV1);
implement!(ListUnitV1);
