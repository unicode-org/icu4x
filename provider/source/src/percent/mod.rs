// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Cow;
use std::collections::HashSet;

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;

use icu::experimental::dimension::provider::percent::*;
use icu_pattern::DoublePlaceholder;
use icu_pattern::DoublePlaceholderPattern;
use icu_pattern::Pattern;
use icu_pattern::SinglePlaceholder;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;
use icu_provider::DataProvider;

impl DataProvider<PercentEssentialsV1Marker> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<PercentEssentialsV1Marker>, DataError> {
        self.check_req::<PercentEssentialsV1Marker>(req)?;

        let numbers_resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(req.id.locale, "numbers.json")?;

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
            .list_locales()?
            .map(DataIdentifierCow::from_locale)
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
        .get("latn")
        .ok_or_else(|| DataError::custom("Could not find the standard pattern"))?;

    // TODO(#3838): these patterns might be numbering system dependent.
    let symbols = &&numbers_resource
        .main
        .value
        .numbers
        .numsys_data
        .symbols
        .get("latn")
        .ok_or_else(|| DataError::custom("Could not find the percent symbol"))?;

    let localized_approximately_sign = symbols.approximately_sign.to_owned();
    let localized_minus_sign = symbols.minus_sign.to_owned();
    let localized_percent_sign = symbols.percent_sign.to_owned();
    let localized_plus_sign = symbols.plus_sign.to_owned();

    let standard_pattern = percent_patterns.standard.to_owned();

    let mut split = standard_pattern.split(';');
    let unsigned_pattern = split.next().unwrap();

    let signed_pattern: Cow<str> = match split.next() {
        Some(negative) => Cow::Borrowed(negative),
        None if standard_pattern.contains('-') => Cow::Borrowed(&standard_pattern),
        // When there is no explicit negative subpattern,
        // an implicit negative subpattern is formed from the positive pattern with a prefixed "-"
        // Ex: "#,##0%" -> "-#,##0%" || "% #,##0" -> "-% #,##0"
        None => Cow::Owned(String::from("-") + &standard_pattern),
    };

    Ok(PercentEssentialsV1 {
        unsigned_pattern: create_unsigned_pattern(unsigned_pattern, &localized_percent_sign)?,
        signed_pattern: create_signed_pattern(&signed_pattern, &localized_percent_sign)?,
        approximately_sign: localized_approximately_sign.into(),
        minus_sign: localized_minus_sign.into(),
        plus_sign: localized_plus_sign.into(),
    })
}

/// Used to create a percent pattern for negative, approximate, or explicit plus.
fn create_signed_pattern<'a>(
    pattern: &str,
    localized_percent_sign: &str,
) -> Result<Pattern<DoublePlaceholder, Cow<'a, str>>, DataError> {
    // While all locales use the `%`, some include non-breaking spaces.
    // Hence using the literal `%` char here.
    let percent_pattern_index = pattern.find('%').unwrap();
    let first_num_pattern_index = pattern.find(['0', '#']).unwrap();
    let last_num_pattern_index = pattern.rfind(['0', '#']).unwrap();
    let sign_pattern_index = pattern.rfind(['-']).unwrap();

    // Grab all the indexes
    let mut symbol_indexes = [
        sign_pattern_index,
        percent_pattern_index,
        first_num_pattern_index,
    ];
    // Sort them to get the correct order
    symbol_indexes.sort();

    // For the prefix, if the first character is a percent sign, then we have no prefix.
    // If the percent sign is first, then all characters before the percent sign are the prefix.
    // If the percent comes after, then all characters between final number and the percent sign are the prefix.
    let percent_prefix = if percent_pattern_index == 0 {
        ""
    } else if percent_pattern_index < first_num_pattern_index
        || percent_pattern_index < sign_pattern_index
    {
        let next_symbol_index = std::cmp::min(first_num_pattern_index, sign_pattern_index);
        &pattern[0..next_symbol_index]
    } else {
        &pattern[last_num_pattern_index + 1..percent_pattern_index]
    };

    // Index of the percent symbol inside the symbols vec.
    // Not to be confused with the index inside the pattern.
    let precent_symbol_index = symbol_indexes
        .iter()
        .position(|&i| i == percent_pattern_index)
        .unwrap();

    // If there are more symbols following the percent, then get all chars between.
    // Otherwise we're the last symbol, so get all chars until end of string
    let percent_suffix: &str = if symbol_indexes.len() > precent_symbol_index + 1 {
        let next_symbol_index = symbol_indexes.get(precent_symbol_index + 1).unwrap();
        &pattern[percent_pattern_index + 1..next_symbol_index.to_owned()]
    } else {
        &pattern[(percent_pattern_index + 1)..]
    };

    // Combine the prefix, localized sign, and suffix to create the localized symbol.
    let percent_symbol = format!("{percent_prefix}{localized_percent_sign}{percent_suffix}");

    let pattern_vec: Vec<&str> = symbol_indexes
        .into_iter()
        .map(|index| {
            if index == percent_pattern_index {
                return percent_symbol.as_str();
            }
            if index == sign_pattern_index {
                return "{1}";
            }

            "{0}"
        })
        .collect();

    // Example: "#,##0%", "#,##0 %", "%#,##0", "% #,##0"
    let pattern = pattern_vec
        .concat()
        .parse::<DoublePlaceholderPattern<_>>()
        .map_err(|e| DataError::custom("Could not parse pattern").with_display_context(&e))?;

    let pattern: Pattern<DoublePlaceholder, Cow<'_, str>> =
        Pattern::from_store_unchecked(Cow::Owned(pattern.take_store()));

    Ok(pattern)
}

