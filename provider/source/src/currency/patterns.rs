// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use std::collections::HashSet;

use icu::experimental::dimension::provider::currency::patterns::*;
use icu::plurals::PluralElements;
use icu_provider::prelude::*;
use icu_provider::DataProvider;

impl DataProvider<CurrencyPatternsDataV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyPatternsDataV1>, DataError> {
        self.check_req::<CurrencyPatternsDataV1>(req)?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let mut default_system = numbers_resource
            .main
            .value
            .numbers
            .default_numbering_system
            .as_str();

        // https://github.com/unicode-org/icu4x/issues/5374
        if *req.id.locale == DataLocale::from(icu::locale::locale!("sd")) {
            default_system = "latn";
        }

        let currency_patterns = &numbers_resource
            .main
            .value
            .numbers
            .numsys_data
            .currency_patterns;

        // `default_patterns` is the patterns that came from the default numbering system
        let patterns = &currency_patterns
            .get(default_system)
            .ok_or(DataErrorKind::IdentifierNotFound.into_error())?;

        //  According to [Unicode Technical Standard #35](https://unicode.org/reports/tr35/tr35-numbers.html),
        //  The `other` pattern must be present in the data.
        if patterns.pattern_other.is_none() {
            return Err(DataErrorKind::IdentifierNotFound.into_error());
        }

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencyPatternsData {
                // TODO(#5334):
                //      Before graduating the currency crate,
                //      Check that the .json data files are completed and no need to fallback chain up to the root.
                patterns: PluralElements::new(patterns.pattern_other.as_deref().ok_or_else(
                    || {
                        DataError::custom("Missing patterns")
                            .with_debug_context(currency_patterns)
                            .with_debug_context(&req.id)
                    },
                )?)
                .with_zero_value(patterns.pattern_zero.as_deref())
                .with_one_value(patterns.pattern_one.as_deref())
                .with_two_value(patterns.pattern_two.as_deref())
                .with_few_value(patterns.pattern_few.as_deref())
                .with_many_value(patterns.pattern_many.as_deref())
                .with_explicit_one_value(patterns.pattern_explicit_one.as_deref())
                .with_explicit_zero_value(patterns.pattern_explicit_zero.as_deref())
                .into(),
            }),
        })
    }
}

impl IterableDataProviderCached<CurrencyPatternsDataV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
            .collect())
    }
}

#[test]
fn test_basic() {
    use icu::locale::langid;
    use icu_pattern::DoublePlaceholderPattern;

    let provider = SourceDataProvider::new_testing();

    let en: DataPayload<CurrencyPatternsDataV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(
        en.get().patterns.elements.decode().map(|(_, p)| p),
        PluralElements::new(
            DoublePlaceholderPattern::try_from_str("{0} {1}", Default::default())
                .unwrap()
                .as_ref()
        )
    );

    let ar: DataPayload<CurrencyPatternsDataV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ar-EG").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(
        ar.get().patterns.elements.decode().map(|(_, p)| p),
        PluralElements::new(
            DoublePlaceholderPattern::try_from_str("{0} {1}", Default::default())
                .unwrap()
                .as_ref()
        )
    );

    let ja: DataPayload<CurrencyPatternsDataV1> = provider
        .load(DataRequest {
            id: DataIdentifierBorrowed::for_locale(&langid!("ja").into()),
            ..Default::default()
        })
        .unwrap()
        .payload;

    assert_eq!(
        ja.get().patterns.elements.decode().map(|(_, p)| p),
        PluralElements::new(
            DoublePlaceholderPattern::try_from_str("{0}{1}", Default::default())
                .unwrap()
                .as_ref()
        )
    );
}
