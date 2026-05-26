// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::RuleBreakDataOverride;
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
    Continue,
    Accept(u8),
    Conditional(Lookahead, u8),
}

impl zerovec::ule::AsULE for Acceptance {
    type ULE = u8;

    fn to_unaligned(self) -> Self::ULE {
        match self {
            Self::Continue => 0b11111 << 3 | 0b111,
            Self::Accept(status) => 0b11111 << 3 | (status & 0b111),
            Self::Conditional(n, status) => n << 3 | (status & 0b111),
        }
    }

    fn from_unaligned(unaligned: Self::ULE) -> Self {
        match (unaligned >> 3, unaligned & 0b111) {
            (0b11111, 0b111) => Self::Continue,
            (0b11111, status) => Self::Accept(status),
            (n, status) => Self::Conditional(n, status),
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
    /// This is used as the absence of a class in overrides.
    pub const NO_CLASS: Class = 255;
}

icu_provider::data_marker!(
    /// `SegmenterBreakLineV2`
    SegmenterBreakLineV2,
    "segmenter/break/line/v2",
    SegmenterStateMachine<'static>,
    is_singleton = true,
);

icu_provider::data_marker!(
    /// `SegmenterBreakWordV2`
    SegmenterBreakWordV2,
    "segmenter/break/word/v2",
    SegmenterStateMachine<'static>,
    is_singleton = true,
);

icu_provider::data_marker!(
    /// `SegmenterBreakGraphemeClusterV2`
    SegmenterBreakGraphemeClusterV2,
    "segmenter/break/grapheme/cluster/v2",
    SegmenterStateMachine<'static>,
    is_singleton = true,
);

icu_provider::data_marker!(
    /// `SegmenterBreakSentenceV2`
    SegmenterBreakSentenceV2,
    "segmenter/break/sentence/v2",
    SegmenterStateMachine<'static>,
    is_singleton = true,
);

icu_provider::data_marker!(
    /// `SegmenterBreakLineOverrideV2`
    SegmenterBreakLineOverrideV2,
    "segmenter/break/line/override/v2",
    RuleBreakDataOverride<'static>,
);

icu_provider::data_marker!(
    /// `SegmenterBreakSentenceOverrideV2`
    SegmenterBreakSentenceOverrideV2,
    "segmenter/break/sentence/override/v2",
    RuleBreakDataOverride<'static>,
);
