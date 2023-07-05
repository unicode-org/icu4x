// @generated
include!("macros.rs");
/// Implement [`DataProvider<M>`](icu_provider::DataProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_unstable` constructors.
///
/// ```compile_fail
/// struct MyDataProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_data_provider(MyDataProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_data_provider {
    ($ provider : path) => {
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
#[doc(inline)]
pub use __impl_data_provider as impl_data_provider;
/// Implement [`AnyProvider`](icu_provider::AnyProvider) on the given struct using the data
/// hardcoded in this module. This allows the struct to be used with
/// `icu`'s `_any` constructors.
///
/// ```compile_fail
/// struct MyAnyProvider;
/// include!("/path/to/generated/mod.rs");
/// impl_any_provider(MyAnyProvider);
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __impl_any_provider {
    ($ provider : path) => {
        #[clippy::msrv = "1.61"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                const RELATIVETIME_LONG_DAY_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_LONG_HOUR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_LONG_MINUTE_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_LONG_MONTH_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_LONG_QUARTER_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_LONG_SECOND_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_LONG_WEEK_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_LONG_YEAR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_DAY_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_HOUR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_MINUTE_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_MONTH_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_QUARTER_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_SECOND_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_WEEK_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_NARROW_YEAR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_DAY_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_HOUR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_MINUTE_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_MONTH_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_QUARTER_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_SECOND_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_WEEK_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const RELATIVETIME_SHORT_YEAR_V1: icu_provider::DataKeyHash = <icu_relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    RELATIVETIME_LONG_DAY_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongDayRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_LONG_HOUR_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongHourRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_LONG_MINUTE_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_LONG_MONTH_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_LONG_QUARTER_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_LONG_SECOND_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_LONG_WEEK_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_LONG_YEAR_V1 => icu_provider::DataProvider::<icu_relativetime::provider::LongYearRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_DAY_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_HOUR_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_MINUTE_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_MONTH_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_QUARTER_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_SECOND_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_WEEK_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_NARROW_YEAR_V1 => icu_provider::DataProvider::<icu_relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_DAY_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_HOUR_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_MINUTE_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_MONTH_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_QUARTER_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_SECOND_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_WEEK_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    RELATIVETIME_SHORT_YEAR_V1 => icu_provider::DataProvider::<icu_relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
#[clippy::msrv = "1.61"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
