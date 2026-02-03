// @generated
include!("timezone_identifiers_iana_extended_v1.rs.data");
include!("timezone_identifiers_windows_v1.rs.data");
include!("timezone_periods_v1.rs.data");
include!("timezone_identifiers_iana_core_v1.rs.data");
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
        #[clippy::msrv = "1.85"]
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
        impl_timezone_identifiers_iana_extended_v1!($provider);
        impl_timezone_identifiers_windows_v1!($provider);
        impl_timezone_periods_v1!($provider);
        impl_timezone_identifiers_iana_core_v1!($provider);
    };
}
