// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[allow(clippy::indexing_slicing, clippy::unwrap_used)] // TODO(#3958): Remove.
mod replaceable;

use crate::provider::{FunctionCall, Rule, RuleULE, SimpleId, VarTable};
use crate::provider::{RuleBasedTransliterator, Segment, TransliteratorRulesV1Marker};
use crate::TransliteratorError;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt::Debug;
use core::ops::Range;
use core::str;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_locid::Locale;
use icu_normalizer::provider::*;
use icu_normalizer::{ComposingNormalizer, DecomposingNormalizer};
use icu_provider::prelude::*;
use litemap::LiteMap;
use replaceable::*;
use zerofrom::ZeroFrom;
use zerovec::VarZeroSlice;

type Filter<'a> = CodePointInversionList<'a>;

// Thought: How about a RunTransliterator trait that is implemented for all internal types, is blanket
//  implemented for CustomTransliterator, and maybe is exposed to users if they want more control?
//  Actually, an alternative would be: CustomTransliterator is just &str -> String, RunTransliterator is
//  (&str, allowed_range) -> String, and some RepTransliterator would just be Replaceable -> ().

/// A type that supports transliteration. Used for overrides in [`Transliterator`] - see
/// [`Transliterator::try_new_with_override`].
pub trait CustomTransliterator: Debug {
    /// Transliterates the portion of the input string specified by the byte indices in the range.
    ///
    /// The returned `String` must just be the transliteration of `input[range]`. The rest is
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

#[derive(Debug)]
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
            Self::Remove => rep.replace_modifiable_with_str(""),
            Self::Dyn(custom) => {
                let replacement = custom.transliterate(rep.as_str(), rep.allowed_range());
                rep.replace_modifiable_with_str(&replacement)
            }
        }
    }
}

type Env = LiteMap<String, InternalTransliterator>;

/// A `Transliterator` allows transliteration based on [UTS #35 transform rules](https://unicode.org/reports/tr35/tr35-general.html#Transforms),
/// including overrides with custom implementations.
#[derive(Debug)]
pub struct Transliterator {
    transliterator: DataPayload<TransliteratorRulesV1Marker>,
    env: Env,
}

impl Transliterator {
    /// Construct a [`Transliterator`] from the given [`Locale`].
    ///
    /// # Examples
    /// ```
    /// use icu_transliteration::Transliterator;
    /// // BCP-47-T ID for Bengali to Arabic transliteration
    /// let locale = "und-Arab-t-und-beng".parse().unwrap();
    /// let t = Transliterator::try_new(locale).unwrap();
    /// let output = t.transliterate("অকার্যতানাযা".to_string());
    ///
    /// assert_eq!(output, "اكاريتانايا");
    /// ```
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

