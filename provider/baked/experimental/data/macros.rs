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
#[path = "macros/percent_essentials_v1.rs.data"]
mod percent_essentials_v1;
#[doc(inline)]
pub use __impl_percent_essentials_v1 as impl_percent_essentials_v1;
#[macro_use]
#[path = "macros/personnames_personnames_v1.rs.data"]
mod personnames_personnames_v1;
#[doc(inline)]
pub use __impl_personnames_personnames_v1 as impl_personnames_personnames_v1;
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
#[path = "macros/units_info_v1.rs.data"]
mod units_info_v1;
#[doc(inline)]
pub use __impl_units_info_v1 as impl_units_info_v1;
