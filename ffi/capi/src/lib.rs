// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        // Exhaustiveness and Debug is not required for Diplomat types
    )
)]
// Diplomat limitations
#![allow(
    clippy::needless_lifetimes,
    clippy::result_unit_err,
    clippy::should_implement_trait
)]

//! This crate contains the `extern "C"` FFI for ICU4X, as well as the [Diplomat](https://github.com/rust-diplomat/diplomat)-generated
//! C, C++, Dart, JavaScript, and TypeScript bindings.
//!
#![allow(rustdoc::invalid_html_tags)]
// attribute split over three lines because `cargo generate-readmes` does not evaluate `#![doc = ]` docs
//! <p style='font-weight: bold; font-size: 24px;'> 🔗 See the <a target='_blank' href='https://icu4x.unicode.org/
#![cfg_attr(doc, doc = core::env!("CARGO_PKG_VERSION"))]
//! '>ICU4X website</a> for FFI docs and examples</p>
//!
//! This crate is `no_std`-compatible, but requires an allocator. If you wish to use it in `no_std` mode, you can either
//! enable the `looping_panic_handler` and `libc_alloc` Cargo features, or write a wrapper crate that defines an
//! allocator/panic handler.
//!
//! <div class="stab unstable">
//! 🚧 While the types in this crate are public, APIs from this crate are <em>not intended to be used from Rust</em> and as
//! such this crate may unpredictably change its Rust API across compatible semver versions.
//!
//! The <code>extern "C"</code> APIs exposed by this crate, while not directly documented, are stable within the same major
//! semver version, as are the bindings in the <code>bindings</code> folder.
//! </div>
//!

// Renamed so you can't accidentally use it
#[cfg(target_arch = "wasm32")]
extern crate std as rust_std;

#[cfg(all(not(feature = "std"), feature = "looping_panic_handler"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;
#[cfg(all(not(feature = "std"), feature = "libc_alloc"))]
extern crate libc_alloc;

#[cfg(feature = "datetime")]
pub(crate) mod datetime_helpers;

/// The Rust API of this crate is **UNSTABLE**; this crate is primarily intended
/// to be used from its FFI bindings, packaged with the crate.
///
/// The C ABI layer is stable.
#[path = "."] // https://github.com/rust-lang/rust/issues/35016
pub mod unstable {
    // Common modules
    pub mod errors;
    pub mod locale_core;
    #[cfg(feature = "logging")]
    pub mod logging;
    #[macro_use]
    pub mod provider;

    // Components

    #[cfg(feature = "properties")]
    pub mod bidi;
    #[cfg(any(feature = "datetime", feature = "timezone", feature = "calendar"))]
    pub mod calendar;
    #[cfg(feature = "casemap")]
    pub mod casemap;
    #[cfg(feature = "collator")]
    pub mod collator;
    #[cfg(feature = "properties")]
    pub mod collections_sets;
    #[cfg(any(feature = "datetime", feature = "timezone", feature = "calendar"))]
    pub mod date;
    #[cfg(feature = "datetime")]
    pub mod date_formatter;
    #[cfg(feature = "datetime")]
    pub mod date_time_formatter;
    #[cfg(any(feature = "datetime", feature = "timezone", feature = "calendar"))]
    pub mod datetime;
    #[cfg(feature = "datetime")]
    pub mod datetime_options;
    #[cfg(feature = "decimal")]
    pub mod decimal;
    #[cfg(feature = "experimental")]
    pub mod displaynames;
    #[cfg(feature = "locale")]
    pub mod exemplar_chars;
    #[cfg(feature = "locale")]
    pub mod fallbacker;
    #[cfg(feature = "decimal")]
    pub mod fixed_decimal;
    #[cfg(any(feature = "datetime", feature = "timezone"))]
    pub mod iana_parser;
    #[cfg(feature = "list")]
    pub mod list;
    #[cfg(feature = "locale")]
    pub mod locale;
    #[cfg(feature = "locale")]
    pub mod locale_directionality;
    #[cfg(feature = "experimental")]
    pub mod measure_unit_parser;
    #[cfg(feature = "normalizer")]
    pub mod normalizer;
    #[cfg(feature = "normalizer")]
    pub mod normalizer_properties;
    #[cfg(feature = "plurals")]
    pub mod pluralrules;
    #[cfg(feature = "properties")]
    pub mod properties_bidi;
    #[cfg(feature = "properties")]
    pub mod properties_enums;
    #[cfg(feature = "properties")]
    pub mod properties_iter;
    #[cfg(feature = "properties")]
    pub mod properties_maps;
    #[cfg(feature = "properties")]
    pub mod properties_names;
    #[cfg(feature = "properties")]
    pub mod properties_sets;
    #[cfg(feature = "properties")]
    pub mod properties_unisets;
    #[cfg(feature = "properties")]
    pub mod script;
    #[cfg(feature = "segmenter")]
    pub mod segmenter_grapheme;
    #[cfg(feature = "segmenter")]
    pub mod segmenter_line;
    #[cfg(feature = "segmenter")]
    pub mod segmenter_sentence;
    #[cfg(feature = "segmenter")]
    pub mod segmenter_word;
    #[cfg(any(feature = "datetime", feature = "timezone", feature = "calendar"))]
    pub mod time;
    #[cfg(feature = "datetime")]
    pub mod time_formatter;
    #[cfg(any(feature = "datetime", feature = "timezone"))]
    pub mod timezone;
    #[cfg(feature = "datetime")]
    pub mod timezone_formatter;
    #[cfg(feature = "experimental")]
    pub mod units_converter;
    #[cfg(any(feature = "datetime", feature = "timezone"))]
    pub mod variant_offset;
    #[cfg(feature = "calendar")]
    pub mod week;
    #[cfg(any(feature = "datetime", feature = "timezone"))]
    pub mod windows_parser;
    #[cfg(feature = "datetime")]
    pub mod zoned_date_formatter;
    #[cfg(feature = "datetime")]
    pub mod zoned_date_time_formatter;
    #[cfg(feature = "datetime")]
    pub mod zoned_datetime;
    #[cfg(feature = "datetime")]
    pub mod zoned_time_formatter;
}
