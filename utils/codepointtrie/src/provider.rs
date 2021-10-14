// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::yoke::{self, Yokeable, ZeroCopyFrom};

#[derive(Yokeable, ZeroCopyFrom)]
pub struct UnicodePropertyMapV1<'data, T: TrieValue> {
    pub codepoint_trie: CodePointTrie<'data, T>,
}

pub struct UnicodePropertyMapV1Marker<T: TrieValue> {
    _phantom: core::marker::PhantomData<T>,
}

impl<'data, T: TrieValue> icu_provider::DataMarker<'data> for UnicodePropertyMapV1Marker<T> {
    type Yokeable = UnicodePropertyMapV1<'static, T>;
    type Cart = UnicodePropertyMapV1<'data, T>;
}
