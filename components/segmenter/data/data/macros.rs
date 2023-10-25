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
#[path = "macros/segmenter_dictionary_w_auto_v1.rs.data"]
mod segmenter_dictionary_w_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_w_auto_v1 as impl_segmenter_dictionary_w_auto_v1;
#[macro_use]
#[path = "macros/segmenter_dictionary_wl_ext_v1.rs.data"]
mod segmenter_dictionary_wl_ext_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_wl_ext_v1 as impl_segmenter_dictionary_wl_ext_v1;
#[macro_use]
#[path = "macros/segmenter_grapheme_v1.rs.data"]
mod segmenter_grapheme_v1;
#[doc(inline)]
pub use __impl_segmenter_grapheme_v1 as impl_segmenter_grapheme_v1;
#[macro_use]
#[path = "macros/segmenter_line_v1.rs.data"]
mod segmenter_line_v1;
#[doc(inline)]
pub use __impl_segmenter_line_v1 as impl_segmenter_line_v1;
#[macro_use]
#[path = "macros/segmenter_lstm_wl_auto_v1.rs.data"]
mod segmenter_lstm_wl_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_lstm_wl_auto_v1 as impl_segmenter_lstm_wl_auto_v1;
#[macro_use]
#[path = "macros/segmenter_sentence_v1.rs.data"]
mod segmenter_sentence_v1;
#[doc(inline)]
pub use __impl_segmenter_sentence_v1 as impl_segmenter_sentence_v1;
#[macro_use]
#[path = "macros/segmenter_word_v1.rs.data"]
mod segmenter_word_v1;
#[doc(inline)]
pub use __impl_segmenter_word_v1 as impl_segmenter_word_v1;
