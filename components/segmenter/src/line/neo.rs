// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::{Acceptance, SegmenterBreakLineV2, SegmenterStateMachine};
use alloc::vec::Vec;
use core::str::CharIndices;
use icu_provider::prelude::*;

#[derive(Debug)]
pub struct LineSegmenter {
    data: DataPayload<SegmenterBreakLineV2>,
}

impl LineSegmenter {
    pub fn new() -> Self {
        Self {
            data: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_SEGMENTER_BREAK_LINE_V2,
            ),
        }
    }

    pub fn segment_str<'a>(&'a self, s: &'a str) -> LineSegmenterIterator<'a> {
        LineSegmenterIterator {
            data: self.data.get(),
            remaining_input: s.char_indices(),
            last_accepting_mandatory: false,
        }
    }
}

#[derive(Debug)]
pub struct LineSegmenterIterator<'a> {
    data: &'a SegmenterStateMachine<'a>,
    remaining_input: CharIndices<'a>,
    last_accepting_mandatory: bool,
}

impl Iterator for LineSegmenterIterator<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining_input.as_str().is_empty() {
            return None;
        }

        let mut state = SegmenterStateMachine::START_STATE;
        let mut iter = self.remaining_input.clone();

        // Dummy value, we don't use this until it has been replaced
        let mut last_accepting: CharIndices<'_> = iter.clone();
        let mut last_accepting_mandatory = false;
        let mut lookahead_positions: Vec<Option<CharIndices<'_>>> =
            alloc::vec![None; self.data.num_lookaheads];

        self.remaining_input = loop {
            let class = if let Some((_, next)) = iter.next() {
                self.data.classes.get(next)
            } else {
                SegmenterStateMachine::EOT_CLASS
            };

            if self
                .data
                .complex_classes
                .iter()
                .find(|&c| c == class)
                .is_some()
            {
                // TODO
            }

            if let Some(next_state) = self
                .data
                .transitions
                .get(usize::from(state) + usize::from(class) * self.data.states.len())
                .filter(|&s| s != SegmenterStateMachine::TRASH_STATE)
            {
                state = next_state;
            } else {
                // No transition, the break point is the last accepting state
                break last_accepting;
            }

            let (acceptance, lookahead) = self
                .data
                .states
                .get(usize::from(state))
                // GIGO
                .unwrap_or((Acceptance::Continue, None));

            match acceptance {
                Acceptance::Accept => {
                    last_accepting = iter.clone();
                    last_accepting_mandatory = false;
                }
                Acceptance::AcceptMandatory => {
                    last_accepting = iter.clone();
                    last_accepting_mandatory = true;
                }
                Acceptance::Continue => (),
                Acceptance::Conditional(l) => {
                    if let Some(last) = &lookahead_positions[usize::from(l)] {
                        // Lookahead hit, the break point is the last position for `l`
                        break last.clone();
                    }
                }
            }

            if let Some(lookahead) = lookahead {
                lookahead_positions[usize::from(lookahead)] = Some(iter.clone());
            }
        };

        self.last_accepting_mandatory =
            last_accepting_mandatory || self.remaining_input.as_str().is_empty();

        Some(self.remaining_input.offset())
    }
}

impl LineSegmenterIterator<'_> {
    pub fn is_mandatory(&self) -> bool {
        self.last_accepting_mandatory
    }
}

#[test]
fn test() {
    use alloc::{vec, vec::Vec};

    let segmenter = LineSegmenter::new();

    let mut actual_breaks = segmenter.segment_str("this has a mandatory\nline break");

    assert_eq!(actual_breaks.next(), Some(5));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(9));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(11));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(21));
    assert_eq!(actual_breaks.is_mandatory(), true);
    assert_eq!(actual_breaks.next(), Some(26));
    assert_eq!(actual_breaks.is_mandatory(), false);
    assert_eq!(actual_breaks.next(), Some(31));
    assert_eq!(actual_breaks.is_mandatory(), true);
    assert_eq!(actual_breaks.next(), None);

    for line in include_str!("../../tests/testdata/LineBreakTest.txt").lines() {
        let line = line.split('#').next().unwrap().trim();
        if line.is_empty() {
            continue;
        }

        let mut test_string = String::new();
        let mut expected_breaks = vec![];
        for s in line.split_ascii_whitespace() {
            match s {
                "×" => (),
                "÷" => expected_breaks.push(test_string.len()),
                s => {
                    test_string
                        .push(char::try_from(u32::from_str_radix(s.trim(), 16).unwrap()).unwrap());
                }
            }
        }

        let actual_breaks = segmenter.segment_str(&test_string).collect::<Vec<_>>();

        assert_eq!(actual_breaks, expected_breaks, "{line}",);
    }
}
