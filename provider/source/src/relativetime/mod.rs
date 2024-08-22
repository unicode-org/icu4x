// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use icu::experimental::relativetime::provider::*;
use icu_pattern::SinglePlaceholder;
use icu_provider::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

pub(crate) static MARKER_FILTERS: OnceLock<HashMap<DataMarkerInfo, &'static str>> = OnceLock::new();

fn marker_filters() -> &'static HashMap<DataMarkerInfo, &'static str> {
    MARKER_FILTERS.get_or_init(|| {
        [
            (LongSecondRelativeTimeFormatDataV1Marker::INFO, "second"),
            (
                ShortSecondRelativeTimeFormatDataV1Marker::INFO,
                "second-short",
            ),
            (
                NarrowSecondRelativeTimeFormatDataV1Marker::INFO,
                "second-narrow",
            ),
            (LongMinuteRelativeTimeFormatDataV1Marker::INFO, "minute"),
            (
                ShortMinuteRelativeTimeFormatDataV1Marker::INFO,
                "minute-short",
            ),
            (
                NarrowMinuteRelativeTimeFormatDataV1Marker::INFO,
                "minute-narrow",
            ),
            (LongHourRelativeTimeFormatDataV1Marker::INFO, "hour"),
            (ShortHourRelativeTimeFormatDataV1Marker::INFO, "hour-short"),
            (
                NarrowHourRelativeTimeFormatDataV1Marker::INFO,
                "hour-narrow",
            ),
            (LongDayRelativeTimeFormatDataV1Marker::INFO, "day"),
            (ShortDayRelativeTimeFormatDataV1Marker::INFO, "day-short"),
            (NarrowDayRelativeTimeFormatDataV1Marker::INFO, "day-narrow"),
            (LongWeekRelativeTimeFormatDataV1Marker::INFO, "week"),
            (ShortWeekRelativeTimeFormatDataV1Marker::INFO, "week-short"),
            (
                NarrowWeekRelativeTimeFormatDataV1Marker::INFO,
                "week-narrow",
            ),
            (LongMonthRelativeTimeFormatDataV1Marker::INFO, "month"),
            (
                ShortMonthRelativeTimeFormatDataV1Marker::INFO,
                "month-short",
            ),
            (
                NarrowMonthRelativeTimeFormatDataV1Marker::INFO,
                "month-narrow",
            ),
            (LongQuarterRelativeTimeFormatDataV1Marker::INFO, "quarter"),
            (
                ShortQuarterRelativeTimeFormatDataV1Marker::INFO,
                "quarter-short",
            ),
            (
                NarrowQuarterRelativeTimeFormatDataV1Marker::INFO,
                "quarter-narrow",
            ),
            (LongYearRelativeTimeFormatDataV1Marker::INFO, "year"),
            (ShortYearRelativeTimeFormatDataV1Marker::INFO, "year-short"),
            (
                NarrowYearRelativeTimeFormatDataV1Marker::INFO,
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
                        payload: DataPayload::from_owned(RelativeTimePatternDataV1 {
                            relatives: data.relatives.iter().map(|r| (&r.count, r.pattern.as_ref())).collect(),
                            past: (&data.past).try_into()?,
                            future: (&data.future).try_into()?,
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

impl TryFrom<&cldr_serde::date_fields::PluralRulesPattern>
    for PluralPattern<'_, SinglePlaceholder>
{
    type Error = DataError;
    fn try_from(field: &cldr_serde::date_fields::PluralRulesPattern) -> Result<Self, Self::Error> {
        PluralPattern::try_new(
            &field.other,
            field.zero.as_deref(),
            field.one.as_deref(),
            field.two.as_deref(),
            field.few.as_deref(),
            field.many.as_deref(),
        )
        .map_err(|_| DataError::custom("Invalid pattern"))
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
    use icu::locale::langid;
    use icu::plurals::PluralCategory;
    use writeable::assert_writeable_eq;

    #[test]
    fn test_basic() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<ShortQuarterRelativeTimeFormatDataV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("en").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        assert_eq!(data.get().relatives.get(&0).unwrap(), "this qtr.");
        assert_writeable_eq!(
            data.get().past.get(PluralCategory::One).interpolate([1]),
            "1 qtr. ago"
        );
        assert_writeable_eq!(
            data.get().past.get(PluralCategory::Other).interpolate([2]),
            "2 qtrs. ago"
        );
        assert_writeable_eq!(
            data.get().future.get(PluralCategory::One).interpolate([1]),
            "in 1 qtr."
        );
    }

    #[test]
    fn test_singular_sub_pattern() {
        let provider = SourceDataProvider::new_testing();
        let data: DataPayload<LongYearRelativeTimeFormatDataV1Marker> = provider
            .load(DataRequest {
                id: DataIdentifierBorrowed::for_locale(&langid!("ar").into()),
                ..Default::default()
            })
            .unwrap()
            .payload;
        assert_eq!(data.get().relatives.get(&-1).unwrap(), "السنة الماضية");

        // past.one, future.two are without a placeholder.
        assert_writeable_eq!(
            data.get().past.get(PluralCategory::One).interpolate([1]),
            "قبل سنة واحدة"
        );
        assert_writeable_eq!(
            data.get().future.get(PluralCategory::Two).interpolate([2]),
            "خلال سنتين"
        );

        assert_writeable_eq!(
            data.get().past.get(PluralCategory::Many).interpolate([5]),
            "قبل 5 سنة"
        );
        assert_writeable_eq!(
            data.get()
                .future
                .get(PluralCategory::Other)
                .interpolate([6]),
            "خلال 6 سنة"
        );
    }
}
