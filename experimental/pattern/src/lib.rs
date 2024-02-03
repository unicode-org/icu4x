// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

mod numeric_placeholder;
#[cfg(feature = "alloc")]
mod parser;

pub use numeric_placeholder::NumericPlaceholderPattern;
pub use numeric_placeholder::NumericPlaceholderPatternItem;
pub use numeric_placeholder::NumericPlaceholderProvider;

#[cfg(feature = "alloc")]
pub use parser::error::ParserError;
