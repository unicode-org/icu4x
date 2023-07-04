// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_datetime::provider::calendar::patterns;
use icu_locid::LanguageIdentifier;
use icu_normalizer::DecomposingNormalizer;
use icu_provider::DataProvider;
use tinystr::TinyStr8;
use zerovec::VarZeroVec;
use zerovec::ZeroMap;
use zerovec::maps::MutableZeroVecLike;

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

use super::cldr_serde::currencies;
use super::cldr_serde::currency_data;

impl DataProvider<CurrencyEssentialV1Maker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CurrencyEssentialV1Maker>, DataError> {
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

        let currency_data: &cldr_serde::currency_data::Resource = self
            .source
            .cldr()?
            .core()
            .read_and_parse("supplemental/currencyData.json")?;

        let result = extract_currency_essential(&currencies_resource, &numbers_resource, &currency_data,&langid);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: Some(DataPayload::from_owned(result?)),
        })
    }
}

impl IterableDataProvider<CurrencyEssentialV1Maker> for crate::DatagenProvider {
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
    currency_data_resource: &cldr_serde::currency_data::Resource,
    langid: &LanguageIdentifier,
) -> Result<CurrencyEssentialV1<'data>, DataError> {
    let currencies = &currencies_resource
        .main
        .0
        .get(&langid)
        .expect("CLDR file contains the expected language")
        .numbers
        .currencies;

    let currency_data = &currency_data_resource
        .supplemental
        .currency_data
        .fractions
        .currencies;

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

    let mut indices_map: ZeroMap<'_, TinyAsciiStr<3>, PatternsIndices> = ZeroMap::<TinyAsciiStr<3>, PatternsIndices>::new();
    // let mut currencies_meta_data = VarZeroVec::<CurrencyPatternWithMetaDataULE>::new();
    let mut place_holders = VarZeroVec::<str>::new();
    for (iso, currency_pattern) in currencies {
        let mut short_pattern_index = u16::MAX;
        let mut narrow_pattern_index = u16::MAX;
        let mut short_place_holder_index: u16;
        let mut narrow_place_holder_index: u16;

       let short_option= &currency_pattern.short;
       match short_option {
        Some(short_place_holder) => {
            short_place_holder_index = place_holders.len() as u16;
            place_holders.zvl_push(&short_place_holder);
        },
        None => short_place_holder_index = u16::MAX,
       }

       let narrow_option = &currency_pattern.narrow;
       match narrow_option {
        Some(narrow_place_holder) => {
            narrow_place_holder_index = place_holders.len() as u16;
            place_holders.zvl_push(&narrow_place_holder);
        },
        None => narrow_place_holder_index = u16::MAX,
       }

       indices_map.insert(iso, &PatternsIndices {
        short_pattern: short_pattern_index,
        narrow_pattern: narrow_pattern_index,
        short_place_holder: short_place_holder_index,
        narrow_place_holder: narrow_place_holder_index,
       });
    }

    // let patterns = currencies_patterns.get()

    // let patterns = VarZeroVec::<'data, str>::new();

    //     .get(&tinystr!(3, "USD"))
    //     .ok_or_else(|| DataError::custom("Could not find the USD data"))?;
    // let symbol = usd.symbol.clone().unwrap_or(String::from("")).into();

    // // let pattern = DecimalPattern::from_str(&currency_formats.standard);
    // // pattern?.positive;

    // let result = CurrencyEssentialV1 {
    //     symbol: symbol,
    //     pattern: CurrencyPattern {
    //         index: 0,
    //         pattern: currency_formats.standard.clone().into(),
    //     },
    // };

    let result = CurrencyEssentialV1 {
        indices_map,
        //    currencies_meta_data,
        place_holders,
    };

    Ok(result)
}

#[test]
fn test_basic() {
    use icu_locid::locale;
    use icu_singlenumberformatter::provider::*;
    let provider = crate::DatagenProvider::for_test();
    let usd: DataPayload<CurrencyEssentialV1Maker> = provider
        .load(DataRequest {
            locale: &locale!("ar-EG").into(),
            metadata: Default::default(),
        })
        .unwrap()
        .take_payload()
        .unwrap();
    // assert_eq!(usd.get().idices_map.get("USD").u, "US$");
    // assert_eq!(
    //     usd.get().pattern.pattern,
    //     "\u{200f}#,##0.00\u{a0}¤;\u{200f}-#,##0.00\u{a0}¤"
    // );
}
