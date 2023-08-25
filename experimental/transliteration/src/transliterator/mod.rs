// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod replaceable;

use core::ops::Range;

use crate::provider::{RuleBasedTransliterator, TransliteratorRulesV1Marker};
use crate::TransliteratorError;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::str;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_provider::_internal::locid::Locale;
use icu_provider::{DataError, DataLocale, DataPayload, DataProvider, DataRequest};

use litemap::LiteMap;
use replaceable::*;

use crate::provider::{FunctionCall, Rule, RuleULE, SimpleId, VarTable};
use alloc::vec::Vec;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use std::ops::RangeInclusive;
use zerofrom::ZeroFrom;
use zerovec::VarZeroSlice;

type Filter<'a> = CodePointInversionList<'a>;

pub trait CustomTransliterator {
    /// Transliterate the portion of the input string specified by the byte indices in the range.
    ///
    /// The returned `String` should just be the transliteration of `input[range]`.
    fn transliterate(&self, input: &str, range: Range<usize>) -> String;
}

struct NFCTransliterator {}

enum InternalTransliterator {
    RuleBased(DataPayload<TransliteratorRulesV1Marker>),
    NFC(NFCTransliterator),
    Dyn(Box<dyn CustomTransliterator>),
}

impl InternalTransliterator {
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        match self {
            Self::RuleBased(rbt) => rbt.get().transliterate(rep, env),
            // TODO(#3910): internal hardcoded transliterators
            Self::NFC(_nfc) => (),
            Self::Dyn(custom) => {
                let replacement = custom.transliterate(rep.as_str(), rep.allowed_range());
                // SAFETY: rep.allowed_range() returns a range with valid UTF-8 bounds, and the bytes are valid UTF-8 as they come from a String
                unsafe { rep.splice(rep.allowed_range(), replacement.as_bytes()) };
            }
        }
    }
}

type Env = LiteMap<String, InternalTransliterator>;

struct Transliterator {
    transliterator: DataPayload<TransliteratorRulesV1Marker>,
    env: Env,
}

impl Transliterator {
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: Locale) -> Result<Transliterator, TransliteratorError> {
        let provider = crate::provider::Baked;
        Self::try_new_unstable(locale, &provider)
    }

    pub fn try_new_unstable<P>(
        locale: Locale,
        provider: &P,
    ) -> Result<Transliterator, TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>,
    {
        debug_assert!(!locale.extensions.transform.is_empty());

        let mut data_locale = DataLocale::default();
        data_locale.set_aux(locale.to_string().parse()?);
        let req = DataRequest {
            locale: &data_locale,
            metadata: Default::default(),
        };
        let payload = provider.load(req)?.take_payload()?;
        let rbt = payload.get();
        if !rbt.visibility {
            // transliterator is internal
            return Err(TransliteratorError::InternalOnly);
        }
        let mut env = LiteMap::new();
        Transliterator::load_dependencies(rbt, &mut env, provider)?;
        Ok(Transliterator {
            transliterator: payload,
            env,
        })
    }

    fn load_dependencies<P>(
        rbt: &RuleBasedTransliterator<'_>,
        env: &mut LiteMap<String, InternalTransliterator>,
        provider: &P,
    ) -> Result<(), TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>,
    {
        for dep in rbt.dependencies.iter() {
            if !env.contains_key(dep) {
                let internal_t = Self::load_nested(dep, provider)?;
                if let InternalTransliterator::RuleBased(rbt) = &internal_t {
                    Self::load_dependencies(rbt.get(), env, provider)?;
                }
                env.insert(dep.to_string(), internal_t);
            }
        }
        Ok(())
    }

    // TODO: add hook for custom
    fn load_nested<P>(id: &str, provider: &P) -> Result<InternalTransliterator, TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>,
    {
        if let Some(special) = id.strip_prefix("x-") {
            // TODO: add more
            match special {
                "Any-NFC" => Ok(InternalTransliterator::NFC(NFCTransliterator {})),
                _ => Ok(InternalTransliterator::NFC(NFCTransliterator {})),
                s => Err(DataError::custom("unavailable transliterator")
                    .with_debug_context(s)
                    .into()),
            }
        } else {
            let mut data_locale = DataLocale::default();
            data_locale.set_aux(id.parse()?);
            let req = DataRequest {
                locale: &data_locale,
                metadata: Default::default(),
            };
            let rbt = provider.load(req)?.take_payload()?;
            Ok(InternalTransliterator::RuleBased(rbt))
        }
    }

    pub fn transliterate(&self, input: String) -> String {
        let mut buffer = input.into_bytes();
        // SAFETY: buffer is constructed from a String
        let rep = unsafe { Replaceable::new(&mut buffer) };
        self.transliterator.get().transliterate(rep, &self.env);
        debug_assert!(str::from_utf8(&buffer[..]).is_ok());
        // SAFETY: Replaceable's invariants ensure that buffer is always valid UTF-8
        unsafe { String::from_utf8_unchecked(buffer) }
    }
}

