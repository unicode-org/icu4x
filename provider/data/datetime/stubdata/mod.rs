// @generated
include!("datetime_names_year_ethiopian_v1.rs.data");
include!("datetime_patterns_date_ethiopian_v1.rs.data");
include!("datetime_patterns_date_roc_v1.rs.data");
include!("datetime_patterns_date_gregorian_v1.rs.data");
include!("datetime_patterns_time_v1.rs.data");
include!("datetime_names_month_gregorian_v1.rs.data");
include!("datetime_patterns_date_indian_v1.rs.data");
include!("datetime_patterns_date_chinese_v1.rs.data");
include!("datetime_names_year_persian_v1.rs.data");
include!("datetime_names_year_hebrew_v1.rs.data");
include!("datetime_names_dayperiod_v1.rs.data");
include!("datetime_patterns_date_persian_v1.rs.data");
include!("datetime_names_month_buddhist_v1.rs.data");
include!("datetime_names_weekday_v1.rs.data");
include!("datetime_names_year_roc_v1.rs.data");
include!("datetime_names_month_dangi_v1.rs.data");
include!("datetime_names_month_persian_v1.rs.data");
include!("datetime_patterns_date_hijri_v1.rs.data");
include!("datetime_names_year_chinese_v1.rs.data");
include!("datetime_names_month_hebrew_v1.rs.data");
include!("datetime_patterns_date_hebrew_v1.rs.data");
include!("datetime_names_month_chinese_v1.rs.data");
include!("timezone_names_locations_root_v1.rs.data");
include!("timezone_names_specific_short_v1.rs.data");
include!("datetime_patterns_glue_v1.rs.data");
include!("datetime_names_month_japanext_v1.rs.data");
include!("timezone_names_cities_root_v1.rs.data");
include!("datetime_names_year_indian_v1.rs.data");
include!("timezone_names_generic_short_v1.rs.data");
include!("datetime_names_year_buddhist_v1.rs.data");
include!("timezone_names_specific_long_v1.rs.data");
include!("datetime_names_year_dangi_v1.rs.data");
include!("datetime_names_month_hijri_v1.rs.data");
include!("timezone_names_essentials_v1.rs.data");
include!("datetime_names_year_hijri_v1.rs.data");
include!("timezone_names_cities_override_v1.rs.data");
include!("datetime_patterns_date_japanese_v1.rs.data");
include!("datetime_patterns_date_japanext_v1.rs.data");
include!("datetime_names_year_coptic_v1.rs.data");
include!("timezone_names_generic_long_v1.rs.data");
include!("datetime_names_month_indian_v1.rs.data");
include!("datetime_names_month_ethiopian_v1.rs.data");
include!("datetime_patterns_date_coptic_v1.rs.data");
include!("datetime_patterns_date_buddhist_v1.rs.data");
include!("timezone_names_standard_long_v1.rs.data");
include!("datetime_names_month_japanese_v1.rs.data");
include!("datetime_names_year_gregorian_v1.rs.data");
include!("datetime_names_month_coptic_v1.rs.data");
include!("timezone_names_locations_override_v1.rs.data");
include!("datetime_names_year_japanext_v1.rs.data");
include!("datetime_patterns_date_dangi_v1.rs.data");
include!("datetime_names_month_roc_v1.rs.data");
include!("datetime_names_year_japanese_v1.rs.data");


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
    ($name:ty) => {
        #[clippy::msrv = "1.83"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::marker::impl_data_provider_never_marker!($name);
    };
}

#[doc(inline)]
pub use __make_provider as make_provider;

// Not public as it will only work locally due to needing access to the other macros.
/// This macro requires the following crates:
/// * `icu`
/// * `icu_provider`
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($provider:ty) => {
        make_provider!($provider);
        impl_datetime_names_year_ethiopian_v1!($provider);
        impl_datetime_patterns_date_ethiopian_v1!($provider);
        impl_datetime_patterns_date_roc_v1!($provider);
        impl_datetime_patterns_date_gregorian_v1!($provider);
        impl_datetime_patterns_time_v1!($provider);
        impl_datetime_names_month_gregorian_v1!($provider);
        impl_datetime_patterns_date_indian_v1!($provider);
        impl_datetime_patterns_date_chinese_v1!($provider);
        impl_datetime_names_year_persian_v1!($provider);
        impl_datetime_names_year_hebrew_v1!($provider);
        impl_datetime_names_dayperiod_v1!($provider);
        impl_datetime_patterns_date_persian_v1!($provider);
        impl_datetime_names_month_buddhist_v1!($provider);
        impl_datetime_names_weekday_v1!($provider);
        impl_datetime_names_year_roc_v1!($provider);
        impl_datetime_names_month_dangi_v1!($provider);
        impl_datetime_names_month_persian_v1!($provider);
        impl_datetime_patterns_date_hijri_v1!($provider);
        impl_datetime_names_year_chinese_v1!($provider);
        impl_datetime_names_month_hebrew_v1!($provider);
        impl_datetime_patterns_date_hebrew_v1!($provider);
        impl_datetime_names_month_chinese_v1!($provider);
        impl_timezone_names_locations_root_v1!($provider);
        impl_timezone_names_specific_short_v1!($provider);
        impl_datetime_patterns_glue_v1!($provider);
        impl_datetime_names_month_japanext_v1!($provider);
        impl_timezone_names_cities_root_v1!($provider);
        impl_datetime_names_year_indian_v1!($provider);
        impl_timezone_names_generic_short_v1!($provider);
        impl_datetime_names_year_buddhist_v1!($provider);
        impl_timezone_names_specific_long_v1!($provider);
        impl_datetime_names_year_dangi_v1!($provider);
        impl_datetime_names_month_hijri_v1!($provider);
        impl_timezone_names_essentials_v1!($provider);
        impl_datetime_names_year_hijri_v1!($provider);
        impl_timezone_names_cities_override_v1!($provider);
        impl_datetime_patterns_date_japanese_v1!($provider);
        impl_datetime_patterns_date_japanext_v1!($provider);
        impl_datetime_names_year_coptic_v1!($provider);
        impl_timezone_names_generic_long_v1!($provider);
        impl_datetime_names_month_indian_v1!($provider);
        impl_datetime_names_month_ethiopian_v1!($provider);
        impl_datetime_patterns_date_coptic_v1!($provider);
        impl_datetime_patterns_date_buddhist_v1!($provider);
        impl_timezone_names_standard_long_v1!($provider);
        impl_datetime_names_month_japanese_v1!($provider);
        impl_datetime_names_year_gregorian_v1!($provider);
        impl_datetime_names_month_coptic_v1!($provider);
        impl_timezone_names_locations_override_v1!($provider);
        impl_datetime_names_year_japanext_v1!($provider);
        impl_datetime_patterns_date_dangi_v1!($provider);
        impl_datetime_names_month_roc_v1!($provider);
        impl_datetime_names_year_japanese_v1!($provider);

    };
}