    /// Construct a [`Transliterator`] from the given [`Locale`] using overrides provided
    /// by `lookup`.
    ///
    /// This allows clients to override the nested transliterators used by this transliterator.
    /// Any nested transliterator will first try to be loaded with `lookup`, and only fall back
    /// to the nested transliterator defined by the data if it returns `None`.
    /// See [`CustomTransliterator`].
    ///
    /// # Example
    /// Overriding `"de-t-de-d0-ascii"`'s dependency on `"und-t-und-Latn-d0-ascii"`:
    /// ```
    /// use icu_transliteration::{Transliterator, CustomTransliterator};
    /// use icu_locid::Locale;
    /// use core::ops::Range;
    ///
    /// #[derive(Debug)]
    /// struct FunkyGermanToAscii;
    /// impl CustomTransliterator for FunkyGermanToAscii {
    ///     fn transliterate(&self, input: &str, allowed_range: Range<usize>) -> String {
    ///         input[allowed_range].replace("oeverride", "overridden")
    ///     }
    /// }
    ///
    /// let override_locale: Locale = "und-t-und-Latn-d0-ascii".parse().unwrap();
    /// let lookup = |lookup_locale: &Locale| -> Option<Box<dyn CustomTransliterator>> {
    ///     override_locale.eq(lookup_locale).then_some(Box::new(FunkyGermanToAscii))
    /// };
    ///
    /// let locale = "de-t-de-d0-ascii".parse().unwrap();
    /// let t = Transliterator::try_new_with_override(locale, lookup).unwrap();
    /// let output = t.transliterate("This is an överride example".to_string());
    ///
    /// assert_eq!(output, "This is an overridden example");
    /// ```
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

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Transliterator::try_new)]
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

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Transliterator::try_new_with_override)]
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

    fn internal_try_new_with_override_unstable<PN, PT, F>(
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
        let payload = Transliterator::load_rbt(&locale.to_string(), transliterator_provider)?;
        let rbt = payload.get();
        if !rbt.visibility {
            // transliterator is internal
            return Err(TransliteratorError::InternalOnly);
        }
        let mut env = LiteMap::new();
        Transliterator::load_dependencies_recursive(
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

    fn load_dependencies_recursive<PT, PN, F>(
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
                // Load the dependency.
                // 1. Insert a placeholder to avoid infinite recursion.
                env.insert(dep.to_string(), InternalTransliterator::Null);
                // 2. Load the transliterator, by checking
                // a) hardcoded specials
                // b) the user-provided override
                // c) the data
                let internal_t = dep
                    .strip_prefix("x-")
                    .map(|special| Transliterator::load_special(special, normalizer_provider))
                    .or_else(|| {
                        lookup.and_then(|lookup| {
                            Transliterator::load_with_override(dep, lookup).transpose()
                        })
                    })
                    .unwrap_or_else(|| {
                        let rbt = Transliterator::load_rbt(dep, transliterator_provider)?;
                        Ok(InternalTransliterator::RuleBased(rbt))
                    })?;
                if let InternalTransliterator::RuleBased(rbt) = &internal_t {
                    // 3. Recursively load the dependencies of the dependency.
                    Self::load_dependencies_recursive(
                        rbt.get(),
                        env,
                        lookup,
                        transliterator_provider,
                        normalizer_provider,
                    )?;
                }
                // 4. Replace the placeholder with the loaded transliterator.
                env.insert(dep.to_string(), internal_t);
            }
        }
        Ok(())
    }

    fn load_special<P>(
        special: &str,
        normalizer_provider: &P,
    ) -> Result<InternalTransliterator, TransliteratorError>
    where
        P: DataProvider<CanonicalDecompositionDataV1Marker>
            + DataProvider<CompatibilityDecompositionSupplementV1Marker>
            + DataProvider<CanonicalDecompositionTablesV1Marker>
            + DataProvider<CompatibilityDecompositionTablesV1Marker>
            + DataProvider<CanonicalCompositionsV1Marker>
            + ?Sized,
    {
        // TODO(#3909, #3910): add more
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
            s => Err(DataError::custom("unavailable transliterator")
                .with_debug_context(s)
                .into()),
        }
    }

    fn load_with_override<F>(
        id: &str,
        lookup: &F,
    ) -> Result<Option<InternalTransliterator>, TransliteratorError>
    where
        F: Fn(&Locale) -> Option<Box<dyn CustomTransliterator>>,
    {
        let locale: Locale = id.parse().map_err(|e| {
            DataError::custom("invalid data: transliterator dependency is not a valid Locale")
                .with_debug_context(&e)
        })?;
        Ok(lookup(&locale).map(InternalTransliterator::Dyn))
    }

    fn load_rbt<P>(
        id: &str,
        provider: &P,
    ) -> Result<DataPayload<TransliteratorRulesV1Marker>, DataError>
    where
        P: DataProvider<TransliteratorRulesV1Marker> + ?Sized,
    {
        let mut data_locale = DataLocale::default();
        data_locale.set_aux(id.parse()?);
        let req = DataRequest {
            locale: &data_locale,
            metadata: Default::default(),
        };
        let payload = provider.load(req)?.take_payload()?;
        let rbt = payload.get();
        if rbt.id_group_list.len() != rbt.rule_group_list.len() {
            return Err(DataError::custom(
                "invalid data: id_group_list and rule_group_list have different lengths",
            ));
        }
        Ok(payload)
    }

    // Before stabilization, consider the input type we want to accept here. We might want to
    // use a data structure internally that works nicely with a &str, but if we don't, a String
    // is good to accept because the user might already have one.
    /// Transliterates `input` and returns its transliteration.
    pub fn transliterate(&self, input: String) -> String {
        // Thought: Seems too much work for the benefits, but maybe have a Cow buffer instead?
        //  Insertable would only actually to_owned if the replaced bytes differ from the ones already there
        let mut buffer = TransliteratorBuffer::from_string(input);
        let rep = Replaceable::new(&mut buffer);
        self.transliterator.get().transliterate(rep, &self.env);
        buffer.into_string()
    }
}

