// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Experimental\] Relative time formatting
//!
//! This module is published as its own crate ([`icu_relativetime`](https://docs.rs/icu_relativetime/latest/icu_relativetime/))
//! and as part of the [`icu`](https://docs.rs/icu/latest/icu/) crate. See the latter for more details on the ICU4X project.
//!
//! <div class="stab unstable">
//! 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
//! of the icu meta-crate. Use with caution.
//! </div>

#![warn(missing_docs)]

extern crate alloc;

mod error;
mod format;
pub mod options;
pub mod provider;
mod relativetime;

pub use error::RelativeTimeError;
pub use format::FormattedRelativeTime;
pub use options::RelativeTimeFormatterOptions;
pub use relativetime::RelativeTimeFormatter;
#[doc(no_inline)]
pub use RelativeTimeError as Error;
