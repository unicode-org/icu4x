// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use core::convert::TryFrom;
use icu_collections::codepointinvlist::CodePointInversionListBuilder;
use icu_collections::codepointtrie::CodePointTrie;
#[cfg(feature = "datagen")]
use icu_collections::codepointtrie::CodePointTrieHeader;
use icu_locid::Locale;
use icu_provider::prelude::*;
use zerovec::ZeroMap;
#[cfg(feature = "datagen")]
use zerovec::ZeroVec;

use crate::error::Error;
#[cfg(feature = "datagen")]
use crate::exceptions_builder::CaseMappingExceptionsBuilder;
use crate::provider::exceptions::CaseMappingExceptions;

use crate::provider::data::{CaseMappingData, DotType, MappingKind};
use crate::provider::exception_helpers::ExceptionSlot;
/// Reverse case folding data. Maps from multi-character strings back
/// to code-points that fold to those strings.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_casemapping::provider),
)]
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct CaseMappingUnfoldData<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[allow(missing_docs)] // This field of a single-field struct doesn't need documentation
    pub map: ZeroMap<'data, str, str>,
}

impl<'data> CaseMappingUnfoldData<'data> {
    // Unfold data is exported by ICU as an array of 16-bit values, representing a short
    // header followed by a two-column key/value table. The header indicates:
    // - The number of rows.
    // - The number of UTF16 code units per row.
    // - The number of UTF16 code units in the first (key) column.
    //   (The number of code units in the value column can be derived from the above.)
    //
    // The key in the first column is the case folding of each of the code points in
    // the second column. Keys/values that are shorter than the column width are
    // null-terminated. The table is sorted by key. Binary search is used to find the value.
    //
    // Rust strings are UTF8 by default. To avoid the cost of converting from UTF16 on access,
    // we convert the ICU data into a more convenient format during construction.
    #[cfg(feature = "datagen")]
    fn try_from_icu(raw: &[u16]) -> Result<Self, Error> {
        const ROWS_INDEX: usize = 0;
        const ROW_WIDTH_INDEX: usize = 1;
        const STRING_WIDTH_INDEX: usize = 2;

        if raw.len() <= STRING_WIDTH_INDEX {
            return Error::invalid("Unfold: header missing");
        }

        let num_rows = raw[ROWS_INDEX] as usize;
        let row_width = raw[ROW_WIDTH_INDEX] as usize;
        let string_width = raw[STRING_WIDTH_INDEX] as usize;

        if row_width == 0 {
            return Error::invalid("Unfold: invalid row width");
        }

        // Header takes up one row.
        let row_data = &raw[row_width..];

        let mut map = ZeroMap::new();

        debug_assert!(num_rows == row_data.chunks_exact(row_width).count());
        for row in row_data.chunks_exact(row_width) {
            let key = Self::decode_string(&row[..string_width])
                .ok_or(Error::Validation("Unfold: unpaired surrogate in key"))?;
            let val = Self::decode_string(&row[string_width..])
                .ok_or(Error::Validation("Unfold: unpaired surrogate in value"))?;
            if map.try_append(key.as_ref(), val.as_ref()).is_some() {
                return Error::invalid("Unfold: keys not sorted/unique");
            }
        }
        Ok(Self { map })
    }

    // Decode a zero-terminated UTF16 string from a slice of u16.
    #[cfg(feature = "datagen")]
    fn decode_string(slice: &[u16]) -> Option<String> {
        let iter = slice.iter().copied().take_while(|&c| c != 0);
        char::decode_utf16(iter).collect::<Result<String, _>>().ok()
    }

    // Given a string, returns another string representing the set of characters
    // that case fold to that string.
    fn get(&self, key: &str) -> Option<&str> {
        self.map.get(key)
    }
}

// Used to control the behavior of CaseMapping::fold.
// Currently only used to decide whether to use Turkic (T) mappings for dotted/dotless i.
#[derive(Default)]
pub struct FoldOptions {
    exclude_special_i: bool,
}

impl FoldOptions {
    pub fn with_turkic_mappings() -> Self {
        Self {
            exclude_special_i: true,
        }
    }
}

