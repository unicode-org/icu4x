// @generated
include!("short_day_relative_v1.rs.data");
include!("long_second_relative_v1.rs.data");
include!("person_names_format_v1.rs.data");
include!("percent_essentials_v1.rs.data");
include!("short_quarter_relative_v1.rs.data");
include!("short_week_relative_v1.rs.data");
include!("locale_display_names_v1.rs.data");
include!("currency_displayname_v1.rs.data");
include!("long_minute_relative_v1.rs.data");
include!("currency_patterns_data_v1.rs.data");
include!("long_quarter_relative_v1.rs.data");
include!("short_month_relative_v1.rs.data");
include!("currency_extended_data_v1.rs.data");
include!("short_compact_decimal_format_data_v1.rs.data");
include!("narrow_hour_relative_v1.rs.data");
include!("long_compact_decimal_format_data_v1.rs.data");
include!("narrow_week_relative_v1.rs.data");
include!("narrow_second_relative_v1.rs.data");
include!("long_week_relative_v1.rs.data");
include!("units_trie_v1.rs.data");
include!("short_minute_relative_v1.rs.data");
include!("units_essentials_v1.rs.data");
include!("language_display_names_v1.rs.data");
include!("short_hour_relative_v1.rs.data");
include!("short_currency_compact_v1.rs.data");
include!("long_month_relative_v1.rs.data");
include!("long_year_relative_v1.rs.data");
include!("region_display_names_v1.rs.data");
include!("currency_essentials_v1.rs.data");
include!("short_year_relative_v1.rs.data");
include!("long_day_relative_v1.rs.data");
include!("long_hour_relative_v1.rs.data");
include!("short_second_relative_v1.rs.data");
include!("variant_display_names_v1.rs.data");
include!("narrow_month_relative_v1.rs.data");
include!("narrow_day_relative_v1.rs.data");
include!("units_display_name_v1.rs.data");
include!("transliterator_rules_v1.rs.data");
include!("narrow_quarter_relative_v1.rs.data");
include!("script_display_names_v1.rs.data");
include!("narrow_minute_relative_v1.rs.data");
include!("digital_duration_data_v1.rs.data");
include!("units_info_v1.rs.data");
include!("narrow_year_relative_v1.rs.data");
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
        #[clippy::msrv = "1.82"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::marker::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
/// This macro requires the following crates:
/// * `icu`
/// * `icu_provider`
/// * `zerotrie`
/// * `zerovec`
#[allow(unused_macros)]
macro_rules! impl_data_provider {
    ($ provider : ty) => {
        make_provider!($provider);
        impl_short_day_relative_v1!($provider);
        impl_long_second_relative_v1!($provider);
        impl_person_names_format_v1!($provider);
        impl_percent_essentials_v1!($provider);
        impl_short_quarter_relative_v1!($provider);
        impl_short_week_relative_v1!($provider);
        impl_locale_display_names_v1!($provider);
        impl_currency_displayname_v1!($provider);
        impl_long_minute_relative_v1!($provider);
        impl_currency_patterns_data_v1!($provider);
        impl_long_quarter_relative_v1!($provider);
        impl_short_month_relative_v1!($provider);
        impl_currency_extended_data_v1!($provider);
        impl_short_compact_decimal_format_data_v1!($provider);
        impl_narrow_hour_relative_v1!($provider);
        impl_long_compact_decimal_format_data_v1!($provider);
        impl_narrow_week_relative_v1!($provider);
        impl_narrow_second_relative_v1!($provider);
        impl_long_week_relative_v1!($provider);
        impl_units_trie_v1!($provider);
        impl_short_minute_relative_v1!($provider);
        impl_units_essentials_v1!($provider);
        impl_language_display_names_v1!($provider);
        impl_short_hour_relative_v1!($provider);
        impl_short_currency_compact_v1!($provider);
        impl_long_month_relative_v1!($provider);
        impl_long_year_relative_v1!($provider);
        impl_region_display_names_v1!($provider);
        impl_currency_essentials_v1!($provider);
        impl_short_year_relative_v1!($provider);
        impl_long_day_relative_v1!($provider);
        impl_long_hour_relative_v1!($provider);
        impl_short_second_relative_v1!($provider);
        impl_variant_display_names_v1!($provider);
        impl_narrow_month_relative_v1!($provider);
        impl_narrow_day_relative_v1!($provider);
        impl_units_display_name_v1!($provider);
        impl_transliterator_rules_v1!($provider);
        impl_narrow_quarter_relative_v1!($provider);
        impl_script_display_names_v1!($provider);
        impl_narrow_minute_relative_v1!($provider);
        impl_digital_duration_data_v1!($provider);
        impl_units_info_v1!($provider);
        impl_narrow_year_relative_v1!($provider);
    };
}
