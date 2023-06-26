// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains most of the actual algorithms for case mapping.
//!
//! Primarily, it implements methods on `CaseMappingV1`, which contains the data model.

use crate::provider::data::{DotType, MappingKind};
use crate::provider::exception_helpers::ExceptionSlot;
use crate::provider::CaseMappingV1;
use crate::set::ClosureSet;
use core::fmt;
use icu_locid::Locale;
use writeable::Writeable;

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

impl<'data> CaseMappingV1<'data> {
    fn simple_helper(&self, c: char, kind: MappingKind) -> char {
        let data = self.lookup_data(c);
        if !data.has_exception() {
            if data.is_relevant_to(kind) {
                let folded = c as i32 + data.delta() as i32;
                // GIGO: delta should be valid
                char::from_u32(folded as u32).unwrap_or(c)
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
                // GIGO: delta should be valid
                char::from_u32(folded as u32).unwrap_or(c)
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
                // GIGO: delta should be valid
                let mapped = char::from_u32(mapped as u32).unwrap_or(c);
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
        is_title: bool,
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
        // ICU4C's non-standard extension for Armenian ligature ech-yiwn.
        if c == '\u{587}' {
            return match (locale, is_title) {
                (CaseMapLocale::Armenian, false) => Some(FullMappingResult::String("ԵՎ")),
                (CaseMapLocale::Armenian, true) => Some(FullMappingResult::String("Եվ")),
                (_, false) => Some(FullMappingResult::String("ԵՒ")),
                (_, true) => Some(FullMappingResult::String("Եւ")),
            };
        }
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
    pub(crate) fn full_helper_writeable<'a: 'data>(
        &'a self,
        src: &'a str,
        locale: CaseMapLocale,
        mapping: MappingKind,
    ) -> impl Writeable + 'a {
        struct FullCaseWriteable<'a> {
            data: &'a CaseMappingV1<'a>,
            src: &'a str,
            locale: CaseMapLocale,
            mapping: MappingKind,
        }

        impl<'a> Writeable for FullCaseWriteable<'a> {
            #[allow(clippy::indexing_slicing)] // last_uncopied_index and i are known to be in bounds
            fn write_to<W: fmt::Write + ?Sized>(&self, sink: &mut W) -> fmt::Result {
                // To speed up the copying of long runs where nothing changes, we keep track
                // of the start of the uncopied chunk, and don't copy it until we have to.
                let mut last_uncopied_idx = 0;

                let src = self.src;
                for (i, c) in src.char_indices() {
                    let context = ContextIterator::new(&src[..i], &src[i..]);
                    match self.data.full_helper(c, context, self.locale, self.mapping) {
                        FullMappingResult::CodePoint(c2) => {
                            if c == c2 {
                                continue;
                            }
                            sink.write_str(&src[last_uncopied_idx..i])?;
                            sink.write_char(c2)?;
                            last_uncopied_idx = i + c.len_utf8();
                        }
                        FullMappingResult::Remove => {
                            sink.write_str(&src[last_uncopied_idx..i])?;
                            last_uncopied_idx = i + c.len_utf8();
                        }
                        FullMappingResult::String(s) => {
                            sink.write_str(&src[last_uncopied_idx..i])?;
                            sink.write_str(s)?;
                            last_uncopied_idx = i + c.len_utf8();
                        }
                    }
                }
                if last_uncopied_idx < src.len() {
                    sink.write_str(&src[last_uncopied_idx..])?;
                }
                Ok(())
            }
            fn writeable_length_hint(&self) -> writeable::LengthHint {
                writeable::LengthHint::at_least(self.src.len())
            }
        }

        FullCaseWriteable {
            data: self,
            src,
            locale,
            mapping,
        }
    }

    /// Adds all simple case mappings and the full case folding for `c` to `set`.
    /// Also adds special case closure mappings.
    /// The character itself is not added.
    /// For example, the mappings
    /// - for s include long s
    /// - for sharp s include ss
    /// - for k include the Kelvin sign
    pub(crate) fn add_case_closure<S: ClosureSet>(&self, c: char, set: &mut S) {
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
                    // GIGO: delta should be valid
                    let mapped = char::from_u32(codepoint as u32).unwrap_or(c);
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

    /// Maps the string to single code points and adds the associated case closure
    /// mappings.
    ///
    /// (see docs on CaseMapping::add_string_case_closure)
    pub(crate) fn add_string_case_closure<S: ClosureSet>(&self, s: &str, set: &mut S) -> bool {
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
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum CaseMapLocale {
    Root,
    Turkish,
    Lithuanian,
    Greek,
    Dutch,
    Armenian,
}

impl CaseMapLocale {
    pub const fn from_locale(loc: &Locale) -> Self {
        use icu_locid::{subtags::Language, subtags_language as language};
        const TR: Language = language!("tr");
        const AZ: Language = language!("az");
        const LT: Language = language!("lt");
        const EL: Language = language!("el");
        const NL: Language = language!("nl");
        const HY: Language = language!("hy");
        match loc.id.language {
            TR | AZ => Self::Turkish,
            LT => Self::Lithuanian,
            EL => Self::Greek,
            NL => Self::Dutch,
            HY => Self::Armenian,
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

pub(crate) struct ContextIterator<'a> {
    before: &'a str,
    after: &'a str,
}

impl<'a> ContextIterator<'a> {
    // Returns a context iterator with the characters before
    // and after the character at a given index, given the preceding
    // string and the succeding string including the character itself
    pub fn new(before: &'a str, char_and_after: &'a str) -> Self {
        let mut char_and_after = char_and_after.chars();
        char_and_after.next(); // skip the character itself
        let after = char_and_after.as_str();
        Self { before, after }
    }

    fn preceded_by_soft_dotted(&self, mapping: &CaseMappingV1) -> bool {
        for c in self.before.chars().rev() {
            match mapping.dot_type(c) {
                DotType::SoftDotted => return true,
                DotType::OtherAccent => continue,
                _ => return false,
            }
        }
        false
    }
    fn preceded_by_capital_i(&self, mapping: &CaseMappingV1) -> bool {
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
    fn preceded_by_cased_letter(&self, mapping: &CaseMappingV1) -> bool {
        for c in self.before.chars().rev() {
            let data = mapping.lookup_data(c);
            if !data.is_ignorable() {
                return data.case_type().is_some();
            }
        }
        false
    }
    fn followed_by_cased_letter(&self, mapping: &CaseMappingV1) -> bool {
        for c in self.after.chars() {
            let data = mapping.lookup_data(c);
            if !data.is_ignorable() {
                return data.case_type().is_some();
            }
        }
        false
    }
    fn followed_by_more_above(&self, mapping: &CaseMappingV1) -> bool {
        for c in self.after.chars() {
            match mapping.dot_type(c) {
                DotType::Above => return true,
                DotType::OtherAccent => continue,
                _ => return false,
            }
        }
        false
    }
    fn followed_by_dot_above(&self, mapping: &CaseMappingV1) -> bool {
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
