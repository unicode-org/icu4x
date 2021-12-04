// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data and APIs for supporting both Script and Script_Extensions property
//! values in an efficient structure.

use crate::props::Script;
use icu_codepointtrie::CodePointTrie;
use icu_provider::yoke::{self, *};
use zerovec::{zerovec::ZeroVecULE, VarZeroVec};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// An internal-use only pseudo-property that represents the values stored in
/// the trie of the special data structure [`ScriptExtensions`].
///
/// Note: The will assume a 12-bit layout. The 2 higher order bits in positions
/// 11..10 will indicate how to deduce the Script value and Script_Extensions,
/// and the lower 10 bits 9..0 indicate either the Script value or the index
/// into the `extensions` structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ScriptWithExt(pub u16);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl ScriptWithExt {
    pub const Unknown: ScriptWithExt = ScriptWithExt(0);
}

/// A data structure that represents the data for both Script and
/// Script_Extensions properties in an efficient way. This structure matches
/// the data and data structures that are stored in the corresponding ICU data
/// file for these properties.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Yokeable, ZeroCopyFrom)]
pub struct ScriptExtensions<'data> {
    /// Note: The `ScriptWithExt` values in this array will assume a 12-bit layout. The 2
    /// higher order bits 11..10 will indicate how to deduce the Script value and
    /// Script_Extensions value, nearly matching the representation
    /// [in ICU](https://github.com/unicode-org/icu/blob/main/icu4c/source/common/uprops.h):
    ///
    /// | High order 2 bits value | Script                                                 | Script_Extensions                                              |
    /// |-------------------------|--------------------------------------------------------|----------------------------------------------------------------|
    /// | 3                       | First value in sub-array, index given by lower 10 bits | Sub-array excluding first value, index given by lower 10 bits  |
    /// | 2                       | Script=Inherited                                       | Entire sub-array, index given by lower 10 bits                 |
    /// | 1                       | Script=Common                                          | Entire sub-array, index given by lower 10 bits                 |
    /// | 0                       | Value in lower 10 bits                                 | `[ Script value ]` single-element array                        |
    ///
    /// When the lower 10 bits of the value are used as an index, that index is
    /// used for the outer-level vector of the nested `extensions` structure.
    #[cfg_attr(feature = "serde", serde(borrow))]
    trie: CodePointTrie<'data, ScriptWithExt>,

    /// This companion structure stores Script_Extensions values, which are
    /// themselves arrays / vectors. This structure only stores the values for
    /// cases in which `scx(cp) != [ sc(cp) ]`. Each sub-vector is distinct. The
    /// sub-vector represents the Script_Extensions array value for a code point,
    /// and may also indicate Script value, as described for the `trie` field.
    #[cfg_attr(feature = "serde", serde(borrow))]
    extensions: VarZeroVec<'data, ZeroVecULE<Script>>,
}
