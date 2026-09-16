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
use tinystr::TinyAsciiStr;
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

        let currencies: &cldr_serde::currencies::data::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "currencies.json")?;

        let overrides = currencies
            .main
            .value
            .numbers
            .currencies
            .get(currency.iso_code().as_str())
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
                .map(|(currency, _)| currency.as_str());

            // The overrides are not scoped to a numbering system, but the rest of the
            // symbols are, so each numbering system needs its own identifier.
            for currency in overriding {
                let Ok(currency) = CurrencyType::try_from_str(currency) else {
                    log::error!(
                        "Skipping non-ISO-4217 currency override for '{currency}' in locale {locale}"
                    );
                    continue;
                };

                for nsname in self
                    .get_supported_numsys_for_langid(&locale, true)?
                    .iter()
                    .map(|nu| Some(Subtag::try_from_str(nu.as_str()).unwrap()))
                    .chain([None])
                {
                    #[allow(const_item_mutation)]
                    ids.insert(
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            CurrencyDecimalSymbolsV1::make_attributes(
                                nsname,
                                currency,
                                &mut TinyAsciiStr::EMPTY,
                            ),
                            &locale,
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
