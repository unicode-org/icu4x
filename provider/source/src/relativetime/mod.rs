// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::cldr_serde;
use icu::experimental::relativetime::provider::*;
use icu::plurals::PluralElements;
use icu::plurals::provider::PluralElementsPackedCow;
use icu_pattern::SinglePlaceholderPattern;
use icu_provider::prelude::*;
use std::collections::HashSet;

macro_rules! make_data_provider {
    ($(($marker: ident, $field: literal)),+ $(,)?) => {
        $(
            impl DataProvider<$marker> for SourceDataProvider {
                fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                    self.check_req::<$marker>(req)?;
                    let resource: &cldr_serde::date_fields::Resource = self
                        .cldr()?
                        .dates(None)
                        .read_and_parse(req.id.locale, "dateFields.json")?;
                    let fields = &resource.main.value.dates.fields;

                    let data = fields.0.get($field).ok_or(DataError::custom(
                        "Field not found in relative time format data.",
                    ))?;

                    let min_count = data.relatives.keys().next().copied().unwrap_or(0).min(0);
                    let max_count = data.relatives.keys().next_back().copied().unwrap_or(-1);
                    let relatives_vec: Vec<&str> = (min_count..=max_count)
                        .map(|c| data.relatives.get(&c).map(String::as_str).unwrap_or(""))
                        .collect();

                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(RelativeTimePatternData {
                            zero_index: min_count.unsigned_abs(),
                            relatives: zerovec::VarZeroVec::from(&relatives_vec),
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
                        .dates(None)
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
    (DatetimeRelativeSecondLongV1, "second"),
    (DatetimeRelativeSecondShortV1, "second-short"),
    (DatetimeRelativeSecondNarrowV1, "second-narrow"),
    (DatetimeRelativeMinuteLongV1, "minute"),
    (DatetimeRelativeMinuteShortV1, "minute-short"),
    (DatetimeRelativeMinuteNarrowV1, "minute-narrow"),
    (DatetimeRelativeHourLongV1, "hour"),
    (DatetimeRelativeHourShortV1, "hour-short"),
    (DatetimeRelativeHourNarrowV1, "hour-narrow"),
    (DatetimeRelativeDayLongV1, "day"),
    (DatetimeRelativeDayShortV1, "day-short"),
    (DatetimeRelativeDayNarrowV1, "day-narrow"),
    (DatetimeRelativeWeekLongV1, "week"),
    (DatetimeRelativeWeekShortV1, "week-short"),
    (DatetimeRelativeWeekNarrowV1, "week-narrow"),
    (DatetimeRelativeMonthLongV1, "month"),
    (DatetimeRelativeMonthShortV1, "month-short"),
    (DatetimeRelativeMonthNarrowV1, "month-narrow"),
    (DatetimeRelativeQuarterLongV1, "quarter"),
    (DatetimeRelativeQuarterShortV1, "quarter-short"),
    (DatetimeRelativeQuarterNarrowV1, "quarter-narrow"),
    (DatetimeRelativeYearLongV1, "year"),
    (DatetimeRelativeYearShortV1, "year-short"),
    (DatetimeRelativeYearNarrowV1, "year-narrow"),
);

#[cfg(test)]
mod tests {
    use super::*;
    use icu::locale::{data_locale, locale};
    use icu::plurals::PluralRules;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<DatetimeRelativeQuarterShortV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("en")),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let rules =
            PluralRules::try_new_cardinal_unstable(&provider, locale!("en").into()).unwrap();
        assert_eq!(data.get().get_relative(0).unwrap(), "this qtr.");
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
        let data: DataPayload<DatetimeRelativeYearLongV1> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&data_locale!("ar")),
                ..Default::default()
            })
            .unwrap()
            .payload;
        let rules =
            PluralRules::try_new_cardinal_unstable(&provider, locale!("ar").into()).unwrap();
        assert_eq!(data.get().get_relative(-1).unwrap(), "السنة الماضية");

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
