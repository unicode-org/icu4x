// @generated
include!("uts46_decomposition_data_v2.rs.data");
include!("non_recursive_decomposition_supplement_v1.rs.data");
include!("canonical_decomposition_tables_v1.rs.data");
include!("canonical_decomposition_data_v2.rs.data");
include!("compatibility_decomposition_tables_v1.rs.data");
include!("compatibility_decomposition_data_v2.rs.data");
include!("canonical_compositions_v1.rs.data");
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
        impl_uts46_decomposition_data_v2!($provider);
        impl_non_recursive_decomposition_supplement_v1!($provider);
        impl_canonical_decomposition_tables_v1!($provider);
        impl_canonical_decomposition_data_v2!($provider);
        impl_compatibility_decomposition_tables_v1!($provider);
        impl_compatibility_decomposition_data_v2!($provider);
        impl_canonical_compositions_v1!($provider);
    };
}
