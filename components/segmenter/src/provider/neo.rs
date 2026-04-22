// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointtrie::CodePointTrie;
use icu_provider::prelude::*;
use zerovec::ZeroVec;

pub type State = u8;
pub type Class = u8;
pub type Lookahead = u8;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
pub enum Acceptance {
    Accept,
    Continue,
    AcceptMandatory,
    Conditional(Lookahead),
}

impl zerovec::ule::AsULE for Acceptance {
    type ULE = u8;

    fn to_unaligned(self) -> Self::ULE {
        match self {
            Self::Accept => 255,
            Self::Continue => 254,
            Self::AcceptMandatory => 253,
            Self::Conditional(n) => n,
        }
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        match unaligned {
            255 => Self::Accept,
            254 => Self::Continue,
            253 => Self::AcceptMandatory,
            n => Self::Conditional(n),
        }
    }
}

#[derive(Debug, yoke::Yokeable, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
pub struct SegmenterStateMachine<'data> {
    // A map from Unicode scalar values to their segmentation classes
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub classes: CodePointTrie<'data, Class>,
    // A dense map of states
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub states: ZeroVec<'data, (Acceptance, Option<Lookahead>)>,
    // A dense map of transitions, indexed by class * states.len() + state
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub transitions: ZeroVec<'data, State>,
    // The number of lookahead classes, used to size the lookahead_positions vector.
    pub num_lookaheads: usize,
}

icu_provider::data_struct!(
    SegmenterStateMachine<'_>,
    #[cfg(feature = "datagen")]
);

impl SegmenterStateMachine<'_> {
    /// The start state
    pub const START_STATE: State = 0;
    /// The trash state. As our transition matrix is dense, we need a state to represent "no transition".
    /// This state is non-accepting and loops to itself on all inputs.
    pub const TRASH_STATE: State = State::MAX;
    /// The end-of-text class. This is a dummy class that only appears at the end of the input,
    /// and allows the state machine to have special transitions on end-of-text.
    pub const EOT_CLASS: Class = 0;
}

icu_provider::data_marker!(
    /// `SegmenterBreakLineV2`
    SegmenterBreakLineV2,
    "segmenter/break/line/v2",
    SegmenterStateMachine<'static>,
    is_singleton = true,
);

#[cfg(test)]
impl SegmenterStateMachine<'_> {
    fn breaks(&self, input: &str) -> impl Iterator<Item = (usize, bool)> {
        let mut remaining_input = input.char_indices();

        core::iter::from_fn(move || {
            if remaining_input.as_str().is_empty() {
                return None;
            }

            let mut state = Self::START_STATE;
            let mut iter = remaining_input.clone();

            // Dummy value, we don't use this until it has been replaced
            let mut last_accepting: core::str::CharIndices<'_> = iter.clone();
            let mut last_accepting_mandatory = false;
            let mut lookahead_positions: Vec<Option<core::str::CharIndices<'_>>> =
                alloc::vec![None; self.num_lookaheads];

            remaining_input = loop {
                let class = if let Some((_, next)) = iter.next() {
                    self.classes.get(next)
                } else {
                    Self::EOT_CLASS
                };

                if let Some(next_state) = self
                    .transitions
                    .get(usize::from(state) + usize::from(class) * self.states.len())
                    .filter(|&s| s != Self::TRASH_STATE)
                {
                    state = next_state;
                } else {
                    // No transition, the break point is the last accepting state
                    break last_accepting;
                }

                let (acceptance, lookahead) = self
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

            Some((
                remaining_input.offset(),
                last_accepting_mandatory || remaining_input.as_str().is_empty(),
            ))
        })
    }
}

#[test]
fn test() {
    use alloc::{vec, vec::Vec};

    let segmenter = crate::provider::Baked::SINGLETON_SEGMENTER_BREAK_LINE_V2;

    let actual_breaks = segmenter
        .breaks("this has a mandatory\nline break")
        .collect::<Vec<_>>();

    assert_eq!(
        actual_breaks,
        [
            (5, false),
            (9, false),
            (11, false),
            (21, true),
            (26, false),
            (31, true)
        ]
    );

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

        let actual_breaks = segmenter
            .breaks(&test_string)
            .map(|(i, _)| i)
            .collect::<Vec<_>>();

        assert_eq!(actual_breaks, expected_breaks, "{line}",);
    }
}
