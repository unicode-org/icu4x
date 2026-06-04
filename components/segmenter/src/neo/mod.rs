// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental reimplementations

use crate::complex::ComplexPayloadsBorrowed;
use crate::provider::{Acceptance, Class, RuleBreakDataOverride, SegmenterStateMachine};
use crate::scaffold::{PotentiallyIllFormedUtf8, RuleBreakType, Utf8, Utf16};
use alloc::collections::VecDeque;
use alloc::vec::Vec;

mod line;
use icu_collections::codepointtrie::CodePointTrie;
pub use line::*;
mod grapheme;
pub use grapheme::*;
mod sentence;
pub use sentence::*;
mod word;
pub use word::*;

pub(crate) trait Tailoring {
    fn class(&self, data: &CodePointTrie<Class>, cp: u32) -> Class;
}

impl Tailoring for () {
    fn class(&self, data: &CodePointTrie<Class>, cp: u32) -> Class {
        data.get32(cp)
    }
}

impl Tailoring for Option<&'_ RuleBreakDataOverride<'_>> {
    fn class(&self, data: &CodePointTrie<Class>, cp: u32) -> Class {
        if let Some(tailoring) = self {
            let c = tailoring.property_table_override.get32(cp);
            if c != SegmenterStateMachine::NO_CLASS {
                return c;
            }
        }

        data.get32(cp)
    }
}

pub(crate) trait RuleBreakTypeWithComplex: RuleBreakType {
    fn handle<'s>(
        data: &ComplexPayloadsBorrowed,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> Vec<usize>;
}

impl RuleBreakTypeWithComplex for Utf8 {
    fn handle<'s>(
        data: &ComplexPayloadsBorrowed,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> Vec<usize> {
        #[allow(clippy::indexing_slicing)] // valid offset
        let complex = &complex.as_str()[..(Self::offset(past_complex) - Self::offset(complex))];
        data.complex_language_segment_str(complex)
    }
}

impl RuleBreakTypeWithComplex for PotentiallyIllFormedUtf8 {
    fn handle<'s>(
        data: &ComplexPayloadsBorrowed,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> Vec<usize> {
        #[allow(clippy::indexing_slicing)] // valid offset
        let complex = &complex.as_slice()[..(Self::offset(past_complex) - Self::offset(complex))];
        let Ok(complex) = core::str::from_utf8(complex) else {
            return alloc::vec![complex.len()];
        };
        data.complex_language_segment_str(complex)
    }
}

impl RuleBreakTypeWithComplex for Utf16 {
    fn handle<'s>(
        data: &ComplexPayloadsBorrowed,
        complex: &Self::IterAttr<'s>,
        past_complex: &Self::IterAttr<'s>,
    ) -> Vec<usize> {
        #[allow(clippy::indexing_slicing)] // valid offset
        let complex = &complex.as_slice()[..(Self::offset(past_complex) - Self::offset(complex))];
        data.complex_language_segment_utf16(complex)
    }
}

#[derive(Debug)]
struct ComplexHandling<'data, 's, Y: RuleBreakType> {
    data: ComplexPayloadsBorrowed<'data>,
    break_at_boundaries: bool,
    break_status: u8,
    handler: fn(&ComplexPayloadsBorrowed, &Y::IterAttr<'s>, &Y::IterAttr<'s>) -> Vec<usize>,
}

/// Implements the [`Iterator`] trait over the line break opportunities of the given string.
///
/// Lifetimes:
///
/// - `'l` = lifetime of the [`LineSegmenter`] object from which this iterator was created
/// - `'s` = lifetime of the string being segmented
///
/// The [`Iterator::Item`] is an [`usize`] representing index of a code unit
/// _after_ the break (for a break at the end of text, this index is the length
/// of the [`str`] or array of code units).
///
/// For examples of use, see [`LineSegmenter`].
#[derive(Debug)]
pub(crate) struct RuleBreakIterator<'data, 's, Y: RuleBreakType, T: Tailoring> {
    data: &'data SegmenterStateMachine<'data>,
    tailoring: T,
    cache: VecDeque<usize>,
    lookahead_positions: Vec<Option<Y::IterAttr<'s>>>,
    remaining_input: Y::IterAttr<'s>,
    last_accepting_status: u8,
    complex: Option<ComplexHandling<'data, 's, Y>>,
}

impl<'data, 's, Y: RuleBreakType, T: Tailoring> RuleBreakIterator<'data, 's, Y, T> {
    pub(crate) fn new_non_complex(
        input: Y::IterAttr<'s>,
        data: &'data SegmenterStateMachine<'data>,
        tailoring: T,
    ) -> Self {
        Self {
            remaining_input: input,
            data,
            tailoring,
            complex: None,
            cache: VecDeque::from_iter([0]),
            lookahead_positions: alloc::vec![None; data.num_lookaheads],
            last_accepting_status: 0,
        }
    }

