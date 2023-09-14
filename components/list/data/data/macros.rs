// @generated
/// Declares a data provider. You can then use `impl_data_provider` or
/// key specific macros, like `impl_core_helloworld_v1` to add implementations.
///
/// ```ignore
/// #[path = "/path/to/generated/macros.rs"];
/// mod macros;
/// macros::create_provider(MyDataProvider);
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
#[path = "macros/list_and_v1.data.rs"]
mod list_and_v1;
#[doc(inline)]
pub use __impl_list_and_v1 as impl_list_and_v1;
#[macro_use]
#[path = "macros/list_or_v1.data.rs"]
mod list_or_v1;
#[doc(inline)]
pub use __impl_list_or_v1 as impl_list_or_v1;
#[macro_use]
#[path = "macros/list_unit_v1.data.rs"]
mod list_unit_v1;
#[doc(inline)]
pub use __impl_list_unit_v1 as impl_list_unit_v1;
