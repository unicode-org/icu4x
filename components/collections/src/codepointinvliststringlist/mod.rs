// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::codepointinvlist::{CodePointInversionList, CodePointInversionListError};
use zerovec::{VarZeroVec, ZeroVec};

pub struct CodePointInversionListStringList<'data> {
    cp_inv_list: CodePointInversionList<'data>,
    str_list: VarZeroVec<'data, str>,
}

impl<'data> CodePointInversionListStringList<'data> {
    pub fn try_from_lists(
        cp_list: ZeroVec<'data, u32>,
        str_list: VarZeroVec<'data, str>,
    ) -> Result<Self, CodePointInversionListError> {
        let cp_inv_list = CodePointInversionList::try_from_inversion_list(cp_list)?;
        Ok(CodePointInversionListStringList {
            cp_inv_list,
            str_list,
        })
    }

    pub fn size(&self) -> usize {
        self.cp_inv_list.size() + self.str_list.len()
    }

    pub fn has_strings(&self) -> bool {
        !self.str_list.is_empty()
    }
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
}
