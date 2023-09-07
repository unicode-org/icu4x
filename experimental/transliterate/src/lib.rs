// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Transliteration
//!
//! This module is published as its own crate ([`icu_transliterate`](https://docs.rs/icu_transliterate/latest/icu_transliterate/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! See [`Transliterator`].
//!
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! </div>

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations
#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

extern crate alloc;

pub mod provider;

#[cfg(feature = "datagen")]
mod compile;
mod error;
#[allow(clippy::indexing_slicing, clippy::unwrap_used)] // TODO(#3958): Remove.
mod transliterator;

pub use error::*;
pub use transliterator::*;

#[doc(no_inline)]
pub use TransliteratorError as Error;

#[cfg(feature = "datagen")]
pub use compile::{legacy_id_to_internal_id, Direction};
