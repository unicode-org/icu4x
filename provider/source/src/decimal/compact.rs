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

impl DataProvider<ShortCompactDecimalFormatDataV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<ShortCompactDecimalFormatDataV1>, DataError> {
        self.check_req::<ShortCompactDecimalFormatDataV1>(req)?;

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

        let result = CompactDecimalPatternData::try_from(
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

impl DataProvider<LongCompactDecimalFormatDataV1> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<LongCompactDecimalFormatDataV1>, DataError> {
        self.check_req::<LongCompactDecimalFormatDataV1>(req)?;

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

        let result = CompactDecimalPatternData::try_from(
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

impl IterableDataProviderCached<ShortCompactDecimalFormatDataV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers_with_locales()
    }
}

impl IterableDataProviderCached<LongCompactDecimalFormatDataV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        self.iter_ids_for_numbers_with_locales()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use icu::locale::langid;
    use icu::plurals::PluralElements;
    use icu_pattern::SinglePlaceholderPattern;

    #[test]

    fn test_compact_long() {
        let provider = SourceDataProvider::new_testing();

        let en_compact_long: DataPayload<LongCompactDecimalFormatDataV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        let nonzero_copy: Box<[_]> = en_compact_long
            .get()
            .patterns
            .iter()
            .map(|t| (t.sized, t.variable.decode().map(|(a, b)| (a.get(), b))))
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    3,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} thousand", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                ),
                (
                    6,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} million", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                ),
                (
                    9,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} billion", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                ),
                (
                    12,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0} trillion", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                ),
            ]
        );
    }

    #[test]
    fn test_compact_short() {
        let provider = SourceDataProvider::new_testing();

        let ja_compact_short: DataPayload<ShortCompactDecimalFormatDataV1> = provider
            .load(DataRequest {
                id: DataIdentifierCow::from_locale(langid!("ja").into()).as_borrowed(),
                ..Default::default()
            })
            .unwrap()
            .payload;

        let nonzero_copy: Box<[_]> = ja_compact_short
            .get()
            .patterns
            .iter()
            .map(|t| (t.sized, t.variable.decode().map(|(a, b)| (a.get(), b))))
            .collect();
        assert_eq!(
            nonzero_copy.as_ref(),
            [
                (
                    4,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0}万", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                ),
                (
                    8,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0}億", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                ),
                (
                    12,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0}兆", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                ),
                (
                    16,
                    PluralElements::new((
                        0,
                        SinglePlaceholderPattern::try_from_str("{0}京", Default::default())
                            .unwrap()
                            .as_ref()
                    ))
                )
            ]
        );
    }
}
