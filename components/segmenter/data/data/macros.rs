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
#[path = "macros/segmenter_dictionary_w_auto_v1.data.rs"]
mod segmenter_dictionary_w_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_w_auto_v1 as impl_segmenter_dictionary_w_auto_v1;
#[macro_use]
#[path = "macros/segmenter_dictionary_wl_ext_v1.data.rs"]
mod segmenter_dictionary_wl_ext_v1;
#[doc(inline)]
pub use __impl_segmenter_dictionary_wl_ext_v1 as impl_segmenter_dictionary_wl_ext_v1;
#[macro_use]
#[path = "macros/segmenter_grapheme_v1.data.rs"]
mod segmenter_grapheme_v1;
#[doc(inline)]
pub use __impl_segmenter_grapheme_v1 as impl_segmenter_grapheme_v1;
#[macro_use]
#[path = "macros/segmenter_line_v1.data.rs"]
mod segmenter_line_v1;
#[doc(inline)]
pub use __impl_segmenter_line_v1 as impl_segmenter_line_v1;
#[macro_use]
#[path = "macros/segmenter_lstm_wl_auto_v1.data.rs"]
mod segmenter_lstm_wl_auto_v1;
#[doc(inline)]
pub use __impl_segmenter_lstm_wl_auto_v1 as impl_segmenter_lstm_wl_auto_v1;
#[macro_use]
#[path = "macros/segmenter_sentence_v1.data.rs"]
mod segmenter_sentence_v1;
#[doc(inline)]
pub use __impl_segmenter_sentence_v1 as impl_segmenter_sentence_v1;
#[macro_use]
#[path = "macros/segmenter_word_v1.data.rs"]
mod segmenter_word_v1;
#[doc(inline)]
pub use __impl_segmenter_word_v1 as impl_segmenter_word_v1;
