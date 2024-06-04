// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::borrow::Borrow;

use crate::provider::transform::cldr::cldr_serde;
use crate::provider::DatagenProvider;
use crate::provider::IterableDataProviderCached;
use icu_experimental::relativetime::provider::*;
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::str::FromStr;
use std::sync::OnceLock;

pub(in crate::provider) static DATAKEY_FILTERS: OnceLock<HashMap<DataKey, &'static str>> =
    OnceLock::new();

fn datakey_filters() -> &'static HashMap<DataKey, &'static str> {
    DATAKEY_FILTERS.get_or_init(|| {
        [
            (LongSecondRelativeTimeFormatDataV1Marker::KEY, "second"),
            (
                ShortSecondRelativeTimeFormatDataV1Marker::KEY,
                "second-short",
            ),
            (
                NarrowSecondRelativeTimeFormatDataV1Marker::KEY,
                "second-narrow",
            ),
            (LongMinuteRelativeTimeFormatDataV1Marker::KEY, "minute"),
            (
                ShortMinuteRelativeTimeFormatDataV1Marker::KEY,
                "minute-short",
            ),
            (
                NarrowMinuteRelativeTimeFormatDataV1Marker::KEY,
                "minute-narrow",
            ),
            (LongHourRelativeTimeFormatDataV1Marker::KEY, "hour"),
            (ShortHourRelativeTimeFormatDataV1Marker::KEY, "hour-short"),
            (NarrowHourRelativeTimeFormatDataV1Marker::KEY, "hour-narrow"),
            (LongDayRelativeTimeFormatDataV1Marker::KEY, "day"),
            (ShortDayRelativeTimeFormatDataV1Marker::KEY, "day-short"),
            (NarrowDayRelativeTimeFormatDataV1Marker::KEY, "day-narrow"),
            (LongWeekRelativeTimeFormatDataV1Marker::KEY, "week"),
            (ShortWeekRelativeTimeFormatDataV1Marker::KEY, "week-short"),
            (NarrowWeekRelativeTimeFormatDataV1Marker::KEY, "week-narrow"),
            (LongMonthRelativeTimeFormatDataV1Marker::KEY, "month"),
            (ShortMonthRelativeTimeFormatDataV1Marker::KEY, "month-short"),
            (
                NarrowMonthRelativeTimeFormatDataV1Marker::KEY,
                "month-narrow",
            ),
            (LongQuarterRelativeTimeFormatDataV1Marker::KEY, "quarter"),
            (
                ShortQuarterRelativeTimeFormatDataV1Marker::KEY,
                "quarter-short",
            ),
            (
                NarrowQuarterRelativeTimeFormatDataV1Marker::KEY,
                "quarter-narrow",
            ),
            (LongYearRelativeTimeFormatDataV1Marker::KEY, "year"),
            (ShortYearRelativeTimeFormatDataV1Marker::KEY, "year-short"),
            (NarrowYearRelativeTimeFormatDataV1Marker::KEY, "year-narrow"),
        ]
        .into_iter()
        .collect()
    })
}
macro_rules! make_data_provider {
    ($($marker: ident),+ $(,)?) => {
        $(
            impl DataProvider<$marker> for DatagenProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let langid = req.locale.get_langid();
                    let resource: &cldr_serde::date_fields::Resource = self
                        .cldr()?
                        .dates("gregorian")
                        .read_and_parse(&langid, "dateFields.json")?;
                    let fields = &resource.main.value.dates.fields;

                    let field = datakey_filters()
                        .get(&$marker::KEY)
                        .ok_or(DataErrorKind::MissingDataKey.into_error())?;

                    let data = fields.0.get(*field).ok_or(DataError::custom(
                        "Field not found in relative time format data.",
                    ))?;

                    Ok(DataResponse {
            metadata: Default::default(),
                        payload: Some(DataPayload::from_owned(data.try_into()?)),
                    })
                }
            }

            impl IterableDataProviderCached<$marker> for DatagenProvider {
                fn supported_requests_cached(&self) -> Result<HashSet<(DataLocale, DataKeyAttributes)>, DataError> {
                    Ok(self
                        .cldr()?
                        .dates("gregorian")
                        .list_langs()?
                        .map(|l| (DataLocale::from(l), Default::default()))
                        .collect())
                }
            }
        )+
    };
}

impl TryFrom<&cldr_serde::date_fields::Field> for RelativeTimePatternDataV1<'_> {
    type Error = DataError;
    fn try_from(field: &cldr_serde::date_fields::Field) -> Result<Self, Self::Error> {
        let mut relatives = BTreeMap::new();
        for relative in &field.relatives {
            relatives.insert(&relative.count, relative.pattern.as_ref());
        }
        Ok(Self {
            relatives: relatives.into_iter().collect(),
            past: PluralRulesCategoryMapping::try_from(&field.past)?,
            future: PluralRulesCategoryMapping::try_from(&field.future)?,
        })
    }
}

