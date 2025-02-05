// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::experimental::relativetime::provider::*;
use icu::plurals::provider::PluralElementsPackedCow;
use icu::plurals::PluralElements;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub(crate) static MARKER_FILTERS: OnceLock<HashMap<DataMarkerInfo, &'static str>> = OnceLock::new();

fn marker_filters() -> &'static HashMap<DataMarkerInfo, &'static str> {
    MARKER_FILTERS.get_or_init(|| {
        [
            (LongSecondRelativeTimeFormatDataV1::INFO, "second"),
            (
                ShortSecondRelativeTimeFormatDataV1::INFO,
                "second-short",
            ),
            (
                NarrowSecondRelativeTimeFormatDataV1::INFO,
                "second-narrow",
            ),
            (LongMinuteRelativeTimeFormatDataV1::INFO, "minute"),
            (
                ShortMinuteRelativeTimeFormatDataV1::INFO,
                "minute-short",
            ),
            (
                NarrowMinuteRelativeTimeFormatDataV1::INFO,
                "minute-narrow",
            ),
            (LongHourRelativeTimeFormatDataV1::INFO, "hour"),
            (ShortHourRelativeTimeFormatDataV1::INFO, "hour-short"),
            (
                NarrowHourRelativeTimeFormatDataV1::INFO,
                "hour-narrow",
            ),
            (LongDayRelativeTimeFormatDataV1::INFO, "day"),
            (ShortDayRelativeTimeFormatDataV1::INFO, "day-short"),
            (NarrowDayRelativeTimeFormatDataV1::INFO, "day-narrow"),
            (LongWeekRelativeTimeFormatDataV1::INFO, "week"),
            (ShortWeekRelativeTimeFormatDataV1::INFO, "week-short"),
            (
                NarrowWeekRelativeTimeFormatDataV1::INFO,
                "week-narrow",
            ),
            (LongMonthRelativeTimeFormatDataV1::INFO, "month"),
            (
                ShortMonthRelativeTimeFormatDataV1::INFO,
                "month-short",
            ),
            (
                NarrowMonthRelativeTimeFormatDataV1::INFO,
                "month-narrow",
            ),
            (LongQuarterRelativeTimeFormatDataV1::INFO, "quarter"),
            (
                ShortQuarterRelativeTimeFormatDataV1::INFO,
                "quarter-short",
            ),
            (
                NarrowQuarterRelativeTimeFormatDataV1::INFO,
                "quarter-narrow",
            ),
            (LongYearRelativeTimeFormatDataV1::INFO, "year"),
            (ShortYearRelativeTimeFormatDataV1::INFO, "year-short"),
            (
                NarrowYearRelativeTimeFormatDataV1::INFO,
                "year-narrow",
            ),
        ]
        .into_iter()
        .collect()
    })
}
macro_rules! make_data_provider {
    ($($marker: ident),+ $(,)?) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let resource: &cldr_serde::date_fields::Resource = self
                        .cldr()?
                        .dates("gregorian")
                        .read_and_parse(req.id.locale, "dateFields.json")?;
                    let fields = &resource.main.value.dates.fields;

                    let field = marker_filters()
                        .get(&$marker::INFO)
                        .ok_or(DataErrorKind::MarkerNotFound.into_error())?;

                    let data = fields.0.get(*field).ok_or(DataError::custom(
                        "Field not found in relative time format data.",
                    ))?;

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(RelativeTimePatternData {
                            relatives: data.relatives.iter().map(|r| (&r.count, r.pattern.as_ref())).collect(),
                            past: (&data.past).into(),
                            future: (&data.future).into(),
                        }),
                    })
                }
            }

            impl IterableDataProviderCached<$marker> for SourceDataProvider {
                fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                    Ok(self
                        .cldr()?
                        .dates("gregorian")
                        .list_locales()?
                        .map(DataIdentifierCow::from_locale)
                        .collect())
                }
            }
        )+
    };
}

impl From<&cldr_serde::date_fields::PluralRulesPattern>
    for PluralElementsPackedCow<'_, SinglePlaceholderPattern>
{
    fn from(field: &cldr_serde::date_fields::PluralRulesPattern) -> Self {
        PluralElements::new(&*field.other)
            .with_zero_value(field.zero.as_deref())
            .with_one_value(field.one.as_deref())
            .with_two_value(field.two.as_deref())
            .with_few_value(field.few.as_deref())
            .with_many_value(field.many.as_deref())
            .with_explicit_one_value(field.explicit_one.as_deref())
            .with_explicit_zero_value(field.explicit_zero.as_deref())
            .into()
    }
}

make_data_provider!(
    LongSecondRelativeTimeFormatDataV1,
    ShortSecondRelativeTimeFormatDataV1,
    NarrowSecondRelativeTimeFormatDataV1,
    LongMinuteRelativeTimeFormatDataV1,
    ShortMinuteRelativeTimeFormatDataV1,
    NarrowMinuteRelativeTimeFormatDataV1,
    LongHourRelativeTimeFormatDataV1,
    ShortHourRelativeTimeFormatDataV1,
    NarrowHourRelativeTimeFormatDataV1,
    LongDayRelativeTimeFormatDataV1,
    ShortDayRelativeTimeFormatDataV1,
    NarrowDayRelativeTimeFormatDataV1,
    LongWeekRelativeTimeFormatDataV1,
    ShortWeekRelativeTimeFormatDataV1,
    NarrowWeekRelativeTimeFormatDataV1,
    LongMonthRelativeTimeFormatDataV1,
    ShortMonthRelativeTimeFormatDataV1,
    NarrowMonthRelativeTimeFormatDataV1,
    LongQuarterRelativeTimeFormatDataV1,
    ShortQuarterRelativeTimeFormatDataV1,
    NarrowQuarterRelativeTimeFormatDataV1,
    LongYearRelativeTimeFormatDataV1,
    ShortYearRelativeTimeFormatDataV1,
    NarrowYearRelativeTimeFormatDataV1,
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu::locale::langid;
    use icu::plurals::PluralRules;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<ShortQuarterRelativeTimeFormatDataV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let rules =
            PluralRules::try_new_cardinal_unstable(&provider, langid!("en").into()).unwrap();
        assert_eq!(data.get().relatives.get(&0).unwrap(), "this qtr.");
        assert_writeable_eq!(
            data.get().past.get(1.into(), &rules).interpolate([1]),
            "1 qtr. ago"
        );
        assert_writeable_eq!(
            data.get().past.get(2.into(), &rules).interpolate([2]),
            "2 qtrs. ago"
        );
        assert_writeable_eq!(
            data.get().future.get(1.into(), &rules).interpolate([1]),
            "in 1 qtr."
        );
    }

    #[test]
    fn test_singular_sub_pattern() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<LongYearRelativeTimeFormatDataV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("ar").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let rules =
            PluralRules::try_new_cardinal_unstable(&provider, langid!("ar").into()).unwrap();
        assert_eq!(data.get().relatives.get(&-1).unwrap(), "السنة الماضية");

        // past.one, future.two are without a placeholder.
        assert_writeable_eq!(
            data.get().past.get(1.into(), &rules).interpolate([1]),
            "قبل سنة واحدة"
        );
        assert_writeable_eq!(
            data.get().future.get(2.into(), &rules).interpolate([2]),
            "خلال سنتين"
        );

        assert_writeable_eq!(
            data.get().past.get(15.into(), &rules).interpolate([15]),
            "قبل 15 سنة"
        );
        assert_writeable_eq!(
            data.get().future.get(100.into(), &rules).interpolate([100]),
            "خلال 100 سنة"
        );
    }
}
