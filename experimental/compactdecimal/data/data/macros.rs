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
macro_rules ! __create_provider { ($ (# [$ meta : meta]) * $ vis : vis $ name : ident) => { # [derive (Debug)] $ (# [$ meta]) * # [clippy :: msrv = "1.66"] $ vis struct $ name ; # [clippy :: msrv = "1.66"] impl $ name { # [doc (hidden)] # [allow (dead_code)] pub const MUST_USE_CREATE_PROVIDER_MACRO : () = () ; } } ; }
#[doc(inline)]
pub use __create_provider as create_provider;
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
