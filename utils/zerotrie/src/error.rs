// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Display)]
pub enum Error {
    /// Non-ASCII data was added to an ASCII-only collection.
    #[displaydoc("Non-ASCII cannot be added to an ASCII-only collection")]
    NonAsciiError,
    /// The collection reached its maximum supported capacity.
    #[displaydoc("Reached maximum capacity of collection")]
    CapacityExceeded,
    /// The builder could not solve the perfect hash function.
    #[displaydoc("Failed to solve the perfect hash function. This is rare! Please report your case to the ICU4X team.")]
    CouldNotSolvePerfectHash,
}
