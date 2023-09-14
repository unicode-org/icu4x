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
        #[clippy::msrv = "1.66"]
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
#[path = "macros/transliterator_rules_v1.data.rs"]
mod transliterator_rules_v1;
#[doc(inline)]
pub use __impl_transliterator_rules_v1 as impl_transliterator_rules_v1;
