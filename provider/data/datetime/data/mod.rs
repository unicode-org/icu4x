// @generated
include!("hebrew_date_neo_skeleton_patterns_v1.rs.data");
include!("japanese_year_names_v1.rs.data");
include!("gregorian_date_neo_skeleton_patterns_v1.rs.data");
include!("japanese_month_names_v1.rs.data");
include!("coptic_year_names_v1.rs.data");
include!("dangi_month_names_v1.rs.data");
include!("metazone_specific_names_short_v1.rs.data");
include!("buddhist_date_neo_skeleton_patterns_v1.rs.data");
include!("hebrew_year_names_v1.rs.data");
include!("indian_year_names_v1.rs.data");
include!("time_neo_skeleton_patterns_v1.rs.data");
include!("dangi_year_names_v1.rs.data");
include!("dangi_date_neo_skeleton_patterns_v1.rs.data");
include!("islamic_month_names_v1.rs.data");
include!("exemplar_cities_root_v1.rs.data");
include!("metazone_generic_names_long_v1.rs.data");
include!("buddhist_year_names_v1.rs.data");
include!("chinese_month_names_v1.rs.data");
include!("ethiopian_date_neo_skeleton_patterns_v1.rs.data");
include!("day_period_names_v1.rs.data");
include!("chinese_date_neo_skeleton_patterns_v1.rs.data");
include!("roc_date_neo_skeleton_patterns_v1.rs.data");
include!("islamic_date_neo_skeleton_patterns_v1.rs.data");
include!("japanese_extended_year_names_v1.rs.data");
include!("ethiopian_month_names_v1.rs.data");
include!("metazone_period_v1.rs.data");
include!("coptic_date_neo_skeleton_patterns_v1.rs.data");
include!("chinese_year_names_v1.rs.data");
include!("persian_month_names_v1.rs.data");
include!("exemplar_cities_v1.rs.data");
include!("roc_year_names_v1.rs.data");
include!("persian_date_neo_skeleton_patterns_v1.rs.data");
include!("locations_v1.rs.data");
include!("buddhist_month_names_v1.rs.data");
include!("ethiopian_year_names_v1.rs.data");
include!("japanese_date_neo_skeleton_patterns_v1.rs.data");
include!("gregorian_month_names_v1.rs.data");
include!("time_zone_essentials_v1.rs.data");
include!("roc_month_names_v1.rs.data");
include!("japanese_extended_month_names_v1.rs.data");
include!("islamic_year_names_v1.rs.data");
include!("glue_pattern_v1.rs.data");
include!("indian_month_names_v1.rs.data");
include!("indian_date_neo_skeleton_patterns_v1.rs.data");
include!("locations_root_v1.rs.data");
include!("persian_year_names_v1.rs.data");
include!("japanese_extended_date_neo_skeleton_patterns_v1.rs.data");
include!("metazone_specific_names_long_v1.rs.data");
include!("hebrew_month_names_v1.rs.data");
include!("weekday_names_v1.rs.data");
include!("gregorian_year_names_v1.rs.data");
include!("coptic_month_names_v1.rs.data");
include!("metazone_generic_names_short_v1.rs.data");
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
        #[clippy::msrv = "1.81"]
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
        impl_hebrew_date_neo_skeleton_patterns_v1!($provider);
        impl_japanese_year_names_v1!($provider);
        impl_gregorian_date_neo_skeleton_patterns_v1!($provider);
        impl_japanese_month_names_v1!($provider);
        impl_coptic_year_names_v1!($provider);
        impl_dangi_month_names_v1!($provider);
        impl_metazone_specific_names_short_v1!($provider);
        impl_buddhist_date_neo_skeleton_patterns_v1!($provider);
        impl_hebrew_year_names_v1!($provider);
        impl_indian_year_names_v1!($provider);
        impl_time_neo_skeleton_patterns_v1!($provider);
        impl_dangi_year_names_v1!($provider);
        impl_dangi_date_neo_skeleton_patterns_v1!($provider);
        impl_islamic_month_names_v1!($provider);
        impl_exemplar_cities_root_v1!($provider);
        impl_metazone_generic_names_long_v1!($provider);
        impl_buddhist_year_names_v1!($provider);
        impl_chinese_month_names_v1!($provider);
        impl_ethiopian_date_neo_skeleton_patterns_v1!($provider);
        impl_day_period_names_v1!($provider);
        impl_chinese_date_neo_skeleton_patterns_v1!($provider);
        impl_roc_date_neo_skeleton_patterns_v1!($provider);
        impl_islamic_date_neo_skeleton_patterns_v1!($provider);
        impl_japanese_extended_year_names_v1!($provider);
        impl_ethiopian_month_names_v1!($provider);
        impl_metazone_period_v1!($provider);
        impl_coptic_date_neo_skeleton_patterns_v1!($provider);
        impl_chinese_year_names_v1!($provider);
        impl_persian_month_names_v1!($provider);
        impl_exemplar_cities_v1!($provider);
        impl_roc_year_names_v1!($provider);
        impl_persian_date_neo_skeleton_patterns_v1!($provider);
        impl_locations_v1!($provider);
        impl_buddhist_month_names_v1!($provider);
        impl_ethiopian_year_names_v1!($provider);
        impl_japanese_date_neo_skeleton_patterns_v1!($provider);
        impl_gregorian_month_names_v1!($provider);
        impl_time_zone_essentials_v1!($provider);
        impl_roc_month_names_v1!($provider);
        impl_japanese_extended_month_names_v1!($provider);
        impl_islamic_year_names_v1!($provider);
        impl_glue_pattern_v1!($provider);
        impl_indian_month_names_v1!($provider);
        impl_indian_date_neo_skeleton_patterns_v1!($provider);
        impl_locations_root_v1!($provider);
        impl_persian_year_names_v1!($provider);
        impl_japanese_extended_date_neo_skeleton_patterns_v1!($provider);
        impl_metazone_specific_names_long_v1!($provider);
        impl_hebrew_month_names_v1!($provider);
        impl_weekday_names_v1!($provider);
        impl_gregorian_year_names_v1!($provider);
        impl_coptic_month_names_v1!($provider);
        impl_metazone_generic_names_short_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.81"]
        impl icu_provider::any::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.id.hashed() {
                    h if h == <icu::datetime::provider::HebrewDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::HebrewDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::GregorianDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::GregorianDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesShortV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::BuddhistDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::BuddhistDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::TimeNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::TimeNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DangiYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DangiYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::DangiDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::DangiDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::ExemplarCitiesRootV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::ExemplarCitiesRootV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesLongV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::EthiopianDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::EthiopianDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::DayPeriodNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::DayPeriodNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::ChineseDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::ChineseDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::RocDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::RocDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::IslamicDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::IslamicDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazonePeriodV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazonePeriodV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::CopticDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::CopticDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::ChineseYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::ChineseYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::ExemplarCitiesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::ExemplarCitiesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::PersianDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::PersianDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::LocationsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::LocationsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::BuddhistMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::BuddhistMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::EthiopianYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::EthiopianYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::JapaneseDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::JapaneseDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::TimeZoneEssentialsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::TimeZoneEssentialsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::RocMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::RocMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::JapaneseExtendedMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::JapaneseExtendedMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IslamicYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IslamicYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GluePatternV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GluePatternV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::IndianMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::IndianMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::IndianDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::IndianDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::LocationsRootV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::LocationsRootV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::PersianYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::PersianYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::JapaneseExtendedDateNeoSkeletonPatternsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::JapaneseExtendedDateNeoSkeletonPatternsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneSpecificNamesLongV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::HebrewMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::HebrewMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::WeekdayNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::WeekdayNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::GregorianYearNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::GregorianYearNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::neo::CopticMonthNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::neo::CopticMonthNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::datetime::provider::time_zones::MetazoneGenericNamesShortV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MarkerNotFound.with_req(marker, req)),
                }
            }
        }
    };
}
