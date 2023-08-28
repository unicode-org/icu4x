// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

mod replaceable;

use core::ops::Range;

use crate::provider::{RuleBasedTransliterator, Segment, TransliteratorRulesV1Marker};
use crate::TransliteratorError;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use core::str;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_provider::_internal::locid::Locale;
use icu_provider::{
    AsDynamicDataProviderAnyMarkerWrap, DataError, DataLocale, DataPayload, DataProvider,
    DataRequest,
};

use litemap::LiteMap;
use replaceable::*;

use crate::provider::{FunctionCall, Rule, RuleULE, SimpleId, VarTable};
use alloc::vec::Vec;
use core::fmt::{Debug, Formatter};
use core::ops::RangeInclusive;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_normalizer::provider::*;
use icu_normalizer::{ComposingNormalizer, DecomposingNormalizer};
use std::collections::VecDeque;
use std::iter::Rev;
use zerofrom::ZeroFrom;
use zerovec::VarZeroSlice;

type Filter<'a> = CodePointInversionList<'a>;

// TODO: How about a RunTransliterator trait that is implemented for all internal types, is blanket
//  implemented for CustomTransliterator, and maybe is exposed to users if they want more control?

pub trait CustomTransliterator {
    /// Transliterate the portion of the input string specified by the byte indices in the range.
    ///
    /// The returned `String` should just be the transliteration of `input[range]`. The rest is
    /// there for context, if necessary.
    fn transliterate(&self, input: &str, range: Range<usize>) -> String;
}

#[derive(Debug)]
struct ComposingTransliterator(ComposingNormalizer);

impl ComposingTransliterator {
    fn try_nfc<P>(provider: &P) -> Result<Self, TransliteratorError>
    where
        P: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
    {
        let inner = ComposingNormalizer::try_new_nfc_unstable(provider)
            .map_err(|e| DataError::custom("failed to load NFC").with_debug_context(&e))?;
        Ok(Self(inner))
    }

    fn try_nfkc<P>(provider: &P) -> Result<Self, TransliteratorError>
    where
        P: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
    {
        let inner = ComposingNormalizer::try_new_nfkc_unstable(provider)
            .map_err(|e| DataError::custom("failed to load NFKC").with_debug_context(&e))?;
        Ok(Self(inner))
    }

    fn transliterate(&self, mut rep: Replaceable, _env: &Env) {
        // would be cool to use `normalize_to` and pass Insertable, but we need to know the
        // input string, which gets replaced by the normalized string.

        let buf = self.0.normalize(rep.as_str_modifiable());
        rep.replace_modifiable_with_str(&buf);
    }
}

// TODO: Deduplicate with ComposingTransliterator?
#[derive(Debug)]
struct DecomposingTransliterator(DecomposingNormalizer);

impl DecomposingTransliterator {
    fn try_nfd<P>(provider: &P) -> Result<Self, TransliteratorError>
    where
        P: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + ?Sized,
    {
        let inner = DecomposingNormalizer::try_new_nfd_unstable(provider)
            .map_err(|e| DataError::custom("failed to load NFD").with_debug_context(&e))?;
        Ok(Self(inner))
    }

    fn try_nfkd<P>(provider: &P) -> Result<Self, TransliteratorError>
    where
        P: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + ?Sized,
    {
        let inner = DecomposingNormalizer::try_new_nfkd_unstable(provider)
            .map_err(|e| DataError::custom("failed to load NFKD").with_debug_context(&e))?;
        Ok(Self(inner))
    }

    fn transliterate(&self, mut rep: Replaceable, _env: &Env) {
        // would be cool to use `normalize_to` and pass Insertable, but we need to know the
        // input string, which gets replaced by the normalized string.

        let buf = self.0.normalize(rep.as_str_modifiable());
        rep.replace_modifiable_with_str(&buf);
    }
}

enum InternalTransliterator {
    RuleBased(DataPayload<TransliteratorRulesV1Marker>),
    Composing(ComposingTransliterator),
    Decomposing(DecomposingTransliterator),
    Null,
    Remove,
    Dyn(Box<dyn CustomTransliterator>),
}

impl InternalTransliterator {
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        match self {
            Self::RuleBased(rbt) => rbt.get().transliterate(rep, env),
            // TODO(#3910): internal hardcoded transliterators
            Self::Composing(t) => t.transliterate(rep, env),
            Self::Decomposing(t) => t.transliterate(rep, env),
            Self::Null => (),
            Self::Remove => {
                rep.replace_modifiable_with_str("")
            }
            Self::Dyn(custom) => {
                let replacement = custom.transliterate(rep.as_str(), rep.allowed_range());
                rep.replace_modifiable_with_str(&replacement)
            }
        }
    }
}

