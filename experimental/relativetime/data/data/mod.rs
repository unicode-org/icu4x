// @generated
include!("macros.rs");
macro_rules! impl_data_provider {
    ($ provider : path) => {
        impl $provider {
            #[doc(hidden)]
            #[allow(dead_code)]
            pub const MUST_USE_CREATE_PROVIDER_MACRO: () = ();
        }
        impl_relativetime_long_day_v1!($provider);
        impl_relativetime_long_hour_v1!($provider);
        impl_relativetime_long_minute_v1!($provider);
        impl_relativetime_long_month_v1!($provider);
        impl_relativetime_long_quarter_v1!($provider);
        impl_relativetime_long_second_v1!($provider);
        impl_relativetime_long_week_v1!($provider);
        impl_relativetime_long_year_v1!($provider);
        impl_relativetime_narrow_day_v1!($provider);
        impl_relativetime_narrow_hour_v1!($provider);
        impl_relativetime_narrow_minute_v1!($provider);
        impl_relativetime_narrow_month_v1!($provider);
        impl_relativetime_narrow_quarter_v1!($provider);
        impl_relativetime_narrow_second_v1!($provider);
        impl_relativetime_narrow_week_v1!($provider);
        impl_relativetime_narrow_year_v1!($provider);
        impl_relativetime_short_day_v1!($provider);
        impl_relativetime_short_hour_v1!($provider);
        impl_relativetime_short_minute_v1!($provider);
        impl_relativetime_short_month_v1!($provider);
        impl_relativetime_short_quarter_v1!($provider);
        impl_relativetime_short_second_v1!($provider);
        impl_relativetime_short_week_v1!($provider);
        impl_relativetime_short_year_v1!($provider);
    };
}
macro_rules! impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.66"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[clippy::msrv = "1.66"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
