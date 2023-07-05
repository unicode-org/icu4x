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
        impl_datetime_buddhist_datelengths_v1!($provider);
        impl_datetime_buddhist_datesymbols_v1!($provider);
        impl_datetime_coptic_datelengths_v1!($provider);
        impl_datetime_coptic_datesymbols_v1!($provider);
        impl_datetime_ethiopic_datelengths_v1!($provider);
        impl_datetime_ethiopic_datesymbols_v1!($provider);
        impl_datetime_gregory_datelengths_v1!($provider);
        impl_datetime_gregory_datesymbols_v1!($provider);
        impl_datetime_indian_datelengths_v1!($provider);
        impl_datetime_indian_datesymbols_v1!($provider);
        impl_datetime_japanese_datelengths_v1!($provider);
        impl_datetime_japanese_datesymbols_v1!($provider);
        impl_datetime_japanext_datelengths_v1!($provider);
        impl_datetime_japanext_datesymbols_v1!($provider);
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
                const DATETIME_BUDDHIST_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_BUDDHIST_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_COPTIC_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::CopticDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_COPTIC_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_ETHIOPIC_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_ETHIOPIC_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_GREGORY_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::GregorianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_GREGORY_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_INDIAN_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::IndianDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_INDIAN_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::IndianDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_JAPANESE_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_JAPANESE_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_JAPANEXT_DATELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_JAPANEXT_DATESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_SKELETONS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_TIMELENGTHS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const DATETIME_TIMESYMBOLS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::calendar::TimeSymbolsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const TIME_ZONE_EXEMPLAR_CITIES_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::ExemplarCitiesV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const TIME_ZONE_FORMATS_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const TIME_ZONE_GENERIC_LONG_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const TIME_ZONE_GENERIC_SHORT_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const TIME_ZONE_SPECIFIC_LONG_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                const TIME_ZONE_SPECIFIC_SHORT_V1: icu_provider::DataKeyHash = <icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::KeyedDataMarker>::KEY.hashed();
                match key.hashed() {
                    DATETIME_BUDDHIST_DATELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::BuddhistDateLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_BUDDHIST_DATESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::BuddhistDateSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_COPTIC_DATELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::CopticDateLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_COPTIC_DATESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::CopticDateSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_ETHIOPIC_DATELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::EthiopianDateLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_ETHIOPIC_DATESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::EthiopianDateSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_GREGORY_DATELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::GregorianDateLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_GREGORY_DATESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::GregorianDateSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_INDIAN_DATELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::IndianDateLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_INDIAN_DATESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::IndianDateSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_JAPANESE_DATELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::JapaneseDateLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_JAPANESE_DATESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::JapaneseDateSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_JAPANEXT_DATELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_JAPANEXT_DATESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_SKELETONS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::DateSkeletonPatternsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_TIMELENGTHS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::TimeLengthsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    DATETIME_TIMESYMBOLS_V1 => icu_provider::DataProvider::<icu_datetime::provider::calendar::TimeSymbolsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    TIME_ZONE_EXEMPLAR_CITIES_V1 => icu_provider::DataProvider::<icu_datetime::provider::time_zones::ExemplarCitiesV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    TIME_ZONE_FORMATS_V1 => icu_provider::DataProvider::<icu_datetime::provider::time_zones::TimeZoneFormatsV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    TIME_ZONE_GENERIC_LONG_V1 => icu_provider::DataProvider::<icu_datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    TIME_ZONE_GENERIC_SHORT_V1 => icu_provider::DataProvider::<icu_datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    TIME_ZONE_SPECIFIC_LONG_V1 => icu_provider::DataProvider::<icu_datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
                    TIME_ZONE_SPECIFIC_SHORT_V1 => icu_provider::DataProvider::<icu_datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker>::load(self, req).and_then(|r| r.take_metadata_and_payload()).map(|(metadata, payload)| icu_provider::AnyResponse { payload: Some(payload.wrap_into_any_payload()), metadata }),
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
