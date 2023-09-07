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
#[path = "macros/collator_data_v1.data.rs"]
mod collator_data_v1;
#[doc(inline)]
pub use __impl_collator_data_v1 as impl_collator_data_v1;
#[macro_use]
#[path = "macros/collator_dia_v1.data.rs"]
mod collator_dia_v1;
#[doc(inline)]
pub use __impl_collator_dia_v1 as impl_collator_dia_v1;
#[macro_use]
#[path = "macros/collator_jamo_v1.data.rs"]
mod collator_jamo_v1;
#[doc(inline)]
pub use __impl_collator_jamo_v1 as impl_collator_jamo_v1;
#[macro_use]
#[path = "macros/collator_meta_v1.data.rs"]
mod collator_meta_v1;
#[doc(inline)]
pub use __impl_collator_meta_v1 as impl_collator_meta_v1;
#[macro_use]
#[path = "macros/collator_prim_v1.data.rs"]
mod collator_prim_v1;
#[doc(inline)]
pub use __impl_collator_prim_v1 as impl_collator_prim_v1;
#[macro_use]
#[path = "macros/collator_reord_v1.data.rs"]
mod collator_reord_v1;
#[doc(inline)]
pub use __impl_collator_reord_v1 as impl_collator_reord_v1;
