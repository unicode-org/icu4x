// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental reimplementations

use crate::provider::{Acceptance, Class, SegmenterStateMachine, SegmenterStateMachineOverride};
use crate::scaffold::RuleBreakType;
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

impl Tailoring for Option<&'_ SegmenterStateMachineOverride<'_>> {
    fn class(&self, data: &CodePointTrie<Class>, cp: u32) -> Class {
        if let Some(tailoring) = self {
            let c = tailoring.classes.get32(cp);
            if c != SegmenterStateMachine::NO_CLASS {
                return c;
            }
        }

        data.get32(cp)
    }
}

pub(crate) trait ComplexHandler<Y: RuleBreakType> {
    const BREAK_STATUS: u8;
    const BREAK_AT_BOUNDARIES: bool;
    type Data<'s>: core::fmt::Debug;

    fn is_complex(data: &Self::Data<'_>, iter: &Y::IterAttr<'_>) -> bool;
    fn handle<'s>(
        _: &Self::Data<'_>,
        _: &Y::IterAttr<'s>,
        _: &Y::IterAttr<'s>,
    ) -> impl Iterator<Item = usize> + use<'s, Self, Y>;
}

#[derive(Debug)]
struct NoComplexHandler;
impl<Y: RuleBreakType> ComplexHandler<Y> for NoComplexHandler {
    const BREAK_STATUS: u8 = 0;
    const BREAK_AT_BOUNDARIES: bool = false;
    type Data<'s> = core::convert::Infallible;

    fn is_complex(&data: &Self::Data<'_>, _iter: &Y::IterAttr<'_>) -> bool {
        match data {}
    }

    fn handle<'s>(
        &data: &Self::Data<'_>,
        _: &<Y as RuleBreakType>::IterAttr<'s>,
        _: &<Y as RuleBreakType>::IterAttr<'s>,
    ) -> impl Iterator<Item = usize> + use<'s, Y> {
        match data {}
        #[allow(unreachable_code)] // ! does not impl Iterator
        core::iter::empty()
    }
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
pub(crate) struct RuleBreakIterator<'data, 's, Y: RuleBreakType, T: Tailoring, C: ComplexHandler<Y>>
{
    data: &'data SegmenterStateMachine<'data>,
    tailoring: T,
    cache: VecDeque<usize>,
    lookahead_positions: Vec<Option<Y::IterAttr<'s>>>,
    remaining_input: Y::IterAttr<'s>,
    last_accepting_status: u8,
    complex: Option<C::Data<'data>>,
}

impl<'data, 's, Y: RuleBreakType, T: Tailoring, C: ComplexHandler<Y>>
    RuleBreakIterator<'data, 's, Y, T, C>
{
    pub(crate) fn new(
        input: Y::IterAttr<'s>,
        data: &'data SegmenterStateMachine<'data>,
        tailoring: T,
        complex: Option<C::Data<'data>>,
    ) -> Self
    where
        Y: RuleBreakType,
        C: ComplexHandler<Y>,
    {
        Self {
            data,
            tailoring,
            complex,
            cache: VecDeque::from_iter([0]),
            lookahead_positions: alloc::vec![None; data.num_lookaheads],
            last_accepting_status: 0,
            remaining_input: input,
        }
    }
}

impl<'s, Y: RuleBreakType, T: Tailoring, C: ComplexHandler<Y>> Iterator
    for RuleBreakIterator<'_, 's, Y, T, C>
{
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
            let class = if let Some((_, next)) = iter.clone().peekable().next() {
                self.tailoring.class(&self.data.classes, next.into())
            } else {
                SegmenterStateMachine::EOT_CLASS
            };

            // Enter complex handling if:
            // * We haven't already started complex handling and are reentering the loop to
            //   find the alternative non-complex break
            // * We have a complex handler
            // * The current code point is complex
            if complex_state.is_none()
                && let Some(complex) = self.complex.as_ref()
                && C::is_complex(complex, &iter)
            {
                let mut past_complex = iter.clone();
                let mut last_complex = past_complex.clone();
                past_complex.next();
                while C::is_complex(complex, &past_complex) {
                    past_complex.next();
                    last_complex.next();
                }

                let offset = Y::offset(&iter);

                // A complex segment of length 1 doesn't need special handling.
                if Y::offset(&last_complex) != offset {
                    self.cache = C::handle(complex, &iter, &past_complex).collect();

                    if C::BREAK_AT_BOUNDARIES {
                        // `self.cache` contains a break point at the end of the run, but not at the start.
                        // Store the position of the end of the run, and return the current position
                        // for the start break point (unless it's 0, which we already returned earlier).
                        self.remaining_input = past_complex;
                        self.last_accepting_status = C::BREAK_STATUS;
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
                        complex_state = Some(last_complex);

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

            if let Some(lookahead) = lookahead
                && let Some(p) = self.lookahead_positions.get_mut(usize::from(lookahead))
            {
                *p = Some(iter.clone())
            }
        };

        let break_index = Y::offset(&self.remaining_input);

        // We encountered complex text and populated the cache
        if let Some(&first_complex_break) = self.cache.front() {
            if let Some(last_complex_cp) = complex_state {
                self.remaining_input = last_complex_cp;
                // return the complex break if it's before the break we calculated using the state machine
                if first_complex_break < break_index {
                    self.last_accepting_status = C::BREAK_STATUS;
                    return self.cache.pop_front();
                }
            } else {
                debug_assert!(false, "self.cache populated but no complex state");
            }
        }

        Some(break_index)
    }
}
