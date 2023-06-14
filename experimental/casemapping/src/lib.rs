// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Case mapping for Unicode characters and strings.
//!
//! This module is published as its own crate ([`icu_casemapping`](https://docs.rs/icu_casemapping/latest/icu_casemapping/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
//! </div>
//!
//! [`ICU4X`]: ../icu/index.html

#![warn(missing_docs)]

mod casemapping;
pub mod provider;

mod error;
#[cfg(feature = "datagen")]
mod exceptions_builder;
mod internals;

pub use casemapping::CaseMapping;
pub use error::Error as CaseMappingError;
#[doc(no_inline)]
pub use CaseMappingError as Error;
