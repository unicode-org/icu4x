// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::decimal::decimal_pattern::DecimalSubPattern;
use icu::decimal::provider::*;
use icu::experimental::dimension::provider::currency::symbols::CurrencyDecimalSymbolsV1;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::VarZeroCow;

/// Splits marker attributes into an optional numbering system and the currency code.
///
/// Returns `Err(IdentifierNotFound)` if the currency is missing or empty.
///
/// Attribute format:
/// - `<currency>` (e.g. `"PTE"`): `Ok((None, "PTE"))`
/// - `<numsys>/<currency>` (e.g. `"arab/PTE"`): `Ok((Some("arab"), "PTE"))`
fn split_currency_attributes(
    attrs: &DataMarkerAttributes,
) -> Result<(Option<&str>, &str), DataError> {
    let (nu, currency) = match attrs.as_str().split_once('/') {
        Some((nu, curr)) => (Some(nu), curr),
        None => (None, attrs.as_str()),
    };
    if currency.is_empty() {
        return Err(DataErrorKind::IdentifierNotFound.into_error());
    }
    Ok((nu, currency))
}

/// Formats marker attributes from an optional numbering system and currency code.
///
/// Returns an error if `currency` is not 3 uppercase ASCII letters (ISO-4217),
/// or if `nu` is not 3-8 lowercase ASCII letters (BCP-47 numbering system subtag).
fn currency_attributes(
    nu: Option<&str>,
    currency: &str,
) -> Result<Box<DataMarkerAttributes>, DataError> {
    if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
        log::error!(
            "currency attribute must be 3 uppercase ASCII letters as an ISO-4217 currency code: {currency}"
        );
        return Err(DataError::custom(
            "currency attribute must be 3 uppercase ASCII letters as an ISO-4217 currency code",
        )
        .with_debug_context(&currency));
    }
    if let Some(nu) = nu
        && (!(3..=8).contains(&nu.len()) || !nu.bytes().all(|b| b.is_ascii_lowercase()))
    {
        return Err(DataError::custom(
            "numbering system attribute must be 3-8 lowercase ASCII letters",
        )
        .with_debug_context(&nu));
    }
    Ok(match nu {
        Some(nu) => DataMarkerAttributes::try_from_string(format!("{nu}/{currency}")),
        None => DataMarkerAttributes::try_from_string(currency.to_owned()),
    }
    .expect("valid marker attributes"))
}

impl DataProvider<CurrencyDecimalSymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyDecimalSymbolsV1>, DataError> {
        self.check_req::<CurrencyDecimalSymbolsV1>(req)?;

        let (nsattr, currency) = split_currency_attributes(req.id.marker_attributes)
            .map_err(|e| e.with_req(CurrencyDecimalSymbolsV1::INFO, req))?;

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let nsname = nsattr.unwrap_or(&numbers.default_numbering_system);

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

        let currencies: &cldr_serde::currencies::data::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "currencies.json")?;

        let overrides = currencies
            .main
            .value
            .numbers
            .currencies
            .get(currency)
            .ok_or_else(|| {
                DataErrorKind::IdentifierNotFound.with_req(CurrencyDecimalSymbolsV1::INFO, req)
            })?;

        let decimal = overrides.decimal.as_deref();
        let group = overrides.group.as_deref();

        if decimal.is_none() && group.is_none() {
            return Err(
                DataErrorKind::IdentifierNotFound.with_req(CurrencyDecimalSymbolsV1::INFO, req)
            );
        }

        let decimal_separator = decimal.unwrap_or(&symbols.decimal);
        let grouping_separator = group.unwrap_or(&symbols.group);

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