/// Try to convert an `Option<String>` to [`SingularSubPattern`].
/// If pattern is `None`, we return `None`
/// If pattern is `Some(pattern)`, we try to parse the pattern as [`SingularSubPattern`] failing
/// if an error is encountered
fn optional_convert<'a, B: Borrow<Option<String>>>(
    pattern: B,
) -> Result<Option<SingularSubPattern<'a>>, DataError> {
    pattern
        .borrow()
        .as_ref()
        .map(|s| SingularSubPattern::from_str(s))
        .transpose()
}

impl TryFrom<&cldr_serde::date_fields::PluralRulesPattern> for PluralRulesCategoryMapping<'_> {
    type Error = DataError;
    fn try_from(
        pattern: &cldr_serde::date_fields::PluralRulesPattern,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            zero: optional_convert(&pattern.zero)?,
            one: optional_convert(&pattern.one)?,
            two: optional_convert(&pattern.two)?,
            few: optional_convert(&pattern.few)?,
            many: optional_convert(&pattern.many)?,
            other: SingularSubPattern::from_str(&pattern.other)?,
        })
    }
}

make_data_provider!(
    LongSecondRelativeTimeFormatDataV1Marker,
    ShortSecondRelativeTimeFormatDataV1Marker,
    NarrowSecondRelativeTimeFormatDataV1Marker,
    LongMinuteRelativeTimeFormatDataV1Marker,
    ShortMinuteRelativeTimeFormatDataV1Marker,
    NarrowMinuteRelativeTimeFormatDataV1Marker,
    LongHourRelativeTimeFormatDataV1Marker,
    ShortHourRelativeTimeFormatDataV1Marker,
    NarrowHourRelativeTimeFormatDataV1Marker,
    LongDayRelativeTimeFormatDataV1Marker,
    ShortDayRelativeTimeFormatDataV1Marker,
    NarrowDayRelativeTimeFormatDataV1Marker,
    LongWeekRelativeTimeFormatDataV1Marker,
    ShortWeekRelativeTimeFormatDataV1Marker,
    NarrowWeekRelativeTimeFormatDataV1Marker,
    LongMonthRelativeTimeFormatDataV1Marker,
    ShortMonthRelativeTimeFormatDataV1Marker,
    NarrowMonthRelativeTimeFormatDataV1Marker,
    LongQuarterRelativeTimeFormatDataV1Marker,
    ShortQuarterRelativeTimeFormatDataV1Marker,
    NarrowQuarterRelativeTimeFormatDataV1Marker,
    LongYearRelativeTimeFormatDataV1Marker,
    ShortYearRelativeTimeFormatDataV1Marker,
    NarrowYearRelativeTimeFormatDataV1Marker,
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu_locale_core::langid;

    #[test]
    fn test_basic() {
        let provider = DatagenProvider::new_testing();
        let data: DataPayload<ShortQuarterRelativeTimeFormatDataV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("en").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(data.get().relatives.get(&0).unwrap(), "this qtr.");
        assert_eq!(data.get().past.one.as_ref().unwrap().pattern, " qtr. ago");
        assert_eq!(data.get().past.one.as_ref().unwrap().index, 0u8);
        assert_eq!(data.get().past.other.pattern, " qtrs. ago");
        assert_eq!(data.get().past.other.index, 0u8);
        assert_eq!(data.get().future.one.as_ref().unwrap().pattern, "in  qtr.");
        assert_eq!(data.get().future.one.as_ref().unwrap().index, 3u8);
    }

    #[test]
    fn test_singular_sub_pattern() {
        let provider = DatagenProvider::new_testing();
        let data: DataPayload<LongYearRelativeTimeFormatDataV1Marker> = provider
            .load(DataRequest {
                locale: &langid!("ar").into(),
                ..Default::default()
            })
            .unwrap()
            .take_payload()
            .unwrap();
        assert_eq!(data.get().relatives.get(&-1).unwrap(), "السنة الماضية");

        // past.one, future.two are without a placeholder.
        assert_eq!(
            data.get().past.one.as_ref().unwrap().pattern,
            "قبل سنة واحدة"
        );
        assert_eq!(data.get().past.one.as_ref().unwrap().index, 255u8);
        assert_eq!(
            data.get().future.two.as_ref().unwrap().pattern,
            "خلال سنتين"
        );
        assert_eq!(data.get().future.two.as_ref().unwrap().index, 255u8);

        assert_eq!(data.get().past.many.as_ref().unwrap().pattern, "قبل  سنة");
        assert_eq!(data.get().past.many.as_ref().unwrap().index, 7u8);
        assert_eq!(data.get().future.other.pattern, "خلال  سنة");
        assert_eq!(data.get().future.other.index, 9u8);
    }
}