impl Debug for InternalTransliterator {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        #[derive(Debug)]
        enum DebugInternalTransliterator<'a> {
            RuleBased(&'a DataPayload<TransliteratorRulesV1Marker>),
            ComposingTransliterator(&'a ComposingTransliterator),
            DecomposingTransliterator(&'a DecomposingTransliterator),
            Null,
            Remove,
            Dyn,
        }
        let d = match self {
            Self::RuleBased(rbt) => DebugInternalTransliterator::RuleBased(rbt),
            Self::Composing(t) => DebugInternalTransliterator::ComposingTransliterator(t),
            Self::Decomposing(t) => DebugInternalTransliterator::DecomposingTransliterator(t),
            Self::Null => DebugInternalTransliterator::Null,
            Self::Remove => DebugInternalTransliterator::Remove,
            Self::Dyn(_) => DebugInternalTransliterator::Dyn,
        };
        d.fmt(f)
    }
}

type Env = LiteMap<String, InternalTransliterator>;

#[derive(Debug)]
pub struct Transliterator {
    transliterator: DataPayload<TransliteratorRulesV1Marker>,
    env: Env,
}

// TODO: Think about non-internal signatures returning a Writeable instead that could be written
//  to Insertable, perhaps? or Replaceable?

impl Transliterator {
    #[cfg(feature = "compiled_data")]
    pub fn try_new(locale: Locale) -> Result<Transliterator, TransliteratorError> {
        let provider = crate::provider::Baked;
        let normalizer_provider = icu_normalizer::provider::Baked;
        Self::internal_try_new_with_override_unstable(
            locale,
            None::<&fn(&Locale) -> Option<Box<dyn CustomTransliterator>>>,
            &provider,
            &normalizer_provider,
        )
    }

    #[cfg(feature = "compiled_data")]
    pub fn try_new_with_override<F>(
        locale: Locale,
        lookup: F,
    ) -> Result<Transliterator, TransliteratorError>
    where
        F: Fn(&Locale) -> Option<Box<dyn CustomTransliterator>>,
    {
        let provider = crate::provider::Baked;
        let normalizer_provider = icu_normalizer::provider::Baked;
        Self::internal_try_new_with_override_unstable(
            locale,
            Some(&lookup),
            &provider,
            &normalizer_provider,
        )
    }

    pub fn try_new_unstable<P>(
        locale: Locale,
        provider: &P,
    ) -> Result<Transliterator, TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>
            + DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
    {
        Self::internal_try_new_with_override_unstable(
            locale,
            None::<&fn(&Locale) -> Option<Box<dyn CustomTransliterator>>>,
            provider,
            provider,
        )
    }

    pub fn try_new_with_override_unstable<P, F>(
        locale: Locale,
        lookup: F,
        provider: &P,
    ) -> Result<Transliterator, TransliteratorError>
    where
        P: DataProvider<TransliteratorRulesV1Marker>
            + DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
        F: Fn(&Locale) -> Option<Box<dyn CustomTransliterator>>,
    {
        Self::internal_try_new_with_override_unstable(locale, Some(&lookup), provider, provider)
    }

    pub fn internal_try_new_with_override_unstable<PN, PT, F>(
        locale: Locale,
        lookup: Option<&F>,
        transliterator_provider: &PT,
        normalizer_provider: &PN,
    ) -> Result<Transliterator, TransliteratorError>
    where
        PT: DataProvider<TransliteratorRulesV1Marker> + ?Sized,
        PN: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
        F: Fn(&Locale) -> Option<Box<dyn CustomTransliterator>>,
    {
        debug_assert!(!locale.extensions.transform.is_empty());

        let mut data_locale = DataLocale::default();
        data_locale.set_aux(locale.to_string().parse()?);
        let req = DataRequest {
            locale: &data_locale,
            metadata: Default::default(),
        };
        let payload = transliterator_provider.load(req)?.take_payload()?;
        let rbt = payload.get();
        if !rbt.visibility {
            // transliterator is internal
            return Err(TransliteratorError::InternalOnly);
        }
        let mut env = LiteMap::new();
        Transliterator::load_dependencies(
            rbt,
            &mut env,
            lookup,
            transliterator_provider,
            normalizer_provider,
        )?;
        Ok(Transliterator {
            transliterator: payload,
            env,
        })
    }

    fn load_dependencies<PT, PN, F>(
        rbt: &RuleBasedTransliterator<'_>,
        env: &mut LiteMap<String, InternalTransliterator>,
        lookup: Option<&F>,
        transliterator_provider: &PT,
        normalizer_provider: &PN,
    ) -> Result<(), TransliteratorError>
    where
        PT: DataProvider<TransliteratorRulesV1Marker> + ?Sized,
        PN: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
        F: Fn(&Locale) -> Option<Box<dyn CustomTransliterator>>,
    {
        for dep in rbt.dependencies.iter() {
            if !env.contains_key(dep) {
                let internal_t =
                    Self::load_nested(dep, lookup, transliterator_provider, normalizer_provider)?;
                if let InternalTransliterator::RuleBased(rbt) = &internal_t {
                    Self::load_dependencies(
                        rbt.get(),
                        env,
                        lookup,
                        transliterator_provider,
                        normalizer_provider,
                    )?;
                }
                env.insert(dep.to_string(), internal_t);
            }
        }
        Ok(())
    }

