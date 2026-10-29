// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use crate::cldr_serde::currencies::data::CurrencyPatterns;

use std::collections::{BTreeMap, HashSet};

use icu::experimental::dimension::provider::currency::symbols::*;
use icu::locale::preferences::extensions::unicode::keywords::CurrencyType;
use icu::properties::props::{GeneralCategory, GeneralCategoryGroup};
use icu::properties::{CodePointMapData, CodePointMapDataBorrowed};
use icu_provider::prelude::*;

/// The symbol CLDR uses when a locale explicitly has no currency symbol, e.g. `PTE` in
/// `pt-PT`, where the cifrão ('$') is the decimal separator instead (CLDR-19771).
///
/// CLDR needs a non-empty value there to block locale inheritance. Datagen already
/// resolves inheritance, so we store the empty string instead: an empty symbol means
/// "use the no-currency pattern". This is different from a missing entry, which falls
/// back to the ISO code.
const CLDR_NO_SYMBOL_MARKER: &str = "\u{200B}";

impl DataProvider<CurrencySymbolsV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencySymbolsV1>, DataError> {
        self.check_req::<CurrencySymbolsV1>(req)?;

        let not_found = || DataErrorKind::IdentifierNotFound.with_req(CurrencySymbolsV1::INFO, req);

        let (currency, width) =
            CurrencySymbolsV1::parse_attributes(req.id.marker_attributes).ok_or_else(not_found)?;
        let symbol = currency_patterns(self, req.id.locale)?
            .get(currency.iso_code().as_str())
            .and_then(|patterns| symbol_for_width(patterns, width))
            .ok_or_else(not_found)?;

        let gc = CodePointMapData::<GeneralCategory>::try_new_unstable(self)?;
        let gc = gc.as_borrowed();

        let symbol = match symbol {
            CLDR_NO_SYMBOL_MARKER => "",
            // If CLDR starts marking "no symbol" in another way (e.g. a different invisible
            // character, or an empty string), it would end up being formatted as a visible
            // symbol, or as the no-currency pattern by accident. Warn so that datagen can
            // be updated.
            symbol if is_invisible(symbol, gc) => {
                log::warn!(
                    "Unexpected invisible currency symbol for {currency} in {}: {symbol:?}; \
                     only {CLDR_NO_SYMBOL_MARKER:?} is mapped to the no-currency pattern",
                    req.id.locale,
                );
                symbol
            }
            symbol => symbol,
        };

        let (starts_with_letter, ends_with_letter) = starts_and_ends_with_letter(symbol, gc);

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
        let mut ids = HashSet::new();

        for locale in self.cldr()?.numbers().list_locales()? {
            for (iso_code, patterns) in currency_patterns(self, &locale)? {
                let Ok(currency) = CurrencyType::try_from_str(iso_code) else {
                    log::error!(
                        "Skipping non-ISO-4217 currency symbol for '{iso_code}' in locale {locale}"
                    );
                    continue;
                };
                for width in [CurrencySymbolsV1::SHORT, CurrencySymbolsV1::NARROW] {
                    // Symbols equal to the ISO code (`USD` in `en-AU`) are kept as data:
                    // without an entry, the runtime would fall back to the parent locale's
                    // symbol (`US$` from `en-001`).
                    if symbol_for_width(patterns, width).is_some() {
                        ids.insert(DataIdentifierCow::from_owned(
                            CurrencySymbolsV1::make_attributes(
                                currency,
                                width,
                                &mut Default::default(),
                            )
                            .to_owned(),
                            locale,
                        ));
                    }
                }
            }
        }

        Ok(ids)
    }
}

/// Returns the `currencies.json` entries of `locale`, keyed by ISO code.
fn currency_patterns<'a>(
    provider: &'a SourceDataProvider,
    locale: &DataLocale,
) -> Result<&'a BTreeMap<String, CurrencyPatterns>, DataError> {
    Ok(&provider
        .cldr()?
        .numbers()
        .read_and_parse::<cldr_serde::currencies::data::Resource>(locale, "currencies.json")?
        .main
        .value
        .numbers
        .currencies)
}

