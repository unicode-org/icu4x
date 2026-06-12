// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use icu::experimental::displaynames::provider::*;
use icu_pattern::DoublePlaceholderPattern;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::VarZeroCow;

impl DataProvider<LocaleNamesEssentialsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<LocaleNamesEssentialsV1>, DataError> {
        self.check_req::<LocaleNamesEssentialsV1>(req)?;

        let data: &cldr_serde::displaynames::locale_display_pattern::Resource = self
            .cldr()?
            .displaynames()
            .read_and_parse(req.id.locale, "localeDisplayNames.json")?;

        let pattern_str = &data
            .main
            .value
            .localedisplaynames
            .locale_display_pattern
            .locale_pattern;
        let separator_str = &data
            .main
            .value
            .localedisplaynames
            .locale_display_pattern
            .locale_separator;

        let pattern = DoublePlaceholderPattern::try_from_str(pattern_str, Default::default())
            .map_err(|e| {
                DataError::custom("Failed to parse localePattern").with_display_context(&e)
            })?;

        let separator = DoublePlaceholderPattern::try_from_str(separator_str, Default::default())
            .map_err(|e| {
            DataError::custom("Failed to parse localeSeparator").with_display_context(&e)
        })?;

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(LocaleNamesEssentials {
                locale_pattern: VarZeroCow::from_encodeable(&pattern),
                locale_separator: VarZeroCow::from_encodeable(&separator),
            }),
        })
    }
}

crate::displaynames::impl_displaynames_legacy_iter_v1!(
    LocaleNamesEssentialsV1,
    "localeDisplayNames.json"
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu::locale::langid;

    #[test]
    fn test_locale_names_essentials() {
        let provider = SourceDataProvider::new_testing();

        let data: DataPayload<LocaleNamesEssentialsV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en-001").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;

        assert_eq!(
            data.get()
                .locale_pattern
                .interpolate(["A", "B"])
                .to_string(),
            "A (B)"
        );
        assert_eq!(
            data.get()
                .locale_separator
                .interpolate(["A", "B"])
                .to_string(),
            "A, B"
        );
    }
}
