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
            (LongSecondRelativeV1::INFO, "second"),
            (ShortSecondRelativeV1::INFO, "second-short"),
            (NarrowSecondRelativeV1::INFO, "second-narrow"),
            (LongMinuteRelativeV1::INFO, "minute"),
            (ShortMinuteRelativeV1::INFO, "minute-short"),
            (NarrowMinuteRelativeV1::INFO, "minute-narrow"),
            (LongHourRelativeV1::INFO, "hour"),
            (ShortHourRelativeV1::INFO, "hour-short"),
            (NarrowHourRelativeV1::INFO, "hour-narrow"),
            (LongDayRelativeV1::INFO, "day"),
            (ShortDayRelativeV1::INFO, "day-short"),
            (NarrowDayRelativeV1::INFO, "day-narrow"),
            (LongWeekRelativeV1::INFO, "week"),
            (ShortWeekRelativeV1::INFO, "week-short"),
            (NarrowWeekRelativeV1::INFO, "week-narrow"),
            (LongMonthRelativeV1::INFO, "month"),
            (ShortMonthRelativeV1::INFO, "month-short"),
            (NarrowMonthRelativeV1::INFO, "month-narrow"),
            (LongQuarterRelativeV1::INFO, "quarter"),
            (ShortQuarterRelativeV1::INFO, "quarter-short"),
            (NarrowQuarterRelativeV1::INFO, "quarter-narrow"),
            (LongYearRelativeV1::INFO, "year"),
            (ShortYearRelativeV1::INFO, "year-short"),
            (NarrowYearRelativeV1::INFO, "year-narrow"),
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
    LongSecondRelativeV1,
    ShortSecondRelativeV1,
    NarrowSecondRelativeV1,
    LongMinuteRelativeV1,
    ShortMinuteRelativeV1,
    NarrowMinuteRelativeV1,
    LongHourRelativeV1,
    ShortHourRelativeV1,
    NarrowHourRelativeV1,
    LongDayRelativeV1,
    ShortDayRelativeV1,
    NarrowDayRelativeV1,
    LongWeekRelativeV1,
    ShortWeekRelativeV1,
    NarrowWeekRelativeV1,
    LongMonthRelativeV1,
    ShortMonthRelativeV1,
    NarrowMonthRelativeV1,
    LongQuarterRelativeV1,
    ShortQuarterRelativeV1,
    NarrowQuarterRelativeV1,
    LongYearRelativeV1,
    ShortYearRelativeV1,
    NarrowYearRelativeV1,
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
        let data: DataPayload<ShortQuarterRelativeV1> = provider
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
        let data: DataPayload<LongYearRelativeV1> = provider
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