/// CaseMappingInternals provides low-level access to the data necessary to
/// convert characters and strings to upper, lower, or title case.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/2535">#2535</a>
/// </div>
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_casemapping::provider),
)]
#[derive(Debug, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[yoke(prove_covariance_manually)]
pub struct CaseMappingInternals<'data> {
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// TODO
    pub trie: CodePointTrie<'data, CaseMappingData>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// TODO
    pub exceptions: CaseMappingExceptions<'data>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    /// TODO
    pub unfold: CaseMappingUnfoldData<'data>,
}

impl<'data> CaseMappingInternals<'data> {
    /// Creates a new CaseMappingInternals using data exported by the
    // `icuexportdata` tool in ICU4C. Validates that the data is
    // consistent.
    #[cfg(feature = "datagen")]
    pub fn try_from_icu(
        trie_header: CodePointTrieHeader,
        trie_index: &[u16],
        trie_data: &[u16],
        exceptions: &[u16],
        unfold: &[u16],
    ) -> Result<Self, Error> {
        let exceptions_builder = CaseMappingExceptionsBuilder::new(exceptions);
        let (exceptions, idx_map) = exceptions_builder.build()?;

        let trie_index = ZeroVec::alloc_from_slice(trie_index);

        let trie_data = trie_data
            .iter()
            .map(|&i| {
                CaseMappingData::try_from_icu_integer(i)
                    .unwrap()
                    .with_updated_exception(&idx_map)
            })
            .collect::<ZeroVec<_>>();

        let trie = CodePointTrie::try_new(trie_header, trie_index, trie_data)?;

        let unfold = CaseMappingUnfoldData::try_from_icu(unfold)?;

        let result = Self {
            trie,
            exceptions,
            unfold,
        };
        result.validate()?;
        Ok(result)
    }

    /// Given an existing CaseMapping, validates that the data is
    /// consistent. A CaseMapping created by the ICU transformer has
    /// already been validated. Calling this function is only
    /// necessary if you are concerned about data corruption after
    /// deserializing.
    pub(crate) fn validate(&self) -> Result<(), Error> {
        // First, validate that exception data is well-formed.
        let valid_exception_indices = self.exceptions.validate()?;

        let validate_delta = |c: char, delta: i32| -> Result<(), Error> {
            let new_c = u32::try_from(c as i32 + delta)
                .map_err(|_| Error::Validation("Delta larger than character"))?;
            char::from_u32(new_c).ok_or(Error::Validation("Invalid delta"))?;
            Ok(())
        };

        for i in 0..char::MAX as u32 {
            if let Some(c) = char::from_u32(i) {
                let data = self.lookup_data(c);
                if data.has_exception() {
                    let idx = data.exception_index();
                    let exception = self.exceptions.get(idx);
                    // Verify that the exception index points to a valid exception header.
                    if !valid_exception_indices.contains(&idx) {
                        return Error::invalid("Invalid exception index in trie data");
                    }
                    exception.validate()?;
                } else {
                    validate_delta(c, data.delta() as i32)?;
                }
            }
        }

        // The unfold data is structurally guaranteed to be valid,
        // so there is nothing left to check.
        Ok(())
    }

    fn lookup_data(&self, c: char) -> CaseMappingData {
        self.trie.get32(c as u32)
    }