impl<'a> RuleBasedTransliterator<'a> {
    /// Transliteration using rules works as follows:
    ///  1. Split the input modifiable range of the Replaceable according into runs according to self.filter
    ///  2. Transliterate each run in sequence
    ///      i. Transliterate the first id_group, then the first rule_group, then the second id_group, etc.
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        // assumes the cursor is at the right position.

        rep.for_each_run(&self.filter, |run| {
            // eprintln!("got RBT filtered_run: {run:?}");
            for (id_group, rule_group) in self.id_group_list.iter().zip(self.rule_group_list.iter())
            {
                // first handle id_group
                for single_id in id_group.iter() {
                    let id = SimpleId::zero_from(single_id);
                    id.transliterate(run.child(), env);
                }

                // then handle rule_group
                let rule_group = RuleGroup::from(rule_group);
                rule_group.transliterate(run.child(), &self.variable_table, env);
            }
            // eprintln!("finished RBT filtered_run transliteration: {run:?}")
        });
    }
}

impl<'a> SimpleId<'a> {
    fn transliterate(&self, mut rep: Replaceable, env: &Env) {
        // eprintln!("transliterating SimpleId: {self:?}");
        let Some(inner) = env.get(self.id.as_ref()) else {
            debug_assert!(false, "missing transliterator {}", &self.id);
            // GIGO behavior, missing recursive transliterator is a noop
            return;
        };
        rep.for_each_run(&self.filter, |run| {
            // eprintln!("transliterating SimpleId run: {rep:?}");
            inner.transliterate(run.child(), env)
        })
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

        // note that we're stopping transliteration at the end _even though empty rules might still match_.
        // this behavior is copied from ICU4C/J.
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
    /// current run.
    fn apply(&self, mut dest: Insertable, data: MatchData, vt: &VarTable, env: &Env) {
        // note: this could be precomputed ignoring segments and function calls.
        // A `rule.used_segments` ZeroVec<u16> could be added at compile time,
        // which would make it easier to take segments into account at runtime.
        // there is no easy way to estimate the size of function calls, though.
        // TODO(#3957): benchmark with and without this optimization
        let replacement_size_estimate = estimate_replacement_size(&self.replacer, &data, vt);

        dest.apply_size_hint(replacement_size_estimate);

        replace_str_with_specials(&self.replacer, &mut dest, &data, vt, env);
    }

    /// Returns `None` if there is no match. If there is a match, returns the associated
    /// [`MatchData`] and [`RepMatcher`].
    // Thought: RepMatcher<true> could be "FinishedRepMatcher"? but we can still match post..
    fn matches<'r1, 'r2>(
        &self,
        mut matcher: RepMatcher<'r1, 'r2, false>,
        vt: &VarTable,
    ) -> Option<(MatchData, RepMatcher<'r1, 'r2, true>)> {
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

    /// Returns whether the ante context matches or not. Fills in `match_data` if applicable.
    ///
    /// This uses reverse matching.
    fn ante_matches(
        &self,
        matcher: &mut impl Utf8Matcher<Reverse>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        if self.ante.is_empty() {
            // fast path for empty queries, which always match
            return true;
        }
        rev_match_str_with_specials(&self.ante, matcher, match_data, vt)
    }

    /// Returns whether the post context matches or not. Fills in `match_data` if applicable.
    ///
    /// This uses forward matching.
    fn post_matches(
        &self,
        matcher: &mut impl Utf8Matcher<Forward>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        if self.post.is_empty() {
            // fast path for empty queries, which always match
            return true;
        }
        match_str_with_specials(&self.post, matcher, match_data, vt)
    }

    /// Returns whether the post context matches or not. Fills in `match_data` if applicable.
    ///
    /// This uses forward matching.
    fn key_matches(
        &self,
        matcher: &mut impl Utf8Matcher<Forward>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        if self.key.is_empty() {
            // fast path for empty queries, which always match
            return true;
        }
        match_str_with_specials(&self.key, matcher, match_data, vt)
    }
}

/// Returns the index of the first special construct that is encoded as a private use char in `s`,
/// if there is one. Returns `None` if the passed string is pure
/// (contains no encoded special constructs).
fn find_special(s: &str) -> Option<usize> {
    // note: full-match (i.e., if this function returns Some(_) or None, could be
    // precomputed + stored at datagen time (there could eg be a reserved char that is at the
    // start/end of key <=> key is completely pure)
    s.char_indices()
        .find(|(_, c)| VarTable::ENCODE_RANGE.contains(c))
        .map(|(i, _)| i)
}

/// Returns the index of the char to the right of the first (from the right) special construct
/// encoded as a private use char. Returns `None` if the passed string is pure
/// (contains no encoded special constructs).
fn rev_find_special(s: &str) -> Option<usize> {
    s.char_indices()
        .rfind(|(_, c)| VarTable::ENCODE_RANGE.contains(c))
        .map(|(i, c)| i + c.len_utf8())
}

/// Recursively estimates the size of the replacement string.
fn estimate_replacement_size(replacement: &str, data: &MatchData, vt: &VarTable) -> usize {
    let mut size;
    let replacement_tail;

    match find_special(replacement) {
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
                debug_assert!(false, "invalid encoded special {:?}", rep_c);
                // GIGO behavior. we just skip invalid encodings
                continue;
            }
        };
        size += replacer.estimate_size(data, vt);
    }

