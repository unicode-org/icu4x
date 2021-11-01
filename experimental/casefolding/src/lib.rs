// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! TODO: module documentation

use core::convert::TryFrom;
use core::num::TryFromIntError;
use icu_codepointtrie::codepointtrie::{CodePointTrie, TrieValue};
use zerovec::ZeroVec;
use zerovec::ule::{AsULE, PlainOldULE};

/// The case of a Unicode character
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum CaseType {
    /// Not a cased letter
    None,
    /// Lowercase letter
    Lower,
    /// Uppercase letter
    Upper,
    /// Titlecase letter
    Title
}

/// The dot type of a Unicode character
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum DotType {
    /// Normal characters with combining class 0
    NoDot = 0,
    /// Soft-dotted characters with combining class 0
    SoftDotted = 1,
    /// "above" accents with combining class 230
    Above = 2,
    /// Other accent characters
    OtherAccent = 3,
}

impl DotType {
    const DOT_MASK: u16 = 0x3;

    // The dot type is stored in either the codepoint trie or the exception table as two bits.
    // After shifting and masking them to get a value between 0 and 3, this function converts
    // to DotType.
    #[inline]
    fn from_masked_bits(b: u16) -> Self {
	debug_assert!(b & Self::DOT_MASK == b);
	match b {
	    0 => DotType::NoDot,
	    1 => DotType::SoftDotted,
	    2 => DotType::Above,
	    _ => DotType::OtherAccent,
	}
    }
}

/// The datatype stored in the codepoint trie for casefolding.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct CaseFoldingData(u16);

impl CaseFoldingData {
    const CASE_MASK: u16 = 0x3;
    const IGNORABLE_FLAG: u16 = 0x4;
    const EXCEPTION_FLAG: u16 = 0x8;
    const SENSITIVE_FLAG: u16 = 0x10;

    const DOT_SHIFT: u16 = 5;

    const DELTA_SHIFT: usize = 7;
    const EXCEPTION_SHIFT: usize = 4;

    #[inline]
    fn case_type(&self) -> CaseType {
	match self.0 & Self::CASE_MASK {
	    0 => CaseType::None,
	    1 => CaseType::Lower,
	    2 => CaseType::Upper,
	    _ => CaseType::Title,
	}
    }

    #[inline]
    fn is_upper_or_title(&self) -> bool {
	match self.case_type() {
	    CaseType::None | CaseType::Lower => false,
	    CaseType::Upper | CaseType::Title => true,
	}
    }

    #[inline]
    fn is_ignorable(&self) -> bool {
	self.0 & Self::IGNORABLE_FLAG != 0
    }

    #[inline]
    fn has_exception(&self) -> bool {
	self.0 & Self::EXCEPTION_FLAG != 0
    }

    #[inline]
    fn is_sensitive(&self) -> bool {
	self.0 & Self::SENSITIVE_FLAG != 0
    }

    #[inline]
    fn dot_type(&self) -> DotType {
	let masked_bits = (self.0 >> Self::DOT_SHIFT) & DotType::DOT_MASK;
	DotType::from_masked_bits(masked_bits)
    }

    #[inline]
    fn delta(&self) -> i16 {
	debug_assert!(!self.has_exception());
	(self.0 as i16) >> Self::DELTA_SHIFT
    }

    #[inline]
    fn exception_index(&self) -> u16 {
	debug_assert!(self.has_exception());
	self.0 >> Self::EXCEPTION_SHIFT
    }
}

impl AsULE for CaseFoldingData {
    type ULE = PlainOldULE<2>;

    #[inline]
    fn as_unaligned(self) -> Self::ULE {
        PlainOldULE(self.0.to_le_bytes())
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        CaseFoldingData(u16::from_le_bytes(unaligned.0))
    }
}

impl TrieValue for CaseFoldingData {
    const DATA_GET_ERROR_VALUE: CaseFoldingData = CaseFoldingData(0);
    type TryFromU32Error = TryFromIntError;

    fn try_from_u32(i: u32) -> Result<Self, Self::TryFromU32Error> {
        u16::try_from(i).map(CaseFoldingData)
    }
}

struct CaseFoldingExceptions<'data> {
    raw: ZeroVec<'data, u16>,
}

#[derive(Copy,Clone)]
enum CaseFoldingExceptionSlot {
    Lower = 0,
    Fold = 1,
    Upper = 2,
    Title = 3,
    Delta = 4,
    // Slot 5 is reserved
    Closure = 6,
    FullMappings = 7,
}

/// Case folding exceptions that can't be represented as a simple offset added to the original character.
/// Following ICU4C, data is stored as a u16 array. The codepoint trie in CaseFolding stores offsets into
/// this array. The u16 at that index contains a set of flags describing the subsequent data.
/// TODO: diagram of layout
impl<'data> CaseFoldingExceptions<'data> {
    // Each slot is 2 u16 elements instead of 1
    const DOUBLE_SLOTS_FLAG: u16 = 0x100;