    fn simple_helper(&self, c: char, kind: MappingKind) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_relevant_to(kind) {
                let folded = c as i32 + data.delta() as i32;
                char::from_u32(folded as u32).expect("Checked in validate()")
            } else {
                c
            }
        } else {
            let idx = data.exception_index();
            let exception = self.exceptions.get(idx);
            if data.is_relevant_to(kind) {
                if let Some(simple) = exception.get_simple_case_slot_for(c) {
                    return simple;
                }
            }
            exception.slot_char_for_kind(kind).unwrap_or(c)
        }
    }

    // Returns the lowercase mapping of the given `char`.
    #[inline]
    pub(crate) fn simple_lower(&self, c: char) -> char {
        self.simple_helper(c, MappingKind::Lower)
    }

    // Returns the uppercase mapping of the given `char`.
    #[inline]
    pub(crate) fn simple_upper(&self, c: char) -> char {
        self.simple_helper(c, MappingKind::Upper)
    }

    // Returns the titlecase mapping of the given `char`.
    #[inline]
    pub(crate) fn simple_title(&self, c: char) -> char {
        self.simple_helper(c, MappingKind::Title)
    }

    // Return the simple case folding mapping of the given char.
    #[inline]
    pub(crate) fn simple_fold(&self, c: char, options: FoldOptions) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_upper_or_title() {
                let folded = c as i32 + data.delta() as i32;
                char::from_u32(folded as u32).expect("Checked in validate()")
            } else {
                c
            }
        } else {
            // TODO: if we move conditional fold and no_simple_case_folding into
            // simple_helper, this function can just call simple_helper.
            let idx = data.exception_index();
            let exception = self.exceptions.get(idx);
            if exception.bits.has_conditional_fold() {
                self.simple_fold_special_case(c, options)
            } else if exception.bits.no_simple_case_folding() {
                c
            } else if data.is_upper_or_title() && exception.has_slot(ExceptionSlot::Delta) {
                // unwrap_or case should never happen but best to avoid panics
                exception.get_simple_case_slot_for(c).unwrap_or('\0')
            } else if let Some(slot_char) = exception.slot_char_for_kind(MappingKind::Fold) {
                slot_char
            } else {
                c
            }
        }
    }

    fn dot_type(&self, c: char) -> DotType {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            data.dot_type()
        } else {
            let idx = data.exception_index();
            self.exceptions.get(idx).bits.dot_type()
        }
    }

    // Returns true if this code point is is case-sensitive.
    // This is not currently exposed.
    #[allow(dead_code)]
    fn is_case_sensitive(&self, c: char) -> bool {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            data.is_sensitive()
        } else {
            let idx = data.exception_index();
            self.exceptions.get(idx).bits.is_sensitive()
        }
    }

    #[inline(always)]
    fn full_helper(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
        kind: MappingKind,
    ) -> FullMappingResult {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_relevant_to(kind) {
                let mapped = c as i32 + data.delta() as i32;
                let mapped = char::from_u32(mapped as u32).expect("Checked in validate()");
                FullMappingResult::CodePoint(mapped)
            } else {
                FullMappingResult::CodePoint(c)
            }
        } else {
            let idx = data.exception_index();
            let exception = self.exceptions.get(idx);
            if exception.bits.has_conditional_special() {
                if let Some(special) = match kind {
                    MappingKind::Lower => self.full_lower_special_case(c, context, locale),
                    MappingKind::Fold => self.full_fold_special_case(c, context, locale),
                    MappingKind::Upper | MappingKind::Title => self
                        .full_upper_or_title_special_case(
                            c,
                            context,
                            locale,
                            kind == MappingKind::Title,
                        ),
                } {
                    return special;
                }
            }
            if let Some(mapped_string) = exception.get_fullmappings_slot_for_kind(kind) {
                if !mapped_string.is_empty() {
                    return FullMappingResult::String(mapped_string);
                }
            }
            if data.is_relevant_to(kind) {
                if let Some(simple) = exception.get_simple_case_slot_for(c) {
                    return FullMappingResult::CodePoint(simple);
                }
            }
            if kind == MappingKind::Fold && exception.bits.no_simple_case_folding() {
                return FullMappingResult::CodePoint(c);
            }

            if let Some(slot_char) = exception.slot_char_for_kind(kind) {
                FullMappingResult::CodePoint(slot_char)
            } else {
                FullMappingResult::CodePoint(c)
            }
        }
    }

    pub(crate) fn to_full_lower(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Lower)
    }

    pub(crate) fn to_full_upper(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Upper)
    }

    // Titlecase mapping is not yet exposed
    #[allow(dead_code)]
    pub(crate) fn to_full_title(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Title)
    }

    // Note: in ICU4C, case folding takes an options bag instead of a locale,
    // with the only defined option being whether or not to use Turkic (T)
    // mappings for dotted/dotless i. In ICU4X, we expose a similar locale-free
    // API for case folding, but internally represent this as a Turkish locale
    // to simplify shared code.
    pub(crate) fn to_full_folding(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> FullMappingResult {
        self.full_helper(c, context, locale, MappingKind::Fold)
    }

    // These constants are used for hardcoded locale-specific foldings.
    const I_DOT: &'static str = "\u{69}\u{307}";
    const J_DOT: &'static str = "\u{6a}\u{307}";
    const I_OGONEK_DOT: &'static str = "\u{12f}\u{307}";
    const I_DOT_GRAVE: &'static str = "\u{69}\u{307}\u{300}";
    const I_DOT_ACUTE: &'static str = "\u{69}\u{307}\u{301}";
    const I_DOT_TILDE: &'static str = "\u{69}\u{307}\u{303}";

    // Special case folding mappings, hardcoded.
    // This handles the special Turkic mappings for uppercase I and dotted uppercase I
    // For non-Turkic languages, this mapping is normally not used.
    // For Turkic languages (tr, az), this mapping can be used instead of the normal mapping for these characters.
    fn simple_fold_special_case(&self, c: char, options: FoldOptions) -> char {
        debug_assert!(c == '\u{49}' || c == '\u{130}');
        let is_turkic = options.exclude_special_i;
        match (c, is_turkic) {
            // Turkic mappings
            ('\u{49}', true) => '\u{131}', // 0049; T; 0131; # LATIN CAPITAL LETTER I
            ('\u{130}', true) => '\u{69}', /* 0130; T; 0069; # LATIN CAPITAL LETTER I WITH DOT ABOVE */

            // Default mappings
            ('\u{49}', false) => '\u{69}', // 0049; C; 0069; # LATIN CAPITAL LETTER I

            // There is no simple case folding for U+130.
            (c, _) => c,
        }
    }

    fn full_lower_special_case(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
    ) -> Option<FullMappingResult> {
        if locale == CaseMapLocale::Lithuanian {
            // Lithuanian retains the dot in a lowercase i when followed by accents.
            // Introduce an explicit dot above when lowercasing capital I's and J's
            // whenever there are more accents above (of the accents used in
            // Lithuanian: grave, acute, and tilde above).

            // Check for accents above I, J, and I-with-ogonek.
            if c == 'I' && context.followed_by_more_above(self) {
                return Some(FullMappingResult::String(Self::I_DOT));
            } else if c == 'J' && context.followed_by_more_above(self) {
                return Some(FullMappingResult::String(Self::J_DOT));
            } else if c == '\u{12e}' && context.followed_by_more_above(self) {
                return Some(FullMappingResult::String(Self::I_OGONEK_DOT));
            }

            // These characters are precomposed with accents above, so we don't
            // have to look at the context.
            if c == '\u{cc}' {
                return Some(FullMappingResult::String(Self::I_DOT_GRAVE));
            } else if c == '\u{cd}' {
                return Some(FullMappingResult::String(Self::I_DOT_ACUTE));
            } else if c == '\u{128}' {
                return Some(FullMappingResult::String(Self::I_DOT_TILDE));
            }
        }

        if locale == CaseMapLocale::Turkish {
            if c == '\u{130}' {
                // I and i-dotless; I-dot and i are case pairs in Turkish and Azeri
                return Some(FullMappingResult::CodePoint('i'));
            } else if c == '\u{307}' && context.preceded_by_capital_i(self) {
                // When lowercasing, remove dot_above in the sequence I + dot_above,
                // which will turn into i. This matches the behaviour of the
                // canonically equivalent I-dot_above.
                return Some(FullMappingResult::Remove);
            } else if c == 'I' && !context.followed_by_dot_above(self) {
                // When lowercasing, unless an I is before a dot_above, it turns
                // into a dotless i.
                return Some(FullMappingResult::CodePoint('\u{131}'));
            }
        }

        if c == '\u{130}' {
            // Preserve canonical equivalence for I with dot. Turkic is handled above.
            return Some(FullMappingResult::String(Self::I_DOT));
        }

        if c == '\u{3a3}'
            && context.preceded_by_cased_letter(self)
            && !context.followed_by_cased_letter(self)
        {
            // Greek capital sigman maps depending on surrounding cased letters.
            return Some(FullMappingResult::CodePoint('\u{3c2}'));
        }

        // No relevant special case mapping. Use a normal mapping.
        None
    }

    fn full_upper_or_title_special_case(
        &self,
        c: char,
        context: ContextIterator,
        locale: CaseMapLocale,
        _is_title: bool,
    ) -> Option<FullMappingResult> {
        if locale == CaseMapLocale::Turkish && c == 'i' {
            // In Turkic languages, i turns into a dotted capital I.
            return Some(FullMappingResult::CodePoint('\u{130}'));
        }
        if locale == CaseMapLocale::Lithuanian
            && c == '\u{307}'
            && context.preceded_by_soft_dotted(self)
        {
            // Lithuanian retains the dot in a lowercase i when followed by accents.
            // Remove dot_above after i with upper or titlecase.
            return Some(FullMappingResult::Remove);
        }
        // TODO: ICU4C has a non-standard extension for Armenian ligature ech-yiwn.
        // Should we also implement it?
        // if c == '\u{587}' {
        //     return match (locale, is_title) {
        // 	(CaseMapLocale::Armenian, false) => Some(FullMappingResult::String("ԵՎ")),
        // 	(CaseMapLocale::Armenian, true) => Some(FullMappingResult::String("Եվ")),
        // 	(_, false) => Some(FullMappingResult::String("ԵՒ")),
        // 	(_, true) => Some(FullMappingResult::String("Եւ")),
        //     }
        // }
        None
    }

    fn full_fold_special_case(
        &self,
        c: char,
        _context: ContextIterator,
        locale: CaseMapLocale,
    ) -> Option<FullMappingResult> {
        let is_turkic = locale == CaseMapLocale::Turkish;
        match (c, is_turkic) {
            // Turkic mappings
            ('\u{49}', true) => Some(FullMappingResult::CodePoint('\u{131}')),
            ('\u{130}', true) => Some(FullMappingResult::CodePoint('\u{69}')),

            // Default mappings
            ('\u{49}', false) => Some(FullMappingResult::CodePoint('\u{69}')),
            ('\u{130}', false) => Some(FullMappingResult::String(Self::I_DOT)),
            (_, _) => None,
        }
    }

    pub(crate) fn full_lowercase(&self, src: &str, locale: CaseMapLocale) -> String {
        let mut result = String::with_capacity(src.len());

        // To speed up the copying of long runs where nothing changes, we keep track
        // of the start of the uncopied chunk, and don't copy it until we have to.
        let mut last_uncopied_idx = 0;

        for (i, c) in src.char_indices() {
            let context = ContextIterator::new(src, i);
            match self.to_full_lower(c, context, locale) {
                FullMappingResult::CodePoint(c2) => {
                    if c == c2 {
                        continue;
                    }
                    result.push_str(&src[last_uncopied_idx..i]);
                    result.push(c2);
                    last_uncopied_idx = i + c.len_utf8();
                }
                FullMappingResult::Remove => {
                    result.push_str(&src[last_uncopied_idx..i]);
                    last_uncopied_idx = i + c.len_utf8();
                }
                FullMappingResult::String(s) => {
                    result.push_str(&src[last_uncopied_idx..i]);
                    result.push_str(s);
                    last_uncopied_idx = i + c.len_utf8();
                }
            }
        }
        if last_uncopied_idx < src.len() {
            result.push_str(&src[last_uncopied_idx..]);
        }
        result
    }

    pub(crate) fn full_uppercase(&self, src: &str, locale: CaseMapLocale) -> String {
        let mut result = String::with_capacity(src.len());

        // To speed up the copying of long runs where nothing changes, we keep track
        // of the start of the uncopied chunk, and don't copy it until we have to.
        let mut last_uncopied_idx = 0;

        for (i, c) in src.char_indices() {
            let context = ContextIterator::new(src, i);
            match self.to_full_upper(c, context, locale) {
                FullMappingResult::CodePoint(c2) => {
                    if c == c2 {
                        continue;
                    }
                    result.push_str(&src[last_uncopied_idx..i]);
                    result.push(c2);
                    last_uncopied_idx = i + c.len_utf8();
                }
                FullMappingResult::Remove => {
                    result.push_str(&src[last_uncopied_idx..i]);
                    last_uncopied_idx = i + c.len_utf8();
                }
                FullMappingResult::String(s) => {
                    result.push_str(&src[last_uncopied_idx..i]);
                    result.push_str(s);
                    last_uncopied_idx = i + c.len_utf8();
                }
            }
        }
        if last_uncopied_idx < src.len() {
            result.push_str(&src[last_uncopied_idx..]);
        }
        result
    }

    pub(crate) fn full_folding(&self, src: &str, locale: CaseMapLocale) -> String {
        let mut result = String::with_capacity(src.len());

        // To speed up the copying of long runs where nothing changes, we keep track
        // of the start of the uncopied chunk, and don't copy it until we have to.
        let mut last_uncopied_idx = 0;

        for (i, c) in src.char_indices() {
            let context = ContextIterator::new(src, i);
            match self.to_full_folding(c, context, locale) {
                FullMappingResult::CodePoint(c2) => {
                    if c == c2 {
                        continue;
                    }
                    result.push_str(&src[last_uncopied_idx..i]);
                    result.push(c2);
                    last_uncopied_idx = i + c.len_utf8();
                }
                FullMappingResult::Remove => {
                    result.push_str(&src[last_uncopied_idx..i]);
                    last_uncopied_idx = i + c.len_utf8();
                }
                FullMappingResult::String(s) => {
                    result.push_str(&src[last_uncopied_idx..i]);
                    result.push_str(s);
                    last_uncopied_idx = i + c.len_utf8();
                }
            }
        }
        if last_uncopied_idx < src.len() {
            result.push_str(&src[last_uncopied_idx..]);
        }
        result
    }

    // Adds all simple case mappings and the full case folding for `c` to `set`.
    // Also adds special case closure mappings.
    // The character itself is not added.
    // For example, the mappings
    // - for s include long s
    // - for sharp s include ss
    // - for k include the Kelvin sign
    fn add_case_closure<S: ClosureSet>(&self, c: char, set: &mut S) {
        // Hardcode the case closure of i and its relatives and ignore the
        // data file data for these characters.
        // The Turkic dotless i and dotted I with their case mapping conditions
        // and case folding option make the related characters behave specially.
        // This code matches their closure behavior to their case folding behavior.
        match c {
            // Regular i and I are in one equivalence class.
            '\u{49}' => {
                set.add_char('\u{69}');
                return;
            }
            '\u{69}' => {
                set.add_char('\u{49}');
                return;
            }

            // Dotted I is in a class with <0069 0307> (for canonical equivalence with <0049 0307>)
            '\u{130}' => {
                set.add_string(Self::I_DOT);
                return;
            }

            // Dotless i is in a class by itself
            '\u{131}' => {
                return;
            }

            _ => {}
        }

        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.case_type().is_some() {
                let delta = data.delta() as i32;
                if delta != 0 {
                    // Add the one simple case mapping, no matter what type it is.
                    let codepoint = c as i32 + delta;
                    let mapped = char::from_u32(codepoint as u32).expect("Checked in validate()");
                    set.add_char(mapped);
                }
            }
            return;
        }

        // c has exceptions, so there may be multiple simple and/or full case mappings.
        let idx = data.exception_index();
        let exception = self.exceptions.get(idx);

        // Add all simple case mappings.
        for slot in [
            ExceptionSlot::Lower,
            ExceptionSlot::Fold,
            ExceptionSlot::Upper,
            ExceptionSlot::Title,
        ] {
            if let Some(simple) = exception.get_char_slot(slot) {
                set.add_char(simple);
            }
        }
        if let Some(simple) = exception.get_simple_case_slot_for(c) {
            set.add_char(simple);
        }

        exception.add_full_and_closure_mappings(set);
    }

    // Maps the string to single code points and adds the associated case closure
    // mappings.
    // The string is mapped to code points if it is their full case folding string.
    // In other words, this performs a reverse full case folding and then
    // adds the case closure items of the resulting code points.
    // If the string is found and its closure applied, then
    // the string itself is added as well as part of its code points' closure.
    //
    // Returns true if the string was found
    #[allow(dead_code)]
    fn add_string_case_closure<S: ClosureSet>(&self, s: &str, set: &mut S) -> bool {
        if s.chars().count() <= 1 {
            // The string is too short to find any match.
            return false;
        }
        match self.unfold.get(s) {
            Some(closure_string) => {
                for c in closure_string.chars() {
                    set.add_char(c);
                    self.add_case_closure(c, set);
                }
                true
            }
            None => false,
        }
    }
}