    pub(crate) fn new_with_complex(
        input: Y::IterAttr<'s>,
        data: &'data SegmenterStateMachine<'data>,
        tailoring: T,
        complex: ComplexPayloadsBorrowed<'data>,
        complex_break_at_boundary: bool,
        complex_status: u8,
    ) -> Self
    where
        Y: RuleBreakTypeWithComplex,
    {
        Self {
            data,
            tailoring,
            complex: Some(ComplexHandling {
                data: complex,
                break_at_boundaries: complex_break_at_boundary,
                break_status: complex_status,
                handler: Y::handle,
            }),
            cache: VecDeque::from_iter([0]),
            lookahead_positions: alloc::vec![None; data.num_lookaheads],
            last_accepting_status: 0,
            remaining_input: input,
        }
    }
}

impl<'s, Y: RuleBreakType, T: Tailoring> Iterator for RuleBreakIterator<'_, 's, Y, T> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.cache.pop_front() {
            return Some(i);
        }

        if Y::is_empty(&self.remaining_input) {
            self.last_accepting_status = 0;
            return None;
        }

        let mut state = SegmenterStateMachine::START_STATE;
        let mut iter = self.remaining_input.clone();

        // Dummy value, we don't use this until it has been replaced
        let mut last_accepting: Y::IterAttr<'s> = iter.clone();
        let mut last_accepting_status = 0;
        self.lookahead_positions.fill(None);

        let mut complex_state = None;

        (self.remaining_input, self.last_accepting_status) = loop {
            let (class, complex_handler) = if let Some((_, next)) = iter.clone().peekable().next() {
                let cp = next.into();
                (
                    self.tailoring.class(&self.data.classes, cp),
                    self.complex.as_ref().filter(|c| c.data.handles(cp)),
                )
            } else {
                (SegmenterStateMachine::EOT_CLASS, None)
            };

            // Enter complex handling if:
            // * We have a complex handler that can handle the current code point
            // * We haven't already started complex handling and are reentering the loop to
            //   find the alternative non-complex break
            // * The input can contain complex code points
            //   * This constant lets the compiler remove this code altogether Latin-1
            if let Some(complex_handler) = complex_handler
                && complex_state.is_none()
                && Y::CAN_CONTAIN_SA
            {
                let mut past_complex = iter.clone();
                let mut last_complex = past_complex.clone();
                past_complex.next();
                while past_complex
                    .clone()
                    .peekable()
                    .next_if(|&(_, c)| complex_handler.data.handles(c.into()))
                    .is_some()
                {
                    past_complex.next();
                    last_complex.next();
                }

                // A complex segment of length 1 doesn't need special handling.
                if Y::offset(&last_complex) != Y::offset(&iter) {
                    let results =
                        (complex_handler.handler)(&complex_handler.data, &iter, &past_complex);

                    let offset = Y::offset(&iter);
                    self.cache = results.into_iter().map(|i| i + offset).collect();

                    if complex_handler.break_at_boundaries {
                        // `self.cache` contains a break point at the end of the run, but not at the start.
                        // Store the position of the end of the run, and return the current position
                        // for the start break point (unless it's 0, which we already returned earlier).
                        self.remaining_input = past_complex;
                        self.last_accepting_status = complex_handler.break_status;
                        return if offset == 0 {
                            self.cache.pop_front()
                        } else {
                            Some(offset)
                        };
                    } else {
                        // Remove the break point at the end of the run, and store `last_complex`, the location
                        // of the last complex code point of the run. We'll later restart the state machine
                        // from this code point, in order to correctly break after it (the state machine will
                        // treat it as Alphabetic).
                        self.cache.pop_back();
                        complex_state = Some((last_complex, complex_handler.break_status));

                        // We keep running the state machine to figure out if there's a break point at the start.
                    }
                }
            }

            iter.next();

            if let Some(next_state) = self
                .data
                .transitions
                .get(usize::from(state) + usize::from(class) * self.data.states.len())
                .filter(|&s| s != SegmenterStateMachine::TRASH_STATE)
            {
                state = next_state;
            } else {
                // No transition, the break point is the last accepting state
                break (last_accepting, last_accepting_status);
            }

            let (acceptance, lookahead) = self
                .data
                .states
                .get(usize::from(state))
                // GIGO
                .unwrap_or((Acceptance::Continue, None));

            match acceptance {
                Acceptance::Continue => (),
                Acceptance::Accept(status) => {
                    last_accepting = iter.clone();
                    last_accepting_status = status;
                }
                Acceptance::Conditional(l, status) => {
                    if let Some(Some(last)) = self.lookahead_positions.get(usize::from(l)) {
                        // Lookahead hit, the break point is the last position for `l`
                        break (last.clone(), status);
                    }
                }
            }

            if let Some(lookahead) = lookahead {
                if let Some(p) = self.lookahead_positions.get_mut(usize::from(lookahead)) {
                    *p = Some(iter.clone())
                }
            }
        };

        let break_index = Y::offset(&self.remaining_input);

        // We encountered complex text and populated the cache
        if let Some(&first_complex_break) = self.cache.front() {
            if let Some((last_complex_cp, complex_status)) = complex_state {
                self.remaining_input = last_complex_cp;
                // return the complex break if it's before the break we calculated using the state machine
                if first_complex_break < break_index {
                    self.last_accepting_status = complex_status;
                    return self.cache.pop_front();
                }
            } else {
                debug_assert!(false, "self.cache populated but no complex state");
            }
        }

        Some(break_index)
    }
}