/// Used only for positive percents.
/// If you need an approximate, explicit plus, or negative percent, use the negative pattern.
fn create_unsigned_pattern<'a>(
    pattern: &str,
    localized_percent_sign: &str,
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

    let percent_symbol = String::new() + percent_prefix + localized_percent_sign + percent_suffix;

    // Example: "#,##0%", "#,##0 %", "%#,##0", "% #,##0"
    let pattern = if percent_sign_index > first_num_index {
        "{0}".to_owned() + &percent_symbol
    } else {
        percent_symbol + "{0}"
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
    assert_writeable_eq!(en_pattern.unsigned_pattern.interpolate(["123"]), "123%");
    assert_writeable_eq!(en_pattern.signed_pattern.interpolate(["123", "+"]), "+123%");

    let tr: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("tr").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    // tr Should resemble "%#,##0"
    let tr_pattern = tr.payload.get().to_owned();
    assert_writeable_eq!(tr_pattern.unsigned_pattern.interpolate(["345"]), "%345");
    assert_writeable_eq!(tr_pattern.signed_pattern.interpolate(["345", "+"]), "+%345");

    let ar_eg: DataResponse<PercentEssentialsV1Marker> = provider
        .load(DataRequest {
            id: DataIdentifierCow::from_locale(langid!("ar-EG").into()).as_borrowed(),
            ..Default::default()
        })
        .unwrap();

    // ar_eg Should resemble "#,##0‎%‎"
    let ar_eg_pattern = ar_eg.payload.get().to_owned();
    assert_writeable_eq!(
        ar_eg_pattern.unsigned_pattern.interpolate(["456"]),
        "456\u{200e}%\u{200e}"
    );
    assert_writeable_eq!(
        ar_eg_pattern.signed_pattern.interpolate(["456", "+"]),
        "+456\u{200e}%\u{200e}"
    );
}

#[test]
fn blo_test() {
    use writeable::assert_writeable_eq;

    let blo_positive_pattern = create_unsigned_pattern("% #,#0", "%").unwrap();
    assert_writeable_eq!(blo_positive_pattern.interpolate(["123"]), "% 123");

    let blo_negative_pattern = create_signed_pattern("% -#,#0", "%").unwrap();
    assert_writeable_eq!(blo_negative_pattern.interpolate(["123", "+"]), "% +123");
    assert_writeable_eq!(blo_negative_pattern.interpolate(["123", "-"]), "% -123");
    assert_writeable_eq!(blo_negative_pattern.interpolate(["123", "~"]), "% ~123");
}
