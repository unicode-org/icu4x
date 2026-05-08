// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental reimplementations

use crate::complex::ComplexPayloadsBorrowed;
use crate::provider::{Acceptance, Class, RuleBreakDataOverride, SegmenterStateMachine};
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

/// TODO
pub trait Tailoring: crate::private::Sealed {
    #[doc(hidden)]
    fn class(&self, data: &CodePointTrie<Class>, cp: u32) -> Class;
}

impl crate::private::Sealed for () {}
impl Tailoring for () {
    fn class(&self, data: &CodePointTrie<Class>, cp: u32) -> Class {
        data.get32(cp)
    }
}

impl crate::private::Sealed for Option<&'_ RuleBreakDataOverride<'_>> {}
impl Tailoring for Option<&'_ RuleBreakDataOverride<'_>> {
    fn class(&self, data: &CodePointTrie<Class>, cp: u32) -> Class {
        if let Some(tailoring) = self {
            let c = tailoring.property_table_override.get32(cp);
            if c != SegmenterStateMachine::EOT_CLASS {
                return c;
            }
        }

        data.get32(cp)
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
pub struct NeoIterator<'data, 's, Y: RuleBreakType, T: Tailoring> {
    data: &'data SegmenterStateMachine<'data>,
    tailoring: T,
    complex: Option<ComplexPayloadsBorrowed<'data>>,
    cache: VecDeque<usize>,
    remaining_input: Y::IterAttr<'s>,
    last_accepting_status: u8,
    // returns a list of break points, whether the start/end are considered breaks, and their status
    #[allow(clippy::type_complexity)]
    handle_complex:
        fn(&ComplexPayloadsBorrowed, &Y::IterAttr<'s>, &Y::IterAttr<'s>) -> (Vec<usize>, bool, u8),
}

impl<'s, Y: RuleBreakType, T: Tailoring> Iterator for NeoIterator<'_, 's, Y, T> {
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
        let mut lookahead_positions: Vec<Option<Y::IterAttr<'s>>> =
            alloc::vec![None; self.data.num_lookaheads];

        let mut last_complex_break = None;

        (self.remaining_input, self.last_accepting_status) = loop {
            let (class, is_complex) = if let Some((_, next)) = iter.clone().peekable().next() {
                let cp = next.into();
                (
                    self.tailoring.class(&self.data.classes, cp),
                    self.complex
                        .as_ref()
                        .map(|c| c.handles(cp))
                        .unwrap_or_default(),
                )
            } else {
                (SegmenterStateMachine::EOT_CLASS, false)
            };

            if Y::CAN_CONTAIN_SA && self.cache.is_empty() && is_complex {
                #[allow(clippy::unwrap_used)] // is_complex implies self.complex is Some
                let complex = self.complex.as_ref().unwrap();

                let mut past_complex = iter.clone();
                past_complex.next();
                while past_complex
                    .clone()
                    .peekable()
                    .next_if(|&(_, c)| complex.handles(c.into()))
                    .is_some()
                {
                    past_complex.next();
                }

                let (results, break_at_boundaries, status) =
                    (self.handle_complex)(complex, &iter, &past_complex);

                let offset = Y::offset(&iter);
                self.cache = results.into_iter().map(|i| i + offset).collect();

                if break_at_boundaries {
                    self.remaining_input = past_complex;
                    self.last_accepting_status = status;
                    return if offset == 0 {
                        self.cache.pop_front()
                    } else {
                        Some(offset)
                    };
                }

                // ignore the break point at the end – it might not be one and we'll run the state
                // machine from the penultimate break point to figure that out
                self.cache.pop_back();

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
                    last_complex_break = Some((at_last_break, status));
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
                    if let Some(Some(last)) = &lookahead_positions.get(usize::from(l)) {
                        // Lookahead hit, the break point is the last position for `l`
                        break (last.clone(), status);
                    }
                }
            }

            if let Some(lookahead) = lookahead {
                if let Some(p) = lookahead_positions.get_mut(usize::from(lookahead)) {
                    *p = Some(iter.clone())
                };
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
