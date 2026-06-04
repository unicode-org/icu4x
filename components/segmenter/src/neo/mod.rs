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
    ignore_complex_until: usize,
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
            ignore_complex_until: usize::MAX,
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
            ignore_complex_until: Y::offset(&input),
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

        let mut last_complex_break = None;

        (self.remaining_input, self.last_accepting_status) = loop {
            let (class, is_complex) = if let Some((_, next)) = iter.clone().peekable().next() {
                let cp = next.into();
                (
                    self.tailoring.class(&self.data.classes, cp),
                    self.complex
                        .as_ref()
                        .map(|c| c.data.handles(cp))
                        .unwrap_or_default(),
                )
            } else {
                (SegmenterStateMachine::EOT_CLASS, false)
            };

            if Y::CAN_CONTAIN_SA
                && self.cache.is_empty()
                && is_complex
                && Y::offset(&iter) >= self.ignore_complex_until
            {
                #[allow(clippy::unwrap_used)] // is_complex implies self.complex is Some
                let complex = self.complex.as_ref().unwrap();

                let mut past_complex = iter.clone();
                past_complex.next();
                while past_complex
                    .clone()
                    .peekable()
                    .next_if(|&(_, c)| complex.data.handles(c.into()))
                    .is_some()
                {
                    past_complex.next();
                }

                let results = (complex.handler)(&complex.data, &iter, &past_complex);

                let offset = Y::offset(&iter);
                self.cache = results.into_iter().map(|i| i + offset).collect();

                if complex.break_at_boundaries {
                    self.remaining_input = past_complex;
                    self.last_accepting_status = complex.break_status;
                    return if offset == 0 {
                        self.cache.pop_front()
                    } else {
                        Some(offset)
                    };
                }

                // ignore the break point at the end – it might not be one and we'll run the state
                // machine from the penultimate break point to figure that out
                self.cache.pop_back();

                // Don't reenter the complex path when restarting from the penumltiate break point.
                // This might produce complex breaks that hadn't been produced before.
                self.ignore_complex_until = Y::offset(&past_complex);

                if let Some(&last_break) = self.cache.back() {
                    let mut at_last_break = iter.clone();
                    while at_last_break
                        .clone()
                        .peekable()
                        .next_if(|&(i, _)| i < last_break)
                        .is_some()
                    {
                        at_last_break.next();
                    }
                    last_complex_break = Some((at_last_break, complex.break_status));
                }

                // keep running the state machine to let it determine whether the start of the complex
                // segment is a break
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

            if let Some(lookahead) = lookahead
                && let Some(p) = self.lookahead_positions.get_mut(usize::from(lookahead))
            {
                *p = Some(iter.clone())
            }
        };

        let break_index = Y::offset(&self.remaining_input);

        // We encountered complex text and populated the cache
        if let Some((last_complex_break, status)) = last_complex_break {
            self.remaining_input = last_complex_break;
            // return the complex break if it's before the break we calculated using the state machine
            if self.cache.front().is_some_and(|&i| i <= break_index) {
                self.last_accepting_status = status;
                return self.cache.pop_front();
            }
        }

        Some(break_index)
    }
}
