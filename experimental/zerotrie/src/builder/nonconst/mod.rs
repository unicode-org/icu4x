// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod builder;
mod store;

pub(crate) use builder::*;
pub(crate) use store::TrieBuilderStore;

impl<S: ?Sized> crate::ZeroTrieSimpleAscii<S> {
    pub(crate) const BUILDER_OPTIONS: ZeroTrieBuilderOptions = ZeroTrieBuilderOptions {
        phf_mode: PhfMode::BinaryOnly,
        ascii_mode: AsciiMode::AsciiOnly,
        capacity_mode: CapacityMode::Normal,
    };
}

impl<S: ?Sized> crate::ZeroTriePerfectHash<S> {
    pub(crate) const BUILDER_OPTIONS: ZeroTrieBuilderOptions = ZeroTrieBuilderOptions {
        phf_mode: PhfMode::UsePhf,
        ascii_mode: AsciiMode::BinarySpans,
        capacity_mode: CapacityMode::Normal,
    };
}

impl<S: ?Sized> crate::ZeroTrieExtendedCapacity<S> {
    pub(crate) const BUILDER_OPTIONS: ZeroTrieBuilderOptions = ZeroTrieBuilderOptions {
        phf_mode: PhfMode::UsePhf,
        ascii_mode: AsciiMode::BinarySpans,
        capacity_mode: CapacityMode::Extended,
    };
}