impl<'a> RuleBasedTransliterator<'a> {
    /// Transliteration using rules works as follows:
    ///  1. Split the input modifiable range of the Replaceable according into runs according to self.filter
    ///  2. Transliterate each run in sequence
    ///      i. Transliterate the first id_group, then the first rule_group, then the second id_group, etc.
    // muster-child signature of internal transliteration
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        // assumes the cursor is at the right position.

        debug_assert_eq!(self.id_group_list.len(), self.rule_group_list.len());
        debug_assert!(rep.allowed_range().contains(&rep.cursor()));

        // TODO: https://unicode-org.atlassian.net/jira/software/c/projects/ICU/issues/ICU-22469
        // TODO: could move the filtered_transliterate functionality (this while loop) on Replaceable, with some generic
        //  T: InternalTransliterator argument maybe?
        while let Some(filtered_run) = unsafe { rep.next_filtered_run(rep.cursor(), &self.filter) }
        {
            let finished_cursor = filtered_run.finished_cursor();
            self.transliterate_run(filtered_run, env);
            unsafe {
                // TODO: the transliteration runs have to assert themselves that they did a complete transliteration
                // SAFETY: finished_cursor() returns a valid UTF-8 index
                rep.set_cursor(finished_cursor);
            }
        }
    }

    /// Transliteration of a single run, i.e., without needing to look at the filter.
    fn transliterate_run(&self, mut rep: Replaceable, env: &Env) {
        // assumes the cursor is at the right position.
        debug_assert!(rep.allowed_range().contains(&rep.cursor()));

        for (id_group, rule_group) in self.id_group_list.iter().zip(self.rule_group_list.iter()) {
            // first handle id_group
            for single_id in id_group.iter() {
                let id = SimpleId::zero_from(single_id);
                id.transliterate(rep.child(), env);
            }

            // then handle rule_group
            let rule_group = RuleGroup::from(rule_group);
            rule_group.transliterate(rep.child(), &self.variable_table, env);
        }
    }
}

impl<'a> SimpleId<'a> {
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        // again split into runs based on self.filter and then transliterate_run
    }
}

struct RuleGroup<'a> {
    rules: &'a VarZeroSlice<RuleULE>,
}

impl<'a> RuleGroup<'a> {
    fn from(rules: &'a VarZeroSlice<RuleULE>) -> Self {
        Self { rules }
    }

    fn transliterate(&self, mut rep: Replaceable, vt: &VarTable, env: &Env) {
        // no need to split into runs, because a RuleGroup has no filters.

        // while the cursor has not reached the end yet, keep trying to apply each rule in order.
        // when a rule matches, apply its replacement and move the cursor according to the replacement

        'main: while !rep.is_finished() {
            for rule in self.rules.iter() {
                let rule: Rule = Rule::zero_from(rule);
                if let Some(data) = rule.matches(&rep, vt) {
                    rule.apply(&mut rep, data, vt, env);
                    // rule application is responsible for updating the cursor
                    continue 'main;
                }
            }
            // no rule matched, so just move the cursor forward by one code point
            rep.step_cursor();
        }
    }
}

impl<'a> Rule<'a> {
    /// Applies this rule's replacement using the given [`MatchData`]. Updates the cursor of the
    /// given [`Replaceable`].
    fn apply(&self, rep: &mut Replaceable, data: MatchData, vt: &VarTable, env: &Env) {
        let mut buf = String::new();
        let replacement_range = rep.cursor()..(rep.cursor() + data.key_match_len);

        helpers::replace_encoded_str(&self.replacer, &mut buf, vt);

        // SAFETY: the range is guaranteed to be valid, as key_match_len is the length of a UTF-8
        // substring. the replacement is guaranteed to be valid UTF-8, as it comes from a String.
        unsafe { rep.splice(replacement_range, buf.as_bytes()) };
    }

