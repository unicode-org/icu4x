// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::Language;
use icu_collections::codepointtrie::CodePointTrie;
use icu_provider::prelude::*;
use zerovec::ZeroVec;

pub type State = u8;
pub type Symbol = u8;
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

#[derive(Debug, yoke::Yokeable, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_segmenter::provider))]
pub struct SegmenterStateMachine<'data> {
    /// A map from Unicode scalar values to their DFA symbol.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub symbols: CodePointTrie<'data, Symbol>,
    /// A dense map of DFA states.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub states: ZeroVec<'data, (Acceptance, Option<Lookahead>)>,
    /// A dense map of DFA transitions, indexed by `symbol * states.len() + state`
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub transitions: ZeroVec<'data, State>,
    /// The number of lookaheads, used to size the `lookahead_positions` vector.
    pub num_lookaheads: usize,
    /// The offset for the pseudo symbols. If the `symbols` trie returns a value larger than this,
    /// it is a pseudo symbol and needs to be looked up in `pseudo_symbol_map`.
    pub pseudo_symbol_shift: u8,
    /// The map from pseudo symbols (symbols `c` where `c > pseudo_symbol_shift`) to their
    /// actual symbol values and complex language.
    ///
    /// Dense linear map, indexed by `c - pseudo_symbol_shift`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pseudo_symbol_map: ZeroVec<'data, (Symbol, Language)>,
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

    /// The end-of-text symbol. This is a dummy symbol that only appears at the end of the input,
    /// and allows the state machine to have special transitions on end-of-text.
    pub const EOT_SYMBOL: Symbol = 0;
}

/// A tailoring for [`SegmenterStateMachine`].
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize,databake::Bake),
    databake(path = icu_segmenter::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SegmenterStateMachineOverride<'data> {
    /// See [`SegmenterStateMachine::pseudo_symbol_map`].
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub pseudo_symbol_map: ZeroVec<'data, (Symbol, Language)>,
}

icu_provider::data_struct!(
    SegmenterStateMachineOverride<'_>,
    #[cfg(feature = "datagen")]
);

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
    SegmenterStateMachineOverride<'static>,
    #[cfg(feature = "datagen")]
    expose_baked_consts = true,
);

icu_provider::data_marker!(
    /// `SegmenterBreakSentenceOverrideV2`
    SegmenterBreakSentenceOverrideV2,
    "segmenter/break/sentence/override/v2",
    SegmenterStateMachineOverride<'static>,
);