    const NO_SIMPLE_CASE_FOLDING_FLAG: u16 = 0x200;
    const DELTA_IS_NEGATIVE_FLAG: u16 = 0x400;
    const SENSITIVE_FLAG: u16 = 0x800;

    const DOT_SHIFT: u16 = 7;

    const CONDITIONAL_SPECIAL_FLAG: u16 = 0x4000;
    const CONDITIONAL_FOLD_FLAG: u16 = 0x8000;

    #[inline(always)]
    fn get(&self, idx: usize) -> u16 {
	<u16 as AsULE>::from_unaligned(self.raw.as_slice()[idx as usize])
    }

    // Returns true if the given slot exists for the exception data starting at the given index
    fn has_slot(&self, idx: u16, slot: CaseFoldingExceptionSlot) -> bool {
	let bit = 1u16 << (slot as u16);
	self.get(idx as usize) & bit != 0
    }

    // Returns the offset of the given slot for the exception data starting at the given index:
    // that is, the number of slots before the given slot.
    fn slot_offset(&self, idx: u16, slot: CaseFoldingExceptionSlot) -> usize {
	debug_assert!(self.has_slot(idx, slot));
	let flags = self.get(idx as usize);
	let slot_bit = 1u16 << (slot as u16);
	let previous_slot_mask = slot_bit - 1;
	let previous_slots = flags & previous_slot_mask;
	previous_slots.count_ones() as usize
    }

    fn slot_value(&self, idx: u16, slot: CaseFoldingExceptionSlot) -> u32 {
	debug_assert!(self.has_slot(idx, slot));
	if self.get(idx as usize) & Self::DOUBLE_SLOTS_FLAG != 0 {
	    let slot_idx = (idx + 1) as usize + self.slot_offset(idx, slot) * 2;
	    let hi = self.get(slot_idx) as u32;
	    let lo = self.get(slot_idx + 1) as u32;
	    hi << 16 | lo
	} else {
	    let slot_idx = (idx + 1) as usize + self.slot_offset(idx, slot);
	    self.get(slot_idx) as u32
	}
    }

    fn slot_char(&self, idx: u16, slot: CaseFoldingExceptionSlot) -> char {
	let raw = self.slot_value(idx, slot);
	char::from_u32(raw).expect("Character should be valid")
    }

    fn delta(&self, idx: u16) -> i32 {
	debug_assert!(self.has_slot(idx, CaseFoldingExceptionSlot::Delta));
	let raw: i32 = self.slot_value(idx, CaseFoldingExceptionSlot::Delta) as _;
	if self.get(idx as usize) & Self::DELTA_IS_NEGATIVE_FLAG != 0 {
	    -raw
	} else {
	    raw
	}
    }

    fn no_simple_case_folding(&self, idx: u16) -> bool {
	self.get(idx as usize) & Self::NO_SIMPLE_CASE_FOLDING_FLAG != 0
    }

    fn is_sensitive(&self, idx: u16) -> bool {
	self.get(idx as usize) & Self::SENSITIVE_FLAG != 0
    }

    fn has_conditional_fold(&self, idx: u16) -> bool {
	self.get(idx as usize) & Self::CONDITIONAL_FOLD_FLAG != 0
    }

    fn has_conditional_special(&self, idx: u16) -> bool {
	self.get(idx as usize) & Self::CONDITIONAL_SPECIAL_FLAG != 0
    }

    fn dot_type(&self, idx: u16) -> DotType {
	let masked_bits = (self.get(idx as usize) >> Self::DOT_SHIFT) & DotType::DOT_MASK;
	DotType::from_masked_bits(masked_bits)
    }
}

pub struct FoldOptions(u32);

impl FoldOptions {
    const EXCLUDE_SPECIAL_I_FLAG: u32 = 0x1;

    pub fn with_turkic_mappings() -> Self {
	Self(Self::EXCLUDE_SPECIAL_I_FLAG)
    }

    pub fn exclude_special_i(&self) -> bool {
	self.0 & Self::EXCLUDE_SPECIAL_I_FLAG != 0
    }
}

impl Default for FoldOptions {
    fn default() -> Self {
	Self(0)
    }
}

pub struct CaseFolding<'data> {
    trie: CodePointTrie<'data, CaseFoldingData>,
    exceptions: CaseFoldingExceptions<'data>
}

impl<'data> CaseFolding<'data> {
    fn lookup_value(&self, c: char) -> CaseFoldingData {
	self.trie.get(c as u32)
    }

