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
#[path = "macros/compactdecimal_long_v1.data.rs"]
mod compactdecimal_long_v1;
#[doc(inline)]
pub use __impl_compactdecimal_long_v1 as impl_compactdecimal_long_v1;
#[macro_use]
#[path = "macros/compactdecimal_short_v1.data.rs"]
mod compactdecimal_short_v1;
#[doc(inline)]
pub use __impl_compactdecimal_short_v1 as impl_compactdecimal_short_v1;
