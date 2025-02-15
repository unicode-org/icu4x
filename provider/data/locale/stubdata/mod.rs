// @generated
include!("exemplar_characters_punctuation_v1.rs.data");
include!("script_direction_v1.rs.data");
include!("likely_subtags_for_language_v1.rs.data");
include!("likely_subtags_for_script_region_v1.rs.data");
include!("exemplar_characters_main_v1.rs.data");
include!("exemplar_characters_auxiliary_v1.rs.data");
include!("aliases_v2.rs.data");
include!("exemplar_characters_index_v1.rs.data");
include!("likely_subtags_extended_v1.rs.data");
include!("parents_v1.rs.data");
include!("exemplar_characters_numbers_v1.rs.data");
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
        impl_exemplar_characters_punctuation_v1!($provider);
        impl_script_direction_v1!($provider);
        impl_likely_subtags_for_language_v1!($provider);
        impl_likely_subtags_for_script_region_v1!($provider);
        impl_exemplar_characters_main_v1!($provider);
        impl_exemplar_characters_auxiliary_v1!($provider);
        impl_aliases_v2!($provider);
        impl_exemplar_characters_index_v1!($provider);
        impl_likely_subtags_extended_v1!($provider);
        impl_parents_v1!($provider);
        impl_exemplar_characters_numbers_v1!($provider);
    };
}
