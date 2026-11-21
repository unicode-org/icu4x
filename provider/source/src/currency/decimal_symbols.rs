// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::decimal::decimal_pattern::DecimalSubPattern;
use icu::decimal::provider::*;
use icu::experimental::dimension::provider::currency::symbols::CurrencyDecimalSymbolsV1;
use icu::locale::preferences::extensions::unicode::keywords::CurrencyType;
use icu::locale::subtags::Subtag;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::VarZeroCow;

impl DataProvider<CurrencyDecimalSymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyDecimalSymbolsV1>, DataError> {
        self.check_req::<CurrencyDecimalSymbolsV1>(req)?;

        let Some((nsattr, currency)) =
            CurrencyDecimalSymbolsV1::parse_attributes(req.id.marker_attributes)
        else {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(CurrencyDecimalSymbolsV1::INFO, req)
            );
        };

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = nsattr
            .as_ref()
            .map(|s| s.as_str())
            .unwrap_or(&numbers.default_numbering_system);

        let Some(symbols) = &numbers.numsys_data.symbols.get(nsname) else {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(CurrencyDecimalSymbolsV1::INFO, req)
            );
        };
        let Some(formats) = &numbers.numsys_data.formats.get(nsname) else {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(CurrencyDecimalSymbolsV1::INFO, req)
            );
        };

        let positive = DecimalSubPattern::try_from_items(&formats.standard.positive)?;
        let negative = formats
            .standard
            .negative
            .as_ref()
            .map(|s| DecimalSubPattern::try_from_items(s))
            .transpose()?;

        let affixes = negative
            .as_ref()
            .map(|n| (n.prefix.as_str(), n.suffix.as_str()))
            .unwrap_or_else(|| ("-", ""));

        // The currency's own separators, if the identifier is for one currency.
        let overrides = match currency {
            Some(currency) => {
                let currencies: &cldr_serde::currencies::data::Resource = self
                    .cldr()?
                    .numbers()
                    .read_and_parse(req.id.locale, "currencies.json")?;

                currencies
                    .main
                    .value
                    .numbers
                    .currencies
                    .get(currency.iso_code().as_str())
            }
            None => None,
        };

        let Some((decimal_separator, grouping_separator)) = currency_separators(symbols, overrides)
        else {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(CurrencyDecimalSymbolsV1::INFO, req)
            );
        };

        let strings = DecimalSymbolStrsBuilder {
            minus_sign_prefix: VarZeroCow::new_owned(
                affixes.0.replace('-', &symbols.minus_sign).into_boxed_str(),
            ),
            minus_sign_suffix: VarZeroCow::new_owned(
                affixes.1.replace('-', &symbols.minus_sign).into_boxed_str(),
            ),
            plus_sign_prefix: VarZeroCow::new_owned(
                affixes.0.replace('-', &symbols.plus_sign).into_boxed_str(),
            ),
            plus_sign_suffix: VarZeroCow::new_owned(
                affixes.1.replace('-', &symbols.plus_sign).into_boxed_str(),
            ),
            decimal_separator: VarZeroCow::new_owned(decimal_separator.to_owned().into_boxed_str()),
            grouping_separator: VarZeroCow::new_owned(
                grouping_separator.to_owned().into_boxed_str(),
            ),
            numsys: VarZeroCow::new_owned(nsname.to_owned().into_boxed_str()),
        }
        .build();

        if let Some(n) = negative.as_ref()
            && (
                positive.max_fraction_digits,
                positive.min_fraction_digits,
                positive.primary_grouping,
                positive.secondary_grouping,
            ) != (
                n.max_fraction_digits,
                n.min_fraction_digits,
                n.primary_grouping,
                n.secondary_grouping,
            )
        {
            return Err(DataError::custom("positive/negative groupings don't match")
                .with_req(CurrencyDecimalSymbolsV1::INFO, req));
        }

        let grouping_sizes = GroupingSizes {
            primary: positive.primary_grouping,
            secondary: positive.secondary_grouping,
            min_grouping: numbers.minimum_grouping_digits,
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(DecimalSymbols {
                strings,
                grouping_sizes,
            }),
        })
    }
}

/// Returns the decimal and grouping separators for formatting currencies, or `None` if
/// CLDR has none that are specific to currencies.
///
/// Each separator comes from the first of these that has it:
/// 1. the currency, if one is given (`<decimal>` and `<group>` in `<currency>`),
/// 2. the locale's symbols for currencies (`<currencyDecimal>` and `<currencyGroup>`),
/// 3. the locale's standard symbols (`<decimal>` and `<group>`).
fn currency_separators<'a>(
    symbols: &'a cldr_serde::numbers::Symbols,
    currency: Option<&'a cldr_serde::currencies::data::CurrencyPatterns>,
) -> Option<(&'a str, &'a str)> {
    let decimal = currency
        .and_then(|c| c.decimal.as_deref())
        .or(symbols.currency_decimal.as_deref());
    let group = currency
        .and_then(|c| c.group.as_deref())
        .or(symbols.currency_group.as_deref());

    if decimal.is_none() && group.is_none() {
        return None;
    }

    Some((
        decimal.unwrap_or(&symbols.decimal),
        group.unwrap_or(&symbols.group),
    ))
}

