// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![allow(non_camel_case_types, non_snake_case)]

use zerotrie::ZeroTrie;
use zerotrie::ZeroTrieExtendedCapacity;
use zerotrie::ZeroTriePerfectHash;
use zerotrie::ZeroTrieSimpleAscii;
use zerovec::ZeroVec;

#[cfg_attr(feature = "yoke", derive(yoke::Yokeable))]
#[cfg_attr(feature = "zerofrom", derive(zerofrom::ZeroFrom))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "databake", derive(databake::Bake), databake(path = crate))]
struct DeriveTest_ZeroTrie_ZeroVec<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    _data: ZeroTrie<ZeroVec<'data, u8>>,
}

#[test]
#[cfg(all(feature = "databake", feature = "alloc"))]
fn bake_ZeroTrie_ZeroVec() {
    use databake::*;
    extern crate std;
    test_bake!(
        DeriveTest_ZeroTrie_ZeroVec<'static>,
        crate::DeriveTest_ZeroTrie_ZeroVec {
            _data: zerotrie::ZeroTrieSimpleAscii {
                store: zerovec::ZeroVec::new(),
            }
            .into_zerotrie()
        },
    );
}

#[cfg_attr(feature = "yoke", derive(yoke::Yokeable))]
#[cfg_attr(feature = "zerofrom", derive(zerofrom::ZeroFrom))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "databake", derive(databake::Bake), databake(path = crate))]
struct DeriveTest_ZeroTrieSimpleAscii_ZeroVec<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    _data: ZeroTrieSimpleAscii<ZeroVec<'data, u8>>,
}

#[test]
#[ignore] // https://github.com/rust-lang/rust/issues/98906
#[cfg(all(feature = "databake", feature = "alloc"))]
fn bake_ZeroTrieSimpleAscii_ZeroVec() {
    use databake::*;
    extern crate std;
    test_bake!(
        DeriveTest_ZeroTrieSimpleAscii_ZeroVec<'static>,
        crate::DeriveTest_ZeroTrieSimpleAscii_ZeroVec {
            _data: zerotrie::ZeroTrieSimpleAscii {
                store: zerovec::ZeroVec::new(),
            }
        },
    );
}

#[cfg_attr(feature = "yoke", derive(yoke::Yokeable))]
#[cfg_attr(feature = "zerofrom", derive(zerofrom::ZeroFrom))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "databake", derive(databake::Bake), databake(path = crate))]
struct DeriveTest_ZeroTriePerfectHash_ZeroVec<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    _data: ZeroTriePerfectHash<ZeroVec<'data, u8>>,
}

#[cfg_attr(feature = "yoke", derive(yoke::Yokeable))]
#[cfg_attr(feature = "zerofrom", derive(zerofrom::ZeroFrom))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "databake", derive(databake::Bake), databake(path = crate))]
struct DeriveTest_ZeroTrieExtendedCapacity_ZeroVec<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    _data: ZeroTrieExtendedCapacity<ZeroVec<'data, u8>>,
}
