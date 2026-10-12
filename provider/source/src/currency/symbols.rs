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

/// The symbol CLDR uses when a locale explicitly has no currency symbol (CLDR-19771).
const CLDR_NO_SYMBOL_MARKER: &str = "\u{200B}";

impl DataProvider<CurrencySymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencySymbolsV1>, DataError> {
        self.check_req::<CurrencySymbolsV1>(req)?;

        let gc = CodePointMapData::<GeneralCategory>::try_new_unstable(self)?;
        let gc = gc.as_borrowed();
        let letters_set = gc.get_set_for_value_group(GeneralCategoryGroup::Letter);

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
        .unwrap()
        .as_str();

        // CLDR uses U+200B ZERO WIDTH SPACE as the symbol when a locale explicitly has no
        // currency symbol, e.g. `PTE` in `pt-PT`, where the cifrão ('$') is the decimal
        // separator instead (CLDR-19771). CLDR needs a non-empty value there to block
        // locale inheritance. Datagen already resolves inheritance, so we store the empty
        // string instead: an empty symbol means "use the no-currency pattern". This is
        // different from a missing entry, which falls back to the ISO code.
        let symbol = if symbol == CLDR_NO_SYMBOL_MARKER {
            ""
        } else {
            // If CLDR starts marking "no symbol" in another way (e.g. a different invisible
            // character, or an empty string), it would end up being formatted as a visible
            // symbol, or as the no-currency pattern by accident. Warn so that datagen
            // can be updated.
            if symbol.chars().all(|c| {
                let c = gc.get(c);
                c == GeneralCategory::Format || GeneralCategoryGroup::Separator.contains(c)
            }) {
                log::warn!(
                    "Unexpected invisible currency symbol for {currency} in {}: {symbol:?}; \
                     only {CLDR_NO_SYMBOL_MARKER:?} is mapped to the no-currency pattern",
                    req.id.locale,
                );
            }
            symbol
        };

        // TODO: This is not entirely correct. We need to look at the first/last grapheme cluster.
        let starts_with_letter = symbol
            .chars()
            .next()
            .is_some_and(|c| letters_set.as_borrowed().contains(c));
        let ends_with_letter = symbol
            .chars()
            .next_back()
            .is_some_and(|c| letters_set.as_borrowed().contains(c));

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
    use icu::experimental::dimension::currency::CurrencyType;
    use icu::locale::preferences::extensions::unicode::keywords::currency;
    use icu::locale::{DataLocale, data_locale};

    const USD: CurrencyType = currency!("USD");
    const EGP: CurrencyType = currency!("EGP");
    const EN: DataLocale = data_locale!("en");
    const AR_EG: DataLocale = data_locale!("ar-EG");

    let provider = SourceDataProvider::new_testing();

    let load = |locale: DataLocale, currency: CurrencyType, width: CurrencySymbolWidth| {
        DataProvider::<CurrencySymbolsV1>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    CurrencySymbolsV1::make_attributes(currency, width, &mut Default::default()),
                    &locale,
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

    // `pt-PT` has no symbol for `PTE`: CLDR marks this with U+200B, which datagen
    // stores as the empty string (= use the no-currency pattern). If this fails, CLDR
    // has changed how it marks "no symbol" (CLDR-19771), and datagen needs updating.
    const PTE: CurrencyType = currency!("PTE");
    const PT_PT: DataLocale = data_locale!("pt-PT");
    assert_eq!(
        load(PT_PT, PTE, CurrencySymbolsV1::SHORT).unwrap().get(),
        &CurrencySymbol::new("", false, false)
    );
}
