// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).
use alloc::string::String;
use displaydoc::Display;

#[derive(PartialEq, Debug, Display)]
pub enum Error {
    #[displaydoc("attempted to insert at an index that is not a character boundary")]
    PositionNotCharBoundary(usize, String),
    #[displaydoc("attempted to insert at an index that outside the string")]
    IndexOutOfBounds(usize),
}