/// Returns the CLDR symbol of the given width, if there is one.
///
/// UTS #35 lateral inheritance: a missing narrow symbol falls back to the standard
/// symbol. Resolving this here means consumers need a single request.
fn symbol_for_width(patterns: &CurrencyPatterns, width: CurrencySymbolWidth) -> Option<&str> {
    match width {
        CurrencySymbolWidth::Narrow => patterns.narrow.as_deref().or(patterns.short.as_deref()),
        _ => patterns.short.as_deref(),
    }
}

/// Returns whether `symbol` only has invisible characters (format characters and
/// separators). This is also true for the empty string.
fn is_invisible(symbol: &str, gc: CodePointMapDataBorrowed<GeneralCategory>) -> bool {
    symbol.chars().all(|c| {
        let gc = gc.get(c);
        gc == GeneralCategory::Format || GeneralCategoryGroup::Separator.contains(gc)
    })
}

/// Returns whether the first and the last character of `symbol` are letters. The
/// formatter uses this to choose the `alphaNextToNumber` patterns.
fn starts_and_ends_with_letter(
    symbol: &str,
    gc: CodePointMapDataBorrowed<GeneralCategory>,
) -> (bool, bool) {
    // TODO: This is not entirely correct. We need to look at the first/last grapheme cluster.
    let is_letter =
        |c: Option<char>| c.is_some_and(|c| GeneralCategoryGroup::Letter.contains(gc.get(c)));
    (
        is_letter(symbol.chars().next()),
        is_letter(symbol.chars().next_back()),
    )
}

#[test]
fn test_symbols() {
    use icu::locale::Locale;

    const SHORT: CurrencySymbolWidth = CurrencySymbolsV1::SHORT;
    const NARROW: CurrencySymbolWidth = CurrencySymbolsV1::NARROW;

    /// Checks the symbol and its starts/ends-with-letter flags, or that there is no entry.
    #[track_caller]
    fn check(
        locale: &str,
        currency: &str,
        width: CurrencySymbolWidth,
        expected: Option<(&str, bool, bool)>,
    ) {
        let locale = DataLocale::from(Locale::try_from_str(locale).unwrap());
        let currency = CurrencyType::try_from_str(currency).unwrap();
        let payload = DataProvider::<CurrencySymbolsV1>::load(
            &SourceDataProvider::new_testing(),
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
        .map(|response| response.payload);
        assert_eq!(
            payload.as_ref().map(|payload| payload.get()),
            expected
                .map(|(symbol, starts, ends)| CurrencySymbol::new(symbol, starts, ends))
                .as_ref()
        );
    }

    check("en", "USD", SHORT, Some(("$", false, false)));
    check("en", "USD", NARROW, Some(("$", false, false)));
    check("ar-EG", "EGP", SHORT, Some(("ج.م.\u{200f}", true, false)));
    check("ar-EG", "EGP", NARROW, Some(("E£", true, false)));
    check("ar-EG", "USD", SHORT, Some(("US$", true, false)));
    check("ar-EG", "USD", NARROW, Some(("US$", true, false)));

    // A symbol equal to the ISO code is stored like any other symbol. Without its own
    // entry, the runtime would fall back to an ancestor's symbol: `es` uses `€` for EUR,
    // and root uses `JP¥` for JPY.
    check("en", "EGP", SHORT, Some(("EGP", true, true)));
    check("en", "EGP", NARROW, Some(("E£", true, false)));
    check("es-AR", "EUR", SHORT, Some(("EUR", true, true)));
    check("fr", "JPY", SHORT, Some(("JPY", true, true)));

    // `fr` defines a standard symbol for FRF but no narrow symbol: the narrow entry
    // carries the standard symbol (UTS #35 lateral inheritance, resolved at datagen).
    check("fr", "FRF", SHORT, Some(("F", true, true)));
    check("fr", "FRF", NARROW, Some(("F", true, true)));

    // No symbol of either width: no narrow entry is synthesized either.
    check("fr", "PTE", SHORT, None);
    check("fr", "PTE", NARROW, None);

    // `pt-PT` has no symbol for `PTE`: CLDR marks this with U+200B, which datagen
    // stores as the empty string (= use the no-currency pattern). If this fails, CLDR
    // has changed how it marks "no symbol" (CLDR-19771), and datagen needs updating.
    check("pt-PT", "PTE", SHORT, Some(("", false, false)));
    check("pt-PT", "PTE", NARROW, Some(("", false, false)));
}
