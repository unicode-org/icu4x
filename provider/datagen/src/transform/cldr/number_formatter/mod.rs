// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locid::LanguageIdentifier;
use icu_provider::DataProvider;

use crate::transform::cldr::cldr_serde;
use icu_plurals::provider::*;
use icu_plurals::rules::runtime::ast::Rule;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use icu_singlenumberformatter::provider::*;
use std::borrow::Cow;
use std::convert::TryFrom;
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
        //.get(&tinystr!(10, "currencies"))
        .get(&tinystr!(3, "USD"))
        .ok_or_else(|| DataError::custom("Could not find the USD data"))?;

    // Cow::from(String::clone(&usd.symbol))
    // Cow::from(usd.symbol.clone())
    // usd.symbol.clone().into()

    let result = CurrencyEssentialV1 {
        symbol: usd.symbol.clone().into(),
        pattern: CurrencyPattern {
            index: 0,
            pattern: Cow::from("not yet"),
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
}