impl IterableDataProviderCached<CurrencyDecimalSymbolsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut ids = HashSet::new();

        for locale in self.cldr()?.numbers().list_locales()? {
            let resource: &cldr_serde::numbers::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&locale, "numbers.json")?;

            let numbers = &resource.main.value.numbers;

            let currencies: &cldr_serde::currencies::data::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&locale, "currencies.json")?;

            // The currencies with separators of their own.
            let mut overriding = Vec::new();
            for (currency, patterns) in &currencies.main.value.numbers.currencies {
                if patterns.decimal.is_none() && patterns.group.is_none() {
                    continue;
                }
                let Ok(currency) = CurrencyType::try_from_str(currency) else {
                    log::error!(
                        "Skipping non-ISO-4217 currency override for '{currency}' in locale {locale}"
                    );
                    continue;
                };
                overriding.push(currency);
            }

            // The overrides of a currency are not scoped to a numbering system, but the
            // rest of the symbols are, so each numbering system needs its own identifier.
            for (nsname, symbols) in &numbers.numsys_data.symbols {
                let nu = (*nsname != numbers.default_numbering_system)
                    .then(|| Subtag::try_from_str(nsname).unwrap());

                let mut insert = |currency| {
                    ids.insert(
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            CurrencyDecimalSymbolsV1::make_attributes(
                                nu,
                                currency,
                                &mut Default::default(),
                            ),
                            &locale,
                        )
                        .into_owned(),
                    );
                };

                // The locale's symbols for all currencies.
                if currency_separators(symbols, None).is_some() {
                    insert(None);
                }
                // The symbols of the currencies with separators of their own.
                for &currency in &overriding {
                    insert(Some(currency));
                }
            }
        }

        Ok(ids)
    }
}

#[test]
fn test_currency_separators() {
    use cldr_serde::currencies::data::CurrencyPatterns;
    use cldr_serde::numbers::Symbols;

    let symbols = |currency_decimal: Option<&str>, currency_group: Option<&str>| Symbols {
        approximately_sign: "~".into(),
        decimal: ",".into(),
        group: " ".into(),
        minus_sign: "-".into(),
        plus_sign: "+".into(),
        percent_sign: "%".into(),
        currency_decimal: currency_decimal.map(Into::into),
        currency_group: currency_group.map(Into::into),
    };
    let standard = symbols(None, None);
    let for_currencies = symbols(Some("."), Some("'"));

    let euro: CurrencyPatterns = serde_json::from_str("{}").unwrap();
    let escudo: CurrencyPatterns = serde_json::from_str(r#"{ "decimal": "$" }"#).unwrap();

    // Nothing is specific to currencies.
    assert_eq!(currency_separators(&standard, None), None);
    assert_eq!(currency_separators(&standard, Some(&euro)), None);

    // The locale's symbols for currencies replace its standard ones.
    assert_eq!(currency_separators(&for_currencies, None), Some((".", "'")));

    // The currency's own separator comes first, and the locale provides the other one.
    assert_eq!(
        currency_separators(&standard, Some(&escudo)),
        Some(("$", " "))
    );
    assert_eq!(
        currency_separators(&for_currencies, Some(&escudo)),
        Some(("$", "'"))
    );
}

#[test]
fn test_currency_symbols() {
    use icu::locale::{LanguageIdentifier, langid};

    let provider = SourceDataProvider::new_testing();

    let load = |locale: LanguageIdentifier, attributes| {
        DataProvider::<CurrencyDecimalSymbolsV1>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic(attributes),
                    &locale.into(),
                ),
                ..Default::default()
            },
        )
    };

    // `pt-PT` formats the Portuguese escudo with `$` as its decimal separator and `,`
    // as its grouping separator, instead of the standard `,` and U+00A0.
    let escudo = load(langid!("pt-PT"), "PTE").unwrap().payload;
    assert_eq!(escudo.get().decimal_separator(), "$");
    assert_eq!(escudo.get().grouping_separator(), ",");
    assert_eq!(escudo.get().numsys(), "latn");

    // `fr-CH` formats all currencies with `.` as the decimal separator, instead of the
    // standard `,`. It has no grouping separator for currencies, so that stays `'`.
    let fr_ch = load(langid!("fr-CH"), "").unwrap().payload;
    assert_eq!(fr_ch.get().decimal_separator(), ".");
    assert_eq!(fr_ch.get().grouping_separator(), "'");
    assert_eq!(fr_ch.get().numsys(), "latn");

    // Currencies without overrides have no identifier of their own, and neither do
    // locales without symbols for currencies; consumers fall back to the next symbols.
    for (locale, attributes) in [
        (langid!("pt-PT"), "EUR"),
        (langid!("pt-PT"), ""),
        (langid!("fr-CH"), "EUR"),
    ] {
        assert_eq!(
            load(locale, attributes).map(|_| ()).unwrap_err().kind,
            DataErrorKind::IdentifierNotFound
        );
    }
}
