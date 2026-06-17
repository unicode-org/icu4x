// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Experimental reimplementations

use crate::complex::ComplexIterator;
use crate::provider::{Acceptance, SegmenterStateMachine, SegmenterStateMachineOverride, Symbol};
use crate::scaffold::RuleBreakType;
use smallvec::SmallVec;

mod line;
pub use line::*;
mod grapheme;
pub use grapheme::*;
mod sentence;
pub use sentence::*;
mod word;
pub use word::*;

pub(crate) trait ComplexHandler<Y: RuleBreakType> {
    const BREAK_STATUS: u8;
    const BREAK_AT_BOUNDARIES: bool;
    type Cache: smallvec::Array<Item = usize>;
    type Data<'s>: core::fmt::Debug;

    fn resolve_symbol(symbol: Symbol) -> Symbol;

    fn handle<'data, 's>(
        symbol: Symbol,
        dfa: &RuleBreakIterator<'_, '_, Y, Self>,
        data: &Self::Data<'data>,
        iter: Y::IterAttr<'s>,
    ) -> Option<(ComplexIterator<'data, 's, Y>, Y::IterAttr<'s>)>;
}

#[derive(Debug)]
struct NoComplexHandler;
impl<Y: RuleBreakType> ComplexHandler<Y> for NoComplexHandler {
    const BREAK_STATUS: u8 = 0;
    const BREAK_AT_BOUNDARIES: bool = false;
    type Cache = [usize; 1];
    type Data<'s> = core::convert::Infallible;

    fn resolve_symbol(symbol: Symbol) -> Symbol {
        symbol
    }

    fn handle<'data, 's>(
        _: Symbol,
        _: &RuleBreakIterator<'_, '_, Y, Self>,
        &data: &Self::Data<'data>,
        _: Y::IterAttr<'s>,
    ) -> Option<(ComplexIterator<'data, 's, Y>, Y::IterAttr<'s>)> {
        match data {}
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
pub(crate) struct RuleBreakIterator<'data, 's, Y: RuleBreakType, C: ComplexHandler<Y> + ?Sized> {
    data: &'data SegmenterStateMachine<'data>,
    pseudo_symbol_map: &'data zerovec::ZeroVec<'data, u8>,
    // We use `IntoIter` so that we can pop from the front in O(1) time.
    cache: smallvec::IntoIter<C::Cache>,
    lookahead_positions: SmallVec<[Option<Y::IterAttr<'s>>; 1]>,
    remaining_input: Y::IterAttr<'s>,
    last_accepting_status: u8,
    complex: Option<C::Data<'data>>,
}

#[test]
fn test_lookahead_positions_stays_on_stack() {
    use crate::provider::Baked;

    for &SegmenterStateMachine { num_lookaheads, .. } in [
        Baked::SINGLETON_SEGMENTER_BREAK_LINE_V2,
        Baked::SINGLETON_SEGMENTER_BREAK_GRAPHEME_CLUSTER_V2,
        Baked::SINGLETON_SEGMENTER_BREAK_SENTENCE_V2,
        Baked::SINGLETON_SEGMENTER_BREAK_WORD_V2,
    ] {
        assert!(num_lookaheads <= 1, "{num_lookaheads}");
    }
}

impl<'data, 's, Y: RuleBreakType, C: ComplexHandler<Y>> RuleBreakIterator<'data, 's, Y, C> {
    pub(crate) fn new(
        input: Y::IterAttr<'s>,
        data: &'data SegmenterStateMachine<'data>,
        tailoring: Option<&'data SegmenterStateMachineOverride<'data>>,
        complex: Option<C::Data<'data>>,
    ) -> Self
    where
        Y: RuleBreakType,
        C: ComplexHandler<Y>,
    {
        Self {
            data,
            pseudo_symbol_map: tailoring
                .map(|t| &t.pseudo_symbol_map)
                .unwrap_or(&data.pseudo_symbol_map),
            complex,
            cache: SmallVec::from_elem(0, 1).into_iter(),
            lookahead_positions: SmallVec::from_elem(None, data.num_lookaheads),
            last_accepting_status: 0,
            remaining_input: input,
        }
    }
}

impl<'s, Y: RuleBreakType, C: ComplexHandler<Y>> Iterator for RuleBreakIterator<'_, 's, Y, C> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(i) = self.cache.next() {
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
            let mut symbol = if let Some((_, next)) = iter.clone().next() {
                self.symbol(next.into())
            } else {
                SegmenterStateMachine::EOT_SYMBOL
            };

            if let Some((complex_breaks, end_of_complex)) = self
                .complex
                .as_ref()
                .filter(|_| complex_state.is_none())
                .map(|data| C::handle(symbol, self, data, iter.clone()))
                .unwrap_or(None)
            {
                self.cache = complex_breaks.collect::<SmallVec<_>>().into_iter();

                if C::BREAK_AT_BOUNDARIES {
                    // `self.cache` contains a break point at the end of the run, but not at the start.
                    // Store the position of the end of the run, and return the current position
                    // for the start break point (unless it's 0, which we already returned earlier).
                    self.remaining_input = end_of_complex;
                    self.last_accepting_status = C::BREAK_STATUS;
                    return if Y::offset(&iter) == 0 {
                        self.cache.next()
                    } else {
                        Some(Y::offset(&iter))
                    };
                } else {
                    // Remove the break point at the end of the run, and store `end_of_complex`, the location
                    // of the last complex code point of the run. We'll later restart the state machine
                    // from this code point, in order to correctly break after it (the state machine will
                    // treat it as Alphabetic).
                    self.cache.next_back();
                    complex_state = Some(end_of_complex);

                    // We keep running the state machine to figure out if there's a break point at the start.
                }
            }

            // Resolve the potentially complex symbol to an actual symbol.
            symbol = C::resolve_symbol(symbol);

            iter.next();

            if let Some(next_state) = self
                .data
                .transitions
                .get(usize::from(state) + usize::from(symbol) * self.data.states.len())
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
        if let Some(&first_complex_break) = self.cache.as_slice().first()
            && let Some(last_complex_cp) = complex_state
        {
            self.remaining_input = last_complex_cp;
            // return the complex break if it's before the break we calculated using the state machine
            if first_complex_break < break_index {
                self.last_accepting_status = C::BREAK_STATUS;
                return self.cache.next();
            }
        }

        Some(break_index)
    }
}

impl<'data, 's, Y: RuleBreakType, C: ComplexHandler<Y>> RuleBreakIterator<'data, 's, Y, C> {
    fn symbol(&self, cp: u32) -> u8 {
        let pseudo_symbol = self.data.symbols.get32(cp);
        if let Some(i) = pseudo_symbol.checked_sub(self.data.pseudo_symbol_shift) {
            self.pseudo_symbol_map
                .get(i as usize)
                .unwrap_or(pseudo_symbol)
        } else {
            pseudo_symbol
        }
    }
}
