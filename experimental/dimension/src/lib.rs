// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental.

use icu_provider::DataPayload;
use options::CurrencyFormatterOptions;

extern crate alloc;

pub mod format;
pub mod options;
pub mod provider;
pub mod ule;

/// A formatter for currencies.
pub struct CurrencyFormatter {
    options: CurrencyFormatterOptions,
    essential: DataPayload<provider::CurrencyEssentialsV1Marker>,
}
