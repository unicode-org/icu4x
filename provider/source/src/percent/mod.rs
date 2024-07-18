// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;
use std::collections::HashSet;

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::percent::*;
use icu_pattern::Pattern;
use icu_pattern::SinglePlaceholder;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;
use icu_provider::DataProvider;
use tinystr::tinystr;

impl DataProvider<PercentEssentialsV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<PercentEssentialsV1Marker>, DataError> {
        self.check_req::<PercentEssentialsV1Marker>(req)?;
        let langid = req.id.locale.get_langid();

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(&langid, "numbers.json")?;

        let result = extract_percent_essentials(numbers_resource);

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(result?),
        })
    }
}

impl IterableDataProviderCached<PercentEssentialsV1Marker> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_langs()?
            .map(|l| DataIdentifierCow::from_locale(DataLocale::from(l)))
            .collect())
    }
}

fn extract_percent_essentials<'data>(
    numbers_resource: &cldr_serde::numbers::Resource,
) -> Result<PercentEssentialsV1<'data>, DataError> {
    // TODO(#3838): these patterns might be numbering system dependent.
    let percent_patterns = &&numbers_resource
        .main
        .value
        .numbers
        .numsys_data
        .percent_patterns
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    // TODO(#3838): these patterns might be numbering system dependent.
    let symbols = &&numbers_resource
        .main
        .value
        .numbers
        .numsys_data
        .symbols
        .get(&tinystr!(8, "latn"))
        .ok_or_else(|| DataError::custom("Could not find the percent symbol"))?;

    let percent_sign = symbols.percent_sign.to_owned();
    let standard_pattern = &percent_patterns.standard.to_owned();

    // If the standard_pattern includes a `;` character,
    // there is both a positive and negative pattern to consider.
    match standard_pattern.contains(';') {
        true => {
            let mut patterns = standard_pattern.split(';');
            let positive_pattern = patterns
                .next()
                .ok_or_else(|| DataError::custom("Could not parse positive pattern."))?;
            let negative_pattern = patterns
                .next()
                .ok_or_else(|| DataError::custom("Could not parse negative pattern."))?;
            Ok(PercentEssentialsV1 {
                positive_pattern: create_pattern(positive_pattern, &percent_sign)?,
                negative_pattern: create_pattern(negative_pattern, &percent_sign)?,
            })
        }
        false => Ok(PercentEssentialsV1 {
            positive_pattern: create_pattern(standard_pattern, &percent_sign)?,
            negative_pattern: create_pattern(standard_pattern, &percent_sign)?,
        }),
    }
}

fn create_pattern<'a>(
    pattern: &str,
    percent_sign: &str,
) -> Result<Pattern<SinglePlaceholder, Cow<'a, str>>, DataError> {
    // While all locales use the `%`, some include non-breaking spaces.
    // Hence using the literal `%` char here.
    let percent_sign_index = pattern.find('%').unwrap();
    let first_num_index = pattern.find(['0', '#']).unwrap();
    let last_num_index = pattern.rfind(['0', '#']).unwrap();

    // For the prefix, if the first character is a percent sign, then we have no prefix.
    // If the percent sign is first, then all characters before the percent sign are the prefix.
    // If the percent comes after, then all characters between final number and the percent sign are the prefix.
    let percent_prefix = if percent_sign_index == 0 {
        ""
    } else if percent_sign_index < first_num_index {
        &pattern[0..percent_sign_index]
    } else {
        &pattern[last_num_index + 1..percent_sign_index]
    };

    // For the suffix, if the first character is a percent sign, OR the percent sign is before the first number,
    // then all characters between are the suffix.
    // If the percent sign comes after the first number, then all proceeding characters are the suffix.
    let percent_suffix = if percent_sign_index == 0 || percent_sign_index < first_num_index {
        &pattern[1..first_num_index]
    } else {
        &pattern[percent_sign_index + 1..]
    };

    let percent_symbol = String::new() + percent_prefix + percent_sign + percent_suffix;

    // Example: "#,##0%", "#,##0 %", "%#,##0", "% #,##0"
    let pattern = match percent_sign_index > first_num_index {
        true => "{0}".to_owned() + &percent_symbol,
        false => percent_symbol + "{0}",
    }
    .parse::<SinglePlaceholderPattern<_>>()
    .map_err(|e| DataError::custom("Could not parse pattern").with_display_context(&e))?;

    let pattern: Pattern<SinglePlaceholder, Cow<'_, str>> =
        Pattern::from_store_unchecked(Cow::Owned(pattern.take_store()));

    Ok(pattern)
}

#[test]
fn test_basic() {
    use icu::experimental::dimension::provider::percent::*;
    use icu::locale::langid;
    use writeable::assert_writeable_eq;

    let provider = SourceDataProvider::new_testing();

    let en: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("en").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    // en Should resemble "#,##0%"
    let en_pattern = en.payload.get().to_owned();
    assert_writeable_eq!(en_pattern.positive_pattern.interpolate(["123"]), "123%");
    assert_writeable_eq!(en_pattern.negative_pattern.interpolate(["-123"]), "-123%");
    assert_eq!(en_pattern.positive_pattern.take_store(), "\u{1}%");

    let fr: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("fr").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    // fr Should resemble "#,##0 %"
    let fr_pattern = fr.payload.get().to_owned();
    assert_writeable_eq!(
        fr_pattern.positive_pattern.interpolate(["234"]),
        "234\u{a0}%"
    );
    assert_writeable_eq!(
        fr_pattern.negative_pattern.interpolate(["-234"]),
        "-234\u{a0}%"
    );
    assert_eq!(fr_pattern.positive_pattern.take_store(), "\u{1}\u{a0}%");

    let tr: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("tr").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    // tr Should resemble "%#,##0"
    let tr_pattern = tr.payload.get().to_owned();
    assert_writeable_eq!(tr_pattern.positive_pattern.interpolate(["345"]), "%345");
    assert_writeable_eq!(tr_pattern.negative_pattern.interpolate(["-345"]), "%-345");
    assert_eq!(tr_pattern.positive_pattern.take_store(), "\u{2}%");

    let ar_eg: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("ar-EG").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    // ar_eg Should resemble "#,##0‎%‎"
    let ar_eg_pattern = ar_eg.payload.get().to_owned();
    assert_writeable_eq!(
        ar_eg_pattern.positive_pattern.interpolate(["456"]),
        "456\u{200e}%\u{200e}"
    );
    assert_writeable_eq!(
        ar_eg_pattern.negative_pattern.interpolate(["-456"]),
        "-456\u{200e}%\u{200e}"
    );
    assert_eq!(
        ar_eg_pattern.positive_pattern.take_store(),
        "\u{1}\u{200e}%\u{200e}"
    );
}