    fn load_nested<PT, PN, F>(
        id: &str,
        lookup: Option<&F>,
        transliterator_provider: &PT,
        normalizer_provider: &PN,
    ) -> Result<InternalTransliterator, TransliteratorError>
    where
        PT: DataProvider<TransliteratorRulesV1Marker> + ?Sized,
        PN: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
        F: Fn(&Locale) -> Option<Box<dyn CustomTransliterator>>,
    {
        if let Some(special) = id.strip_prefix("x-") {
            // TODO: add more
            match special {
                "any-nfc" => Ok(InternalTransliterator::Composing(
                    ComposingTransliterator::try_nfc(normalizer_provider)?,
                )),
                "any-nfkc" => Ok(InternalTransliterator::Composing(
                    ComposingTransliterator::try_nfkc(normalizer_provider)?,
                )),
                "any-nfd" => Ok(InternalTransliterator::Decomposing(
                    DecomposingTransliterator::try_nfd(normalizer_provider)?,
                )),
                "any-nfkd" => Ok(InternalTransliterator::Decomposing(
                    DecomposingTransliterator::try_nfkd(normalizer_provider)?,
                )),
                "any-null" => Ok(InternalTransliterator::Null),
                "any-remove" => Ok(InternalTransliterator::Remove),
                _ => {
                    // TODO: remove
                    // eprintln!("unavailable transliterator: {}", id);
                    Ok(InternalTransliterator::Null)
                }
                s => Err(DataError::custom("unavailable transliterator")
                    .with_debug_context(s)
                    .into()),
            }
        } else {
            // this must be a valid -t- locale
            if let Some(lookup) = lookup {
                let locale: Locale = id.parse().map_err(|e| {
                    DataError::custom("internal: transliterator dependency is not a valid Locale")
                        .with_debug_context(&e)
                })?;
                let custom = lookup(&locale);
                if let Some(custom) = custom {
                    return Ok(InternalTransliterator::Dyn(custom));
                }
            }

            let mut data_locale = DataLocale::default();
            data_locale.set_aux(id.parse()?);
            let req = DataRequest {
                locale: &data_locale,
                metadata: Default::default(),
            };
            let rbt = transliterator_provider.load(req)?.take_payload()?;
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

        rep.for_each_run(&self.filter, |run| {
            // eprintln!("got RBT filtered_run: {run:?}");
            self.transliterate_run(run, env);
            // eprintln!("finished RBT filtered_run transliteration: {run:?}")
        });
    }

    /// Transliteration of a single run, i.e., without needing to look at the filter.
    fn transliterate_run(&self, rep: &mut Replaceable, env: &Env) {
        // assumes the cursor is at the right position.
        debug_assert!(
            rep.allowed_range().contains(&rep.cursor()),
            "cursor {} is out of bounds for replaceable {rep:?}",
            rep.cursor()
        );

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
        // eprintln!("transliterating SimpleId: {self:?}");
        rep.for_each_run(&self.filter, |run| self.transliterate_run(run, env))
    }

    fn transliterate_run(&self, rep: &mut Replaceable, env: &Env) {
        // eprintln!("transliterating SimpleId run: {rep:?}");
        match env.get(self.id.as_ref()) {
            None => {
                debug_assert!(false, "missing transliterator {}", &self.id);
                // GIGO behavior, missing recursive transliterator is a noop
            }
            Some(internal_t) => internal_t.transliterate(rep.child(), env),
        }
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

        if self.rules.is_empty() {
            // empty rule group, nothing to do
            return;
        }

        // while the cursor has not reached the end yet, keep trying to apply each rule in order.
        // when a rule matches, apply its replacement and move the cursor according to the replacement

        // TODO: do we allow empty keys? if so, rep.finished() would cause too early of a stop
        //  the cursor would be at the end, but an empty key rule could still force insertion of more
        //  text
        //  I would say yes: https://util.unicode.org/UnicodeJsps/transform.jsp?a=%23+%3A%3A+%5Ba%5D%3B%0D%0A%0D%0Ad+%7B+x%3F+%3E+match+%3B&b=db
        //  So add to gdoc.
        //  Actually, this is what I'm talking about: https://util.unicode.org/UnicodeJsps/transform.jsp?a=%23+%3A%3A+%5Ba%5D%3B%0D%0A%0D%0Ad+%7B+x%3F+%3E+match+%3B&b=d
        //  no empty match at the end.
        'main: while !rep.is_finished() {
            // eprintln!("ongoing RuleGroup transliteration:\n{rep:?}");
            for rule in self.rules.iter() {
                let rule: Rule = Rule::zero_from(rule);
                // eprintln!("trying rule: {rule:?}");
                let matcher = rep.start_match();
                if let Some((data, matcher)) = rule.matches(matcher, vt) {
                    rule.apply(matcher.finish_match(), data, vt, env);
                    // eprintln!("finished applying replacement: {rep:?}");
                    // eprintln!("applied rule!");
                    // rule application is responsible for updating the cursor
                    continue 'main;
                }
            }
            // eprintln!("no rule matched, moving cursor forward");
            // no rule matched, so just move the cursor forward by one code point
            rep.step_cursor();
        }
    }
}

impl<'a> Rule<'a> {
    /// Applies this rule's replacement using the given [`MatchData`]. Updates the cursor of the
    /// given [`Replaceable`].
    fn apply(&self, mut dest: Insertable, data: MatchData, vt: &VarTable, env: &Env) {
        // note: this could be precomputed ignoring segments and function calls.
        // A `rule.used_segments` ZeroVec<u16> could be added at compile time,
        // which would make it easier to take segments into account at runtime.
        // there is no easy way to estimate the size of function calls, though.
        // TODO: benchmark with and without this optimization
        let replacement_size_estimate =
            helpers::estimate_replacement_size(&self.replacer, &data, vt);

        dest.apply_size_hint(replacement_size_estimate);

        helpers::replace_encoded_str(&self.replacer, &mut dest, &data, vt, env);
    }

