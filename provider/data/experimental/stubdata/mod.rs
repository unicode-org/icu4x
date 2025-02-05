// @generated
include!("short_currency_compact_v1.rs.data");
include!("long_week_relative_v1.rs.data");
include!("narrow_minute_relative_v1.rs.data");
include!("transliterator_rules_v1.rs.data");
include!("short_compact_decimal_format_data_v1.rs.data");
include!("digital_duration_data_v1.rs.data");
include!("narrow_week_relative_v1.rs.data");
include!("short_second_relative_v1.rs.data");
include!("long_month_relative_v1.rs.data");
include!("short_year_relative_v1.rs.data");
include!("currency_essentials_v1.rs.data");
include!("short_week_relative_v1.rs.data");
include!("narrow_second_relative_v1.rs.data");
include!("short_quarter_relative_v1.rs.data");
include!("narrow_hour_relative_v1.rs.data");
include!("short_minute_relative_v1.rs.data");
include!("narrow_quarter_relative_v1.rs.data");
include!("long_minute_relative_v1.rs.data");
include!("region_display_names_v1.rs.data");
include!("units_trie_v1.rs.data");
include!("language_display_names_v1.rs.data");
include!("long_quarter_relative_v1.rs.data");
include!("short_day_relative_v1.rs.data");
include!("narrow_day_relative_v1.rs.data");
include!("units_essentials_v1.rs.data");
include!("percent_essentials_v1.rs.data");
include!("variant_display_names_v1.rs.data");
include!("short_month_relative_v1.rs.data");
include!("person_names_format_v1.rs.data");
include!("long_day_relative_v1.rs.data");
include!("units_info_v1.rs.data");
include!("short_hour_relative_v1.rs.data");
include!("long_year_relative_v1.rs.data");
include!("long_second_relative_v1.rs.data");
include!("long_hour_relative_v1.rs.data");
include!("long_compact_decimal_format_data_v1.rs.data");
include!("locale_display_names_v1.rs.data");
include!("script_display_names_v1.rs.data");
include!("narrow_year_relative_v1.rs.data");
include!("currency_extended_data_v1.rs.data");
include!("units_display_name_v1.rs.data");
include!("narrow_month_relative_v1.rs.data");
include!("currency_displayname_v1.rs.data");
include!("currency_patterns_data_v1.rs.data");
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
        impl_short_currency_compact_v1!($provider);
        impl_long_week_relative_v1!($provider);
        impl_narrow_minute_relative_v1!($provider);
        impl_transliterator_rules_v1!($provider);
        impl_short_compact_decimal_format_data_v1!($provider);
        impl_digital_duration_data_v1!($provider);
        impl_narrow_week_relative_v1!($provider);
        impl_short_second_relative_v1!($provider);
        impl_long_month_relative_v1!($provider);
        impl_short_year_relative_v1!($provider);
        impl_currency_essentials_v1!($provider);
        impl_short_week_relative_v1!($provider);
        impl_narrow_second_relative_v1!($provider);
        impl_short_quarter_relative_v1!($provider);
        impl_narrow_hour_relative_v1!($provider);
        impl_short_minute_relative_v1!($provider);
        impl_narrow_quarter_relative_v1!($provider);
        impl_long_minute_relative_v1!($provider);
        impl_region_display_names_v1!($provider);
        impl_units_trie_v1!($provider);
        impl_language_display_names_v1!($provider);
        impl_long_quarter_relative_v1!($provider);
        impl_short_day_relative_v1!($provider);
        impl_narrow_day_relative_v1!($provider);
        impl_units_essentials_v1!($provider);
        impl_percent_essentials_v1!($provider);
        impl_variant_display_names_v1!($provider);
        impl_short_month_relative_v1!($provider);
        impl_person_names_format_v1!($provider);
        impl_long_day_relative_v1!($provider);
        impl_units_info_v1!($provider);
        impl_short_hour_relative_v1!($provider);
        impl_long_year_relative_v1!($provider);
        impl_long_second_relative_v1!($provider);
        impl_long_hour_relative_v1!($provider);
        impl_long_compact_decimal_format_data_v1!($provider);
        impl_locale_display_names_v1!($provider);
        impl_script_display_names_v1!($provider);
        impl_narrow_year_relative_v1!($provider);
        impl_currency_extended_data_v1!($provider);
        impl_units_display_name_v1!($provider);
        impl_narrow_month_relative_v1!($provider);
        impl_currency_displayname_v1!($provider);
        impl_currency_patterns_data_v1!($provider);
    };
}
#[allow(unused_macros)]
macro_rules! impl_any_provider {
    ($ provider : ty) => {
        #[clippy::msrv = "1.81"]
        impl icu_provider::any::AnyProvider for $provider {
            fn load_any(&self, marker: icu_provider::DataMarkerInfo, req: icu_provider::DataRequest) -> Result<icu_provider::AnyResponse, icu_provider::DataError> {
                match marker.id.hashed() {
                    h if h == <icu::experimental::dimension::provider::currency_compact::ShortCurrencyCompactV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::currency_compact::ShortCurrencyCompactV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongWeekRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongWeekRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowMinuteRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowMinuteRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::transliterate::provider::TransliteratorRulesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::transliterate::provider::TransliteratorRulesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::compactdecimal::provider::ShortCompactDecimalFormatDataV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::duration::provider::DigitalDurationDataV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::duration::provider::DigitalDurationDataV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowWeekRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowWeekRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortSecondRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortSecondRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongMonthRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongMonthRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortYearRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortYearRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::currency::CurrencyEssentialsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::currency::CurrencyEssentialsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortWeekRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortWeekRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowSecondRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowSecondRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortQuarterRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortQuarterRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowHourRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowHourRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortMinuteRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortMinuteRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowQuarterRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowQuarterRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongMinuteRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongMinuteRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::RegionDisplayNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::RegionDisplayNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::measure::provider::trie::UnitsTrieV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::measure::provider::trie::UnitsTrieV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::LanguageDisplayNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::LanguageDisplayNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongQuarterRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongQuarterRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortDayRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortDayRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowDayRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowDayRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::units_essentials::UnitsEssentialsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::percent::PercentEssentialsV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::percent::PercentEssentialsV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::VariantDisplayNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::VariantDisplayNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortMonthRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortMonthRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::personnames::provider::PersonNamesFormatV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::personnames::provider::PersonNamesFormatV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongDayRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongDayRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::units::provider::UnitsInfoV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::units::provider::UnitsInfoV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::ShortHourRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::ShortHourRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongYearRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongYearRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongSecondRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongSecondRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::LongHourRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::LongHourRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::compactdecimal::provider::LongCompactDecimalFormatDataV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::LocaleDisplayNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::LocaleDisplayNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::displaynames::provider::ScriptDisplayNamesV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::displaynames::provider::ScriptDisplayNamesV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowYearRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowYearRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::extended_currency::CurrencyExtendedDataV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::extended_currency::CurrencyExtendedDataV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::units::UnitsDisplayNameV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::units::UnitsDisplayNameV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::relativetime::provider::NarrowMonthRelativeV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::relativetime::provider::NarrowMonthRelativeV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::currency_displayname::CurrencyDisplaynameV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::currency_displayname::CurrencyDisplaynameV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    h if h == <icu::experimental::dimension::provider::currency_patterns::CurrencyPatternsDataV1 as icu_provider::DataMarker>::INFO.id.hashed() => icu_provider::DataProvider::<icu::experimental::dimension::provider::currency_patterns::CurrencyPatternsDataV1>::load(self, req).map(icu_provider::DataResponse::wrap_into_any_response),
                    _ => Err(icu_provider::DataErrorKind::MarkerNotFound.with_req(marker, req)),
                }
            }
        }
    };
}