    pub fn to_lower(&self, c: char) -> char {
	let value = self.lookup_value(c);
	if !value.has_exception() {
	    if value.is_upper_or_title() {
		let lower = c as i32 + value.delta() as i32;
		debug_assert!(lower > 0);
		char::from_u32(lower as u32).expect("Character should be valid")
	    } else {
		c
	    }
	} else {
	    let idx = value.exception_index();
	    if value.is_upper_or_title() && self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Delta) {
		let lower = c as i32 + self.exceptions.delta(idx);
		debug_assert!(lower > 0);
		char::from_u32(lower as u32).expect("Character should be valid")
	    } else if self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Lower) {
		self.exceptions.slot_char(idx, CaseFoldingExceptionSlot::Lower)
	    } else {
		c
	    }
	}
    }

    pub fn to_upper(&self, c: char) -> char {
	let value = self.lookup_value(c);
	if !value.has_exception() {
	    if value.case_type() == CaseType::Lower {
		let upper = c as i32 + value.delta() as i32;
		debug_assert!(upper > 0);
		char::from_u32(upper as u32).expect("Character should be valid")
	    } else {
		c
	    }
	} else {
	    let idx = value.exception_index();
	    if value.case_type() == CaseType::Lower && self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Delta) {
		let upper = c as i32 + self.exceptions.delta(idx);
		debug_assert!(upper > 0);
		char::from_u32(upper as u32).expect("Character should be valid")
	    } else if self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Upper) {
		self.exceptions.slot_char(idx, CaseFoldingExceptionSlot::Upper)
	    } else {
		c
	    }
	}
    }

    pub fn to_title(&self, c: char) -> char {
	let value = self.lookup_value(c);
	if !value.has_exception() {
	    // In general, titlecase is the same as uppercase.
	    if value.case_type() == CaseType::Lower {
		let upper = c as i32 + value.delta() as i32;
		debug_assert!(upper > 0);
		char::from_u32(upper as u32).expect("Character should be valid")
	    } else {
		c
	    }
	} else {
	    let idx = value.exception_index();
	    if value.case_type() == CaseType::Lower && self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Delta) {
		let upper = c as i32 + self.exceptions.delta(idx);
		debug_assert!(upper > 0);
		char::from_u32(upper as u32).expect("Character should be valid")
	    } else if self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Title) {
		self.exceptions.slot_char(idx, CaseFoldingExceptionSlot::Title)
	    } else if self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Upper) {
		self.exceptions.slot_char(idx, CaseFoldingExceptionSlot::Upper)
	    } else {
		c
	    }
	}
    }

    /// Return the simple case folding mapping for `c`
    pub fn fold(&self, c: char, options: FoldOptions) -> char {
	let value = self.lookup_value(c);
	if !value.has_exception() {
	    if value.is_upper_or_title() {
		let folded = c as i32 + value.delta() as i32;
		debug_assert!(folded > 0);
		char::from_u32(folded as u32).expect("Character should be valid")
	    } else {
		c
	    }
	} else {
	    let idx = value.exception_index();
	    if self.exceptions.has_conditional_fold(idx) {
		self.conditional_fold(c, options)
	    } else if self.exceptions.no_simple_case_folding(idx) {
		c
	    } else if value.is_upper_or_title() && self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Delta) {
		let folded = c as i32 + self.exceptions.delta(idx);
		debug_assert!(folded > 0);
		char::from_u32(folded as u32).expect("Character should be valid")
	    } else if self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Fold) {
		self.exceptions.slot_char(idx, CaseFoldingExceptionSlot::Fold)
	    } else if self.exceptions.has_slot(idx, CaseFoldingExceptionSlot::Lower) {
		self.exceptions.slot_char(idx, CaseFoldingExceptionSlot::Lower)
	    } else {
		c
	    }
	}
    }

    // Special case folding mappings, hardcoded.
    // This handles the special Turkic mappings for uppercase I and dotted uppercase I
    // For non-Turkic languages, this mapping is normally not used.
    // For Turkic languages (tr, az), this mapping can be used instead of the normal mapping for these characters.
    fn conditional_fold(&self, c: char, options: FoldOptions) -> char {
	debug_assert!(c == '\u{49}' || c == '\u{130}');
	let is_turkic = options.exclude_special_i();
	match (c, is_turkic) {
	    // Turkic mappings
	    ('\u{49}', true) => '\u{131}', // 0049; T; 0131; # LATIN CAPITAL LETTER I
	    ('\u{130}', true) => '\u{69}', // 0130; T; 0069; # LATIN CAPITAL LETTER I WITH DOT ABOVE

	    // Default mappings
	    ('\u{49}', false) => '\u{69}', // 0049; C; 0069; # LATIN CAPITAL LETTER I

	    // There is no simple case folding for U+130.
	    (c, _) => c
	}
    }

    pub fn case_type(&self, c: char) -> CaseType {
	self.lookup_value(c).case_type()
    }

    pub fn is_ignorable(&self, c: char) -> bool {
	self.lookup_value(c).is_ignorable()
    }

    pub fn dot_type(&self, c: char) -> DotType {
	let value = self.lookup_value(c);
	if !value.has_exception() {
	    value.dot_type()
	} else {
	    let idx = value.exception_index();
	    self.exceptions.dot_type(idx)
	}
    }

    pub fn is_soft_dotted(&self, c: char) -> bool {
	self.lookup_value(c).dot_type() == DotType::SoftDotted
    }

    pub fn is_case_sensitive(&self, c: char) -> bool {
	let value = self.lookup_value(c);
	if !value.has_exception() {
	    value.is_sensitive()
	} else {
	    let idx = value.exception_index();
	    self.exceptions.is_sensitive(idx)
	}
    }
}