// An internal representation of locale. Non-Root values of this
// enumeration imply that hard-coded special cases exist for this
// language.
#[derive(Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum CaseMapLocale {
    Root,
    Turkish,
    Lithuanian,
    Greek,
    Dutch,
    Armenian,
}

impl From<&Locale> for CaseMapLocale {
    fn from(loc: &Locale) -> Self {
        match loc.id.language.as_str() {
            "tr" | "az" => Self::Turkish,
            "lt" => Self::Lithuanian,
            "el" => Self::Greek,
            "nl" => Self::Dutch,
            "hy" => Self::Armenian,
            _ => Self::Root,
        }
    }
}

pub enum FullMappingResult<'a> {
    Remove,
    CodePoint(char),
    String(&'a str),
}

impl<'a> FullMappingResult<'a> {
    #[allow(dead_code)]
    fn add_to_set<S: ClosureSet>(&self, set: &mut S) {
        match self {
            FullMappingResult::CodePoint(c) => set.add_char(*c),
            FullMappingResult::String(s) => set.add_string(s),
            FullMappingResult::Remove => {}
        }
    }
}

// Interface for adding items to a closure set.
pub trait ClosureSet {
    /// Add a character to the set
    fn add_char(&mut self, c: char);
    /// Add a string to the set
    fn add_string(&mut self, string: &str);
}

