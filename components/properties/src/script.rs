// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data and APIs for supporting both Script and Script_Extensions property
//! values in an efficient structure.

use crate::error::PropertiesError;
use crate::props::Script;

use icu_codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::yoke::{self, *};
use icu_uniset::{UnicodeSet, UnicodeSetBuilder};
use zerovec::{VarZeroVec, ZeroSlice};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const CODE_POINT_MAX: u32 = 0x10ffff;
const SCRIPT_X_SCRIPT_VAL: u16 = 0x03FF;
const SCRIPT_VAL_LENGTH: u16 = 10;

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

impl ScriptWithExt {
    pub fn is_common(&self) -> bool {
        self.0 >> SCRIPT_VAL_LENGTH == 1
    }

    pub fn is_inherited(&self) -> bool {
        self.0 >> SCRIPT_VAL_LENGTH == 2
    }

    pub fn is_other(&self) -> bool {
        self.0 >> SCRIPT_VAL_LENGTH == 3
    }
}

impl From<ScriptWithExt> for u32 {
    fn from(swe: ScriptWithExt) -> Self {
        swe.0 as u32
    }
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
    extensions: VarZeroVec<'data, ZeroSlice<Script>>,
}

impl<'data> ScriptExtensions<'data> {
    pub fn try_new(
        trie: CodePointTrie<'data, ScriptWithExt>,
        extensions: VarZeroVec<'data, ZeroSlice<Script>>,
    ) -> Result<ScriptExtensions<'data>, PropertiesError> {
        // TODO: do validation here

        Ok(ScriptExtensions { trie, extensions })
    }

    pub fn get_script_val(&self, code_point: u32) -> Script {
        let sc_with_ext = self.trie.get(code_point);

        if sc_with_ext.is_other() {
            let ext_idx = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            let scx_val = self.extensions.get(ext_idx as usize);
            let scx_first_sc = scx_val.and_then(|scx| scx.get(0));

            let default_sc_val = <Script as TrieValue>::DATA_GET_ERROR_VALUE;

            scx_first_sc.unwrap_or(default_sc_val)
        } else if sc_with_ext.is_common() {
            Script::Common
        } else if sc_with_ext.is_inherited() {
            Script::Inherited
        } else {
            let script_val = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            Script(script_val)
        }
    }

    pub fn get_script_extensions_val(&self, code_point: u32) -> &ZeroSlice<Script> {
        let sc_with_ext = self.trie.get(code_point);

        if sc_with_ext.is_other() {
            let ext_idx = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            let ext_subarray = self.extensions.get(ext_idx as usize);
            // In the OTHER case, where the 2 higher-order bits of the
            // `ScriptWithExt` value in the trie doesn't indicate the Script value,
            // the Script value is copied/inserted into the first position of the
            // `extensions` array. So we must remove it to return the actual scx array val.
            let scx_slice = ext_subarray
                .and_then(|zslice| zslice.as_ule_slice().get(1..))
                .unwrap_or_default();
            ZeroSlice::from_ule_slice(scx_slice)
        } else if sc_with_ext.is_common() || sc_with_ext.is_inherited() {
            let ext_idx = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            let scx_val = self.extensions.get(ext_idx as usize);
            scx_val.unwrap_or_default()
        } else {
            let script_with_ext_ule = self.trie.get_ule(code_point);
            let script_ule_slice = script_with_ext_ule
                .map(|swe| core::slice::from_ref(swe))
                .unwrap_or_default();
            ZeroSlice::from_ule_slice(script_ule_slice)
        }
    }

    pub fn has_script(&self, code_point: u32, script: Script) -> bool {
        if script == self.get_script_val(code_point) {
            true
        } else {
            let scx_val = self.get_script_extensions_val(code_point);
            let script_find = scx_val.iter().find(|&sc| sc == script);
            script_find.is_some()
        }
    }

    pub fn get_script_extensions_set(&self, script: Script) -> UnicodeSet {
        let mut builder = UnicodeSetBuilder::new();
        let mut range_start: u32 = 0;

        loop {
            let next_cpm_range = self.trie.get_range(range_start);

            match next_cpm_range {
                Some(cpm_range) => {
                    let end = cpm_range.get_end();
                    if end <= CODE_POINT_MAX {
                        let start = cpm_range.get_start();

                        if self.has_script(start, script) {
                            builder.add_range_u32(&(start..=end));
                        }

                        if end == CODE_POINT_MAX {
                            break;
                        } else {
                            range_start = end + 1;
                        }
                    } else {
                        break;
                        // This should never happen -- the get_range() code doesn't create
                        // a range with an end code point beyond CODE_POINT_MAX. Also, it will
                        // return `None` in the event of errors.
                    }
                }
                _ => {
                    break;
                }
            }
        }

        builder.build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_common() {
        assert!(ScriptWithExt(0x04FF).is_common());
        assert!(ScriptWithExt(0x0400).is_common());

        assert!(!ScriptWithExt(0x08FF).is_common());
        assert!(!ScriptWithExt(0x0800).is_common());

        assert!(!ScriptWithExt(0x0CFF).is_common());
        assert!(!ScriptWithExt(0x0C00).is_common());

        assert!(!ScriptWithExt(0xFF).is_common());
        assert!(!ScriptWithExt(0x0).is_common());
    }

    #[test]
    fn test_is_inherited() {
        assert!(!ScriptWithExt(0x04FF).is_inherited());
        assert!(!ScriptWithExt(0x0400).is_inherited());

        assert!(ScriptWithExt(0x08FF).is_inherited());
        assert!(ScriptWithExt(0x0800).is_inherited());

        assert!(!ScriptWithExt(0x0CFF).is_inherited());
        assert!(!ScriptWithExt(0x0C00).is_inherited());

        assert!(!ScriptWithExt(0xFF).is_inherited());
        assert!(!ScriptWithExt(0x0).is_inherited());
    }

    #[test]
    fn test_is_other() {
        assert!(!ScriptWithExt(0x04FF).is_other());
        assert!(!ScriptWithExt(0x0400).is_other());

        assert!(!ScriptWithExt(0x08FF).is_other());
        assert!(!ScriptWithExt(0x0800).is_other());

        assert!(ScriptWithExt(0x0CFF).is_other());
        assert!(ScriptWithExt(0x0C00).is_other());

        assert!(!ScriptWithExt(0xFF).is_other());
        assert!(!ScriptWithExt(0x0).is_other());
    }
}
