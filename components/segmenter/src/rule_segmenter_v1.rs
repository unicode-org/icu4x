// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::complex::ComplexPayloadsBorrowed;
use crate::provider::*;
use crate::scaffold::{PotentiallyIllFormedUtf8, RuleBreakType, Utf8, Utf16};
use alloc::vec::Vec;

pub(crate) trait ComplexRunSegmenter: RuleBreakType {
    fn segment_complex_run(
        complex: ComplexPayloadsBorrowed<'_>,
        input: &Self::IterAttr<'_>,
        start: usize,
        end: usize,
    ) -> Vec<usize>;
}

impl ComplexRunSegmenter for Utf8 {
    fn segment_complex_run(
        complex: ComplexPayloadsBorrowed<'_>,
        input: &Self::IterAttr<'_>,
        start: usize,
        end: usize,
    ) -> Vec<usize> {
        #[allow(clippy::indexing_slicing)] // valid offsets from CharIndices
        let input = &input.as_str()[start..end];
        complex.segment_str(input)
    }
}

impl ComplexRunSegmenter for PotentiallyIllFormedUtf8 {
    fn segment_complex_run(
        complex: ComplexPayloadsBorrowed<'_>,
        input: &Self::IterAttr<'_>,
        start: usize,
        end: usize,
    ) -> Vec<usize> {
        #[allow(clippy::indexing_slicing)] // valid offsets from Utf8CharIndices
        let input = &input.as_slice()[start..end];
        complex.segment_utf8(input)
    }
}

impl ComplexRunSegmenter for Utf16 {
    fn segment_complex_run(
        complex: ComplexPayloadsBorrowed<'_>,
        input: &Self::IterAttr<'_>,
        start: usize,
        end: usize,
    ) -> Vec<usize> {
        #[allow(clippy::indexing_slicing)] // valid offsets from Utf16Indices
        let input = &input.as_slice()[start..end];
        complex.segment_utf16(input)
    }
}

/// Implements the [`Iterator`] trait over the segmenter boundaries of the given string.
///
/// Lifetimes:
///
/// - `'l` = lifetime of the segmenter object from which this iterator was created
/// - `'data` = lifetime of data borrowed by segmenter object
///   (this largely exists because segmenter data is invariant due to `ZeroMap` constraints,
///   think of it as a second 'l)
/// - `'s` = lifetime of the string being segmented
///
/// The [`Iterator::Item`] is an [`usize`] representing index of a code unit
/// _after_ the boundary (for a boundary at the end of text, this index is the length
/// of the [`str`] or array of code units).
#[derive(Debug)]
pub struct RuleBreakIterator<'data, 's, Y: RuleBreakType> {
    pub(crate) input: Y::IterAttr<'s>,
    pub(crate) iter: Y::IterAttr<'s>,
    pub(crate) len: usize,
    pub(crate) current_pos_data: Option<(usize, Y::CharType)>,
    pub(crate) result_cache: Vec<usize>,
    pub(crate) data: &'data RuleBreakData<'data>,
    pub(crate) complex: Option<ComplexPayloadsBorrowed<'data>>,
    // The property associated with the previous break
    pub(crate) boundary_property: u8,
    pub(crate) locale_override: Option<&'data RuleBreakDataOverride<'data>>,
    // Should return None if there is no complex script handling
    pub(crate) handle_complex:
        fn(&mut RuleBreakIterator<'data, 's, Y>, Y::CharType) -> Option<usize>,
}

pub(crate) fn empty_handle_complex<Y: RuleBreakType>(
    _i: &mut RuleBreakIterator<'_, '_, Y>,
    _c: Y::CharType,
) -> Option<usize> {
    debug_assert!(
        false,
        "grapheme/sentence segmenters should never need complex handling"
    );
    None
}

