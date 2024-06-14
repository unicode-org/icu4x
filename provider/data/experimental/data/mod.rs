// @generated
include!("compactdecimal_long_v1.rs.data");
include!("compactdecimal_short_v1.rs.data");
include!("currency_essentials_v1.rs.data");
include!("displaynames_languages_v1.rs.data");
include!("displaynames_locales_v1.rs.data");
include!("displaynames_regions_v1.rs.data");
include!("displaynames_scripts_v1.rs.data");
include!("displaynames_variants_v1.rs.data");
include!("percent_essentials_v1.rs.data");
include!("personnames_personnames_v1.rs.data");
include!("relativetime_long_day_v1.rs.data");
include!("relativetime_long_hour_v1.rs.data");
include!("relativetime_long_minute_v1.rs.data");
include!("relativetime_long_month_v1.rs.data");
include!("relativetime_long_quarter_v1.rs.data");
include!("relativetime_long_second_v1.rs.data");
include!("relativetime_long_week_v1.rs.data");
include!("relativetime_long_year_v1.rs.data");
include!("relativetime_narrow_day_v1.rs.data");
include!("relativetime_narrow_hour_v1.rs.data");
include!("relativetime_narrow_minute_v1.rs.data");
include!("relativetime_narrow_month_v1.rs.data");
include!("relativetime_narrow_quarter_v1.rs.data");
include!("relativetime_narrow_second_v1.rs.data");
include!("relativetime_narrow_week_v1.rs.data");
include!("relativetime_narrow_year_v1.rs.data");
include!("relativetime_short_day_v1.rs.data");
include!("relativetime_short_hour_v1.rs.data");
include!("relativetime_short_minute_v1.rs.data");
include!("relativetime_short_month_v1.rs.data");
include!("relativetime_short_quarter_v1.rs.data");
include!("relativetime_short_second_v1.rs.data");
include!("relativetime_short_week_v1.rs.data");
include!("relativetime_short_year_v1.rs.data");
include!("units_info_v1.rs.data");
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
        impl_compactdecimal_long_v1!($provider);
        impl_compactdecimal_short_v1!($provider);
        impl_currency_essentials_v1!($provider);
        impl_displaynames_languages_v1!($provider);
        impl_displaynames_locales_v1!($provider);
        impl_displaynames_regions_v1!($provider);
        impl_displaynames_scripts_v1!($provider);
        impl_displaynames_variants_v1!($provider);
        impl_percent_essentials_v1!($provider);
        impl_personnames_personnames_v1!($provider);
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
        impl_units_info_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.70"]
        impl icu_provider::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.path.hashed() {
                    h if h == <icu::experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::currency::CurrencyEssentialsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::currency::CurrencyEssentialsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::LanguageDisplayNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::LanguageDisplayNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::LocaleDisplayNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::LocaleDisplayNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::RegionDisplayNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::RegionDisplayNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::ScriptDisplayNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::ScriptDisplayNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::VariantDisplayNamesV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::VariantDisplayNamesV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::percent::PercentEssentialsV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::percent::PercentEssentialsV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::personnames::provider::PersonNamesFormatV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::personnames::provider::PersonNamesFormatV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongDayRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongHourRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongMinuteRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongMonthRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongQuarterRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongSecondRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongWeekRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongYearRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowDayRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowHourRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowMinuteRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowMonthRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowQuarterRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowSecondRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowWeekRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowYearRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortDayRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortHourRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortMinuteRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortMonthRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortQuarterRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortSecondRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortWeekRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortYearRelativeTimeFormatDataV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::units::provider::UnitsInfoV1Marker as icu_provider::DataMarker>::INFO.path.hashed() => icu_provider::DataProvider::<icu::experimental::units::provider::UnitsInfoV1Marker>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MissingDataMarker.with_req(marker, req)),
                }
            }
        }
    };
}
