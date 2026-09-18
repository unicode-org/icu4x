// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::ops::Range;
use writeable::Writeable;

/// A changed source range and its replacement in the case mapped output.
///
/// Characteristics:
/// - Both ranges use UTF-8 byte offsets, relative to the beginning of the input and
///   output.
/// - Endpoints are character boundaries.
/// - An empty destination range represents a deletion.
/// - Equal length ranges can still represent a change, for example `a` to `A`.
/// - Edits are ordered, non-overlapping, and reported separately for each changed
///   source character.
/// - Gaps between edits are unchanged text.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct CaseMapEdit {
    /// The range replaced in the source string.
    pub source: Range<usize>,
    /// The replacement range in the output string.
    pub destination: Range<usize>,
}

/// A case mapping result that can also report how the source text changed.
///
/// # Examples
///
/// ```
/// use icu::casemap::{CaseMapper, CaseMapWriteable};
/// use icu::locale::langid;
///
/// let (output, edits) = CaseMapper::new()
///     .uppercase("cß!", &langid!("und"))
///     .to_string_with_edits();
/// assert_eq!(output, "CSS!");
/// assert_eq!(edits.len(), 2);
/// assert_eq!(edits[0].source, 0..1);
/// assert_eq!(edits[0].destination, 0..1);
/// assert_eq!(edits[1].source, 1..3);
/// assert_eq!(edits[1].destination, 1..3);
/// ```
pub trait CaseMapWriteable: Writeable {
    /// Writes the mapped text and calls `record_edit` for each change.
    ///
    /// Errors from `sink` are propagated immediately. On error, the sink and
    /// callback may have received partial output.
    ///
    /// See [`CaseMapEdit`].
    fn write_to_with_edits<W: fmt::Write + ?Sized>(
        &self,
        sink: &mut W,
        record_edit: impl FnMut(CaseMapEdit),
    ) -> fmt::Result;

    /// Returns the mapped string together with its changes.
    ///
    /// Prefer `write_to_with_edits` to avoid allocations.
    fn to_string_with_edits(&self) -> (String, Vec<CaseMapEdit>) {
        let mut output = String::new();
        let mut edits = Vec::new();
        let _ = self.write_to_with_edits(&mut output, |edit| edits.push(edit));
        (output, edits)
    }
}
