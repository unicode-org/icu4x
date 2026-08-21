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

/// Marker attribute suffix identifying the monetary variant of a locale's decimal symbols.
///
/// The attribute is `currency` for the locale's default numbering system, and
/// `<numsys>-currency` otherwise, mirroring the standard `<numsys>` attributes.
const CURRENCY_ATTRIBUTE: &str = "currency";

/// Splits a marker attribute into its numbering system part and whether the monetary
/// variant was requested. An empty numbering system means the locale's default one.
fn split_currency_attribute(attributes: &str) -> (&str, bool) {
    if attributes == CURRENCY_ATTRIBUTE {
        ("", true)
    } else if let Some(nsname) = attributes.strip_suffix(&format!("-{CURRENCY_ATTRIBUTE}")) {
        (nsname, true)
    } else {
        (attributes, false)
    }
}

/// The marker attribute addressing the monetary symbols of `nsname`, which is the
/// locale's default numbering system if `is_default` is set.
fn currency_attribute(nsname: &str, is_default: bool) -> String {
    if is_default {
        CURRENCY_ATTRIBUTE.to_owned()
    } else {
        format!("{nsname}-{CURRENCY_ATTRIBUTE}")
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

        let (nsattr, is_currency) = split_currency_attribute(req.id.marker_attributes.as_str());

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

        // UTS 35 `<currencyDecimal>`/`<currencyGroup>`: locales can override the separators
        // used in monetary contexts. Only locales that carry at least one override have a
        // `-currency` identifier; everything else uses the standard symbols.
        let (decimal_separator, grouping_separator) = if is_currency {
            if symbols.currency_decimal.is_none() && symbols.currency_group.is_none() {
                return Err(DataErrorKind::IdentifierNotFound.with_req(DecimalSymbolsV1::INFO, req));
            }
            (
                symbols
                    .currency_decimal
                    .as_deref()
                    .unwrap_or(&symbols.decimal),
                symbols.currency_group.as_deref().unwrap_or(&symbols.group),
            )
        } else {
            (symbols.decimal.as_str(), symbols.group.as_str())
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

        // Monetary symbols are only generated for locales that actually override a
        // separator; consumers fall back to the standard symbols otherwise.
        for locale in self.cldr()?.numbers().list_locales()? {
            let resource: &cldr_serde::numbers::Resource = self
                .cldr()?
                .numbers()
                .read_and_parse(&locale, "numbers.json")?;

            let numbers = &resource.main.value.numbers;

            for (nsname, symbols) in numbers.numsys_data.symbols.iter() {
                if symbols.currency_decimal.is_none() && symbols.currency_group.is_none() {
                    continue;
                }
                let attribute =
                    currency_attribute(nsname, *nsname == numbers.default_numbering_system);
                let Ok(attribute) = DataMarkerAttributes::try_from_str(&attribute) else {
                    continue;
                };
                ids.insert(
                    DataIdentifierBorrowed::for_marker_attributes_and_locale(attribute, &locale)
                        .into_owned(),
                );
            }
        }

        Ok(ids)
    }
}

#[test]
fn test_currency_attribute_roundtrip() {
    assert_eq!(split_currency_attribute(""), ("", false));
    assert_eq!(split_currency_attribute("latn"), ("latn", false));
    assert_eq!(split_currency_attribute("currency"), ("", true));
    assert_eq!(split_currency_attribute("latn-currency"), ("latn", true));

    assert_eq!(currency_attribute("latn", true), "currency");
    assert_eq!(currency_attribute("latn", false), "latn-currency");
}

#[test]
fn test_currency_symbols_only_for_overriding_locales() {
    use icu::locale::langid;

    let provider = SourceDataProvider::new_testing();

    // No locale in the test data overrides `<currencyDecimal>`/`<currencyGroup>`, so
    // consumers get `IdentifierNotFound` and fall back to the standard symbols.
    let result = DataProvider::<DecimalSymbolsV1>::load(
        &provider,
        DataRequest {
            id: DataIdentifierBorrowed::for_marker_attributes_and_locale(
                DataMarkerAttributes::from_str_or_panic(CURRENCY_ATTRIBUTE),
                &langid!("en-ZA").into(),
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
            .all(|id| !split_currency_attribute(id.marker_attributes.as_str()).1)
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