    /// Returns `None` if there is no match. If there is a match, returns the associated
    /// [`MatchData`].
    fn matches<'r1, 'r2>(&self, mut matcher: RepMatcher<'r1, 'r2, false>, vt: &VarTable) -> Option<(MatchData, RepMatcher<'r1, 'r2, true>)> {
        let mut match_data = MatchData::new();

        if !self.ante_matches(&mut matcher, &mut match_data, vt) {
            return None;
        }

        if !self.key_matches(&mut matcher, &mut match_data, vt) {
            return None;
        }

        let mut matcher = matcher.finish_key();

        if !self.post_matches(&mut matcher, &mut match_data, vt) {
            return None;
        }

        Some((match_data, matcher))
    }

    /// Returns `None` if the ante context does not match. If there is a match, returns the length
    /// of the match. Fills in `match_data` if applicable.
    ///
    /// This must be a right-aligned match, i.e., the input must _end_ with a substring that is a
    /// match for this rule's ante context. We also call right-aligned matches `rev`erse matches.
    fn ante_matches(
        &self,
        matcher: &mut impl Utf8Matcher<Reverse>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        if self.ante.is_empty() {
            // empty queries always match
            return true;
        }
        helpers::rev_match_encoded_str(&self.ante, matcher, match_data, vt)
    }

    /// Returns `None` if the post context does not match. If there is a match, returns the length
    /// of the match. Fills in `match_data` if applicable.
    ///
    /// This must be a left-aligned match, i.e., the input must _start_ with a substring that is a
    /// match for this rule's post context.
    fn post_matches(
        &self,
        matcher: &mut impl Utf8Matcher<Forward>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        if self.post.is_empty() {
            // empty queries always match
            return true;
        }
        helpers::match_encoded_str(&self.post, matcher, match_data, vt)
    }

    /// Returns `None` if the key does not match. If there is a match, returns the length of the
    /// match. Fills in `match_data` if applicable.
    ///
    /// This must be a left-aligned match, i.e., the input must _start_ with a substring that is a
    /// match for this rule's key.
    fn key_matches(
        &self,
        matcher: &mut impl Utf8Matcher<Forward>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        if self.key.is_empty() {
            // empty queries always match
            return true;
        }
        helpers::match_encoded_str(&self.key, matcher, match_data, vt)
    }
}

mod helpers {
    use super::*;

    /// Returns the index of the first encoded char in `s`, if there is one. Returns `None` if the
    /// passed string is pure (contains no encoded special constructs).
    pub(super) fn find_encoded(s: &str) -> Option<usize> {
        // note: full-match (i.e., if this function returns Some(_) or None, could be
        // precomputed + stored at datagen time (there could eg be a reserved char that is at the
        // start/end of key <=> key is completely pure)
        s.char_indices()
            .find(|(_, c)| VarTable::ENCODE_RANGE.contains(c))
            .map(|(i, _)| i)
    }

