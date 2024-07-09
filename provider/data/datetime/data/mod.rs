// @generated
include!("buddhist_date_lengths_v1_marker.rs.data");
include!("buddhist_date_symbols_v1_marker.rs.data");
include!("chinese_date_lengths_v1_marker.rs.data");
include!("chinese_date_symbols_v1_marker.rs.data");
include!("coptic_date_lengths_v1_marker.rs.data");
include!("coptic_date_symbols_v1_marker.rs.data");
include!("dangi_date_lengths_v1_marker.rs.data");
include!("dangi_date_symbols_v1_marker.rs.data");
include!("ethiopian_date_lengths_v1_marker.rs.data");
include!("ethiopian_date_symbols_v1_marker.rs.data");
include!("gregorian_date_lengths_v1_marker.rs.data");
include!("gregorian_date_symbols_v1_marker.rs.data");
include!("hebrew_date_lengths_v1_marker.rs.data");
include!("hebrew_date_symbols_v1_marker.rs.data");
include!("indian_date_lengths_v1_marker.rs.data");
include!("indian_date_symbols_v1_marker.rs.data");
include!("islamic_date_lengths_v1_marker.rs.data");
include!("islamic_date_symbols_v1_marker.rs.data");
include!("japanese_date_lengths_v1_marker.rs.data");
include!("japanese_date_symbols_v1_marker.rs.data");
include!("japanese_extended_date_lengths_v1_marker.rs.data");
include!("japanese_extended_date_symbols_v1_marker.rs.data");
include!("buddhist_date_pattern_v1_marker.rs.data");
include!("buddhist_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("chinese_date_pattern_v1_marker.rs.data");
include!("chinese_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("coptic_date_pattern_v1_marker.rs.data");
include!("coptic_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("dangi_date_pattern_v1_marker.rs.data");
include!("dangi_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("date_time_pattern_v1_marker.rs.data");
include!("ethiopian_date_pattern_v1_marker.rs.data");
include!("ethiopian_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("gregorian_date_pattern_v1_marker.rs.data");
include!("gregorian_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("hebrew_date_pattern_v1_marker.rs.data");
include!("hebrew_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("indian_date_pattern_v1_marker.rs.data");
include!("indian_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("islamic_date_pattern_v1_marker.rs.data");
include!("islamic_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("japanese_date_pattern_v1_marker.rs.data");
include!("japanese_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("japanese_extended_date_pattern_v1_marker.rs.data");
include!("japanese_extended_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("persian_date_pattern_v1_marker.rs.data");
include!("persian_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("roc_date_pattern_v1_marker.rs.data");
include!("roc_date_neo_skeleton_patterns_v1_marker.rs.data");
include!("time_pattern_v1_marker.rs.data");
include!("time_neo_skeleton_patterns_v1_marker.rs.data");
include!("persian_date_lengths_v1_marker.rs.data");
include!("persian_date_symbols_v1_marker.rs.data");
include!("roc_date_lengths_v1_marker.rs.data");
include!("roc_date_symbols_v1_marker.rs.data");
include!("buddhist_month_names_v1_marker.rs.data");
include!("buddhist_year_names_v1_marker.rs.data");
include!("chinese_month_names_v1_marker.rs.data");
include!("chinese_year_names_v1_marker.rs.data");
include!("coptic_month_names_v1_marker.rs.data");
include!("coptic_year_names_v1_marker.rs.data");
include!("dangi_month_names_v1_marker.rs.data");
include!("dangi_year_names_v1_marker.rs.data");
include!("day_period_names_v1_marker.rs.data");
include!("ethiopian_month_names_v1_marker.rs.data");
include!("ethiopian_year_names_v1_marker.rs.data");
include!("gregorian_month_names_v1_marker.rs.data");
include!("gregorian_year_names_v1_marker.rs.data");
include!("hebrew_month_names_v1_marker.rs.data");
include!("hebrew_year_names_v1_marker.rs.data");
include!("indian_month_names_v1_marker.rs.data");
include!("indian_year_names_v1_marker.rs.data");
include!("islamic_month_names_v1_marker.rs.data");
include!("islamic_year_names_v1_marker.rs.data");
include!("japanese_month_names_v1_marker.rs.data");
include!("japanese_year_names_v1_marker.rs.data");
include!("japanese_extended_month_names_v1_marker.rs.data");
include!("japanese_extended_year_names_v1_marker.rs.data");
include!("persian_month_names_v1_marker.rs.data");
include!("persian_year_names_v1_marker.rs.data");
include!("roc_month_names_v1_marker.rs.data");
include!("roc_year_names_v1_marker.rs.data");
include!("weekday_names_v1_marker.rs.data");
include!("time_lengths_v1_marker.rs.data");
include!("time_symbols_v1_marker.rs.data");
include!("exemplar_cities_v1_marker.rs.data");
include!("time_zone_formats_v1_marker.rs.data");
include!("metazone_generic_names_long_v1_marker.rs.data");
include!("metazone_generic_names_short_v1_marker.rs.data");
include!("metazone_specific_names_long_v1_marker.rs.data");
include!("metazone_specific_names_short_v1_marker.rs.data");
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
        icu_provider::marker::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_buddhist_date_lengths_v1_marker!($provider);
        impl_buddhist_date_symbols_v1_marker!($provider);
        impl_chinese_date_lengths_v1_marker!($provider);
        impl_chinese_date_symbols_v1_marker!($provider);
        impl_coptic_date_lengths_v1_marker!($provider);
        impl_coptic_date_symbols_v1_marker!($provider);
        impl_dangi_date_lengths_v1_marker!($provider);
        impl_dangi_date_symbols_v1_marker!($provider);
        impl_ethiopian_date_lengths_v1_marker!($provider);
        impl_ethiopian_date_symbols_v1_marker!($provider);
        impl_gregorian_date_lengths_v1_marker!($provider);
        impl_gregorian_date_symbols_v1_marker!($provider);
        impl_hebrew_date_lengths_v1_marker!($provider);
        impl_hebrew_date_symbols_v1_marker!($provider);
        impl_indian_date_lengths_v1_marker!($provider);
        impl_indian_date_symbols_v1_marker!($provider);
        impl_islamic_date_lengths_v1_marker!($provider);
        impl_islamic_date_symbols_v1_marker!($provider);
        impl_japanese_date_lengths_v1_marker!($provider);
        impl_japanese_date_symbols_v1_marker!($provider);
        impl_japanese_extended_date_lengths_v1_marker!($provider);
        impl_japanese_extended_date_symbols_v1_marker!($provider);
        impl_buddhist_date_pattern_v1_marker!($provider);
        impl_buddhist_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_chinese_date_pattern_v1_marker!($provider);
        impl_chinese_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_coptic_date_pattern_v1_marker!($provider);
        impl_coptic_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_dangi_date_pattern_v1_marker!($provider);
        impl_dangi_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_date_time_pattern_v1_marker!($provider);
        impl_ethiopian_date_pattern_v1_marker!($provider);
        impl_ethiopian_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_gregorian_date_pattern_v1_marker!($provider);
        impl_gregorian_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_hebrew_date_pattern_v1_marker!($provider);
        impl_hebrew_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_indian_date_pattern_v1_marker!($provider);
        impl_indian_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_islamic_date_pattern_v1_marker!($provider);
        impl_islamic_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_japanese_date_pattern_v1_marker!($provider);
        impl_japanese_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_japanese_extended_date_pattern_v1_marker!($provider);
        impl_japanese_extended_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_persian_date_pattern_v1_marker!($provider);
        impl_persian_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_roc_date_pattern_v1_marker!($provider);
        impl_roc_date_neo_skeleton_patterns_v1_marker!($provider);
        impl_time_pattern_v1_marker!($provider);
        impl_time_neo_skeleton_patterns_v1_marker!($provider);
        impl_persian_date_lengths_v1_marker!($provider);
        impl_persian_date_symbols_v1_marker!($provider);
        impl_roc_date_lengths_v1_marker!($provider);
        impl_roc_date_symbols_v1_marker!($provider);
        impl_buddhist_month_names_v1_marker!($provider);
        impl_buddhist_year_names_v1_marker!($provider);
        impl_chinese_month_names_v1_marker!($provider);
        impl_chinese_year_names_v1_marker!($provider);
        impl_coptic_month_names_v1_marker!($provider);
        impl_coptic_year_names_v1_marker!($provider);
        impl_dangi_month_names_v1_marker!($provider);
        impl_dangi_year_names_v1_marker!($provider);
        impl_day_period_names_v1_marker!($provider);
        impl_ethiopian_month_names_v1_marker!($provider);
        impl_ethiopian_year_names_v1_marker!($provider);
        impl_gregorian_month_names_v1_marker!($provider);
        impl_gregorian_year_names_v1_marker!($provider);
        impl_hebrew_month_names_v1_marker!($provider);
        impl_hebrew_year_names_v1_marker!($provider);
        impl_indian_month_names_v1_marker!($provider);
        impl_indian_year_names_v1_marker!($provider);
        impl_islamic_month_names_v1_marker!($provider);
        impl_islamic_year_names_v1_marker!($provider);
        impl_japanese_month_names_v1_marker!($provider);
        impl_japanese_year_names_v1_marker!($provider);
        impl_japanese_extended_month_names_v1_marker!($provider);
        impl_japanese_extended_year_names_v1_marker!($provider);
        impl_persian_month_names_v1_marker!($provider);
        impl_persian_year_names_v1_marker!($provider);
        impl_roc_month_names_v1_marker!($provider);
        impl_roc_year_names_v1_marker!($provider);
        impl_weekday_names_v1_marker!($provider);
        impl_time_lengths_v1_marker!($provider);
        impl_time_symbols_v1_marker!($provider);
        impl_exemplar_cities_v1_marker!($provider);
        impl_time_zone_formats_v1_marker!($provider);
        impl_metazone_generic_names_long_v1_marker!($provider);
        impl_metazone_generic_names_short_v1_marker!($provider);
        impl_metazone_specific_names_long_v1_marker!($provider);
        impl_metazone_specific_names_short_v1_marker!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.70"]
        impl icu_provider::any::AnyProvider for $provider {
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
                    _ => Err(icu_provider::DataErrorKind::MarkerNotFound.with_req(marker, req)),
                }
            }
        }
    };
}
