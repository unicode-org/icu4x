// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;

use std::collections::HashSet;

use icu::experimental::dimension::provider::currency::symbols::*;
use icu::properties::CodePointMapData;
use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
use icu_provider::DataProvider;
use icu_provider::prelude::*;

impl DataProvider<CurrencySymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencySymbolsV1>, DataError> {
        self.check_req::<CurrencySymbolsV1>(req)?;

        let letters_set = CodePointMapData::<GeneralCategory>::try_new_unstable(self)?
            .as_borrowed()
            .get_set_for_value_group(GeneralCategoryGroup::Letter);

        let (length, currency) = req.id.marker_attributes.as_str().split_once('/').unwrap();

        let currency_pattern = self
            .cldr()?
            .numbers()
            .read_and_parse::<cldr_serde::currencies::data::Resource>(
                req.id.locale,
                "currencies.json",
            )?
            .main
            .value
            .numbers
            .currencies
            .get(currency)
            .unwrap();

        let symbol = match length {
            s if s == CurrencySymbolsV1::SHORT.as_str() => currency_pattern.short.as_ref(),
            n if n == CurrencySymbolsV1::NARROW.as_str() => currency_pattern.narrow.as_ref(),
            _ => unreachable!(),
        }
        .unwrap();

        // TODO: This is not entirely correct. We need to look at the first/last grapheme cluster.
        let starts_with_letter = letters_set
            .as_borrowed()
            .contains(symbol.chars().next().unwrap());
        let ends_with_letter = letters_set
            .as_borrowed()
            .contains(symbol.chars().next_back().unwrap());

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CurrencySymbol::new(
                symbol,
                starts_with_letter,
                ends_with_letter,
            )),
        })
    }
}

impl IterableDataProviderCached<CurrencySymbolsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let cldr = self.cldr()?.numbers();

        let mut ids = HashSet::new();

        for locale in cldr.list_locales()? {
            for (currency, patterns) in &cldr
                .read_and_parse::<cldr_serde::currencies::data::Resource>(
                    &locale,
                    "currencies.json",
                )?
                .main
                .value
                .numbers
                .currencies
            {
                if patterns.short.as_ref().is_some_and(|s| s != currency) {
                    ids.insert(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_string(format!("s/{currency}")).unwrap(),
                        locale,
                    ));
                }
                if patterns.narrow.as_ref().is_some_and(|s| s != currency) {
                    ids.insert(DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_string(format!("n/{currency}")).unwrap(),
                        locale,
                    ));
                }
            }
        }

        Ok(ids)
    }
}

#[test]
fn test_symbols() {
    use icu::experimental::dimension::currency::CurrencyCode;
    use icu::locale::{LanguageIdentifier, langid};
    use tinystr::{TinyAsciiStr, tinystr};

    const USD: CurrencyCode = CurrencyCode(tinystr!(3, "USD"));
    const EGP: CurrencyCode = CurrencyCode(tinystr!(3, "EGP"));
    const EN: LanguageIdentifier = langid!("en");
    const AR_EG: LanguageIdentifier = langid!("ar-EG");

    let provider = SourceDataProvider::new_testing();

    #[allow(const_item_mutation)]
    let load = |locale: LanguageIdentifier, currency: CurrencyCode, width: TinyAsciiStr<1>| {
        DataProvider::<CurrencySymbolsV1>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    CurrencySymbolsV1::make_attributes(currency, width, &mut TinyAsciiStr::EMPTY),
                    &locale.into(),
                ),
                ..Default::default()
            },
        )
        .allow_identifier_not_found()
        .unwrap()
        .map(|r| r.payload)
    };

    assert_eq!(
        load(EN, USD, CurrencySymbolsV1::SHORT).unwrap().get(),
        &CurrencySymbol::new("$", false, false)
    );
    assert_eq!(
        load(EN, USD, CurrencySymbolsV1::NARROW).unwrap().get(),
        &CurrencySymbol::new("$", false, false)
    );

    assert_eq!(load(EN, EGP, CurrencySymbolsV1::SHORT), None);
    assert_eq!(
        load(EN, EGP, CurrencySymbolsV1::NARROW).unwrap().get(),
        &CurrencySymbol::new("E£", true, false)
    );

    assert_eq!(
        load(AR_EG, EGP, CurrencySymbolsV1::SHORT).unwrap().get(),
        &CurrencySymbol::new("ج.م.\u{200f}", true, false)
    );
    assert_eq!(
        load(AR_EG, EGP, CurrencySymbolsV1::NARROW).unwrap().get(),
        &CurrencySymbol::new("E£", true, false)
    );

    assert_eq!(
        load(AR_EG, USD, CurrencySymbolsV1::SHORT).unwrap().get(),
        &CurrencySymbol::new("US$", true, false)
    );
    assert_eq!(
        load(AR_EG, USD, CurrencySymbolsV1::NARROW).unwrap().get(),
        &CurrencySymbol::new("US$", true, false)
    );
}
