// @generated
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
        #[clippy::msrv = "1.67"]
        impl $name {
            #[doc(hidden)]
            #[allow(dead_code)]
            pub const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[macro_use]
#[path = "macros/datetime_buddhist_datelengths_v1.data.rs"]
mod datetime_buddhist_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_buddhist_datelengths_v1 as impl_datetime_buddhist_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_buddhist_datesymbols_v1.data.rs"]
mod datetime_buddhist_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_buddhist_datesymbols_v1 as impl_datetime_buddhist_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_chinese_datelengths_v1.data.rs"]
mod datetime_chinese_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_chinese_datelengths_v1 as impl_datetime_chinese_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_chinese_datesymbols_v1.data.rs"]
mod datetime_chinese_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_chinese_datesymbols_v1 as impl_datetime_chinese_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_coptic_datelengths_v1.data.rs"]
mod datetime_coptic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_coptic_datelengths_v1 as impl_datetime_coptic_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_coptic_datesymbols_v1.data.rs"]
mod datetime_coptic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_coptic_datesymbols_v1 as impl_datetime_coptic_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_dangi_datelengths_v1.data.rs"]
mod datetime_dangi_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_dangi_datelengths_v1 as impl_datetime_dangi_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_dangi_datesymbols_v1.data.rs"]
mod datetime_dangi_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_dangi_datesymbols_v1 as impl_datetime_dangi_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_ethiopic_datelengths_v1.data.rs"]
mod datetime_ethiopic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_ethiopic_datelengths_v1 as impl_datetime_ethiopic_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_ethiopic_datesymbols_v1.data.rs"]
mod datetime_ethiopic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_ethiopic_datesymbols_v1 as impl_datetime_ethiopic_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_gregory_datelengths_v1.data.rs"]
mod datetime_gregory_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_gregory_datelengths_v1 as impl_datetime_gregory_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_gregory_datesymbols_v1.data.rs"]
mod datetime_gregory_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_gregory_datesymbols_v1 as impl_datetime_gregory_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_hebrew_datelengths_v1.data.rs"]
mod datetime_hebrew_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_hebrew_datelengths_v1 as impl_datetime_hebrew_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_hebrew_datesymbols_v1.data.rs"]
mod datetime_hebrew_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_hebrew_datesymbols_v1 as impl_datetime_hebrew_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_indian_datelengths_v1.data.rs"]
mod datetime_indian_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_indian_datelengths_v1 as impl_datetime_indian_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_indian_datesymbols_v1.data.rs"]
mod datetime_indian_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_indian_datesymbols_v1 as impl_datetime_indian_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_islamic_datelengths_v1.data.rs"]
mod datetime_islamic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_islamic_datelengths_v1 as impl_datetime_islamic_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_islamic_datesymbols_v1.data.rs"]
mod datetime_islamic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_islamic_datesymbols_v1 as impl_datetime_islamic_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_japanese_datelengths_v1.data.rs"]
mod datetime_japanese_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_japanese_datelengths_v1 as impl_datetime_japanese_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_japanese_datesymbols_v1.data.rs"]
mod datetime_japanese_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_japanese_datesymbols_v1 as impl_datetime_japanese_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_japanext_datelengths_v1.data.rs"]
mod datetime_japanext_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_japanext_datelengths_v1 as impl_datetime_japanext_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_japanext_datesymbols_v1.data.rs"]
mod datetime_japanext_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_japanext_datesymbols_v1 as impl_datetime_japanext_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_patterns_buddhist_date_v1.data.rs"]
mod datetime_patterns_buddhist_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_buddhist_date_v1 as impl_datetime_patterns_buddhist_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_chinese_date_v1.data.rs"]
mod datetime_patterns_chinese_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_chinese_date_v1 as impl_datetime_patterns_chinese_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_coptic_date_v1.data.rs"]
mod datetime_patterns_coptic_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_coptic_date_v1 as impl_datetime_patterns_coptic_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_dangi_date_v1.data.rs"]
mod datetime_patterns_dangi_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_dangi_date_v1 as impl_datetime_patterns_dangi_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_datetime_v1.data.rs"]
mod datetime_patterns_datetime_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_datetime_v1 as impl_datetime_patterns_datetime_v1;
#[macro_use]
#[path = "macros/datetime_patterns_ethiopic_date_v1.data.rs"]
mod datetime_patterns_ethiopic_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_ethiopic_date_v1 as impl_datetime_patterns_ethiopic_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_gregory_date_v1.data.rs"]
mod datetime_patterns_gregory_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_gregory_date_v1 as impl_datetime_patterns_gregory_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_hebrew_date_v1.data.rs"]
mod datetime_patterns_hebrew_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_hebrew_date_v1 as impl_datetime_patterns_hebrew_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_indian_date_v1.data.rs"]
mod datetime_patterns_indian_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_indian_date_v1 as impl_datetime_patterns_indian_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_islamic_date_v1.data.rs"]
mod datetime_patterns_islamic_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_islamic_date_v1 as impl_datetime_patterns_islamic_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_japanese_date_v1.data.rs"]
mod datetime_patterns_japanese_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_japanese_date_v1 as impl_datetime_patterns_japanese_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_japanext_date_v1.data.rs"]
mod datetime_patterns_japanext_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_japanext_date_v1 as impl_datetime_patterns_japanext_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_persian_date_v1.data.rs"]
mod datetime_patterns_persian_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_persian_date_v1 as impl_datetime_patterns_persian_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_roc_date_v1.data.rs"]
mod datetime_patterns_roc_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_roc_date_v1 as impl_datetime_patterns_roc_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_time_v1.data.rs"]
mod datetime_patterns_time_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_time_v1 as impl_datetime_patterns_time_v1;
#[macro_use]
#[path = "macros/datetime_persian_datelengths_v1.data.rs"]
mod datetime_persian_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_persian_datelengths_v1 as impl_datetime_persian_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_persian_datesymbols_v1.data.rs"]
mod datetime_persian_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_persian_datesymbols_v1 as impl_datetime_persian_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_roc_datelengths_v1.data.rs"]
mod datetime_roc_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_roc_datelengths_v1 as impl_datetime_roc_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_roc_datesymbols_v1.data.rs"]
mod datetime_roc_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_roc_datesymbols_v1 as impl_datetime_roc_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_skeletons_v1.data.rs"]
mod datetime_skeletons_v1;
#[doc(inline)]
pub use __impl_datetime_skeletons_v1 as impl_datetime_skeletons_v1;
#[macro_use]
#[path = "macros/datetime_symbols_buddhist_months_v1.data.rs"]
mod datetime_symbols_buddhist_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_buddhist_months_v1 as impl_datetime_symbols_buddhist_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_buddhist_years_v1.data.rs"]
mod datetime_symbols_buddhist_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_buddhist_years_v1 as impl_datetime_symbols_buddhist_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_chinese_months_v1.data.rs"]
mod datetime_symbols_chinese_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_chinese_months_v1 as impl_datetime_symbols_chinese_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_chinese_years_v1.data.rs"]
mod datetime_symbols_chinese_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_chinese_years_v1 as impl_datetime_symbols_chinese_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_coptic_months_v1.data.rs"]
mod datetime_symbols_coptic_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_coptic_months_v1 as impl_datetime_symbols_coptic_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_coptic_years_v1.data.rs"]
mod datetime_symbols_coptic_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_coptic_years_v1 as impl_datetime_symbols_coptic_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_dangi_months_v1.data.rs"]
mod datetime_symbols_dangi_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_dangi_months_v1 as impl_datetime_symbols_dangi_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_dangi_years_v1.data.rs"]
mod datetime_symbols_dangi_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_dangi_years_v1 as impl_datetime_symbols_dangi_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_dayperiods_v1.data.rs"]
mod datetime_symbols_dayperiods_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_dayperiods_v1 as impl_datetime_symbols_dayperiods_v1;
#[macro_use]
#[path = "macros/datetime_symbols_ethiopic_months_v1.data.rs"]
mod datetime_symbols_ethiopic_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_ethiopic_months_v1 as impl_datetime_symbols_ethiopic_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_ethiopic_years_v1.data.rs"]
mod datetime_symbols_ethiopic_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_ethiopic_years_v1 as impl_datetime_symbols_ethiopic_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_gregory_months_v1.data.rs"]
mod datetime_symbols_gregory_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_gregory_months_v1 as impl_datetime_symbols_gregory_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_gregory_years_v1.data.rs"]
mod datetime_symbols_gregory_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_gregory_years_v1 as impl_datetime_symbols_gregory_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_hebrew_months_v1.data.rs"]
mod datetime_symbols_hebrew_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_hebrew_months_v1 as impl_datetime_symbols_hebrew_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_hebrew_years_v1.data.rs"]
mod datetime_symbols_hebrew_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_hebrew_years_v1 as impl_datetime_symbols_hebrew_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_indian_months_v1.data.rs"]
mod datetime_symbols_indian_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_indian_months_v1 as impl_datetime_symbols_indian_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_indian_years_v1.data.rs"]
mod datetime_symbols_indian_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_indian_years_v1 as impl_datetime_symbols_indian_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_islamic_months_v1.data.rs"]
mod datetime_symbols_islamic_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_islamic_months_v1 as impl_datetime_symbols_islamic_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_islamic_years_v1.data.rs"]
mod datetime_symbols_islamic_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_islamic_years_v1 as impl_datetime_symbols_islamic_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanese_months_v1.data.rs"]
mod datetime_symbols_japanese_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanese_months_v1 as impl_datetime_symbols_japanese_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanese_years_v1.data.rs"]
mod datetime_symbols_japanese_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanese_years_v1 as impl_datetime_symbols_japanese_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanext_months_v1.data.rs"]
mod datetime_symbols_japanext_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanext_months_v1 as impl_datetime_symbols_japanext_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanext_years_v1.data.rs"]
mod datetime_symbols_japanext_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanext_years_v1 as impl_datetime_symbols_japanext_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_persian_months_v1.data.rs"]
mod datetime_symbols_persian_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_persian_months_v1 as impl_datetime_symbols_persian_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_persian_years_v1.data.rs"]
mod datetime_symbols_persian_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_persian_years_v1 as impl_datetime_symbols_persian_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_roc_months_v1.data.rs"]
mod datetime_symbols_roc_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_roc_months_v1 as impl_datetime_symbols_roc_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_roc_years_v1.data.rs"]
mod datetime_symbols_roc_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_roc_years_v1 as impl_datetime_symbols_roc_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_weekdays_v1.data.rs"]
mod datetime_symbols_weekdays_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_weekdays_v1 as impl_datetime_symbols_weekdays_v1;
#[macro_use]
#[path = "macros/datetime_timelengths_v1.data.rs"]
mod datetime_timelengths_v1;
#[doc(inline)]
pub use __impl_datetime_timelengths_v1 as impl_datetime_timelengths_v1;
#[macro_use]
#[path = "macros/datetime_timesymbols_v1.data.rs"]
mod datetime_timesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_timesymbols_v1 as impl_datetime_timesymbols_v1;
#[macro_use]
#[path = "macros/time_zone_exemplar_cities_v1.data.rs"]
mod time_zone_exemplar_cities_v1;
#[doc(inline)]
pub use __impl_time_zone_exemplar_cities_v1 as impl_time_zone_exemplar_cities_v1;
#[macro_use]
#[path = "macros/time_zone_formats_v1.data.rs"]
mod time_zone_formats_v1;
#[doc(inline)]
pub use __impl_time_zone_formats_v1 as impl_time_zone_formats_v1;
#[macro_use]
#[path = "macros/time_zone_generic_long_v1.data.rs"]
mod time_zone_generic_long_v1;
#[doc(inline)]
pub use __impl_time_zone_generic_long_v1 as impl_time_zone_generic_long_v1;
#[macro_use]
#[path = "macros/time_zone_generic_short_v1.data.rs"]
mod time_zone_generic_short_v1;
#[doc(inline)]
pub use __impl_time_zone_generic_short_v1 as impl_time_zone_generic_short_v1;
#[macro_use]
#[path = "macros/time_zone_specific_long_v1.data.rs"]
mod time_zone_specific_long_v1;
#[doc(inline)]
pub use __impl_time_zone_specific_long_v1 as impl_time_zone_specific_long_v1;
#[macro_use]
#[path = "macros/time_zone_specific_short_v1.data.rs"]
mod time_zone_specific_short_v1;
#[doc(inline)]
pub use __impl_time_zone_specific_short_v1 as impl_time_zone_specific_short_v1;
