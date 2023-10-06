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
#[path = "macros/time_zone_bcp47_to_iana_v1.data.rs"]
mod time_zone_bcp47_to_iana_v1;
#[doc(inline)]
pub use __impl_time_zone_bcp47_to_iana_v1 as impl_time_zone_bcp47_to_iana_v1;
#[macro_use]
#[path = "macros/time_zone_iana_to_bcp47_v1.data.rs"]
mod time_zone_iana_to_bcp47_v1;
#[doc(inline)]
pub use __impl_time_zone_iana_to_bcp47_v1 as impl_time_zone_iana_to_bcp47_v1;
#[macro_use]
#[path = "macros/time_zone_metazone_period_v1.data.rs"]
mod time_zone_metazone_period_v1;
#[doc(inline)]
pub use __impl_time_zone_metazone_period_v1 as impl_time_zone_metazone_period_v1;
