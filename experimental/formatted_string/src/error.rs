// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use displaydoc::Display;

#[derive(PartialEq, Debug, Display)]
pub enum Error {
    /// index {0} is not a character boundary in {1:?}
    PositionNotCharBoundary(usize, String),
    /// index {0} is out of bounds
    IndexOutOfBounds(usize),
}