impl ClosureSet for CodePointInversionListBuilder {
    fn add_char(&mut self, c: char) {
        self.add_char(c)
    }

    // The current version of CodePointInversionList doesn't include strings.
    // Trying to add a string is a no-op that will be optimized away.
    #[inline]
    fn add_string(&mut self, _string: &str) {}
}

pub(crate) struct ContextIterator<'a> {
    before: &'a str,
    after: &'a str,
}

impl<'a> ContextIterator<'a> {
    // Returns a context iterator with the characters before
    // and after the character at a given index.
    pub fn new(s: &'a str, idx: usize) -> Self {
        let before = &s[..idx];
        let mut char_and_after = s[idx..].chars();
        char_and_after.next(); // skip the character itself
        let after = char_and_after.as_str();
        Self { before, after }
    }

    fn preceded_by_soft_dotted(&self, mapping: &CaseMappingInternals) -> bool {
        for c in self.before.chars().rev() {
            match mapping.dot_type(c) {
                DotType::SoftDotted => return true,
                DotType::OtherAccent => continue,
                _ => return false,
            }
        }
        false
    }
    fn preceded_by_capital_i(&self, mapping: &CaseMappingInternals) -> bool {
        for c in self.before.chars().rev() {
            if c == 'I' {
                return true;
            }
            if mapping.dot_type(c) != DotType::OtherAccent {
                break;
            }
        }
        false
    }
    fn preceded_by_cased_letter(&self, mapping: &CaseMappingInternals) -> bool {
        for c in self.before.chars().rev() {
            let data = mapping.lookup_data(c);
            if !data.is_ignorable() {
                return data.case_type().is_some();
            }
        }
        false
    }
    fn followed_by_cased_letter(&self, mapping: &CaseMappingInternals) -> bool {
        for c in self.after.chars() {
            let data = mapping.lookup_data(c);
            if !data.is_ignorable() {
                return data.case_type().is_some();
            }
        }
        false
    }
    fn followed_by_more_above(&self, mapping: &CaseMappingInternals) -> bool {
        for c in self.after.chars() {
            match mapping.dot_type(c) {
                DotType::Above => return true,
                DotType::OtherAccent => continue,
                _ => return false,
            }
        }
        false
    }
    fn followed_by_dot_above(&self, mapping: &CaseMappingInternals) -> bool {
        for c in self.after.chars() {
            if c == '\u{307}' {
                return true;
            }
            if mapping.dot_type(c) != DotType::OtherAccent {
                return false;
            }
        }
        false
    }
}
