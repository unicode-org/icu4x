// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::experimental::compactdecimal::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::convert::TryFrom;

impl DataProvider<ShortCompactDecimalFormatDataV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ShortCompactDecimalFormatDataV1Marker>, DataError> {
        self.check_req::<ShortCompactDecimalFormatDataV1Marker>(req)?;

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = if !req.id.marker_attributes.is_empty() {
            req.id.marker_attributes.as_str()
        } else {
            &numbers.default_numbering_system
        };

        let result = CompactDecimalPatternDataV1::try_from(
            &numbers
                .numsys_data
                .formats
                .get(nsname)
                .ok_or_else(|| {
                    DataError::custom("Could not find formats for numbering system")
                        .with_display_context(nsname)
                })?
                .short
                .decimal_format,
        )
        .map_err(|s| {
            DataError::custom("Could not create compact decimal patterns")
                .with_display_context(&s)
                .with_display_context(nsname)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl DataProvider<LongCompactDecimalFormatDataV1Marker> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LongCompactDecimalFormatDataV1Marker>, DataError> {
        self.check_req::<LongCompactDecimalFormatDataV1Marker>(req)?;

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = if !req.id.marker_attributes.is_empty() {
            req.id.marker_attributes.as_str()
        } else {
            &numbers.default_numbering_system
        };

        let result = CompactDecimalPatternDataV1::try_from(
            &numbers
                .numsys_data
                .formats
                .get(nsname)
                .ok_or_else(|| {
                    DataError::custom("Could not find formats for numbering system")
                        .with_display_context(nsname)
                })?
                .long
                .decimal_format,
        )
        .map_err(|s| {
            DataError::custom("Could not create compact decimal patterns")
                .with_display_context(&s)
                .with_display_context(nsname)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result),
        })
    }
}

impl IterableDataProviderCached<ShortCompactDecimalFormatDataV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers()
    }
}

impl IterableDataProviderCached<LongCompactDecimalFormatDataV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers()
    }
}

#[cfg(test)]

mod tests {
    use super::*;
    use icu::{locale::langid, plurals::PluralCategory};
    use std::borrow::Cow;
    use zerofrom::ZeroFrom;
    use zerovec::ule::AsULE;

    #[test]

    fn test_compact_long() {
        let provider = SourceDataProvider::new_testing();

        let fr_compact_long: DataPayload<LongCompactDecimalFormatDataV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        let nonzero_copy: Box<[(i8, PluralCategory, Pattern)]> = fr_compact_long
            .get()
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1().map(move |(k, v)| {
                    (
                        key0,
                        PluralCategory::from_unaligned(*k),
                        Pattern::zero_from(v),
                    )
                })
            })
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    3,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 3,
                        pattern: Cow::Borrowed("\x01 thousand")
                    }
                ),
                (
                    6,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 6,
                        pattern: Cow::Borrowed("\x01 million")
                    }
                ),
                (
                    9,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 9,
                        pattern: Cow::Borrowed("\x01 billion")
                    }
                ),
                (
                    12,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 12,
                        pattern: Cow::Borrowed("\x01 trillion")
                    }
                ),
            ]
        );
    }

    #[test]
    fn test_compact_short() {
        let provider = SourceDataProvider::new_testing();

        let ja_compact_short: DataResponse<ShortCompactDecimalFormatDataV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierCow::from_locale(langid!("ja").into()).as_borrowed(),
                ..Default::default()
            })
            .unwrap();

        let nonzero_copy: Box<[(i8, PluralCategory, Pattern)]> = ja_compact_short
            .payload
            .get()
            .patterns
            .iter0()
            .flat_map(|kkv| {
                let key0 = *kkv.key0();
                kkv.into_iter1().map(move |(k, v)| {
                    (
                        key0,
                        PluralCategory::from_unaligned(*k),
                        Pattern::zero_from(v),
                    )
                })
            })
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    4,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 4,
                        pattern: Cow::Borrowed("\x01万")
                    }
                ),
                (
                    8,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 8,
                        pattern: Cow::Borrowed("\x01億")
                    }
                ),
                (
                    12,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 12,
                        pattern: Cow::Borrowed("\x01兆")
                    }
                ),
                (
                    16,
                    PluralCategory::Other,
                    Pattern {
                        exponent: 16,
                        pattern: Cow::Borrowed("\x01京")
                    }
                )
            ]
        );
    }
}
