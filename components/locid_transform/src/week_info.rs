// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use crate::LocaleTransformError;
use icu_provider::prelude::*;
// use zerovec::ZeroVec;

/// Represents Week Information for a given locale, for calendar purposes.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct LocaleWeekInfo {
    /// The first day of a week.
    pub first_weekday: IsoWeekday,
    /// For a given week, the minimum number of that week's days present in a given month or year
    /// for the week to be considered part of that month or year.
    pub min_week_days: u8,
    // List of weekday values indicating which days of the week are considered as part of the 'weekend', for calendar purposes.
    // The number of days in the weekend may be different between locales, and may not be contiguous days within a locale.
    // pub weekend: Vec<IsoWeekday>,

    // TODO: [CODE REVIEW]
    // should we use Vec, ZeroVec, a tuple or something else here ?
    // pub weekend: ZeroVec<'data, IsoWeekday>,
}

impl LocaleWeekInfo {
    // FIXME: Baked trait bound issue.
    // icu_provider::gen_any_buffer_data_constructors!(
    //     locale: include,
    //     options: skip,
    //     error: LocaleTransformError,
    //     /// Creates a new [`LocaleWeekInfo`] from compiled locale data.
    // );

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::try_new)]
    pub fn try_new_unstable<P>(
        provider: &P,
        locale: &DataLocale,
    ) -> Result<Self, LocaleTransformError>
    where
        P: DataProvider<WeekDataV2Marker>,
    {
        provider
            .load(DataRequest {
                locale,
                metadata: Default::default(),
            })
            .and_then(DataResponse::take_payload)
            .map(|payload| payload.get().into())
            .map_err(Into::into)
    }
}

impl From<WeekDataV2<'_>> for LocaleWeekInfo {
    fn from(value: WeekDataV2) -> Self {
        Self {
            first_weekday: value.first_weekday,
            min_week_days: value.min_week_days,
            // weekend: value.weekend,
        }
    }
}

impl From<&WeekDataV2<'_>> for LocaleWeekInfo {
    fn from(value: &WeekDataV2) -> Self {
        Self {
            first_weekday: value.first_weekday,
            min_week_days: value.min_week_days,
            // weekend: value.weekend,ß
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LocaleWeekInfo;
    use crate::provider::*;
    use icu_locid::locale;
    // TODO: validate how to properly test these flows.
    use icu_provider_adapters::any_payload::AnyPayloadProvider;
    use zerovec::ZeroVec;

    #[test]
    fn test_week_info_data() {
        let provider =
            AnyPayloadProvider::from_owned::<WeekDataV2Marker>(crate::provider::WeekDataV2 {
                first_weekday: IsoWeekday::Saturday,
                min_week_days: 1,
                weekend: ZeroVec::alloc_from_slice(&[IsoWeekday::Saturday, IsoWeekday::Sunday]),
            });

        let week_info = LocaleWeekInfo::try_new_unstable(&provider, &locale!("en").into())
            .expect("Unable to load Locale Week Info from test provider");
        assert_eq!(week_info.first_weekday, IsoWeekday::Saturday);
        assert_eq!(week_info.min_week_days, 1);
    }
}
