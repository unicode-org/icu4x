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
    // Complex classes
    pub complex_classes: ZeroVec<'data, Class>,
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
