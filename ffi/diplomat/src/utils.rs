// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// Equivalent to a ? operator, but useful inside functions that return DiplomatResult<_, ICU4XError>
macro_rules! try_icu4x {
    ($e:expr) => {
        match $e {
            Result::Ok(r) => r,
            Result::Err(e) => return DiplomatResult::from(Err(ICU4XError::from(e))),
        }
    };
}
