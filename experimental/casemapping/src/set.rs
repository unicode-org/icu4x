// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointinvlist::CodePointInversionListBuilder;

/// A setlike object that can hold characters and/or strings
/// to be used with [`CaseMapping::add_string_case_closure()`]
///
/// [`CaseMapping::add_string_case_closure()`]: crate::CaseMapping::add_string_case_closure
pub trait ClosureSet {
    /// Add a character to the set
    fn add_char(&mut self, c: char);
    /// Add a string to the set
    fn add_string(&mut self, string: &str);
}

impl ClosureSet for CodePointInversionListBuilder {
    fn add_char(&mut self, c: char) {
        self.add_char(c)
    }

    // The current version of CodePointInversionList doesn't include strings.
    // Trying to add a string is a no-op that will be optimized away.
    #[inline]
    fn add_string(&mut self, _string: &str) {}
}