impl<Y: RuleBreakType> Iterator for RuleBreakIterator<'_, '_, Y> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // If we have break point cache by previous run, return this result
        if let Some(&first_result) = self.result_cache.first() {
            let mut i = 0;
            loop {
                if i == first_result {
                    self.result_cache = self.result_cache.iter().skip(1).map(|r| r - i).collect();
                    return self.get_current_position();
                }
                i += self.get_current_codepoint().map_or(0, Y::char_len);
                self.advance_iter();
                if self.is_eof() {
                    self.result_cache.clear();
                    self.boundary_property = self.data.complex_property;
                    return Some(self.len);
                }
            }
        }

        if self.is_eof() {
            self.advance_iter();
            if self.is_eof() && self.len == 0 {
                // Empty string. Since `self.current_pos_data` is always going to be empty,
                // we never read `self.len` except for here, so we can use it to mark that
                // we have already returned the single empty-string breakpoint.
                self.len = 1;
                return Some(0);
            }
            let Some(right_prop) = self.get_current_break_property() else {
                // iterator already reaches to EOT. Reset boundary property.
                self.boundary_property = 0;
                return None;
            };
            // SOT x anything
            if matches!(
                self.data
                    .get_break_state_from_table(self.data.sot_property, right_prop),
                BreakState::Break | BreakState::NoMatch
            ) {
                self.boundary_property = 0; // SOT is special type
                return self.get_current_position();
            }
        }

        'a: loop {
            debug_assert!(!self.is_eof());
            let left_codepoint = self.get_current_codepoint()?;
            let left_prop = self.get_break_property(left_codepoint);
            self.advance_iter();

            let Some(right_prop) = self.get_current_break_property() else {
                self.boundary_property = left_prop;
                return Some(self.len);
            };

            // Some scripts rules doesn't have segmentation rules, we have to use LSTM (or dictionary) segmenter.
            // If property is marked as SA, use it
            if Y::CAN_CONTAIN_SA && right_prop == self.data.complex_property {
                if left_prop != self.data.complex_property {
                    // break before SA
                    self.boundary_property = left_prop;
                    return self.get_current_position();
                }
                let break_offset = (self.handle_complex)(self, left_codepoint);
                self.boundary_property = self.data.complex_property;
                if break_offset.is_some() {
                    return break_offset;
                }
            }

            match self.data.get_break_state_from_table(left_prop, right_prop) {
                BreakState::Keep => continue,
                BreakState::Break | BreakState::NoMatch => {
                    self.boundary_property = left_prop;
                    return self.get_current_position();
                }
                BreakState::Index(mut index) | BreakState::Intermediate(mut index) => {
                    // This isn't simple rule set. We need marker to restore iterator to previous position.
                    let mut previous_iter = self.iter.clone();
                    let mut previous_pos_data = self.current_pos_data;
                    let mut previous_left_prop = left_prop;

                    loop {
                        self.advance_iter();

                        let Some(prop) = self.get_current_break_property() else {
                            // Reached EOF. But we are analyzing multiple characters now, so next break may be previous point.
                            self.boundary_property = index;
                            if (self
                                .data
                                .get_break_state_from_table(index, self.data.eot_property))
                                == BreakState::NoMatch
                            {
                                self.boundary_property = previous_left_prop;
                                self.iter = previous_iter;
                                self.current_pos_data = previous_pos_data;
                                return self.get_current_position();
                            }
                            // EOF
                            return Some(self.len);
                        };

                        let previous_break_state_is_cp_prop =
                            index <= self.data.last_codepoint_property;

                        match self.data.get_break_state_from_table(index, prop) {
                            BreakState::Keep => continue 'a,
                            BreakState::NoMatch => {
                                self.boundary_property = previous_left_prop;
                                self.iter = previous_iter;
                                self.current_pos_data = previous_pos_data;
                                return self.get_current_position();
                            }
                            BreakState::Break => return self.get_current_position(),
                            BreakState::Intermediate(i) => {
                                index = i;
                                if previous_break_state_is_cp_prop {
                                    // Move marker
                                    previous_left_prop = index;
                                }
                                previous_iter = self.iter.clone();
                                previous_pos_data = self.current_pos_data;
                            }
                            BreakState::Index(i) => {
                                index = i;
                                if previous_break_state_is_cp_prop {
                                    // Move marker
                                    previous_iter = self.iter.clone();
                                    previous_pos_data = self.current_pos_data;
                                    previous_left_prop = index;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl<Y: RuleBreakType> RuleBreakIterator<'_, '_, Y> {
    pub(crate) fn advance_iter(&mut self) {
        self.current_pos_data = self.iter.next();
    }

    pub(crate) fn is_eof(&self) -> bool {
        self.current_pos_data.is_none()
    }

    pub(crate) fn get_current_break_property(&self) -> Option<u8> {
        self.get_current_codepoint()
            .map(|c| self.get_break_property(c))
    }

    pub(crate) fn get_current_position(&self) -> Option<usize> {
        self.current_pos_data.map(|(pos, _)| pos)
    }

    pub(crate) fn get_current_codepoint(&self) -> Option<Y::CharType> {
        self.current_pos_data.map(|(_, codepoint)| codepoint)
    }

    fn get_break_property(&self, codepoint: Y::CharType) -> u8 {
        // Note: Default value is 0 == UNKNOWN
        if let Some(locale_override) = &self.locale_override {
            let property = locale_override
                .property_table_override
                .get32(codepoint.into());
            if property != 0 {
                return property;
            }
        }
        self.data.property_table.get32(codepoint.into())
    }

    /// Return the status value of break boundary.
    pub(crate) fn rule_status(&self) -> u8 {
        if self.boundary_property == 0 {
            // break position is SOT / Any
            return 0;
        }
        self.data
            .rule_status_table
            .get((self.boundary_property - 1) as usize)
            .unwrap_or_default()
    }
}