impl IterableDataProviderCached<CurrencyDecimalSymbolsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut ids = HashSet::new();

        for locale in self.cldr()?.numbers().list_locales()? {
            let currencies: &cldr_serde::currencies::data::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&locale, "currencies.json")?;

            let overriding = currencies
                .main
                .value
                .numbers
                .currencies
                .iter()
                .filter(|(_, patterns)| patterns.decimal.is_some() || patterns.group.is_some())
                .map(|(currency, _)| currency.as_str())
                .collect::<Vec<_>>();

            if overriding.is_empty() {
                continue;
            }

            // The overrides are not scoped to a numbering system, but the rest of the
            // symbols are, so each numbering system needs its own identifier.
            let numsys = self.get_supported_numsys_for_langid(&locale, true)?;

            for currency in overriding {
                if currency.len() != 3 || !currency.bytes().all(|b| b.is_ascii_uppercase()) {
                    log::error!(
                        "Skipping non-ISO-4217 currency override for '{currency}' in locale {locale}"
                    );
                    continue;
                }
                let default_attr = currency_attributes(None, currency)?;
                ids.insert(
                    DataIdentifierBorrowed::for_marker_attributes_and_locale(
                        &default_attr,
                        &locale,
                    )
                    .into_owned(),
                );
                for nsname in &numsys {
                    let attr = currency_attributes(Some(nsname.as_str()), currency)?;
                    ids.insert(
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(&attr, &locale)
                            .into_owned(),
                    );
                }
            }
        }

        Ok(ids)
    }
}

#[test]
fn test_currency_attributes_roundtrip() {
    let pte = DataMarkerAttributes::from_str_or_panic("PTE");
    let arab_pte = DataMarkerAttributes::from_str_or_panic("arab/PTE");
    assert_eq!(split_currency_attributes(pte).unwrap(), (None, "PTE"));
    assert_eq!(
        split_currency_attributes(arab_pte).unwrap(),
        (Some("arab"), "PTE")
    );
    let empty = DataMarkerAttributes::from_str_or_panic("");
    assert!(split_currency_attributes(empty).is_err());

    assert_eq!(currency_attributes(None, "PTE").unwrap().as_str(), "PTE");
    assert_eq!(
        currency_attributes(Some("arab"), "PTE").unwrap().as_str(),
        "arab/PTE"
    );
}

#[test]
fn test_currency_attributes_validation() {
    // Valid attributes:
    assert!(currency_attributes(None, "USD").is_ok());
    assert!(currency_attributes(Some("latn"), "PTE").is_ok());
    assert!(currency_attributes(Some("arabext"), "PTE").is_ok());

    // Invalid currency (must be 3 uppercase ASCII letters as an ISO-4217 currency code):
    assert!(currency_attributes(None, "usd").is_err());
    assert!(currency_attributes(None, "US").is_err());
    assert!(currency_attributes(None, "USDD").is_err());
    assert!(currency_attributes(None, "").is_err());
    assert!(currency_attributes(None, "123").is_err());

    // Invalid numbering system (must be 3-8 lowercase ASCII letters):
    assert!(currency_attributes(Some(""), "USD").is_err());
    assert!(currency_attributes(Some("la"), "USD").is_err());
    assert!(currency_attributes(Some("latn/"), "USD").is_err());
    assert!(currency_attributes(Some("LATN"), "USD").is_err());
    assert!(currency_attributes(Some("123"), "USD").is_err());
    assert!(currency_attributes(Some("toolongsystem"), "USD").is_err());
}

#[test]
fn test_currency_symbols() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    let load = |currency| {
        DataProvider::<CurrencyDecimalSymbolsV1>::load(
            &provider,
            DataRequest {
                id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::from_str_or_panic(currency),
                    &langid!("pt-PT").into(),
                ),
                ..Default::default()
            },
        )
    };

    // `pt-PT` formats the Portuguese escudo with `$` as its decimal separator and `,`
    // as its grouping separator, instead of the standard `,` and U+00A0.
    let escudo = load("PTE").unwrap().payload;
    assert_eq!(escudo.get().decimal_separator(), "$");
    assert_eq!(escudo.get().grouping_separator(), ",");
    assert_eq!(escudo.get().numsys(), "latn");

    // Currencies without overrides have no identifier of their own; consumers fall back
    // to the locale's standard symbols.
    assert_eq!(
        load("EUR").map(|_| ()).unwrap_err().kind,
        DataErrorKind::IdentifierNotFound
    );
}
