// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Case mapping for Unicode characters and strings.
//!
//! This module is published as its own crate ([`icu_casemap`](https://docs.rs/icu_casemap/latest/icu_casemap/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! # Examples
//!
//! ```rust
//! use icu_casemap::CaseMapper;
//! use icu_locid::langid;
//!
//! let cm = CaseMapper::new();
//!
//! assert_eq!(cm.uppercase_to_string("hello world", &langid!("und")), "HELLO WORLD");
//! assert_eq!(cm.lowercase_to_string("Γειά σου Κόσμε", &langid!("und")), "γειά σου κόσμε");
//! ```
//!
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
//! </div>
//!
//! [`ICU4X`]: ../icu/index.html

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
// We're using Greek identifiers here on purpose. These lints can only be disabled at the crate level
#![allow(confusable_idents, uncommon_codepoints)]

extern crate alloc;

mod casemapper;
mod closer;
pub mod provider;
mod set;
pub mod titlecase;

#[doc(hidden)]
#[allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
pub mod greek_to_me;
mod internals;

pub use casemapper::CaseMapper;
pub use closer::CaseMapCloser;
pub use set::ClosureSink;
pub use titlecase::TitlecaseMapper;
