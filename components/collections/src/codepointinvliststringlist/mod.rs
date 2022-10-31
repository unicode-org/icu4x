// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::{String, ToString};
use crate::codepointinvlist::{CodePointInversionList, CodePointInversionListError};
use displaydoc::Display;
use zerovec::{VarZeroVec, ZeroVec};

pub struct CodePointInversionListStringList<'data> {
    cp_inv_list: CodePointInversionList<'data>,
    str_list: VarZeroVec<'data, str>,
}

impl<'data> CodePointInversionListStringList<'data> {
    pub fn try_from_lists(
        cp_list: ZeroVec<'data, u32>,
        str_list: VarZeroVec<'data, str>,
    ) -> Result<Self, CodePointInversionListStringListError> {
        let cp_inv_list = CodePointInversionList::try_from_inversion_list(cp_list)
            .map_err(|cpile| 
                CodePointInversionListStringListError::InvalidCodePointInversionList(cpile))?;

        // Invariants:
        //     - no input string is length 1 (a length 1 string should be a code point)
        //     - no input string is length 0 (?)
        for s in str_list.iter() {
            if s.len() < 2 {
                return Err(CodePointInversionListStringListError::InvalidStringLength(s.to_string()));
            }
        }

        Ok(CodePointInversionListStringList {
            cp_inv_list,
            str_list,
        })
    }

    pub fn from_sorted_lists_unchecked(
        cp_inv_list: CodePointInversionList<'data>,
        str_list: VarZeroVec<'data, str>,
    ) -> Self {
        CodePointInversionListStringList {
            cp_inv_list,
            str_list,
        }
    }

    pub fn size(&self) -> usize {
        self.cp_inv_list.size() + self.str_list.len()
    }

    pub fn has_strings(&self) -> bool {
        !self.str_list.is_empty()
    }
}

/// Custom Errors for [`CodePointInversionListStringList`].
///
/// Re-exported as [`Error`](Error).
#[derive(Display, Debug)]
pub enum CodePointInversionListStringListError {
    /// A CodePointInversionList was constructed
    #[displaydoc("Invalid code point inversion list: {0:?}")]
    InvalidCodePointInversionList(CodePointInversionListError),
    /// A String in the string list had an invalid length
    #[displaydoc("Invalid string length for string: {0}")]
    InvalidStringLength(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_has_strings() {
        let cp_slice = &[0, 1, 0x7F, 0x80, 0xFFFF, 0x1_0000, 0x10_FFFF, 0x11_0000];
        let cp_list = ZeroVec::alloc_from_slice(cp_slice);
        let str_slice = &["zero", "ascii_max", "bmp_max", "unicode_max"];
        let str_list = VarZeroVec::<str>::from(str_slice);

        let cpilsl = CodePointInversionListStringList::try_from_lists(cp_list, str_list).unwrap();

        assert!(cpilsl.has_strings());
        assert_eq!(8, cpilsl.size());
    }

    #[test]
    fn test_invalid_string() {
        let cp_slice = &[0, 1, 0x7F, 0x80, 0xFFFF, 0x1_0000, 0x10_FFFF, 0x11_0000];
        let cp_list = ZeroVec::alloc_from_slice(cp_slice);
        let str_slice = &[""];
        let str_list = VarZeroVec::<str>::from(str_slice);

        let cpilsl = CodePointInversionListStringList::try_from_lists(cp_list, str_list);

        assert!(matches!(cpilsl, Err(CodePointInversionListStringListError::InvalidStringLength(_))));
    }
}
