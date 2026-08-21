// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::decimal::decimal_pattern::DecimalSubPattern;
use icu::decimal::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use zerovec::VarZeroCow;

/// Splits a marker attribute into its numbering system and, for monetary symbols, the
/// ISO code of the currency whose separator overrides apply.
///
/// The attribute is `<currency>` for the locale's default numbering system and
/// `<numsys>/<currency>` otherwise, mirroring the standard `<numsys>` attributes.
/// An empty numbering system means the locale's default one.
fn split_currency_attribute(attributes: &str) -> (&str, Option<&str>) {
    if let Some((nsname, currency)) = attributes.split_once('/') {
        (nsname, Some(currency))
    } else if is_currency_code(attributes) {
        ("", Some(attributes))
    } else {
        (attributes, None)
    }
}

/// Currency codes are three uppercase ASCII letters, so they cannot be confused with a
/// numbering system name, which is lowercase.
fn is_currency_code(attributes: &str) -> bool {
    attributes.len() == 3 && attributes.bytes().all(|b| b.is_ascii_uppercase())
}

/// The marker attribute addressing the symbols of `currency` in `nsname`, which is the
/// locale's default numbering system if `is_default` is set.
fn currency_attribute(nsname: &str, currency: &str, is_default: bool) -> String {
    if is_default {
        currency.to_owned()
    } else {
        format!("{nsname}/{currency}")
    }
}

impl DataProvider<DecimalSymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<DecimalSymbolsV1>, DataError> {
        self.check_req::<DecimalSymbolsV1>(req)?;

        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        let (nsattr, currency) = split_currency_attribute(req.id.marker_attributes.as_str());

        let nsname = if !nsattr.is_empty() {
            nsattr
        } else {
            &numbers.default_numbering_system
        };

        let Some(symbols) = &numbers.numsys_data.symbols.get(nsname) else {
            return Err(DataErrorKind::IdentifierNotFound.with_req(DecimalSymbolsV1::INFO, req));
        };
        let Some(formats) = &numbers.numsys_data.formats.get(nsname) else {
            return Err(DataErrorKind::IdentifierNotFound.with_req(DecimalSymbolsV1::INFO, req));
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

        // UTS 35 lets a locale override the decimal and grouping separators of individual
        // currencies, e.g. the Portuguese escudo, which uses `$` as its decimal separator
        // in `pt-PT`. Only those currencies get an identifier of their own; every other
        // currency formats with the locale's standard symbols.
        let (decimal_separator, grouping_separator) = match currency {
            Some(currency) => {
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
                    .filter(|patterns| patterns.decimal.is_some() || patterns.group.is_some())
                    .ok_or_else(|| {
                        DataErrorKind::IdentifierNotFound.with_req(DecimalSymbolsV1::INFO, req)
                    })?;

                (
                    overrides.decimal.as_deref().unwrap_or(&symbols.decimal),
                    overrides.group.as_deref().unwrap_or(&symbols.group),
                )
            }
            None => (symbols.decimal.as_str(), symbols.group.as_str()),
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
                .with_req(DecimalSymbolsV1::INFO, req));
        }

        let grouping_sizes = GroupingSizes {
            primary: positive.primary_grouping,
            secondary: positive.secondary_grouping,
            min_grouping: numbers.minimum_grouping_digits,
        };

        // TODO: do something with `numbers.(min/max)_fraction_digits`

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(DecimalSymbols {
                strings,
                grouping_sizes,
            }),
        })
    }
}

impl IterableDataProviderCached<DecimalSymbolsV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        let mut ids = self.iter_ids_for_numbers_with_locales()?;

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
                let attributes = core::iter::once(currency_attribute("", currency, true)).chain(
                    numsys
                        .iter()
                        .map(|nsname| currency_attribute(nsname.as_str(), currency, false)),
                );

                for attribute in attributes {
                    let Ok(attribute) = DataMarkerAttributes::try_from_str(&attribute) else {
                        continue;
                    };
                    ids.insert(
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            attribute, &locale,
                        )
                        .into_owned(),
                    );
                }
            }
        }

        Ok(ids)
    }
}

#[test]
fn test_currency_attribute_roundtrip() {
    assert_eq!(split_currency_attribute(""), ("", None));
    assert_eq!(split_currency_attribute("latn"), ("latn", None));
    assert_eq!(split_currency_attribute("PTE"), ("", Some("PTE")));
    assert_eq!(split_currency_attribute("arab/PTE"), ("arab", Some("PTE")));

    assert_eq!(currency_attribute("latn", "PTE", true), "PTE");
    assert_eq!(currency_attribute("arab", "PTE", false), "arab/PTE");
}

#[test]
fn test_currency_symbols_only_for_overriding_currencies() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    // No currency in the test data overrides its separators, so consumers get
    // `IdentifierNotFound` and fall back to the locale's standard symbols.
    let result = DataProvider::<DecimalSymbolsV1>::load(
        &provider,
        DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic("PTE"),
                &langid!("en").into(),
            ),
            ..Default::default()
        },
    );
    assert_eq!(
        result.map(|_| ()).unwrap_err().kind,
        DataErrorKind::IdentifierNotFound
    );

    assert!(
        IterableDataProviderCached::<DecimalSymbolsV1>::iter_ids_cached(&provider)
            .unwrap()
            .iter()
            .all(|id| split_currency_attribute(id.marker_attributes.as_str())
                .1
                .is_none())
    );
}

#[test]
fn test_basic() {
    use icu::locale::data_locale;

    let provider = SourceDataProvider::new_testing();

    let ar_decimal: DataResponse<DecimalSymbolsV1> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(data_locale!("ar-EG")).as_borrowed(),
            ..Default::default()
        })
        .unwrap();
    assert_eq!(ar_decimal.payload.get().decimal_separator(), "٫");
    assert_eq!(ar_decimal.payload.get().numsys(), "arab");
}
