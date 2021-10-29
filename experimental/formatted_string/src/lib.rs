// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(not(any(test, feature = "std")), no_std)]

extern crate alloc;

mod error;
mod formatted_string;

pub use crate::formatted_string::FormattedString;
pub use crate::formatted_string::FormattedStringBuilder;
pub use crate::formatted_string::LayeredFormattedString;
pub use crate::formatted_string::LayeredFormattedStringBuilder;
pub use error::Error as FormattedStringBuilderError;