    /// Returns `None` if there is no match. If there is a match, returns the associated
    /// [`MatchData`].
    fn matches(&self, rep: &Replaceable, vt: &VarTable) -> Option<MatchData> {
        let mut match_data = MatchData::new();

        let ante_input = rep.as_str_ante();
        if !self.ante_matches(ante_input, &mut match_data, vt) {
            return None;
        }

        let key_input = rep.as_str_key();
        let key_match_len = self.key_matches(key_input, &mut match_data, vt)?;
        match_data.key_match_len = key_match_len;

        let post_input = rep.as_str_post(key_match_len);
        if !self.post_matches(post_input, &mut match_data, vt) {
            return None;
        }

        Some(match_data)
    }

    /// Returns whether there is a match for ante. Fills segments in `match_data` if applicable.
    fn ante_matches(&self, input: &str, match_data: &mut MatchData, vt: &VarTable) -> bool {
        // note: this could be precomputed + stored at datagen time
        // (there could eg be a reserved char that is at the start/end of ante <=> ante is pure)
        if helpers::is_pure(&self.ante) {
            return input.ends_with(self.ante.as_ref());
        }

        false
    }

    /// Returns whether there is a match for post. Fills segments in `match_data` if applicable.
    fn post_matches(&self, input: &str, match_data: &mut MatchData, vt: &VarTable) -> bool {
        helpers::match_encoded_str(&self.post, input, match_data, vt).is_some()
    }

    /// Returns `None` if the key does not match. If there is a match, returns the length of the
    /// match. Fills in `match_data` if applicable.
    fn key_matches(&self, input: &str, match_data: &mut MatchData, vt: &VarTable) -> Option<usize> {
        helpers::match_encoded_str(&self.key, input, match_data, vt)
    }
}

mod helpers {
    use crate::provider::VarTable;
    use crate::transliterator::MatchData;

    pub(super) fn is_pure(s: &str) -> bool {
        s.chars().any(|c| VarTable::ENCODE_RANGE.contains(&c))
    }

    pub(super) fn replace_encoded_str(replacement: &str, buf: &mut String, vt: &VarTable) {
        if is_pure(replacement) {
            buf.push_str(replacement);
            return;
        }
        // TODO: special replacers
    }

