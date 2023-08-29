// @generated
include!("macros.rs");
/// Implement `DataProvider<M>` on the given struct using the data
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
        impl_datetime_buddhist_datelengths_v1!($provider);
        impl_datetime_buddhist_datesymbols_v1!($provider);
        impl_datetime_chinese_datelengths_v1!($provider);
        impl_datetime_chinese_datesymbols_v1!($provider);
        impl_datetime_coptic_datelengths_v1!($provider);
        impl_datetime_coptic_datesymbols_v1!($provider);
        impl_datetime_dangi_datelengths_v1!($provider);
        impl_datetime_dangi_datesymbols_v1!($provider);
        impl_datetime_ethiopic_datelengths_v1!($provider);
        impl_datetime_ethiopic_datesymbols_v1!($provider);
        impl_datetime_gregory_datelengths_v1!($provider);
        impl_datetime_gregory_datesymbols_v1!($provider);
        impl_datetime_hebrew_datelengths_v1!($provider);
        impl_datetime_hebrew_datesymbols_v1!($provider);
        impl_datetime_indian_datelengths_v1!($provider);
        impl_datetime_indian_datesymbols_v1!($provider);
        impl_datetime_islamic_datelengths_v1!($provider);
        impl_datetime_islamic_datesymbols_v1!($provider);
        impl_datetime_japanese_datelengths_v1!($provider);
        impl_datetime_japanese_datesymbols_v1!($provider);
        impl_datetime_japanext_datelengths_v1!($provider);
        impl_datetime_japanext_datesymbols_v1!($provider);
        impl_datetime_persian_datelengths_v1!($provider);
        impl_datetime_persian_datesymbols_v1!($provider);
        impl_datetime_roc_datelengths_v1!($provider);
        impl_datetime_roc_datesymbols_v1!($provider);
        impl_datetime_skeletons_v1!($provider);
        impl_datetime_timelengths_v1!($provider);
        impl_datetime_timesymbols_v1!($provider);
        impl_time_zone_exemplar_cities_v1!($provider);
        impl_time_zone_formats_v1!($provider);
        impl_time_zone_generic_long_v1!($provider);
        impl_time_zone_generic_short_v1!($provider);
        impl_time_zone_specific_long_v1!($provider);
        impl_time_zone_specific_short_v1!($provider);
    };
}
#[doc(inline)]
pub use __impl_data_provider as impl_data_provider;
/// Implement `AnyProvider` on the given struct using the data
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
        #[clippy::msrv = "1.65"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, key: icu_provider::DataKey, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match key.hashed() {
                    h if h == <icu::datetime::provider::calendar::BuddhistDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::BuddhistDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::BuddhistDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::BuddhistDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::ChineseDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::ChineseDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::CopticDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::CopticDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::CopticDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DangiDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DangiDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DangiDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DangiDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::EthiopianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::EthiopianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::EthiopianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::GregorianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::GregorianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::GregorianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::GregorianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::HebrewDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::HebrewDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::HebrewDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::HebrewDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IndianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IndianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IndianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IndianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IslamicDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IslamicDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IslamicDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IslamicDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::PersianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::PersianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::PersianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::PersianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::RocDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::RocDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::RocDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::RocDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DateSkeletonPatternsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DateSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::TimeLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::TimeSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::TimeSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::ExemplarCitiesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::ExemplarCitiesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::TimeZoneFormatsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataKey.with_req(key, req)),
                }
            }
        }
    };
}
#[doc(inline)]
pub use __impl_any_provider as impl_any_provider;
#[clippy::msrv = "1.65"]
pub struct BakedDataProvider;
impl_data_provider!(BakedDataProvider);
