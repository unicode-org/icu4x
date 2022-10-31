// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::codepointinvlist::{CodePointInversionList, CodePointInversionListError};
use alloc::string::{String, ToString};
use displaydoc::Display;
use yoke::Yokeable;
use zerofrom::ZeroFrom;
use zerovec::{ule::AsULE, VarZeroVec, ZeroVec};

#[derive(Debug, Eq, PartialEq, Clone, Yokeable, ZeroFrom)]
pub struct CodePointInversionListStringList<'data> {
    cp_inv_list: CodePointInversionList<'data>,
    str_list: VarZeroVec<'data, str>,
}

impl<'data> CodePointInversionListStringList<'data> {
    pub fn try_from(
        cp_inv_list: CodePointInversionList<'data>,
        str_list: VarZeroVec<'data, str>,
    ) -> Result<Self, CodePointInversionListStringListError> {
        // Invariants:
        //   - no input string is length 1 (a length 1 string should be a code point)
        //   - the list is sorted
        //   - the elements in the list are unique

        // Verify invariants:
        // Do so by using the equivalent of str_list.iter().windows(2) to get
        // overlapping windows of size 2. The above putative code is not possible
        // because `.windows()` exists on a slice, but VarZeroVec cannot return a slice
        // because the non-fixed size elements necessitate at least some type
        // of allocation.
        {
            let mut it = str_list.iter();
            if let Some(mut x) = it.next() {
                if x.len() == 1 {
                    return Err(CodePointInversionListStringListError::InvalidStringLength(
                        x.to_string(),
                    ));
                }
                while let Some(y) = it.next() {
                    if x.len() == 1 {
                        return Err(CodePointInversionListStringListError::InvalidStringLength(
                            x.to_string(),
                        ));
                    } else if x == y {
                        return Err(CodePointInversionListStringListError::StringListNotUnique(
                            x.to_string(),
                        ));
                    } else if x > y {
                        return Err(CodePointInversionListStringListError::StringListNotSorted(
                            x.to_string(),
                            y.to_string(),
                        ));
                    }

                    // Next window begins. Update `x` here, `y` will be updated in next loop iteration.
                    x = y;
                }
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

    ///
    /// # Examples
    /// ```
    /// use icu_collections::codepointinvlist::CodePointInversionList;
    /// use icu_collections::codepointinvliststringlist::CodePointInversionListStringList;
    /// use zerovec::VarZeroVec;
    /// 
    /// let cp_slice = &[0, 0x0080, 0xFFFF, 0x1_0000, 0x10_FFFF, 0x11_0000];
    /// let cp_list =
    ///    CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
    /// let str_slice = &["", "ascii_max", "bmp_max", "unicode_max", "zero"];
    /// let str_list = VarZeroVec::<str>::from(str_slice);
    /// 
    /// let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list).unwrap();
    ///
    /// assert!(cpilsl.contains("ascii_max"));
    /// assert!(cpilsl.contains(""));
    /// assert!(cpilsl.contains("A"));
    /// assert!(!cpilsl.contains("bazinga!"));
    /// ```
    pub fn contains(&self, s: &str) -> bool {
        if s.len() == 1 {
            if let Some(ch) = s.chars().next() {
                self.contains_char(ch)
            } else {
                false
            }
        } else {
            self.str_list.iter().filter(|&x| x == s).next().is_some()
        }
    }

    ///
    /// # Examples
    /// ```
    /// use icu_collections::codepointinvlist::CodePointInversionList;
    /// use icu_collections::codepointinvliststringlist::CodePointInversionListStringList;
    /// use zerovec::VarZeroVec;
    /// 
    /// let cp_slice = &[0, 0x80, 0xFFFF, 0x1_0000, 0x10_FFFF, 0x11_0000];
    /// let cp_list =
    ///     CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
    /// let str_slice = &["", "ascii_max", "bmp_max", "unicode_max", "zero"];
    /// let str_list = VarZeroVec::<str>::from(str_slice);
    /// 
    /// let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list).unwrap();
    /// 
    /// assert!(cpilsl.contains_u32(0));
    /// assert!(cpilsl.contains_u32(0x0042));
    /// assert!(!cpilsl.contains_u32(0x0080));
    /// ```
    pub fn contains_u32(&self, cp: u32) -> bool {
        self.cp_inv_list.contains32(cp)
    }

    ///
    /// # Examples
    /// ```
    /// use icu_collections::codepointinvlist::CodePointInversionList;
    /// use icu_collections::codepointinvliststringlist::CodePointInversionListStringList;
    /// use zerovec::VarZeroVec;
    /// 
    /// let cp_slice = &[0, 0x80, 0xFFFF, 0x1_0000, 0x10_FFFF, 0x11_0000];
    /// let cp_list =
    ///     CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
    /// let str_slice = &["", "ascii_max", "bmp_max", "unicode_max", "zero"];
    /// let str_list = VarZeroVec::<str>::from(str_slice);
    /// 
    /// let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list).unwrap();
    /// 
    /// assert!(cpilsl.contains_char('A'));
    /// assert!(!cpilsl.contains_char(0x0080 as char));
    /// assert!(!cpilsl.contains_char('¡'));
    pub fn contains_char(&self, ch: char) -> bool {
        self.contains_u32(ch as u32)
    }
}

/// Custom Errors for [`CodePointInversionListStringList`].
///
/// Re-exported as [`Error`](Error).
#[derive(Display, Debug)]
pub enum CodePointInversionListStringListError {
    /// An invalid CodePointInversionList was constructed
    #[displaydoc("Invalid code point inversion list: {0:?}")]
    InvalidCodePointInversionList(CodePointInversionListError),
    /// A string in the string list had an invalid length
    #[displaydoc("Invalid string length for string: {0}")]
    InvalidStringLength(String),
    /// A string in the string list appears more than once
    #[displaydoc("String list has duplicate: {0}")]
    StringListNotUnique(String),
    /// Two strings in the string list compare to each other opposite of sorted order
    #[displaydoc("Strings in string list not in sorted order: ({0}, {1})")]
    StringListNotSorted(String, String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_has_strings() {
        let cp_slice = &[0, 1, 0x7F, 0x80, 0xFFFF, 0x1_0000, 0x10_FFFF, 0x11_0000];
        let cp_list =
            CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
        let str_slice = &["ascii_max", "bmp_max", "unicode_max", "zero"];
        let str_list = VarZeroVec::<str>::from(str_slice);

        let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list).unwrap();

        assert!(cpilsl.has_strings());
        assert_eq!(8, cpilsl.size());
    }

    #[test]
    fn test_empty_string_allowed() {
        let cp_slice = &[0, 1, 0x7F, 0x80, 0xFFFF, 0x1_0000, 0x10_FFFF, 0x11_0000];
        let cp_list =
            CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
        let str_slice = &["", "ascii_max", "bmp_max", "unicode_max", "zero"];
        let str_list = VarZeroVec::<str>::from(str_slice);

        let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list).unwrap();

        assert!(cpilsl.has_strings());
        assert_eq!(9, cpilsl.size());
    }

    #[test]
    fn test_invalid_string() {
        let cp_slice = &[0, 1];
        let cp_list =
            CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
        let str_slice = &["a"];
        let str_list = VarZeroVec::<str>::from(str_slice);

        let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list);

        assert!(matches!(
            cpilsl,
            Err(CodePointInversionListStringListError::InvalidStringLength(
                _
            ))
        ));
    }

    #[test]
    fn test_invalid_string_list_has_duplicate() {
        let cp_slice = &[0, 1];
        let cp_list =
            CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
        let str_slice = &["abc", "abc"];
        let str_list = VarZeroVec::<str>::from(str_slice);

        let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list);

        assert!(matches!(
            cpilsl,
            Err(CodePointInversionListStringListError::StringListNotUnique(
                _
            ))
        ));
    }

    #[test]
    fn test_invalid_string_list_not_sorted() {
        let cp_slice = &[0, 1];
        let cp_list =
            CodePointInversionList::try_clone_from_inversion_list_slice(cp_slice).unwrap();
        let str_slice = &["xyz", "abc"];
        let str_list = VarZeroVec::<str>::from(str_slice);

        let cpilsl = CodePointInversionListStringList::try_from(cp_list, str_list);

        assert!(matches!(
            cpilsl,
            Err(CodePointInversionListStringListError::StringListNotSorted(
                _,
                _
            ))
        ));
    }
}