    pub(super) fn match_encoded_str(
        query: &str,
        input: &str,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> Option<usize> {
        // note: this could be precomputed + stored at datagen time
        // (there could eg be a reserved char that is at the start/end of key <=> key is pure)
        if is_pure(query) {
            return if input.starts_with(query) {
                Some(query.len())
            } else {
                None
            };
        }

        let mut remaining_input_to_match = input;
        // iterate char-by-char, and try to match each char
        // note: might be good to avoid the UTF-8 => char conversion?
        for query_c in query.chars() {
            if !VarTable::ENCODE_RANGE.contains(&query_c) {
                // regular char
                // note: could have InputMatcher that wraps the &str and has match_and_consume functionality. keeps a ref to the vartable
                let (len, input_c) = remaining_input_to_match.char_indices().next()?;
                if query_c != input_c {
                    return None;
                }
                remaining_input_to_match = &remaining_input_to_match[len..];
                continue;
            }
            // must be special matcher

            let matcher = match vt.lookup_matcher(query_c) {
                Some(matcher) => matcher,
                None => {
                    debug_assert!(false, "invalid encoded char");
                    // GIGO behavior. we just skip invalid encoded chars
                    continue;
                }
            };
            let len = matcher.matches(remaining_input_to_match, match_data, vt)?;
            remaining_input_to_match = &remaining_input_to_match[len..];
        }

        None
    }
}

/// Stores the state for a single conversion rule.
struct MatchData {
    /// The length (in bytes) of the matched key. This portion will be replaced.
    key_match_len: usize,
}

impl MatchData {
    fn new() -> Self {
        Self {
            key_match_len: 0,
        }
    }
}

enum QuantifierKind {
    ZeroOrOne,
    ZeroOrMore,
    OneOrMore,
}

enum SpecialMatcher<'a> {
    Compound(&'a str),
    Quantifier(QuantifierKind, &'a str),
    Segment(&'a str),
    UnicodeSet(CodePointInversionListAndStringList<'a>),
    AnchorStart,
    AnchorEnd,
}

impl<'a> SpecialMatcher<'a> {
    /// Returns `None` if the input does not match. If there is a match, returns the length of the
    /// match.
    fn matches(&self, input: &str, match_data: &mut MatchData, vt: &VarTable) -> Option<usize> {
        match self {
            Self::Compound(query) => helpers::match_encoded_str(query, input, match_data, vt),
            Self::UnicodeSet(set) => {
                // TODO: handle start anchors somehow

                // TODO: check in which order a unicodeset matches
                //  (chars first? strings first? longest first? shortest first?)
                //  ICU4J: UnicodeSet::matches says strings first, longest first, then chars
                //  TODO ^ add this to gdoc

                if input.is_empty() {
                    // the only way an empty input is matched by a set
                    return set.contains("").then_some(0);
                }

                let mut max_str_match: Option<usize> = None;
                for s in set.strings().iter() {
                    // strings are sorted. we can optimize by early-breaking when we encounter
                    // an `s` that is lexicographically larger than `input`

                    if input.starts_with(s) {
                        max_str_match = max_str_match.map(|m| m.max(s.len())).or(Some(s.len()));
                        continue;
                    }

                    match (s.chars().next(), input.chars().next()) {
                        // break early. since s_c is > input_c, we know that s > input, thus all
                        // strings from here on out are > input, and thus cannot match
                        (Some(s_c), Some(input_c)) if s_c > input_c => break,
                        _ => (),
                    }
                }
                if let Some(max) = max_str_match {
                    // some string matched
                    return Some(max);
                }

                let input_c = input.chars().next()?;
                if set.contains_char(input_c) {
                    return Some(input_c.len_utf8());
                }
                None
            }
            // TODO: do more
            _ => None,
        }
    }
}

enum SpecialReplacer<'a> {
    Compound(&'a str),
    FunctionCall(FunctionCall<'a>),
    BackReference(u16),
    LeftPlaceholderCursor(u16),
    RightPlaceholderCursor(u16),
    PureCursor,
}

enum VarTableElement<'a> {
    Compound(&'a str),
    Quantifier(QuantifierKind, &'a str),
    Segment(&'a str),
    UnicodeSet(CodePointInversionListAndStringList<'a>),
    FunctionCall(FunctionCall<'a>),
    BackReference(u16),
    AnchorStart,
    AnchorEnd,
    LeftPlaceholderCursor(u16),
    RightPlaceholderCursor(u16),
    PureCursor,
}

impl<'a> VarTableElement<'a> {
    fn to_replacer(self) -> Option<SpecialReplacer<'a>> {
        Some(match self {
            VarTableElement::Compound(elt) => SpecialReplacer::Compound(elt),
            VarTableElement::FunctionCall(elt) => SpecialReplacer::FunctionCall(elt),
            VarTableElement::BackReference(elt) => SpecialReplacer::BackReference(elt),
            VarTableElement::LeftPlaceholderCursor(elt) => {
                SpecialReplacer::LeftPlaceholderCursor(elt)
            }
            VarTableElement::RightPlaceholderCursor(elt) => {
                SpecialReplacer::RightPlaceholderCursor(elt)
            }
            VarTableElement::PureCursor => SpecialReplacer::PureCursor,
            _ => return None,
        })
    }

    fn to_matcher(self) -> Option<SpecialMatcher<'a>> {
        Some(match self {
            VarTableElement::Compound(elt) => SpecialMatcher::Compound(elt),
            VarTableElement::Quantifier(kind, elt) => SpecialMatcher::Quantifier(kind, elt),
            VarTableElement::Segment(elt) => SpecialMatcher::Segment(elt),
            VarTableElement::UnicodeSet(elt) => SpecialMatcher::UnicodeSet(elt),
            VarTableElement::AnchorEnd => SpecialMatcher::AnchorEnd,
            VarTableElement::AnchorStart => SpecialMatcher::AnchorStart,
            _ => return None,
        })
    }
}

impl<'a> VarTable<'a> {
    // TODO: these must be the same as during datagen. Find some place to define them *once*
    const BASE: u32 = '\u{F0000}' as u32;
    const MAX_DYNAMIC: u32 = '\u{FFFF0}' as u32;
    const RESERVED_PURE_CURSOR: u32 = '\u{FFFFB}' as u32;
    const RESERVED_ANCHOR_START: u32 = '\u{FFFFC}' as u32;
    const RESERVED_ANCHOR_END: u32 = '\u{FFFFD}' as u32;

    const ENCODE_RANGE: RangeInclusive<char> = '\u{F0000}'..='\u{FFFFD}';

    fn lookup(&'a self, query: char) -> Option<VarTableElement<'a>> {
        let query = query as u32;
        if query < Self::BASE {
            return None;
        }
        if query > Self::MAX_DYNAMIC {
            return match query {
                Self::RESERVED_PURE_CURSOR => Some(VarTableElement::PureCursor),
                Self::RESERVED_ANCHOR_END => Some(VarTableElement::AnchorEnd),
                Self::RESERVED_ANCHOR_START => Some(VarTableElement::AnchorStart),
                _ => None,
            };
        }
        let idx = query - Self::BASE;
        let mut idx = idx as usize;

        // TODO: these lookups must be in the same order as during datagen. Best way to enforce this?
        // note: might be worth trying to speed up these lookups by binary searching?
        let mut next_base = self.compounds.len();
        if idx < next_base {
            return Some(VarTableElement::Compound(&self.compounds[idx]));
        }
        // no underflow for all these idx subtractions, as idx is always >= next_base
        idx -= next_base;
        next_base = self.quantifiers_opt.len();
        if idx < next_base {
            return Some(VarTableElement::Quantifier(
                QuantifierKind::ZeroOrOne,
                &self.quantifiers_opt[idx],
            ));
        }
        idx -= next_base;
        next_base = self.quantifiers_kleene.len();
        if idx < next_base {
            return Some(VarTableElement::Quantifier(
                QuantifierKind::ZeroOrMore,
                &self.quantifiers_kleene[idx],
            ));
        }
        idx -= next_base;
        next_base = self.quantifiers_kleene_plus.len();
        if idx < next_base {
            return Some(VarTableElement::Quantifier(
                QuantifierKind::OneOrMore,
                &self.quantifiers_kleene_plus[idx],
            ));
        }
        idx -= next_base;
        next_base = self.segments.len();
        if idx < next_base {
            return Some(VarTableElement::Segment(&self.segments[idx]));
        }
        idx -= next_base;
        next_base = self.unicode_sets.len();
        if idx < next_base {
            return Some(VarTableElement::UnicodeSet(
                CodePointInversionListAndStringList::zero_from(&self.unicode_sets[idx]),
            ));
        }
        idx -= next_base;
        next_base = self.function_calls.len();
        if idx < next_base {
            return Some(VarTableElement::FunctionCall(FunctionCall::zero_from(
                &self.function_calls[idx],
            )));
        }
        idx -= next_base;
        next_base = self.max_left_placeholder_count as usize;
        if idx < next_base {
            // + 1 because index 0 represents 1 placeholder, etc.
            // cast is guaranteed because query is inside a range of less than 2^16 elements
            return Some(VarTableElement::LeftPlaceholderCursor(idx as u16 + 1));
        }
        idx -= next_base;
        next_base = self.max_right_placeholder_count as usize;
        if idx < next_base {
            // + 1 because index 0 represents 1 placeholder, etc.
            // cast is guaranteed because query is inside a range of less than 2^16 elements
            return Some(VarTableElement::RightPlaceholderCursor(idx as u16 + 1));
        }
        idx -= next_base;
        // idx must be a backreference (an u16 encoded as <itself> indices past the last valid other index)
        // + 1 because index 0 represents $1, etc.
        // cast is guaranteed because query is inside a range of less than 2^16 elements
        Some(VarTableElement::BackReference(idx as u16 + 1))
    }

    fn lookup_matcher(&'a self, query: char) -> Option<SpecialMatcher<'a>> {
        let elt = self.lookup(query)?;
        elt.to_matcher()
    }

    fn lookup_replacer(&'a self, query: char) -> Option<SpecialReplacer<'a>> {
        let elt = self.lookup(query)?;
        elt.to_replacer()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let t = Transliterator::try_new("de-t-de-d0-ascii".parse().unwrap()).unwrap();
        let input =
            r"Über ältere Lügner lästern ist sehr a\u{0308}rgerlich. Ja, SEHR ÄRGERLICH! - ꜵ";
        let output =
            "Ueber aeltere Luegner laestern ist sehr aergerlich. Ja, SEHR AERGERLICH! - ao";
        assert_eq!(t.transliterate(input.to_string()), output);
    }
}