    /// Returns the index of the char to the right of the first (from the right) encoded char in
    /// `s`, if there is one.
    /// Returns `None` if the passed string is pure (contains no encoded special constructs).
    pub(super) fn rev_find_encoded(s: &str) -> Option<usize> {
        s.char_indices()
            .rfind(|(_, c)| VarTable::ENCODE_RANGE.contains(c))
            .map(|(i, c)| i + c.len_utf8())
    }

    /// Recursively estimates the size of the replacement string.
    pub(super) fn estimate_replacement_size(
        replacement: &str,
        data: &MatchData,
        vt: &VarTable,
    ) -> usize {
        let mut size;
        let replacement_tail;

        match find_encoded(replacement) {
            None => return replacement.len(),
            Some(idx) => {
                size = idx;
                replacement_tail = &replacement[idx..];
            }
        }

        for rep_c in replacement_tail.chars() {
            if !VarTable::ENCODE_RANGE.contains(&rep_c) {
                // regular char
                size += rep_c.len_utf8();
                continue;
            }
            // must be special replacer

            let replacer = match vt.lookup_replacer(rep_c) {
                Some(replacer) => replacer,
                None => {
                    debug_assert!(false, "invalid encoded char");
                    // GIGO behavior. we just skip invalid encoded chars
                    continue;
                }
            };
            size += replacer.estimate_size(data, vt);
        }

        size
    }

    /// Applies the replacements from the encoded `replacement` to `buf`. Returns the offset of the
    /// cursor after the replacement, if a non-default one exists.
    pub(super) fn replace_encoded_str(
        replacement: &str,
        dest: &mut Insertable,
        data: &MatchData,
        vt: &VarTable,
        env: &Env,
    ) {
        let replacement = match find_encoded(replacement) {
            None => {
                // pure string
                dest.push_str(replacement);
                return;
            }
            Some(idx) => {
                dest.push_str(&replacement[..idx]);
                &replacement[idx..]
            }
        };

        for rep_c in replacement.chars() {
            if !VarTable::ENCODE_RANGE.contains(&rep_c) {
                // regular char
                dest.push(rep_c);
                continue;
            }
            // must be special replacer

            let replacer = match vt.lookup_replacer(rep_c) {
                Some(replacer) => replacer,
                None => {
                    debug_assert!(false, "invalid encoded char");
                    // GIGO behavior. we just skip invalid encoded chars
                    continue;
                }
            };
            replacer.replace(dest, data, vt, env);
        }
    }

    /// Tries to match the encoded `query` on `input`. Returns the length of the match, if there is
    /// one. Fills in `match_data` if applicable.
    pub(super) fn match_encoded_str(
        query: &str,
        matcher: &mut impl Utf8Matcher<Forward>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        // eprintln!("trying to match query {query:?} on input {matcher:?}");

        let query = match find_encoded(query) {
            None => {
                // pure string
                return matcher.match_and_consume_str(query);
            }
            Some(idx) => {
                if !matcher.match_and_consume_str(&query[..idx]) {
                    return false;
                }
                &query[idx..]
            }
        };

        // iterate char-by-char, and try to match each char
        // note: might be good to avoid the UTF-8 => char conversion?
        for query_c in query.chars() {
            if !VarTable::ENCODE_RANGE.contains(&query_c) {
                // regular char
                if !matcher.match_and_consume_char(query_c) {
                    return false;
                }
                continue;
            }
            // must be special matcher

            let special_matcher = match vt.lookup_matcher(query_c) {
                Some(matcher) => matcher,
                None => {
                    debug_assert!(false, "invalid encoded char");
                    // GIGO behavior. we just skip invalid encoded chars
                    continue;
                }
            };
            if !special_matcher.matches(matcher, match_data, vt) {
                return false;
            }
        }

        // matched the full query string successfully
        true
    }

    /// Tries to match the encoded `query` on `input` from the right. Returns the length of the
    /// match, if there is one. Fills in `match_data` if applicable.
    pub(super) fn rev_match_encoded_str(
        query: &str,
        matcher: &mut impl Utf8Matcher<Reverse>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        let query = match rev_find_encoded(query) {
            None => {
                // pure string
                return matcher.match_and_consume_str(query);
            }
            Some(idx) => {
                if !matcher.match_and_consume_str(&query[idx..]) {
                    return false;
                }
                &query[..idx]
            }
        };

        // iterate char-by-char, and try to match each char
        // note: might be good to avoid the UTF-8 => char conversion?
        for query_c in query.chars().rev() {
            if !VarTable::ENCODE_RANGE.contains(&query_c) {
                // regular char
                if !matcher.match_and_consume_char(query_c) {
                    return false;
                }
                continue;
            }
            // must be special matcher

            let special_matcher = match vt.lookup_matcher(query_c) {
                Some(matcher) => matcher,
                None => {
                    debug_assert!(false, "invalid encoded char");
                    // GIGO behavior. we just skip invalid encoded chars
                    continue;
                }
            };
            if !special_matcher.rev_matches(matcher, match_data, vt) {
                return false;
            }
        }

        // matched the full query string successfully
        true
    }
}

/// Stores the state for a single conversion rule.
#[derive(Debug)]
struct MatchData {
    /// Stored matches of segments.
    segments: Vec<String>,
}

impl MatchData {
    fn new() -> Self {
        Self {
            segments: Vec::new(),
        }
    }

