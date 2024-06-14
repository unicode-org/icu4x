// @generated
include!("datetime_buddhist_datelengths_v1.rs.data");
include!("datetime_buddhist_datesymbols_v1.rs.data");
include!("datetime_chinese_datelengths_v1.rs.data");
include!("datetime_chinese_datesymbols_v1.rs.data");
include!("datetime_coptic_datelengths_v1.rs.data");
include!("datetime_coptic_datesymbols_v1.rs.data");
include!("datetime_dangi_datelengths_v1.rs.data");
include!("datetime_dangi_datesymbols_v1.rs.data");
include!("datetime_ethiopic_datelengths_v1.rs.data");
include!("datetime_ethiopic_datesymbols_v1.rs.data");
include!("datetime_gregory_datelengths_v1.rs.data");
include!("datetime_gregory_datesymbols_v1.rs.data");
include!("datetime_hebrew_datelengths_v1.rs.data");
include!("datetime_hebrew_datesymbols_v1.rs.data");
include!("datetime_indian_datelengths_v1.rs.data");
include!("datetime_indian_datesymbols_v1.rs.data");
include!("datetime_islamic_datelengths_v1.rs.data");
include!("datetime_islamic_datesymbols_v1.rs.data");
include!("datetime_japanese_datelengths_v1.rs.data");
include!("datetime_japanese_datesymbols_v1.rs.data");
include!("datetime_japanext_datelengths_v1.rs.data");
include!("datetime_japanext_datesymbols_v1.rs.data");
include!("datetime_patterns_buddhist_date_v1.rs.data");
include!("datetime_patterns_buddhist_skeleton_v1.rs.data");
include!("datetime_patterns_chinese_date_v1.rs.data");
include!("datetime_patterns_chinese_skeleton_v1.rs.data");
include!("datetime_patterns_coptic_date_v1.rs.data");
include!("datetime_patterns_coptic_skeleton_v1.rs.data");
include!("datetime_patterns_dangi_date_v1.rs.data");
include!("datetime_patterns_dangi_skeleton_v1.rs.data");
include!("datetime_patterns_datetime_v1.rs.data");
include!("datetime_patterns_ethiopic_date_v1.rs.data");
include!("datetime_patterns_ethiopic_skeleton_v1.rs.data");
include!("datetime_patterns_gregory_date_v1.rs.data");
include!("datetime_patterns_gregory_skeleton_v1.rs.data");
include!("datetime_patterns_hebrew_date_v1.rs.data");
include!("datetime_patterns_hebrew_skeleton_v1.rs.data");
include!("datetime_patterns_indian_date_v1.rs.data");
include!("datetime_patterns_indian_skeleton_v1.rs.data");
include!("datetime_patterns_islamic_date_v1.rs.data");
include!("datetime_patterns_islamic_skeleton_v1.rs.data");
include!("datetime_patterns_japanese_date_v1.rs.data");
include!("datetime_patterns_japanese_skeleton_v1.rs.data");
include!("datetime_patterns_japanext_date_v1.rs.data");
include!("datetime_patterns_japanext_skeleton_v1.rs.data");
include!("datetime_patterns_persian_date_v1.rs.data");
include!("datetime_patterns_persian_skeleton_v1.rs.data");
include!("datetime_patterns_roc_date_v1.rs.data");
include!("datetime_patterns_roc_skeleton_v1.rs.data");
include!("datetime_patterns_time_v1.rs.data");
include!("datetime_patterns_time_skeleton_v1.rs.data");
include!("datetime_persian_datelengths_v1.rs.data");
include!("datetime_persian_datesymbols_v1.rs.data");
include!("datetime_roc_datelengths_v1.rs.data");
include!("datetime_roc_datesymbols_v1.rs.data");
include!("datetime_symbols_buddhist_months_v1.rs.data");
include!("datetime_symbols_buddhist_years_v1.rs.data");
include!("datetime_symbols_chinese_months_v1.rs.data");
include!("datetime_symbols_chinese_years_v1.rs.data");
include!("datetime_symbols_coptic_months_v1.rs.data");
include!("datetime_symbols_coptic_years_v1.rs.data");
include!("datetime_symbols_dangi_months_v1.rs.data");
include!("datetime_symbols_dangi_years_v1.rs.data");
include!("datetime_symbols_dayperiods_v1.rs.data");
include!("datetime_symbols_ethiopic_months_v1.rs.data");
include!("datetime_symbols_ethiopic_years_v1.rs.data");
include!("datetime_symbols_gregory_months_v1.rs.data");
include!("datetime_symbols_gregory_years_v1.rs.data");
include!("datetime_symbols_hebrew_months_v1.rs.data");
include!("datetime_symbols_hebrew_years_v1.rs.data");
include!("datetime_symbols_indian_months_v1.rs.data");
include!("datetime_symbols_indian_years_v1.rs.data");
include!("datetime_symbols_islamic_months_v1.rs.data");
include!("datetime_symbols_islamic_years_v1.rs.data");
include!("datetime_symbols_japanese_months_v1.rs.data");
include!("datetime_symbols_japanese_years_v1.rs.data");
include!("datetime_symbols_japanext_months_v1.rs.data");
include!("datetime_symbols_japanext_years_v1.rs.data");
include!("datetime_symbols_persian_months_v1.rs.data");
include!("datetime_symbols_persian_years_v1.rs.data");
include!("datetime_symbols_roc_months_v1.rs.data");
include!("datetime_symbols_roc_years_v1.rs.data");
include!("datetime_symbols_weekdays_v1.rs.data");
include!("datetime_timelengths_v1.rs.data");
include!("datetime_timesymbols_v1.rs.data");
include!("time_zone_exemplar_cities_v1.rs.data");
include!("time_zone_formats_v1.rs.data");
include!("time_zone_generic_long_v1.rs.data");
include!("time_zone_generic_short_v1.rs.data");
include!("time_zone_specific_long_v1.rs.data");
include!("time_zone_specific_short_v1.rs.data");
/// Marks a type as a data provider. You can then use macros like
/// `impl_core_helloworld_v1` to add implementations.
///
/// ```ignore
/// struct MyProvider;
/// const _: () = {
///     include!("path/to/generated/macros.rs");
///     make_provider!(MyProvider);
///     impl_core_helloworld_v1!(MyProvider);
/// }
/// ```
#[doc(hidden)]
#[macro_export]
macro_rules! __make_provider {
    ($ name : ty) => {
        #[clippy::msrv = "1.70"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
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
        impl_datetime_patterns_buddhist_date_v1!($provider);
        impl_datetime_patterns_buddhist_skeleton_v1!($provider);
        impl_datetime_patterns_chinese_date_v1!($provider);
        impl_datetime_patterns_chinese_skeleton_v1!($provider);
        impl_datetime_patterns_coptic_date_v1!($provider);
        impl_datetime_patterns_coptic_skeleton_v1!($provider);
        impl_datetime_patterns_dangi_date_v1!($provider);
        impl_datetime_patterns_dangi_skeleton_v1!($provider);
        impl_datetime_patterns_datetime_v1!($provider);
        impl_datetime_patterns_ethiopic_date_v1!($provider);
        impl_datetime_patterns_ethiopic_skeleton_v1!($provider);
        impl_datetime_patterns_gregory_date_v1!($provider);
        impl_datetime_patterns_gregory_skeleton_v1!($provider);
        impl_datetime_patterns_hebrew_date_v1!($provider);
        impl_datetime_patterns_hebrew_skeleton_v1!($provider);
        impl_datetime_patterns_indian_date_v1!($provider);
        impl_datetime_patterns_indian_skeleton_v1!($provider);
        impl_datetime_patterns_islamic_date_v1!($provider);
        impl_datetime_patterns_islamic_skeleton_v1!($provider);
        impl_datetime_patterns_japanese_date_v1!($provider);
        impl_datetime_patterns_japanese_skeleton_v1!($provider);
        impl_datetime_patterns_japanext_date_v1!($provider);
        impl_datetime_patterns_japanext_skeleton_v1!($provider);
        impl_datetime_patterns_persian_date_v1!($provider);
        impl_datetime_patterns_persian_skeleton_v1!($provider);
        impl_datetime_patterns_roc_date_v1!($provider);
        impl_datetime_patterns_roc_skeleton_v1!($provider);
        impl_datetime_patterns_time_v1!($provider);
        impl_datetime_patterns_time_skeleton_v1!($provider);
        impl_datetime_persian_datelengths_v1!($provider);
        impl_datetime_persian_datesymbols_v1!($provider);
        impl_datetime_roc_datelengths_v1!($provider);
        impl_datetime_roc_datesymbols_v1!($provider);
        impl_datetime_symbols_buddhist_months_v1!($provider);
        impl_datetime_symbols_buddhist_years_v1!($provider);
        impl_datetime_symbols_chinese_months_v1!($provider);
        impl_datetime_symbols_chinese_years_v1!($provider);
        impl_datetime_symbols_coptic_months_v1!($provider);
        impl_datetime_symbols_coptic_years_v1!($provider);
        impl_datetime_symbols_dangi_months_v1!($provider);
        impl_datetime_symbols_dangi_years_v1!($provider);
        impl_datetime_symbols_dayperiods_v1!($provider);
        impl_datetime_symbols_ethiopic_months_v1!($provider);
        impl_datetime_symbols_ethiopic_years_v1!($provider);
        impl_datetime_symbols_gregory_months_v1!($provider);
        impl_datetime_symbols_gregory_years_v1!($provider);
        impl_datetime_symbols_hebrew_months_v1!($provider);
        impl_datetime_symbols_hebrew_years_v1!($provider);
        impl_datetime_symbols_indian_months_v1!($provider);
        impl_datetime_symbols_indian_years_v1!($provider);
        impl_datetime_symbols_islamic_months_v1!($provider);
        impl_datetime_symbols_islamic_years_v1!($provider);
        impl_datetime_symbols_japanese_months_v1!($provider);
        impl_datetime_symbols_japanese_years_v1!($provider);
        impl_datetime_symbols_japanext_months_v1!($provider);
        impl_datetime_symbols_japanext_years_v1!($provider);
        impl_datetime_symbols_persian_months_v1!($provider);
        impl_datetime_symbols_persian_years_v1!($provider);
        impl_datetime_symbols_roc_months_v1!($provider);
        impl_datetime_symbols_roc_years_v1!($provider);
        impl_datetime_symbols_weekdays_v1!($provider);
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
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.70"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.path.hashed() {
                    h if h == <icu::datetime::provider::calendar::BuddhistDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::BuddhistDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::BuddhistDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::BuddhistDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::ChineseDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::ChineseDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::ChineseDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::CopticDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::CopticDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::CopticDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::CopticDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DangiDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DangiDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DangiDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DangiDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::EthiopianDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::EthiopianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::EthiopianDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::EthiopianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::GregorianDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::GregorianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::GregorianDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::GregorianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::HebrewDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::HebrewDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::HebrewDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::HebrewDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IndianDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IndianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IndianDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IndianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IslamicDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IslamicDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::IslamicDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::IslamicDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseExtendedDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::JapaneseExtendedDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DateTimePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DateTimePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocDatePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocDatePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocDateNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocDateNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::TimePatternV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::TimePatternV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::TimeNeoSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::TimeNeoSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::PersianDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::PersianDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::PersianDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::PersianDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::RocDateLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::RocDateLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::RocDateSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::RocDateSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::DateSkeletonPatternsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::DateSkeletonPatternsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DayPeriodNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DayPeriodNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocMonthNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocMonthNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocYearNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocYearNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::WeekdayNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::WeekdayNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::TimeLengthsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::TimeLengthsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::calendar::TimeSymbolsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::calendar::TimeSymbolsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::ExemplarCitiesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::ExemplarCitiesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::TimeZoneFormatsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::TimeZoneFormatsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataMarker.with_req(marker, req)),
                }
            }
        }
    };
}
