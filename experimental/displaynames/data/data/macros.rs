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
#[path = "macros/displaynames_languages_v1.data.rs"]
mod displaynames_languages_v1;
#[doc(inline)]
pub use __impl_displaynames_languages_v1 as impl_displaynames_languages_v1;
#[macro_use]
#[path = "macros/displaynames_locales_v1.data.rs"]
mod displaynames_locales_v1;
#[doc(inline)]
pub use __impl_displaynames_locales_v1 as impl_displaynames_locales_v1;
#[macro_use]
#[path = "macros/displaynames_regions_v1.data.rs"]
mod displaynames_regions_v1;
#[doc(inline)]
pub use __impl_displaynames_regions_v1 as impl_displaynames_regions_v1;
#[macro_use]
#[path = "macros/displaynames_scripts_v1.data.rs"]
mod displaynames_scripts_v1;
#[doc(inline)]
pub use __impl_displaynames_scripts_v1 as impl_displaynames_scripts_v1;
#[macro_use]
#[path = "macros/displaynames_variants_v1.data.rs"]
mod displaynames_variants_v1;
#[doc(inline)]
pub use __impl_displaynames_variants_v1 as impl_displaynames_variants_v1;
