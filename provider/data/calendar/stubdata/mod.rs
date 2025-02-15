// @generated
include!("chinese_cache_v1.rs.data");
include!("japanese_eras_v1.rs.data");
include!("islamic_observational_cache_v1.rs.data");
include!("japanese_extended_eras_v1.rs.data");
include!("dangi_cache_v1.rs.data");
include!("week_data_v2.rs.data");
include!("islamic_umm_al_qura_cache_v1.rs.data");
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
        impl_chinese_cache_v1!($provider);
        impl_japanese_eras_v1!($provider);
        impl_islamic_observational_cache_v1!($provider);
        impl_japanese_extended_eras_v1!($provider);
        impl_dangi_cache_v1!($provider);
        impl_week_data_v2!($provider);
        impl_islamic_umm_al_qura_cache_v1!($provider);
    };
}