    fn update_segment(&mut self, i: usize, s: String) {
        if i >= self.segments.len() {
            self.segments.resize_with(i + 1, Default::default);
        }
        self.segments[i] = s;
    }

    fn get_segment(&self, i: usize) -> &str {
        if let Some(s) = self.segments.get(i) {
            return s;
        }
        // two cases: we have not (at runtime) encountered a segment with a high enough index
        // to populate this index, in which case it is defined to be "",
        // or this is GIGO, which is again ""
        ""
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
    Segment(Segment<'a>),
    UnicodeSet(CodePointInversionListAndStringList<'a>),
    AnchorStart,
    AnchorEnd,
}

impl<'a> SpecialMatcher<'a> {
    // TODO: a lot of duplicated code in matches and rev_matches. deduplicate.
    //  maybe by being generic over Input/RevInput? doesn't work for some special cases, though

    /// Returns `None` if the input does not match. If there is a match, returns the length of the
    /// match.
    fn matches(&self, matcher: &mut impl Utf8Matcher<Forward>, match_data: &mut MatchData, vt: &VarTable) -> bool {
        match self {
            Self::Compound(query) => helpers::match_encoded_str(query, matcher, match_data, vt),
            Self::UnicodeSet(set) => {
                // eprintln!("checking if set {set:?} matches input {matcher:?}");

                // TODO: check in which order a unicodeset matches
                //  (chars first? strings first? longest first? shortest first?)
                //  ICU4J: UnicodeSet::matches says strings first, longest first, then chars
                //  TODO ^ add this to gdoc

                if matcher.is_empty() {
                    if set.contains("") {
                        return true;
                    }
                    if set.contains("\u{FFFF}") {
                        if matcher.match_end_anchor() {
                            return true;
                        }
                        if matcher.match_start_anchor() {
                            return true;
                        }
                    }
                    // only an empty string or an anchor could match an empty input
                    return false;
                }

                let mut max_str_match: Option<usize> = None;
                for s in set.strings().iter() {
                    // strings are sorted. we can optimize by early-breaking when we encounter
                    // an `s` that is lexicographically larger than `input`

                    if matcher.match_str(s) {
                        max_str_match = max_str_match.map(|m| m.max(s.len())).or(Some(s.len()));
                        continue;
                    }

                    match (s.chars().next(), matcher.next_char()) {
                        // break early. since s_c is > input_c, we know that s > input, thus all
                        // strings from here on out are > input, and thus cannot match
                        (Some(s_c), Some(input_c)) if s_c > input_c => break,
                        _ => (),
                    }
                }
                if let Some(max) = max_str_match {
                    // some string matched
                    return matcher.consume(max);
                }

                if let Some(input_c) = matcher.next_char() {
                    // eprintln!("checking if set {set:?} contains char {input_c:?}");
                    if set.contains_char(input_c) {
                        // eprintln!("contains!");
                        return matcher.consume(input_c.len_utf8());
                    }
                }

                false
            }
            Self::AnchorEnd => matcher.match_end_anchor(),
            Self::AnchorStart => matcher.match_start_anchor(),
            Self::Segment(segment) => {
                let start = matcher.cursor();
                if !helpers::match_encoded_str(&segment.content, matcher, match_data, vt) {
                    return false;
                }
                let end = matcher.cursor();
                let matched = matcher.str_range(start..end).unwrap();
                // note: at the moment we could just store start..end
                match_data.update_segment(segment.idx as usize, matched.to_string());
                true
            }
            Self::Quantifier(kind, query) => {
                let (min_matches, max_matches) = match kind {
                    QuantifierKind::ZeroOrOne => (0, 1),
                    QuantifierKind::ZeroOrMore => (0, usize::MAX),
                    QuantifierKind::OneOrMore => (1, usize::MAX),
                };

                let mut matches = 0;

                while matches < max_matches {
                    let pre_cursor = matcher.cursor();
                    if !helpers::match_encoded_str(query, matcher, match_data, vt) {
                        break;
                    }
                    let post_cursor = matcher.cursor();
                    matches += 1;
                    if pre_cursor == post_cursor {
                        // no progress was made but there was still a match. this means we could
                        // recurse infinitely. break out of the loop.
                        break;
                    }
                }

                matches >= min_matches
            }
        }
    }

    /// Returns `None` if the input does not match from the right. If there is a match, returns the
    /// length of the match.
    fn rev_matches(&self, matcher: &mut impl Utf8Matcher<Reverse>, match_data: &mut MatchData, vt: &VarTable) -> bool {
        match self {
            Self::Compound(query) => helpers::rev_match_encoded_str(query, matcher, match_data, vt),
            Self::UnicodeSet(set) => {
                // eprintln!("checking if set {set:?} reverse matches input {matcher:?}");

                if matcher.is_empty() {
                    if set.contains("") {
                        return true;
                    }
                    if set.contains("\u{FFFF}") {
                        if matcher.match_end_anchor() {
                            return true;
                        }
                        if matcher.match_start_anchor() {
                            return true;
                        }
                    }
                    // only an empty string or an anchor could match an empty input
                    return false;
                }

                // because we are reverse matching, we cannot do the same early breaking as in
                // forward matching.
                let max_str_match = set
                    .strings()
                    .iter()
                    .filter(|s| matcher.match_str(s))
                    .map(str::len)
                    .max();
                if let Some(max) = max_str_match {
                    // some string matched
                    return matcher.consume(max);
                }

                if let Some(input_c) = matcher.next_char() {
                    // eprintln!("checking if set {set:?} contains char {input_c:?}");
                    if set.contains_char(input_c) {
                        // eprintln!("contains!");
                        return matcher.consume(input_c.len_utf8());
                    }
                }

                false
            }
            Self::AnchorEnd => matcher.match_end_anchor(),
            Self::AnchorStart => matcher.match_start_anchor(),
            Self::Segment(segment) => {
                let end = matcher.cursor();
                if !helpers::rev_match_encoded_str(&segment.content, matcher, match_data, vt) {
                    return false;
                }
                let start = matcher.cursor();
                let matched = &matcher.str_range(start..end).unwrap();
                // note: at the moment we could just store start..end
                match_data.update_segment(segment.idx as usize, matched.to_string());
                true
            }
            Self::Quantifier(kind, query) => {
                let (min_matches, max_matches) = match kind {
                    QuantifierKind::ZeroOrOne => (0, 1),
                    QuantifierKind::ZeroOrMore => (0, usize::MAX),
                    QuantifierKind::OneOrMore => (1, usize::MAX),
                };

                let mut matches = 0;

                while matches < max_matches {
                    let pre_cursor = matcher.cursor();
                    if !helpers::rev_match_encoded_str(query, matcher, match_data, vt) {
                        break;
                    }
                    let post_cursor = matcher.cursor();
                    matches += 1;
                    if pre_cursor == post_cursor {
                        // no progress was made but there was still a match. this means we could
                        // recurse infinitely. break out of the loop.
                        break;
                    }
                }

                matches >= min_matches
            }
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

impl<'a> SpecialReplacer<'a> {
    /// Estimates the size of the replacement string produced by this Replacer.
    fn estimate_size(&self, data: &MatchData, vt: &VarTable) -> usize {
        match self {
            Self::Compound(replacer) => helpers::estimate_replacement_size(replacer, data, vt),
            Self::FunctionCall(call) => {
                // this is the only inexact case, so we estimate that the transliteration will stay
                // roughly the same size as the input
                helpers::estimate_replacement_size(&call.arg, data, vt)
            }
            &Self::BackReference(num) => data.get_segment(num as usize).len(),
            Self::LeftPlaceholderCursor(_) | Self::RightPlaceholderCursor(_) | Self::PureCursor => {
                return 0;
            }
        }
    }

    /// Applies the replacement from this replacer to `buf`. Returns the offset of the cursor after
    /// the replacement, if a non-default one exists.
    fn replace(
        &self,
        dest: &mut Insertable,
        data: &MatchData,
        vt: &VarTable,
        env: &Env,
    ) {
        match self {
            Self::Compound(replacer) => helpers::replace_encoded_str(replacer, dest, data, vt, env),
            Self::PureCursor => dest.set_offset_to_here(),
            &Self::LeftPlaceholderCursor(num) => {
                // must occur at the very end of a replacement
                dest.set_offset_to_chars_off_end(num);
            }
            &Self::RightPlaceholderCursor(num) => {
                // must occur at the very beginning of the replacement
                debug_assert_eq!(
                    dest.curr_replacement_len(),
                    0,
                    "pre-start cursor not the first replacement"
                );
                dest.set_offset_to_chars_off_start(num);
            }
            &Self::BackReference(num) => {
                dest.push_str(data.get_segment(num as usize));
            }
            Self::FunctionCall(call) => {
                // the way function call replacing works is as such:
                // use the same backing buffer as the parent Replaceable, but set
                // the visible content range of the Replaceable appropriately.

                // eprintln!("dest before function call: {dest:?}");

                let mut range_aggregator = dest.start_replaceable_adapter();
                helpers::replace_encoded_str(&call.arg, range_aggregator.child_for_range(), data, vt, env);

                call.translit.transliterate(range_aggregator.as_replaceable().child(), env);
            }
        }
    }
}

enum VarTableElement<'a> {
    Compound(&'a str),
    Quantifier(QuantifierKind, &'a str),
    Segment(Segment<'a>),
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
            Self::Compound(elt) => SpecialReplacer::Compound(elt),
            Self::FunctionCall(elt) => SpecialReplacer::FunctionCall(elt),
            Self::BackReference(elt) => SpecialReplacer::BackReference(elt),
            Self::LeftPlaceholderCursor(elt) => SpecialReplacer::LeftPlaceholderCursor(elt),
            Self::RightPlaceholderCursor(elt) => SpecialReplacer::RightPlaceholderCursor(elt),
            Self::PureCursor => SpecialReplacer::PureCursor,
            _ => return None,
        })
    }

    fn to_matcher(self) -> Option<SpecialMatcher<'a>> {
        Some(match self {
            Self::Compound(elt) => SpecialMatcher::Compound(elt),
            Self::Quantifier(kind, elt) => SpecialMatcher::Quantifier(kind, elt),
            Self::Segment(elt) => SpecialMatcher::Segment(elt),
            Self::UnicodeSet(elt) => SpecialMatcher::UnicodeSet(elt),
            Self::AnchorEnd => SpecialMatcher::AnchorEnd,
            Self::AnchorStart => SpecialMatcher::AnchorStart,
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
            return Some(VarTableElement::Segment(Segment::zero_from(
                &self.segments[idx],
            )));
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
        // cast is guaranteed because query is inside a range of less than 2^16 elements
        Some(VarTableElement::BackReference(idx as u16))
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
    fn test_empty_matches() {
        let cases = [
            ("ax", "amatch"),
            ("a", "a"),
            ("a1", "amatch1"),
            ("b", "b"),
            ("b1", "bmatch1"),
        ];

        let t = Transliterator::try_new("und-t-und-s0-test-d0-test-m0-emtymach".parse().unwrap())
            .unwrap();

        for (input, output) in cases {
            assert_eq!(t.transliterate(input.to_string()), output);
        }
    }

    #[test]
    fn test_recursive_suite() {
        let t = Transliterator::try_new("und-t-und-s0-test-d0-test-m0-rectestr".parse().unwrap())
            .unwrap();

        let input = "XXXabcXXXdXXe";
        let output = "XXXXXXaWORKEDcXXXXXXdXXXXXe";
        assert_eq!(t.transliterate(input.to_string()), output);
    }

    #[test]
    fn test_cursor_placeholders_filters() {
        let t = Transliterator::try_new("und-t-und-s0-test-d0-test-m0-cursfilt".parse().unwrap())
            .unwrap();

        let input = "xa";
        let output = "xb";
        assert_eq!(t.transliterate(input.to_string()), output);
    }

    #[test]
    fn test_functionality() {
        let t =
            Transliterator::try_new("und-t-und-s0-test-d0-test-m0-niels".parse().unwrap()).unwrap();

        let input = "abädefghijkl!";
        let output = "FIfiunremovedtbxyzftbxyzxyzXYZjkT!";
        assert_eq!(t.transliterate(input.to_string()), output);
    }

    #[test]
    fn test_de_ascii() {
        let t = Transliterator::try_new("de-t-de-d0-ascii".parse().unwrap()).unwrap();
        let input =
            "Über ältere Lügner lästern ist sehr a\u{0308}rgerlich. Ja, SEHR ÄRGERLICH! - ꜵ";
        let output =
            "Ueber aeltere Luegner laestern ist sehr aergerlich. Ja, SEHR AERGERLICH! - ao";
        assert_eq!(t.transliterate(input.to_string()), output);
    }

    #[test]
    fn test_override() {
        struct MyT;
        impl CustomTransliterator for MyT {
            fn transliterate(&self, input: &str, range: Range<usize>) -> String {
                let input = &input[range];
                input.replace("ꜵ", "maoam")
            }
        }

        let want_locale = "und-t-und-latn-d0-ascii".parse().unwrap();
        let t =
            Transliterator::try_new_with_override("de-t-de-d0-ascii".parse().unwrap(), |locale| {
                locale.eq(&want_locale).then_some(Box::new(MyT))
            })
            .unwrap();

        let input = "Ich liebe ꜵ über alles";
        let output = "Ich liebe maoam ueber alles";
        assert_eq!(t.transliterate(input.to_string()), output);
    }

    #[test]
    fn test_nfc_nfd() {
        let t = Transliterator::try_new("und-t-und-latn-d0-ascii".parse().unwrap()).unwrap();
        let input = "äa\u{0308}";
        let output = "aa";
        assert_eq!(t.transliterate(input.to_string()), output);
    }
}
