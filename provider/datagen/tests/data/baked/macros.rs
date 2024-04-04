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
        icu_provider::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[macro_use]
#[path = "macros/calendar_chinesecache_v1.rs.data"]
mod calendar_chinesecache_v1;
#[doc(inline)]
pub use __impl_calendar_chinesecache_v1 as impl_calendar_chinesecache_v1;
#[macro_use]
#[path = "macros/calendar_dangicache_v1.rs.data"]
mod calendar_dangicache_v1;
#[doc(inline)]
pub use __impl_calendar_dangicache_v1 as impl_calendar_dangicache_v1;
#[macro_use]
#[path = "macros/calendar_japanese_v1.rs.data"]
mod calendar_japanese_v1;
#[doc(inline)]
pub use __impl_calendar_japanese_v1 as impl_calendar_japanese_v1;
#[macro_use]
#[path = "macros/calendar_japanext_v1.rs.data"]
mod calendar_japanext_v1;
#[doc(inline)]
pub use __impl_calendar_japanext_v1 as impl_calendar_japanext_v1;
#[macro_use]
#[path = "macros/collator_data_v1.rs.data"]
mod collator_data_v1;
#[doc(inline)]
pub use __impl_collator_data_v1 as impl_collator_data_v1;
#[macro_use]
#[path = "macros/collator_dia_v1.rs.data"]
mod collator_dia_v1;
#[doc(inline)]
pub use __impl_collator_dia_v1 as impl_collator_dia_v1;
#[macro_use]
#[path = "macros/collator_jamo_v1.rs.data"]
mod collator_jamo_v1;
#[doc(inline)]
pub use __impl_collator_jamo_v1 as impl_collator_jamo_v1;
#[macro_use]
#[path = "macros/collator_meta_v1.rs.data"]
mod collator_meta_v1;
#[doc(inline)]
pub use __impl_collator_meta_v1 as impl_collator_meta_v1;
#[macro_use]
#[path = "macros/collator_prim_v1.rs.data"]
mod collator_prim_v1;
#[doc(inline)]
pub use __impl_collator_prim_v1 as impl_collator_prim_v1;
#[macro_use]
#[path = "macros/collator_reord_v1.rs.data"]
mod collator_reord_v1;
#[doc(inline)]
pub use __impl_collator_reord_v1 as impl_collator_reord_v1;
#[macro_use]
#[path = "macros/compactdecimal_long_v1.rs.data"]
mod compactdecimal_long_v1;
#[doc(inline)]
pub use __impl_compactdecimal_long_v1 as impl_compactdecimal_long_v1;
#[macro_use]
#[path = "macros/compactdecimal_short_v1.rs.data"]
mod compactdecimal_short_v1;
#[doc(inline)]
pub use __impl_compactdecimal_short_v1 as impl_compactdecimal_short_v1;
#[macro_use]
#[path = "macros/currency_essentials_v1.rs.data"]
mod currency_essentials_v1;
#[doc(inline)]
pub use __impl_currency_essentials_v1 as impl_currency_essentials_v1;
#[macro_use]
#[path = "macros/datetime_buddhist_datelengths_v1.rs.data"]
mod datetime_buddhist_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_buddhist_datelengths_v1 as impl_datetime_buddhist_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_buddhist_datesymbols_v1.rs.data"]
mod datetime_buddhist_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_buddhist_datesymbols_v1 as impl_datetime_buddhist_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_chinese_datelengths_v1.rs.data"]
mod datetime_chinese_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_chinese_datelengths_v1 as impl_datetime_chinese_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_chinese_datesymbols_v1.rs.data"]
mod datetime_chinese_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_chinese_datesymbols_v1 as impl_datetime_chinese_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_coptic_datelengths_v1.rs.data"]
mod datetime_coptic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_coptic_datelengths_v1 as impl_datetime_coptic_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_coptic_datesymbols_v1.rs.data"]
mod datetime_coptic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_coptic_datesymbols_v1 as impl_datetime_coptic_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_dangi_datelengths_v1.rs.data"]
mod datetime_dangi_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_dangi_datelengths_v1 as impl_datetime_dangi_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_dangi_datesymbols_v1.rs.data"]
mod datetime_dangi_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_dangi_datesymbols_v1 as impl_datetime_dangi_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_ethiopic_datelengths_v1.rs.data"]
mod datetime_ethiopic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_ethiopic_datelengths_v1 as impl_datetime_ethiopic_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_ethiopic_datesymbols_v1.rs.data"]
mod datetime_ethiopic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_ethiopic_datesymbols_v1 as impl_datetime_ethiopic_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_gregory_datelengths_v1.rs.data"]
mod datetime_gregory_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_gregory_datelengths_v1 as impl_datetime_gregory_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_gregory_datesymbols_v1.rs.data"]
mod datetime_gregory_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_gregory_datesymbols_v1 as impl_datetime_gregory_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_hebrew_datelengths_v1.rs.data"]
mod datetime_hebrew_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_hebrew_datelengths_v1 as impl_datetime_hebrew_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_hebrew_datesymbols_v1.rs.data"]
mod datetime_hebrew_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_hebrew_datesymbols_v1 as impl_datetime_hebrew_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_indian_datelengths_v1.rs.data"]
mod datetime_indian_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_indian_datelengths_v1 as impl_datetime_indian_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_indian_datesymbols_v1.rs.data"]
mod datetime_indian_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_indian_datesymbols_v1 as impl_datetime_indian_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_islamic_datelengths_v1.rs.data"]
mod datetime_islamic_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_islamic_datelengths_v1 as impl_datetime_islamic_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_islamic_datesymbols_v1.rs.data"]
mod datetime_islamic_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_islamic_datesymbols_v1 as impl_datetime_islamic_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_japanese_datelengths_v1.rs.data"]
mod datetime_japanese_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_japanese_datelengths_v1 as impl_datetime_japanese_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_japanese_datesymbols_v1.rs.data"]
mod datetime_japanese_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_japanese_datesymbols_v1 as impl_datetime_japanese_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_japanext_datelengths_v1.rs.data"]
mod datetime_japanext_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_japanext_datelengths_v1 as impl_datetime_japanext_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_japanext_datesymbols_v1.rs.data"]
mod datetime_japanext_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_japanext_datesymbols_v1 as impl_datetime_japanext_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_patterns_buddhist_date_v1.rs.data"]
mod datetime_patterns_buddhist_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_buddhist_date_v1 as impl_datetime_patterns_buddhist_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_chinese_date_v1.rs.data"]
mod datetime_patterns_chinese_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_chinese_date_v1 as impl_datetime_patterns_chinese_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_coptic_date_v1.rs.data"]
mod datetime_patterns_coptic_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_coptic_date_v1 as impl_datetime_patterns_coptic_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_dangi_date_v1.rs.data"]
mod datetime_patterns_dangi_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_dangi_date_v1 as impl_datetime_patterns_dangi_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_datetime_v1.rs.data"]
mod datetime_patterns_datetime_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_datetime_v1 as impl_datetime_patterns_datetime_v1;
#[macro_use]
#[path = "macros/datetime_patterns_ethiopic_date_v1.rs.data"]
mod datetime_patterns_ethiopic_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_ethiopic_date_v1 as impl_datetime_patterns_ethiopic_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_gregory_date_v1.rs.data"]
mod datetime_patterns_gregory_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_gregory_date_v1 as impl_datetime_patterns_gregory_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_hebrew_date_v1.rs.data"]
mod datetime_patterns_hebrew_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_hebrew_date_v1 as impl_datetime_patterns_hebrew_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_indian_date_v1.rs.data"]
mod datetime_patterns_indian_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_indian_date_v1 as impl_datetime_patterns_indian_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_islamic_date_v1.rs.data"]
mod datetime_patterns_islamic_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_islamic_date_v1 as impl_datetime_patterns_islamic_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_japanese_date_v1.rs.data"]
mod datetime_patterns_japanese_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_japanese_date_v1 as impl_datetime_patterns_japanese_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_japanext_date_v1.rs.data"]
mod datetime_patterns_japanext_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_japanext_date_v1 as impl_datetime_patterns_japanext_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_persian_date_v1.rs.data"]
mod datetime_patterns_persian_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_persian_date_v1 as impl_datetime_patterns_persian_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_roc_date_v1.rs.data"]
mod datetime_patterns_roc_date_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_roc_date_v1 as impl_datetime_patterns_roc_date_v1;
#[macro_use]
#[path = "macros/datetime_patterns_time_v1.rs.data"]
mod datetime_patterns_time_v1;
#[doc(inline)]
pub use __impl_datetime_patterns_time_v1 as impl_datetime_patterns_time_v1;
#[macro_use]
#[path = "macros/datetime_persian_datelengths_v1.rs.data"]
mod datetime_persian_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_persian_datelengths_v1 as impl_datetime_persian_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_persian_datesymbols_v1.rs.data"]
mod datetime_persian_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_persian_datesymbols_v1 as impl_datetime_persian_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_roc_datelengths_v1.rs.data"]
mod datetime_roc_datelengths_v1;
#[doc(inline)]
pub use __impl_datetime_roc_datelengths_v1 as impl_datetime_roc_datelengths_v1;
#[macro_use]
#[path = "macros/datetime_roc_datesymbols_v1.rs.data"]
mod datetime_roc_datesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_roc_datesymbols_v1 as impl_datetime_roc_datesymbols_v1;
#[macro_use]
#[path = "macros/datetime_skeletons_v1.rs.data"]
mod datetime_skeletons_v1;
#[doc(inline)]
pub use __impl_datetime_skeletons_v1 as impl_datetime_skeletons_v1;
#[macro_use]
#[path = "macros/datetime_symbols_buddhist_months_v1.rs.data"]
mod datetime_symbols_buddhist_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_buddhist_months_v1 as impl_datetime_symbols_buddhist_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_buddhist_years_v1.rs.data"]
mod datetime_symbols_buddhist_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_buddhist_years_v1 as impl_datetime_symbols_buddhist_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_chinese_months_v1.rs.data"]
mod datetime_symbols_chinese_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_chinese_months_v1 as impl_datetime_symbols_chinese_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_chinese_years_v1.rs.data"]
mod datetime_symbols_chinese_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_chinese_years_v1 as impl_datetime_symbols_chinese_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_coptic_months_v1.rs.data"]
mod datetime_symbols_coptic_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_coptic_months_v1 as impl_datetime_symbols_coptic_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_coptic_years_v1.rs.data"]
mod datetime_symbols_coptic_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_coptic_years_v1 as impl_datetime_symbols_coptic_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_dangi_months_v1.rs.data"]
mod datetime_symbols_dangi_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_dangi_months_v1 as impl_datetime_symbols_dangi_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_dangi_years_v1.rs.data"]
mod datetime_symbols_dangi_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_dangi_years_v1 as impl_datetime_symbols_dangi_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_dayperiods_v1.rs.data"]
mod datetime_symbols_dayperiods_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_dayperiods_v1 as impl_datetime_symbols_dayperiods_v1;
#[macro_use]
#[path = "macros/datetime_symbols_ethiopic_months_v1.rs.data"]
mod datetime_symbols_ethiopic_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_ethiopic_months_v1 as impl_datetime_symbols_ethiopic_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_ethiopic_years_v1.rs.data"]
mod datetime_symbols_ethiopic_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_ethiopic_years_v1 as impl_datetime_symbols_ethiopic_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_gregory_months_v1.rs.data"]
mod datetime_symbols_gregory_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_gregory_months_v1 as impl_datetime_symbols_gregory_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_gregory_years_v1.rs.data"]
mod datetime_symbols_gregory_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_gregory_years_v1 as impl_datetime_symbols_gregory_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_hebrew_months_v1.rs.data"]
mod datetime_symbols_hebrew_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_hebrew_months_v1 as impl_datetime_symbols_hebrew_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_hebrew_years_v1.rs.data"]
mod datetime_symbols_hebrew_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_hebrew_years_v1 as impl_datetime_symbols_hebrew_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_indian_months_v1.rs.data"]
mod datetime_symbols_indian_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_indian_months_v1 as impl_datetime_symbols_indian_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_indian_years_v1.rs.data"]
mod datetime_symbols_indian_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_indian_years_v1 as impl_datetime_symbols_indian_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_islamic_months_v1.rs.data"]
mod datetime_symbols_islamic_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_islamic_months_v1 as impl_datetime_symbols_islamic_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_islamic_years_v1.rs.data"]
mod datetime_symbols_islamic_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_islamic_years_v1 as impl_datetime_symbols_islamic_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanese_months_v1.rs.data"]
mod datetime_symbols_japanese_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanese_months_v1 as impl_datetime_symbols_japanese_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanese_years_v1.rs.data"]
mod datetime_symbols_japanese_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanese_years_v1 as impl_datetime_symbols_japanese_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanext_months_v1.rs.data"]
mod datetime_symbols_japanext_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanext_months_v1 as impl_datetime_symbols_japanext_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_japanext_years_v1.rs.data"]
mod datetime_symbols_japanext_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_japanext_years_v1 as impl_datetime_symbols_japanext_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_persian_months_v1.rs.data"]
mod datetime_symbols_persian_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_persian_months_v1 as impl_datetime_symbols_persian_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_persian_years_v1.rs.data"]
mod datetime_symbols_persian_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_persian_years_v1 as impl_datetime_symbols_persian_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_roc_months_v1.rs.data"]
mod datetime_symbols_roc_months_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_roc_months_v1 as impl_datetime_symbols_roc_months_v1;
#[macro_use]
#[path = "macros/datetime_symbols_roc_years_v1.rs.data"]
mod datetime_symbols_roc_years_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_roc_years_v1 as impl_datetime_symbols_roc_years_v1;
#[macro_use]
#[path = "macros/datetime_symbols_weekdays_v1.rs.data"]
mod datetime_symbols_weekdays_v1;
#[doc(inline)]
pub use __impl_datetime_symbols_weekdays_v1 as impl_datetime_symbols_weekdays_v1;
#[macro_use]
#[path = "macros/datetime_timelengths_v1.rs.data"]
mod datetime_timelengths_v1;
#[doc(inline)]
pub use __impl_datetime_timelengths_v1 as impl_datetime_timelengths_v1;
#[macro_use]
#[path = "macros/datetime_timesymbols_v1.rs.data"]
mod datetime_timesymbols_v1;
#[doc(inline)]
pub use __impl_datetime_timesymbols_v1 as impl_datetime_timesymbols_v1;
#[macro_use]
#[path = "macros/datetime_week_data_v1.rs.data"]
mod datetime_week_data_v1;
#[doc(inline)]
pub use __impl_datetime_week_data_v1 as impl_datetime_week_data_v1;
#[macro_use]
#[path = "macros/datetime_week_data_v2.rs.data"]
mod datetime_week_data_v2;
#[doc(inline)]
pub use __impl_datetime_week_data_v2 as impl_datetime_week_data_v2;
#[macro_use]
#[path = "macros/decimal_symbols_v1.rs.data"]
mod decimal_symbols_v1;
#[doc(inline)]
pub use __impl_decimal_symbols_v1 as impl_decimal_symbols_v1;
#[macro_use]
#[path = "macros/displaynames_languages_v1.rs.data"]
mod displaynames_languages_v1;
#[doc(inline)]
pub use __impl_displaynames_languages_v1 as impl_displaynames_languages_v1;
#[macro_use]
#[path = "macros/displaynames_locales_v1.rs.data"]
mod displaynames_locales_v1;
#[doc(inline)]
pub use __impl_displaynames_locales_v1 as impl_displaynames_locales_v1;
#[macro_use]
#[path = "macros/displaynames_regions_v1.rs.data"]
mod displaynames_regions_v1;
#[doc(inline)]
pub use __impl_displaynames_regions_v1 as impl_displaynames_regions_v1;
#[macro_use]
#[path = "macros/displaynames_scripts_v1.rs.data"]
mod displaynames_scripts_v1;
#[doc(inline)]
pub use __impl_displaynames_scripts_v1 as impl_displaynames_scripts_v1;
#[macro_use]
#[path = "macros/displaynames_variants_v1.rs.data"]
mod displaynames_variants_v1;
#[doc(inline)]
pub use __impl_displaynames_variants_v1 as impl_displaynames_variants_v1;
#[macro_use]
#[path = "macros/fallback_likelysubtags_v1.rs.data"]
mod fallback_likelysubtags_v1;
#[doc(inline)]
pub use __impl_fallback_likelysubtags_v1 as impl_fallback_likelysubtags_v1;
#[macro_use]
#[path = "macros/fallback_parents_v1.rs.data"]
mod fallback_parents_v1;
#[doc(inline)]
pub use __impl_fallback_parents_v1 as impl_fallback_parents_v1;
#[macro_use]
#[path = "macros/fallback_supplement_co_v1.rs.data"]
mod fallback_supplement_co_v1;
#[doc(inline)]
pub use __impl_fallback_supplement_co_v1 as impl_fallback_supplement_co_v1;
#[macro_use]
#[path = "macros/list_and_v1.rs.data"]
mod list_and_v1;
#[doc(inline)]
pub use __impl_list_and_v1 as impl_list_and_v1;
#[macro_use]
#[path = "macros/list_or_v1.rs.data"]
mod list_or_v1;
#[doc(inline)]
pub use __impl_list_or_v1 as impl_list_or_v1;
#[macro_use]
#[path = "macros/list_unit_v1.rs.data"]
mod list_unit_v1;
#[doc(inline)]
pub use __impl_list_unit_v1 as impl_list_unit_v1;
#[macro_use]
#[path = "macros/locid_transform_aliases_v1.rs.data"]
mod locid_transform_aliases_v1;
#[doc(inline)]
pub use __impl_locid_transform_aliases_v1 as impl_locid_transform_aliases_v1;
#[macro_use]
#[path = "macros/locid_transform_aliases_v2.rs.data"]
mod locid_transform_aliases_v2;
#[doc(inline)]
pub use __impl_locid_transform_aliases_v2 as impl_locid_transform_aliases_v2;
#[macro_use]
#[path = "macros/locid_transform_likelysubtags_v1.rs.data"]
mod locid_transform_likelysubtags_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_v1 as impl_locid_transform_likelysubtags_v1;
#[macro_use]
#[path = "macros/locid_transform_likelysubtags_ext_v1.rs.data"]
mod locid_transform_likelysubtags_ext_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_ext_v1 as impl_locid_transform_likelysubtags_ext_v1;
#[macro_use]
#[path = "macros/locid_transform_likelysubtags_l_v1.rs.data"]
mod locid_transform_likelysubtags_l_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_l_v1 as impl_locid_transform_likelysubtags_l_v1;
#[macro_use]
#[path = "macros/locid_transform_likelysubtags_sr_v1.rs.data"]
mod locid_transform_likelysubtags_sr_v1;
#[doc(inline)]
pub use __impl_locid_transform_likelysubtags_sr_v1 as impl_locid_transform_likelysubtags_sr_v1;
#[macro_use]
#[path = "macros/locid_transform_script_dir_v1.rs.data"]
mod locid_transform_script_dir_v1;
#[doc(inline)]
pub use __impl_locid_transform_script_dir_v1 as impl_locid_transform_script_dir_v1;
#[macro_use]
#[path = "macros/normalizer_comp_v1.rs.data"]
mod normalizer_comp_v1;
#[doc(inline)]
pub use __impl_normalizer_comp_v1 as impl_normalizer_comp_v1;
#[macro_use]
#[path = "macros/normalizer_decomp_v1.rs.data"]
mod normalizer_decomp_v1;
#[doc(inline)]
pub use __impl_normalizer_decomp_v1 as impl_normalizer_decomp_v1;
#[macro_use]
#[path = "macros/normalizer_nfd_v1.rs.data"]
mod normalizer_nfd_v1;
#[doc(inline)]
pub use __impl_normalizer_nfd_v1 as impl_normalizer_nfd_v1;
#[macro_use]
#[path = "macros/normalizer_nfdex_v1.rs.data"]
mod normalizer_nfdex_v1;
#[doc(inline)]
pub use __impl_normalizer_nfdex_v1 as impl_normalizer_nfdex_v1;
#[macro_use]
#[path = "macros/normalizer_nfkd_v1.rs.data"]
mod normalizer_nfkd_v1;
#[doc(inline)]
pub use __impl_normalizer_nfkd_v1 as impl_normalizer_nfkd_v1;
#[macro_use]
#[path = "macros/normalizer_nfkdex_v1.rs.data"]
mod normalizer_nfkdex_v1;
#[doc(inline)]
pub use __impl_normalizer_nfkdex_v1 as impl_normalizer_nfkdex_v1;
#[macro_use]
#[path = "macros/normalizer_uts46d_v1.rs.data"]
mod normalizer_uts46d_v1;
#[doc(inline)]
pub use __impl_normalizer_uts46d_v1 as impl_normalizer_uts46d_v1;
#[macro_use]
#[path = "macros/percent_essentials_v1.rs.data"]
mod percent_essentials_v1;
#[doc(inline)]
pub use __impl_percent_essentials_v1 as impl_percent_essentials_v1;
#[macro_use]
#[path = "macros/plurals_cardinal_v1.rs.data"]
mod plurals_cardinal_v1;
#[doc(inline)]
pub use __impl_plurals_cardinal_v1 as impl_plurals_cardinal_v1;
#[macro_use]
#[path = "macros/plurals_ordinal_v1.rs.data"]
mod plurals_ordinal_v1;
#[doc(inline)]
pub use __impl_plurals_ordinal_v1 as impl_plurals_ordinal_v1;
#[macro_use]
#[path = "macros/plurals_ranges_v1.rs.data"]
mod plurals_ranges_v1;
#[doc(inline)]
pub use __impl_plurals_ranges_v1 as impl_plurals_ranges_v1;
#[macro_use]
#[path = "macros/propnames_from_gcb_v1.rs.data"]
mod propnames_from_gcb_v1;
#[doc(inline)]
pub use __impl_propnames_from_gcb_v1 as impl_propnames_from_gcb_v1;
#[macro_use]
#[path = "macros/propnames_from_insc_v1.rs.data"]
mod propnames_from_insc_v1;
#[doc(inline)]
pub use __impl_propnames_from_insc_v1 as impl_propnames_from_insc_v1;
#[macro_use]
#[path = "macros/propnames_from_sb_v1.rs.data"]
mod propnames_from_sb_v1;
#[doc(inline)]
pub use __impl_propnames_from_sb_v1 as impl_propnames_from_sb_v1;
#[macro_use]
#[path = "macros/propnames_from_wb_v1.rs.data"]
mod propnames_from_wb_v1;
#[doc(inline)]
pub use __impl_propnames_from_wb_v1 as impl_propnames_from_wb_v1;
#[macro_use]
#[path = "macros/propnames_from_bc_v1.rs.data"]
mod propnames_from_bc_v1;
#[doc(inline)]
pub use __impl_propnames_from_bc_v1 as impl_propnames_from_bc_v1;
#[macro_use]
#[path = "macros/propnames_from_ccc_v1.rs.data"]
mod propnames_from_ccc_v1;
#[doc(inline)]
pub use __impl_propnames_from_ccc_v1 as impl_propnames_from_ccc_v1;
#[macro_use]
#[path = "macros/propnames_from_ea_v1.rs.data"]
mod propnames_from_ea_v1;
#[doc(inline)]
pub use __impl_propnames_from_ea_v1 as impl_propnames_from_ea_v1;
#[macro_use]
#[path = "macros/propnames_from_gc_v1.rs.data"]
mod propnames_from_gc_v1;
#[doc(inline)]
pub use __impl_propnames_from_gc_v1 as impl_propnames_from_gc_v1;
#[macro_use]
#[path = "macros/propnames_from_gcm_v1.rs.data"]
mod propnames_from_gcm_v1;
#[doc(inline)]
pub use __impl_propnames_from_gcm_v1 as impl_propnames_from_gcm_v1;
#[macro_use]
#[path = "macros/propnames_from_jt_v1.rs.data"]
mod propnames_from_jt_v1;
#[doc(inline)]
pub use __impl_propnames_from_jt_v1 as impl_propnames_from_jt_v1;
#[macro_use]
#[path = "macros/propnames_from_lb_v1.rs.data"]
mod propnames_from_lb_v1;
#[doc(inline)]
pub use __impl_propnames_from_lb_v1 as impl_propnames_from_lb_v1;
#[macro_use]
#[path = "macros/propnames_from_sc_v1.rs.data"]
mod propnames_from_sc_v1;
#[doc(inline)]
pub use __impl_propnames_from_sc_v1 as impl_propnames_from_sc_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_gcb_v1.rs.data"]
mod propnames_to_long_linear_gcb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_gcb_v1 as impl_propnames_to_long_linear_gcb_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_insc_v1.rs.data"]
mod propnames_to_long_linear_insc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_insc_v1 as impl_propnames_to_long_linear_insc_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_sb_v1.rs.data"]
mod propnames_to_long_linear_sb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_sb_v1 as impl_propnames_to_long_linear_sb_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_wb_v1.rs.data"]
mod propnames_to_long_linear_wb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_wb_v1 as impl_propnames_to_long_linear_wb_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_bc_v1.rs.data"]
mod propnames_to_long_linear_bc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_bc_v1 as impl_propnames_to_long_linear_bc_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_ea_v1.rs.data"]
mod propnames_to_long_linear_ea_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_ea_v1 as impl_propnames_to_long_linear_ea_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_gc_v1.rs.data"]
mod propnames_to_long_linear_gc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_gc_v1 as impl_propnames_to_long_linear_gc_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_jt_v1.rs.data"]
mod propnames_to_long_linear_jt_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_jt_v1 as impl_propnames_to_long_linear_jt_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_lb_v1.rs.data"]
mod propnames_to_long_linear_lb_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_lb_v1 as impl_propnames_to_long_linear_lb_v1;
#[macro_use]
#[path = "macros/propnames_to_long_linear_sc_v1.rs.data"]
mod propnames_to_long_linear_sc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_linear_sc_v1 as impl_propnames_to_long_linear_sc_v1;
#[macro_use]
#[path = "macros/propnames_to_long_sparse_ccc_v1.rs.data"]
mod propnames_to_long_sparse_ccc_v1;
#[doc(inline)]
pub use __impl_propnames_to_long_sparse_ccc_v1 as impl_propnames_to_long_sparse_ccc_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_gcb_v1.rs.data"]
mod propnames_to_short_linear_gcb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_gcb_v1 as impl_propnames_to_short_linear_gcb_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_insc_v1.rs.data"]
mod propnames_to_short_linear_insc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_insc_v1 as impl_propnames_to_short_linear_insc_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_sb_v1.rs.data"]
mod propnames_to_short_linear_sb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_sb_v1 as impl_propnames_to_short_linear_sb_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_wb_v1.rs.data"]
mod propnames_to_short_linear_wb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_wb_v1 as impl_propnames_to_short_linear_wb_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_bc_v1.rs.data"]
mod propnames_to_short_linear_bc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_bc_v1 as impl_propnames_to_short_linear_bc_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_ea_v1.rs.data"]
mod propnames_to_short_linear_ea_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_ea_v1 as impl_propnames_to_short_linear_ea_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_gc_v1.rs.data"]
mod propnames_to_short_linear_gc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_gc_v1 as impl_propnames_to_short_linear_gc_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_jt_v1.rs.data"]
mod propnames_to_short_linear_jt_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_jt_v1 as impl_propnames_to_short_linear_jt_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear_lb_v1.rs.data"]
mod propnames_to_short_linear_lb_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear_lb_v1 as impl_propnames_to_short_linear_lb_v1;
#[macro_use]
#[path = "macros/propnames_to_short_linear4_sc_v1.rs.data"]
mod propnames_to_short_linear4_sc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_linear4_sc_v1 as impl_propnames_to_short_linear4_sc_v1;
#[macro_use]
#[path = "macros/propnames_to_short_sparse_ccc_v1.rs.data"]
mod propnames_to_short_sparse_ccc_v1;
#[doc(inline)]
pub use __impl_propnames_to_short_sparse_ccc_v1 as impl_propnames_to_short_sparse_ccc_v1;
#[macro_use]
#[path = "macros/props_ahex_v1.rs.data"]
mod props_ahex_v1;
#[doc(inline)]
pub use __impl_props_ahex_v1 as impl_props_ahex_v1;
#[macro_use]
#[path = "macros/props_alpha_v1.rs.data"]
mod props_alpha_v1;
#[doc(inline)]
pub use __impl_props_alpha_v1 as impl_props_alpha_v1;
#[macro_use]
#[path = "macros/props_basic_emoji_v1.rs.data"]
mod props_basic_emoji_v1;
#[doc(inline)]
pub use __impl_props_basic_emoji_v1 as impl_props_basic_emoji_v1;
#[macro_use]
#[path = "macros/props_bidi_c_v1.rs.data"]
mod props_bidi_c_v1;
#[doc(inline)]
pub use __impl_props_bidi_c_v1 as impl_props_bidi_c_v1;
#[macro_use]
#[path = "macros/props_bidi_m_v1.rs.data"]
mod props_bidi_m_v1;
#[doc(inline)]
pub use __impl_props_bidi_m_v1 as impl_props_bidi_m_v1;
#[macro_use]
#[path = "macros/props_ci_v1.rs.data"]
mod props_ci_v1;
#[doc(inline)]
pub use __impl_props_ci_v1 as impl_props_ci_v1;
#[macro_use]
#[path = "macros/props_cwcf_v1.rs.data"]
mod props_cwcf_v1;
#[doc(inline)]
pub use __impl_props_cwcf_v1 as impl_props_cwcf_v1;
#[macro_use]
#[path = "macros/props_cwcm_v1.rs.data"]
mod props_cwcm_v1;
#[doc(inline)]
pub use __impl_props_cwcm_v1 as impl_props_cwcm_v1;
#[macro_use]
#[path = "macros/props_cwkcf_v1.rs.data"]
mod props_cwkcf_v1;
#[doc(inline)]
pub use __impl_props_cwkcf_v1 as impl_props_cwkcf_v1;
#[macro_use]
#[path = "macros/props_cwl_v1.rs.data"]
mod props_cwl_v1;
#[doc(inline)]
pub use __impl_props_cwl_v1 as impl_props_cwl_v1;
#[macro_use]
#[path = "macros/props_cwt_v1.rs.data"]
mod props_cwt_v1;
#[doc(inline)]
pub use __impl_props_cwt_v1 as impl_props_cwt_v1;
#[macro_use]
#[path = "macros/props_cwu_v1.rs.data"]
mod props_cwu_v1;
#[doc(inline)]
pub use __impl_props_cwu_v1 as impl_props_cwu_v1;
#[macro_use]
#[path = "macros/props_cased_v1.rs.data"]
mod props_cased_v1;
#[doc(inline)]
pub use __impl_props_cased_v1 as impl_props_cased_v1;
#[macro_use]
#[path = "macros/props_comp_ex_v1.rs.data"]
mod props_comp_ex_v1;
#[doc(inline)]
pub use __impl_props_comp_ex_v1 as impl_props_comp_ex_v1;
#[macro_use]
#[path = "macros/props_di_v1.rs.data"]
mod props_di_v1;
#[doc(inline)]
pub use __impl_props_di_v1 as impl_props_di_v1;
#[macro_use]
#[path = "macros/props_dash_v1.rs.data"]
mod props_dash_v1;
#[doc(inline)]
pub use __impl_props_dash_v1 as impl_props_dash_v1;
#[macro_use]
#[path = "macros/props_dep_v1.rs.data"]
mod props_dep_v1;
#[doc(inline)]
pub use __impl_props_dep_v1 as impl_props_dep_v1;
#[macro_use]
#[path = "macros/props_dia_v1.rs.data"]
mod props_dia_v1;
#[doc(inline)]
pub use __impl_props_dia_v1 as impl_props_dia_v1;
#[macro_use]
#[path = "macros/props_ebase_v1.rs.data"]
mod props_ebase_v1;
#[doc(inline)]
pub use __impl_props_ebase_v1 as impl_props_ebase_v1;
#[macro_use]
#[path = "macros/props_ecomp_v1.rs.data"]
mod props_ecomp_v1;
#[doc(inline)]
pub use __impl_props_ecomp_v1 as impl_props_ecomp_v1;
#[macro_use]
#[path = "macros/props_emod_v1.rs.data"]
mod props_emod_v1;
#[doc(inline)]
pub use __impl_props_emod_v1 as impl_props_emod_v1;
#[macro_use]
#[path = "macros/props_epres_v1.rs.data"]
mod props_epres_v1;
#[doc(inline)]
pub use __impl_props_epres_v1 as impl_props_epres_v1;
#[macro_use]
#[path = "macros/props_emoji_v1.rs.data"]
mod props_emoji_v1;
#[doc(inline)]
pub use __impl_props_emoji_v1 as impl_props_emoji_v1;
#[macro_use]
#[path = "macros/props_ext_v1.rs.data"]
mod props_ext_v1;
#[doc(inline)]
pub use __impl_props_ext_v1 as impl_props_ext_v1;
#[macro_use]
#[path = "macros/props_extpict_v1.rs.data"]
mod props_extpict_v1;
#[doc(inline)]
pub use __impl_props_extpict_v1 as impl_props_extpict_v1;
#[macro_use]
#[path = "macros/props_gcb_v1.rs.data"]
mod props_gcb_v1;
#[doc(inline)]
pub use __impl_props_gcb_v1 as impl_props_gcb_v1;
#[macro_use]
#[path = "macros/props_gr_base_v1.rs.data"]
mod props_gr_base_v1;
#[doc(inline)]
pub use __impl_props_gr_base_v1 as impl_props_gr_base_v1;
#[macro_use]
#[path = "macros/props_gr_ext_v1.rs.data"]
mod props_gr_ext_v1;
#[doc(inline)]
pub use __impl_props_gr_ext_v1 as impl_props_gr_ext_v1;
#[macro_use]
#[path = "macros/props_gr_link_v1.rs.data"]
mod props_gr_link_v1;
#[doc(inline)]
pub use __impl_props_gr_link_v1 as impl_props_gr_link_v1;
#[macro_use]
#[path = "macros/props_hex_v1.rs.data"]
mod props_hex_v1;
#[doc(inline)]
pub use __impl_props_hex_v1 as impl_props_hex_v1;
#[macro_use]
#[path = "macros/props_hyphen_v1.rs.data"]
mod props_hyphen_v1;
#[doc(inline)]
pub use __impl_props_hyphen_v1 as impl_props_hyphen_v1;
#[macro_use]
#[path = "macros/props_idc_v1.rs.data"]
mod props_idc_v1;
#[doc(inline)]
pub use __impl_props_idc_v1 as impl_props_idc_v1;
#[macro_use]
#[path = "macros/props_ids_v1.rs.data"]
mod props_ids_v1;
#[doc(inline)]
pub use __impl_props_ids_v1 as impl_props_ids_v1;
#[macro_use]
#[path = "macros/props_idsb_v1.rs.data"]
mod props_idsb_v1;
#[doc(inline)]
pub use __impl_props_idsb_v1 as impl_props_idsb_v1;
#[macro_use]
#[path = "macros/props_idst_v1.rs.data"]
mod props_idst_v1;
#[doc(inline)]
pub use __impl_props_idst_v1 as impl_props_idst_v1;
#[macro_use]
#[path = "macros/props_ideo_v1.rs.data"]
mod props_ideo_v1;
#[doc(inline)]
pub use __impl_props_ideo_v1 as impl_props_ideo_v1;
#[macro_use]
#[path = "macros/props_insc_v1.rs.data"]
mod props_insc_v1;
#[doc(inline)]
pub use __impl_props_insc_v1 as impl_props_insc_v1;
#[macro_use]
#[path = "macros/props_join_c_v1.rs.data"]
mod props_join_c_v1;
#[doc(inline)]
pub use __impl_props_join_c_v1 as impl_props_join_c_v1;
#[macro_use]
#[path = "macros/props_loe_v1.rs.data"]
mod props_loe_v1;
#[doc(inline)]
pub use __impl_props_loe_v1 as impl_props_loe_v1;
#[macro_use]
#[path = "macros/props_lower_v1.rs.data"]
mod props_lower_v1;
#[doc(inline)]
pub use __impl_props_lower_v1 as impl_props_lower_v1;
#[macro_use]
#[path = "macros/props_math_v1.rs.data"]
mod props_math_v1;
#[doc(inline)]
pub use __impl_props_math_v1 as impl_props_math_v1;
#[macro_use]
#[path = "macros/props_nchar_v1.rs.data"]
mod props_nchar_v1;
#[doc(inline)]
pub use __impl_props_nchar_v1 as impl_props_nchar_v1;
#[macro_use]
#[path = "macros/props_pcm_v1.rs.data"]
mod props_pcm_v1;
#[doc(inline)]
pub use __impl_props_pcm_v1 as impl_props_pcm_v1;
#[macro_use]
#[path = "macros/props_pat_syn_v1.rs.data"]
mod props_pat_syn_v1;
#[doc(inline)]
pub use __impl_props_pat_syn_v1 as impl_props_pat_syn_v1;
#[macro_use]
#[path = "macros/props_pat_ws_v1.rs.data"]
mod props_pat_ws_v1;
#[doc(inline)]
pub use __impl_props_pat_ws_v1 as impl_props_pat_ws_v1;
#[macro_use]
#[path = "macros/props_qmark_v1.rs.data"]
mod props_qmark_v1;
#[doc(inline)]
pub use __impl_props_qmark_v1 as impl_props_qmark_v1;
#[macro_use]
#[path = "macros/props_ri_v1.rs.data"]
mod props_ri_v1;
#[doc(inline)]
pub use __impl_props_ri_v1 as impl_props_ri_v1;
#[macro_use]
#[path = "macros/props_radical_v1.rs.data"]
mod props_radical_v1;
#[doc(inline)]
pub use __impl_props_radical_v1 as impl_props_radical_v1;
#[macro_use]
#[path = "macros/props_sb_v1.rs.data"]
mod props_sb_v1;
#[doc(inline)]
pub use __impl_props_sb_v1 as impl_props_sb_v1;
#[macro_use]
#[path = "macros/props_sd_v1.rs.data"]
mod props_sd_v1;
#[doc(inline)]
pub use __impl_props_sd_v1 as impl_props_sd_v1;
#[macro_use]
#[path = "macros/props_sterm_v1.rs.data"]
mod props_sterm_v1;
#[doc(inline)]
pub use __impl_props_sterm_v1 as impl_props_sterm_v1;
#[macro_use]
#[path = "macros/props_sensitive_v1.rs.data"]
mod props_sensitive_v1;
#[doc(inline)]
pub use __impl_props_sensitive_v1 as impl_props_sensitive_v1;
#[macro_use]
#[path = "macros/props_term_v1.rs.data"]
mod props_term_v1;
#[doc(inline)]
pub use __impl_props_term_v1 as impl_props_term_v1;
#[macro_use]
#[path = "macros/props_uideo_v1.rs.data"]
mod props_uideo_v1;
#[doc(inline)]
pub use __impl_props_uideo_v1 as impl_props_uideo_v1;
#[macro_use]
#[path = "macros/props_upper_v1.rs.data"]
mod props_upper_v1;
#[doc(inline)]
pub use __impl_props_upper_v1 as impl_props_upper_v1;
#[macro_use]
#[path = "macros/props_vs_v1.rs.data"]
mod props_vs_v1;
#[doc(inline)]
pub use __impl_props_vs_v1 as impl_props_vs_v1;
#[macro_use]
#[path = "macros/props_wb_v1.rs.data"]
mod props_wb_v1;
#[doc(inline)]
pub use __impl_props_wb_v1 as impl_props_wb_v1;
#[macro_use]
#[path = "macros/props_wspace_v1.rs.data"]
mod props_wspace_v1;
#[doc(inline)]
pub use __impl_props_wspace_v1 as impl_props_wspace_v1;
#[macro_use]
#[path = "macros/props_xidc_v1.rs.data"]
mod props_xidc_v1;
#[doc(inline)]
pub use __impl_props_xidc_v1 as impl_props_xidc_v1;
#[macro_use]
#[path = "macros/props_xids_v1.rs.data"]
mod props_xids_v1;
#[doc(inline)]
pub use __impl_props_xids_v1 as impl_props_xids_v1;
#[macro_use]
#[path = "macros/props_alnum_v1.rs.data"]
mod props_alnum_v1;
#[doc(inline)]
pub use __impl_props_alnum_v1 as impl_props_alnum_v1;
#[macro_use]
#[path = "macros/props_bc_v1.rs.data"]
mod props_bc_v1;
#[doc(inline)]
pub use __impl_props_bc_v1 as impl_props_bc_v1;
#[macro_use]
#[path = "macros/props_bidiauxiliaryprops_v1.rs.data"]
mod props_bidiauxiliaryprops_v1;
#[doc(inline)]
pub use __impl_props_bidiauxiliaryprops_v1 as impl_props_bidiauxiliaryprops_v1;
#[macro_use]
#[path = "macros/props_blank_v1.rs.data"]
mod props_blank_v1;
#[doc(inline)]
pub use __impl_props_blank_v1 as impl_props_blank_v1;
#[macro_use]
#[path = "macros/props_casemap_v1.rs.data"]
mod props_casemap_v1;
#[doc(inline)]
pub use __impl_props_casemap_v1 as impl_props_casemap_v1;
#[macro_use]
#[path = "macros/props_casemap_unfold_v1.rs.data"]
mod props_casemap_unfold_v1;
#[doc(inline)]
pub use __impl_props_casemap_unfold_v1 as impl_props_casemap_unfold_v1;
#[macro_use]
#[path = "macros/props_ccc_v1.rs.data"]
mod props_ccc_v1;
#[doc(inline)]
pub use __impl_props_ccc_v1 as impl_props_ccc_v1;
#[macro_use]
#[path = "macros/props_ea_v1.rs.data"]
mod props_ea_v1;
#[doc(inline)]
pub use __impl_props_ea_v1 as impl_props_ea_v1;
#[macro_use]
#[path = "macros/props_exemplarchars_auxiliary_v1.rs.data"]
mod props_exemplarchars_auxiliary_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_auxiliary_v1 as impl_props_exemplarchars_auxiliary_v1;
#[macro_use]
#[path = "macros/props_exemplarchars_index_v1.rs.data"]
mod props_exemplarchars_index_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_index_v1 as impl_props_exemplarchars_index_v1;
#[macro_use]
#[path = "macros/props_exemplarchars_main_v1.rs.data"]
mod props_exemplarchars_main_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_main_v1 as impl_props_exemplarchars_main_v1;
#[macro_use]
#[path = "macros/props_exemplarchars_numbers_v1.rs.data"]
mod props_exemplarchars_numbers_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_numbers_v1 as impl_props_exemplarchars_numbers_v1;
#[macro_use]
#[path = "macros/props_exemplarchars_punctuation_v1.rs.data"]
mod props_exemplarchars_punctuation_v1;
#[doc(inline)]
pub use __impl_props_exemplarchars_punctuation_v1 as impl_props_exemplarchars_punctuation_v1;
#[macro_use]
#[path = "macros/props_gc_v1.rs.data"]
mod props_gc_v1;
#[doc(inline)]
pub use __impl_props_gc_v1 as impl_props_gc_v1;
#[macro_use]
#[path = "macros/props_graph_v1.rs.data"]
mod props_graph_v1;
#[doc(inline)]
pub use __impl_props_graph_v1 as impl_props_graph_v1;
#[macro_use]
#[path = "macros/props_jt_v1.rs.data"]
mod props_jt_v1;
#[doc(inline)]
pub use __impl_props_jt_v1 as impl_props_jt_v1;
#[macro_use]
#[path = "macros/props_lb_v1.rs.data"]
mod props_lb_v1;
#[doc(inline)]
pub use __impl_props_lb_v1 as impl_props_lb_v1;
#[macro_use]
#[path = "macros/props_nfcinert_v1.rs.data"]
mod props_nfcinert_v1;
#[doc(inline)]
pub use __impl_props_nfcinert_v1 as impl_props_nfcinert_v1;
#[macro_use]
#[path = "macros/props_nfdinert_v1.rs.data"]
mod props_nfdinert_v1;
#[doc(inline)]
pub use __impl_props_nfdinert_v1 as impl_props_nfdinert_v1;
#[macro_use]
#[path = "macros/props_nfkcinert_v1.rs.data"]
mod props_nfkcinert_v1;
#[doc(inline)]
pub use __impl_props_nfkcinert_v1 as impl_props_nfkcinert_v1;
#[macro_use]
#[path = "macros/props_nfkdinert_v1.rs.data"]
mod props_nfkdinert_v1;
#[doc(inline)]
pub use __impl_props_nfkdinert_v1 as impl_props_nfkdinert_v1;
#[macro_use]
#[path = "macros/props_print_v1.rs.data"]
mod props_print_v1;
#[doc(inline)]
pub use __impl_props_print_v1 as impl_props_print_v1;
#[macro_use]
#[path = "macros/props_sc_v1.rs.data"]
mod props_sc_v1;
#[doc(inline)]
pub use __impl_props_sc_v1 as impl_props_sc_v1;
#[macro_use]
#[path = "macros/props_scx_v1.rs.data"]
mod props_scx_v1;
#[doc(inline)]
pub use __impl_props_scx_v1 as impl_props_scx_v1;
#[macro_use]
#[path = "macros/props_segstart_v1.rs.data"]
mod props_segstart_v1;
#[doc(inline)]
pub use __impl_props_segstart_v1 as impl_props_segstart_v1;
#[macro_use]
#[path = "macros/props_xdigit_v1.rs.data"]
mod props_xdigit_v1;
#[doc(inline)]
pub use __impl_props_xdigit_v1 as impl_props_xdigit_v1;
#[macro_use]
#[path = "macros/relativetime_long_day_v1.rs.data"]
mod relativetime_long_day_v1;
#[doc(inline)]
pub use __impl_relativetime_long_day_v1 as impl_relativetime_long_day_v1;
#[macro_use]
#[path = "macros/relativetime_long_hour_v1.rs.data"]
mod relativetime_long_hour_v1;
#[doc(inline)]
pub use __impl_relativetime_long_hour_v1 as impl_relativetime_long_hour_v1;
#[macro_use]
#[path = "macros/relativetime_long_minute_v1.rs.data"]
mod relativetime_long_minute_v1;
#[doc(inline)]
pub use __impl_relativetime_long_minute_v1 as impl_relativetime_long_minute_v1;
#[macro_use]
#[path = "macros/relativetime_long_month_v1.rs.data"]
mod relativetime_long_month_v1;
#[doc(inline)]
pub use __impl_relativetime_long_month_v1 as impl_relativetime_long_month_v1;
#[macro_use]
#[path = "macros/relativetime_long_quarter_v1.rs.data"]
mod relativetime_long_quarter_v1;
#[doc(inline)]
pub use __impl_relativetime_long_quarter_v1 as impl_relativetime_long_quarter_v1;
#[macro_use]
#[path = "macros/relativetime_long_second_v1.rs.data"]
mod relativetime_long_second_v1;
#[doc(inline)]
pub use __impl_relativetime_long_second_v1 as impl_relativetime_long_second_v1;
#[macro_use]
#[path = "macros/relativetime_long_week_v1.rs.data"]
mod relativetime_long_week_v1;
#[doc(inline)]
pub use __impl_relativetime_long_week_v1 as impl_relativetime_long_week_v1;
#[macro_use]
#[path = "macros/relativetime_long_year_v1.rs.data"]
mod relativetime_long_year_v1;
#[doc(inline)]
pub use __impl_relativetime_long_year_v1 as impl_relativetime_long_year_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_day_v1.rs.data"]
mod relativetime_narrow_day_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_day_v1 as impl_relativetime_narrow_day_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_hour_v1.rs.data"]
mod relativetime_narrow_hour_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_hour_v1 as impl_relativetime_narrow_hour_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_minute_v1.rs.data"]
mod relativetime_narrow_minute_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_minute_v1 as impl_relativetime_narrow_minute_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_month_v1.rs.data"]
mod relativetime_narrow_month_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_month_v1 as impl_relativetime_narrow_month_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_quarter_v1.rs.data"]
mod relativetime_narrow_quarter_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_quarter_v1 as impl_relativetime_narrow_quarter_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_second_v1.rs.data"]
mod relativetime_narrow_second_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_second_v1 as impl_relativetime_narrow_second_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_week_v1.rs.data"]
mod relativetime_narrow_week_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_week_v1 as impl_relativetime_narrow_week_v1;
#[macro_use]
#[path = "macros/relativetime_narrow_year_v1.rs.data"]
mod relativetime_narrow_year_v1;
#[doc(inline)]
pub use __impl_relativetime_narrow_year_v1 as impl_relativetime_narrow_year_v1;
#[macro_use]
#[path = "macros/relativetime_short_day_v1.rs.data"]
mod relativetime_short_day_v1;
#[doc(inline)]
pub use __impl_relativetime_short_day_v1 as impl_relativetime_short_day_v1;
#[macro_use]
#[path = "macros/relativetime_short_hour_v1.rs.data"]
mod relativetime_short_hour_v1;
#[doc(inline)]
pub use __impl_relativetime_short_hour_v1 as impl_relativetime_short_hour_v1;
#[macro_use]
#[path = "macros/relativetime_short_minute_v1.rs.data"]
mod relativetime_short_minute_v1;
#[doc(inline)]
pub use __impl_relativetime_short_minute_v1 as impl_relativetime_short_minute_v1;
#[macro_use]
#[path = "macros/relativetime_short_month_v1.rs.data"]
mod relativetime_short_month_v1;
#[doc(inline)]
pub use __impl_relativetime_short_month_v1 as impl_relativetime_short_month_v1;
#[macro_use]
#[path = "macros/relativetime_short_quarter_v1.rs.data"]
mod relativetime_short_quarter_v1;
#[doc(inline)]
pub use __impl_relativetime_short_quarter_v1 as impl_relativetime_short_quarter_v1;
#[macro_use]
#[path = "macros/relativetime_short_second_v1.rs.data"]
mod relativetime_short_second_v1;
#[doc(inline)]
pub use __impl_relativetime_short_second_v1 as impl_relativetime_short_second_v1;
#[macro_use]
#[path = "macros/relativetime_short_week_v1.rs.data"]
mod relativetime_short_week_v1;
#[doc(inline)]
pub use __impl_relativetime_short_week_v1 as impl_relativetime_short_week_v1;
#[macro_use]
#[path = "macros/relativetime_short_year_v1.rs.data"]
mod relativetime_short_year_v1;
#[doc(inline)]
pub use __impl_relativetime_short_year_v1 as impl_relativetime_short_year_v1;
#[macro_use]
#[path = "macros/segmenter_dictionary_w_auto_v1.rs.data"]
mod segmenter_dictionary_w_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_w_auto_v1 as impl_segmenter_dictionary_w_auto_v1;
#[macro_use]
#[path = "macros/segmenter_dictionary_wl_ext_v1.rs.data"]
mod segmenter_dictionary_wl_ext_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_wl_ext_v1 as impl_segmenter_dictionary_wl_ext_v1;
#[macro_use]
#[path = "macros/segmenter_grapheme_v1.rs.data"]
mod segmenter_grapheme_v1;
#[doc(inline)]
pub use __impl_segmenter_grapheme_v1 as impl_segmenter_grapheme_v1;
#[macro_use]
#[path = "macros/segmenter_line_v1.rs.data"]
mod segmenter_line_v1;
#[doc(inline)]
pub use __impl_segmenter_line_v1 as impl_segmenter_line_v1;
#[macro_use]
#[path = "macros/segmenter_lstm_wl_auto_v1.rs.data"]
mod segmenter_lstm_wl_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_lstm_wl_auto_v1 as impl_segmenter_lstm_wl_auto_v1;
#[macro_use]
#[path = "macros/segmenter_sentence_v1.rs.data"]
mod segmenter_sentence_v1;
#[doc(inline)]
pub use __impl_segmenter_sentence_v1 as impl_segmenter_sentence_v1;
#[macro_use]
#[path = "macros/segmenter_word_v1.rs.data"]
mod segmenter_word_v1;
#[doc(inline)]
pub use __impl_segmenter_word_v1 as impl_segmenter_word_v1;
#[macro_use]
#[path = "macros/time_zone_bcp47_to_iana_v1.rs.data"]
mod time_zone_bcp47_to_iana_v1;
#[doc(inline)]
pub use __impl_time_zone_bcp47_to_iana_v1 as impl_time_zone_bcp47_to_iana_v1;
#[macro_use]
#[path = "macros/time_zone_exemplar_cities_v1.rs.data"]
mod time_zone_exemplar_cities_v1;
#[doc(inline)]
pub use __impl_time_zone_exemplar_cities_v1 as impl_time_zone_exemplar_cities_v1;
#[macro_use]
#[path = "macros/time_zone_formats_v1.rs.data"]
mod time_zone_formats_v1;
#[doc(inline)]
pub use __impl_time_zone_formats_v1 as impl_time_zone_formats_v1;
#[macro_use]
#[path = "macros/time_zone_generic_long_v1.rs.data"]
mod time_zone_generic_long_v1;
#[doc(inline)]
pub use __impl_time_zone_generic_long_v1 as impl_time_zone_generic_long_v1;
#[macro_use]
#[path = "macros/time_zone_generic_short_v1.rs.data"]
mod time_zone_generic_short_v1;
#[doc(inline)]
pub use __impl_time_zone_generic_short_v1 as impl_time_zone_generic_short_v1;
#[macro_use]
#[path = "macros/time_zone_iana_to_bcp47_v1.rs.data"]
mod time_zone_iana_to_bcp47_v1;
#[doc(inline)]
pub use __impl_time_zone_iana_to_bcp47_v1 as impl_time_zone_iana_to_bcp47_v1;
#[macro_use]
#[path = "macros/time_zone_iana_to_bcp47_v2.rs.data"]
mod time_zone_iana_to_bcp47_v2;
#[doc(inline)]
pub use __impl_time_zone_iana_to_bcp47_v2 as impl_time_zone_iana_to_bcp47_v2;
#[macro_use]
#[path = "macros/time_zone_metazone_period_v1.rs.data"]
mod time_zone_metazone_period_v1;
#[doc(inline)]
pub use __impl_time_zone_metazone_period_v1 as impl_time_zone_metazone_period_v1;
#[macro_use]
#[path = "macros/time_zone_specific_long_v1.rs.data"]
mod time_zone_specific_long_v1;
#[doc(inline)]
pub use __impl_time_zone_specific_long_v1 as impl_time_zone_specific_long_v1;
#[macro_use]
#[path = "macros/time_zone_specific_short_v1.rs.data"]
mod time_zone_specific_short_v1;
#[doc(inline)]
pub use __impl_time_zone_specific_short_v1 as impl_time_zone_specific_short_v1;
#[macro_use]
#[path = "macros/transliterator_rules_v1.rs.data"]
mod transliterator_rules_v1;
#[doc(inline)]
pub use __impl_transliterator_rules_v1 as impl_transliterator_rules_v1;
#[macro_use]
#[path = "macros/units_info_v1.rs.data"]
mod units_info_v1;
#[doc(inline)]
pub use __impl_units_info_v1 as impl_units_info_v1;
