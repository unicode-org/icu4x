// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_normalizer::DecomposingNormalizer;
use icu_provider::DataProvider;
use tinystr::TinyStr8;

use crate::transform::cldr::cldr_serde;
use crate::transform::cldr::decimal::decimal_pattern::DecimalPattern;
use icu_plurals::provider::*;
use icu_plurals::rules::runtime::ast::Rule;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_singlenumberformatter::provider::*;
use std::borrow::Cow;
use std::convert::TryFrom;
use std::str::FromStr;
use tinystr::tinystr;
use tinystr::TinyAsciiStr;

impl DataProvider<CurrencyEssentialUsdV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<CurrencyEssentialUsdV1Marker>, DataError> {
        let langid = req.locale.get_langid();

        let currencies_resource: &cldr_serde::currencies::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "currencies.json")?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let result = extract_currency_essential(&currencies_resource, &numbers_resource, &langid);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result?)),
        })
    }
}

impl IterableDataProvider<CurrencyEssentialUsdV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self
            .source
            .cldr()?
            .numbers()
            .list_langs()?
            .map(DataLocale::from)
            .collect())
    }
}

fn extract_currency_essential<'data>(
    currencies_resource: &cldr_serde::currencies::Resource,
    numbers_resource: &cldr_serde::numbers::Resource,
    langid: &LanguageIdentifier,
) -> Result<CurrencyEssentialV1<'data>, DataError> {
    let usd = &currencies_resource
        .main
        .0
        .get(&langid)
        .expect("CLDR file contains the expected language")
        .numbers
        .currencies
        .get(&tinystr!(3, "USD"))
        .ok_or_else(|| DataError::custom("Could not find the USD data"))?;
    let symbol = usd.symbol.clone().unwrap_or(String::from("")).into();

    let currency_formats = &&numbers_resource
        .main
        .0
        .get(&langid)
        .expect("CLDR file contains the expected language")
        .numbers
        .numsys_data
        .currency_formats
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    // let pattern = DecimalPattern::from_str(&currency_formats.standard);
    // pattern?.positive;

    let result = CurrencyEssentialV1 {
        symbol: symbol,
        pattern: CurrencyPattern {
            index: 0,
            pattern: currency_formats.standard.clone().into(),
        },
    };

    Ok(result)
}

#[test]
fn test_basic() {
    use icu_locid::locale;
    use icu_singlenumberformatter::provider::*;
    let provider = crate::DatagenProvider::for_test();
    let usd: DataPayload<CurrencyEssentialUsdV1Marker> = provider
        .load(DataRequest {
            locale: &locale!("ar-EG").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    assert_eq!(usd.get().symbol, "US$");
    assert_eq!(usd.get().pattern.pattern, "\u{200f}#,##0.00\u{a0}¤;\u{200f}-#,##0.00\u{a0}¤");
}
