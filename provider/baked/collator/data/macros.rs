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
        #[clippy::msrv = "1.70"]
        impl $name {
            #[allow(dead_code)]
            pub(crate) const MUST_USE_MAKE_PROVIDER_MACRO: () = ();
        }
        icu_provider::impl_data_provider_never_marker!($name);
    };
}
#[doc(inline)]
pub use __make_provider as make_provider;
#[macro_use]
#[path = "macros/collator_data_v1.rs.data"]
mod collator_data_v1;
#[doc(inline)]
pub use __impl_collator_data_v1 as impl_collator_data_v1;
#[doc(inline)]
pub use __impliterable_collator_data_v1 as impliterable_collator_data_v1;
#[macro_use]
#[path = "macros/collator_dia_v1.rs.data"]
mod collator_dia_v1;
#[doc(inline)]
pub use __impl_collator_dia_v1 as impl_collator_dia_v1;
#[doc(inline)]
pub use __impliterable_collator_dia_v1 as impliterable_collator_dia_v1;
#[macro_use]
#[path = "macros/collator_jamo_v1.rs.data"]
mod collator_jamo_v1;
#[doc(inline)]
pub use __impl_collator_jamo_v1 as impl_collator_jamo_v1;
#[doc(inline)]
pub use __impliterable_collator_jamo_v1 as impliterable_collator_jamo_v1;
#[macro_use]
#[path = "macros/collator_meta_v1.rs.data"]
mod collator_meta_v1;
#[doc(inline)]
pub use __impl_collator_meta_v1 as impl_collator_meta_v1;
#[doc(inline)]
pub use __impliterable_collator_meta_v1 as impliterable_collator_meta_v1;
#[macro_use]
#[path = "macros/collator_prim_v1.rs.data"]
mod collator_prim_v1;
#[doc(inline)]
pub use __impl_collator_prim_v1 as impl_collator_prim_v1;
#[doc(inline)]
pub use __impliterable_collator_prim_v1 as impliterable_collator_prim_v1;
#[macro_use]
#[path = "macros/collator_reord_v1.rs.data"]
mod collator_reord_v1;
#[doc(inline)]
pub use __impl_collator_reord_v1 as impl_collator_reord_v1;
#[doc(inline)]
pub use __impliterable_collator_reord_v1 as impliterable_collator_reord_v1;