    size
}

/// Applies the replacements from the `replacement`, which may contain encoded special replacers, to `dest`,
/// including non-default cursor updates.
fn replace_str_with_specials(
    replacement: &str,
    dest: &mut Insertable,
    data: &MatchData,
    vt: &VarTable,
    env: &Env,
) {
    let replacement = match find_special(replacement) {
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
                debug_assert!(false, "invalid encoded special {:?}", rep_c);
                // GIGO behavior. we just skip invalid encodings
                continue;
            }
        };
        replacer.replace(dest, data, vt, env);
    }
}

/// Tries to match `query`, which may contain encoded special matchers, on `matcher`. Fills in `match_data` if applicable.
fn match_str_with_specials(
    query: &str,
    matcher: &mut impl Utf8Matcher<Forward>,
    match_data: &mut MatchData,
    vt: &VarTable,
) -> bool {
    // eprintln!("trying to match query {query:?} on input {matcher:?}");

    let query = match find_special(query) {
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
                debug_assert!(false, "invalid encoded special {:?}", query_c);
                // GIGO behavior. we just skip invalid encodings
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

/// Tries to match `query`, which may contain encoded special matchers, on `matcher` from the right. Fills in `match_data` if applicable.
fn rev_match_str_with_specials(
    query: &str,
    matcher: &mut impl Utf8Matcher<Reverse>,
    match_data: &mut MatchData,
    vt: &VarTable,
) -> bool {
    let query = match rev_find_special(query) {
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
                debug_assert!(false, "invalid encoded special {:?}", query_c);
                // GIGO behavior. we just skip invalid encodings
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
    // Thought: a lot of duplicated code in matches and rev_matches. deduplicate.
    //  maybe by being generic over Direction? doesn't work for some special cases, though, like segments and sets

    /// Returns whether there is a match or not. Fills in `match_data` if applicable.
    fn matches(
        &self,
        matcher: &mut impl Utf8Matcher<Forward>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        match self {
            Self::Compound(query) => match_str_with_specials(query, matcher, match_data, vt),
            Self::UnicodeSet(set) => {
                // eprintln!("checking if set {set:?} matches input {matcher:?}");

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
                // Thought: Would it be cool to have a similar functionality as Insertable::start_replaceable_adapter
                //  that takes care of this `start`/`end` shenanigans? here it's not a safety issue, merely a panic issue.
                let start = matcher.cursor();
                if !match_str_with_specials(&segment.content, matcher, match_data, vt) {
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
                    if !match_str_with_specials(query, matcher, match_data, vt) {
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

    /// Returns whether there is a match or not. Fills in `match_data` if applicable.
    fn rev_matches(
        &self,
        matcher: &mut impl Utf8Matcher<Reverse>,
        match_data: &mut MatchData,
        vt: &VarTable,
    ) -> bool {
        match self {
            Self::Compound(query) => rev_match_str_with_specials(query, matcher, match_data, vt),
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
                if !rev_match_str_with_specials(&segment.content, matcher, match_data, vt) {
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
                    if !rev_match_str_with_specials(query, matcher, match_data, vt) {
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
            Self::Compound(replacer) => estimate_replacement_size(replacer, data, vt),
            Self::FunctionCall(call) => {
                // this is the only inexact case, so we estimate that the transliteration will stay
                // roughly the same size as the input
                estimate_replacement_size(&call.arg, data, vt)
            }
            &Self::BackReference(num) => data.get_segment(num as usize).len(),
            Self::LeftPlaceholderCursor(_) | Self::RightPlaceholderCursor(_) | Self::PureCursor => {
                0
            }
        }
    }

    /// Applies the replacement from this replacer to `dest`. Also applies any updates to the cursor.
    fn replace(&self, dest: &mut Insertable, data: &MatchData, vt: &VarTable, env: &Env) {
        match self {
            Self::Compound(replacer) => replace_str_with_specials(replacer, dest, data, vt, env),
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
                replace_str_with_specials(&call.arg, &mut range_aggregator, data, vt, env);

                call.translit
                    .transliterate(range_aggregator.as_replaceable().child(), env);
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
    fn into_replacer(self) -> Option<SpecialReplacer<'a>> {
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

    fn into_matcher(self) -> Option<SpecialMatcher<'a>> {
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
    fn lookup(&'a self, query: char) -> Option<VarTableElement<'a>> {
        match query {
            Self::BASE..=Self::MAX_DYNAMIC => {}
            Self::RESERVED_PURE_CURSOR => return Some(VarTableElement::PureCursor),
            Self::RESERVED_ANCHOR_END => return Some(VarTableElement::AnchorEnd),
            Self::RESERVED_ANCHOR_START => return Some(VarTableElement::AnchorStart),
            _ => return None,
        };
        let idx = query as u32 - Self::BASE as u32;
        let mut idx = idx as usize;

        // TODO(#3957): might it be worth trying to speed up these lookups by binary searching?
        // TODO(#3957): we can special-case lookup_matcher, lookup_replacer. lookup_matcher does not need
        //  to check past UnicodeSets, lookup_replacer needs to check the range for Compounds, then skip
        //  quantifiers, segments, unicodesets completely, then continue with segments, function calls,
        //  cursors and backreferences
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
        elt.into_matcher()
    }

    fn lookup_replacer(&'a self, query: char) -> Option<SpecialReplacer<'a>> {
        let elt = self.lookup(query)?;
        elt.into_replacer()
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
        #[derive(Debug)]
        struct MaoamTranslit;
        impl CustomTransliterator for MaoamTranslit {
            fn transliterate(&self, input: &str, range: Range<usize>) -> String {
                let input = &input[range];
                input.replace('ꜵ', "maoam")
            }
        }

        let want_locale = "und-t-und-latn-d0-ascii".parse().unwrap();
        let t =
            Transliterator::try_new_with_override("de-t-de-d0-ascii".parse().unwrap(), |locale| {
                locale.eq(&want_locale).then_some(Box::new(MaoamTranslit))
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
