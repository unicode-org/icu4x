use core::borrow::Borrow;

use alloc::boxed::Box;

use crate::regex_automata::{
    hybrid::{
        dfa::{self, DFA},
        error::BuildError,
        OverlappingState,
    },
    nfa::thompson,
    util::{
        matchtypes::{MatchError, MatchKind, MultiMatch},
        prefilter::{self, Prefilter},
    },
};

#[derive(Debug)]
pub struct Regex {
    pre: Option<Box<dyn Prefilter>>,
    forward: DFA,
    reverse: DFA,
    utf8: bool,
}

impl Regex {
    pub fn new(pattern: &str) -> Result<Regex, BuildError> {
        Regex::builder().build(pattern)
    }

    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, BuildError> {
        Regex::builder().build_many(patterns)
    }

    pub fn config() -> Config {
        Config::new()
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn create_cache(&self) -> Cache {
        Cache::new(self)
    }

    pub fn reset_cache(&self, cache: &mut Cache) {
        self.forward().reset_cache(&mut cache.forward);
        self.reverse().reset_cache(&mut cache.reverse);
    }
}

impl Regex {
    pub fn is_match(&self, cache: &mut Cache, haystack: &[u8]) -> bool {
        self.try_is_match(cache, haystack).unwrap()
    }

    pub fn find_earliest(&self, cache: &mut Cache, haystack: &[u8]) -> Option<MultiMatch> {
        self.try_find_earliest(cache, haystack).unwrap()
    }

    pub fn find_leftmost(&self, cache: &mut Cache, haystack: &[u8]) -> Option<MultiMatch> {
        self.try_find_leftmost(cache, haystack).unwrap()
    }

    pub fn find_overlapping(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        state: &mut OverlappingState,
    ) -> Option<MultiMatch> {
        self.try_find_overlapping(cache, haystack, state).unwrap()
    }

    pub fn find_earliest_iter<'r, 'c, 't>(
        &'r self,
        cache: &'c mut Cache,
        haystack: &'t [u8],
    ) -> FindEarliestMatches<'r, 'c, 't> {
        FindEarliestMatches::new(self, cache, haystack)
    }

    pub fn find_leftmost_iter<'r, 'c, 't>(
        &'r self,
        cache: &'c mut Cache,
        haystack: &'t [u8],
    ) -> FindLeftmostMatches<'r, 'c, 't> {
        FindLeftmostMatches::new(self, cache, haystack)
    }

    pub fn find_overlapping_iter<'r, 'c, 't>(
        &'r self,
        cache: &'c mut Cache,
        haystack: &'t [u8],
    ) -> FindOverlappingMatches<'r, 'c, 't> {
        FindOverlappingMatches::new(self, cache, haystack)
    }
}

impl Regex {
    pub fn is_match_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> bool {
        self.try_is_match_at(cache, haystack, start, end).unwrap()
    }

    pub fn find_earliest_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Option<MultiMatch> {
        self.try_find_earliest_at(cache, haystack, start, end)
            .unwrap()
    }

    pub fn find_leftmost_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Option<MultiMatch> {
        self.try_find_leftmost_at(cache, haystack, start, end)
            .unwrap()
    }

    pub fn find_overlapping_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Option<MultiMatch> {
        self.try_find_overlapping_at(cache, haystack, start, end, state)
            .unwrap()
    }
}

impl Regex {
    pub fn try_is_match(&self, cache: &mut Cache, haystack: &[u8]) -> Result<bool, MatchError> {
        self.try_is_match_at(cache, haystack, 0, haystack.len())
    }

    pub fn try_find_earliest(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_earliest_at(cache, haystack, 0, haystack.len())
    }

    pub fn try_find_leftmost(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_leftmost_at(cache, haystack, 0, haystack.len())
    }

    pub fn try_find_overlapping(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        state: &mut OverlappingState,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_overlapping_at(cache, haystack, 0, haystack.len(), state)
    }

    pub fn try_find_earliest_iter<'r, 'c, 't>(
        &'r self,
        cache: &'c mut Cache,
        haystack: &'t [u8],
    ) -> TryFindEarliestMatches<'r, 'c, 't> {
        TryFindEarliestMatches::new(self, cache, haystack)
    }

    pub fn try_find_leftmost_iter<'r, 'c, 't>(
        &'r self,
        cache: &'c mut Cache,
        haystack: &'t [u8],
    ) -> TryFindLeftmostMatches<'r, 'c, 't> {
        TryFindLeftmostMatches::new(self, cache, haystack)
    }

    pub fn try_find_overlapping_iter<'r, 'c, 't>(
        &'r self,
        cache: &'c mut Cache,
        haystack: &'t [u8],
    ) -> TryFindOverlappingMatches<'r, 'c, 't> {
        TryFindOverlappingMatches::new(self, cache, haystack)
    }
}

impl Regex {
    pub fn try_is_match_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<bool, MatchError> {
        self.forward()
            .find_leftmost_fwd_at(
                &mut cache.forward,
                self.scanner().as_mut(),
                None,
                haystack,
                start,
                end,
            )
            .map(|x| x.is_some())
    }

    pub fn try_find_earliest_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_earliest_at_imp(self.scanner().as_mut(), cache, haystack, start, end)
    }

    pub fn try_find_leftmost_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_leftmost_at_imp(self.scanner().as_mut(), cache, haystack, start, end)
    }

    pub fn try_find_overlapping_at(
        &self,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_overlapping_at_imp(
            self.scanner().as_mut(),
            cache,
            haystack,
            start,
            end,
            state,
        )
    }
}

impl Regex {
    #[inline(always)]
    fn try_find_earliest_at_imp(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        let (fdfa, rdfa) = (self.forward(), self.reverse());
        let (fcache, rcache) = (&mut cache.forward, &mut cache.reverse);
        let end = match fdfa.find_earliest_fwd_at(fcache, pre, None, haystack, start, end)? {
            None => return Ok(None),
            Some(end) => end,
        };
        // N.B. The only time we need to tell the reverse searcher the pattern
        // to match is in the overlapping case, since it's ambiguous. In the
        // earliest case, I have tentatively convinced myself that it isn't
        // necessary and the reverse search will always find the same pattern
        // to match as the forward search. But I lack a rigorous proof. Why not
        // just provide the pattern anyway? Well, if it is needed, then leaving
        // it out gives us a chance to find a witness.
        let start = rdfa
            .find_earliest_rev_at(rcache, None, haystack, start, end.offset())?
            .expect("reverse search must match if forward search does");
        assert_eq!(
            start.pattern(),
            end.pattern(),
            "forward and reverse search must match same pattern",
        );
        assert!(start.offset() <= end.offset());
        Ok(Some(MultiMatch::new(
            end.pattern(),
            start.offset(),
            end.offset(),
        )))
    }

    #[inline(always)]
    fn try_find_leftmost_at_imp(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        let (fdfa, rdfa) = (self.forward(), self.reverse());
        let (fcache, rcache) = (&mut cache.forward, &mut cache.reverse);
        let end = match fdfa.find_leftmost_fwd_at(fcache, pre, None, haystack, start, end)? {
            None => return Ok(None),
            Some(end) => end,
        };
        // N.B. The only time we need to tell the reverse searcher the pattern
        // to match is in the overlapping case, since it's ambiguous. In the
        // leftmost case, I have tentatively convinced myself that it isn't
        // necessary and the reverse search will always find the same pattern
        // to match as the forward search. But I lack a rigorous proof. Why not
        // just provide the pattern anyway? Well, if it is needed, then leaving
        // it out gives us a chance to find a witness.
        let start = rdfa
            .find_leftmost_rev_at(rcache, None, haystack, start, end.offset())?
            .expect("reverse search must match if forward search does");
        assert_eq!(
            start.pattern(),
            end.pattern(),
            "forward and reverse search must match same pattern",
        );
        assert!(start.offset() <= end.offset());
        Ok(Some(MultiMatch::new(
            end.pattern(),
            start.offset(),
            end.offset(),
        )))
    }

    #[inline(always)]
    fn try_find_overlapping_at_imp(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        cache: &mut Cache,
        haystack: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Result<Option<MultiMatch>, MatchError> {
        let (fdfa, rdfa) = (self.forward(), self.reverse());
        let (fcache, rcache) = (&mut cache.forward, &mut cache.reverse);
        let end =
            match fdfa.find_overlapping_fwd_at(fcache, pre, None, haystack, start, end, state)? {
                None => return Ok(None),
                Some(end) => end,
            };
        // Unlike the leftmost cases, the reverse overlapping search may match
        // a different pattern than the forward search. See test failures when
        // using `None` instead of `Some(end.pattern())` below. Thus, we must
        // run our reverse search using the pattern that matched in the forward
        // direction.
        let start = rdfa
            .find_leftmost_rev_at(rcache, Some(end.pattern()), haystack, 0, end.offset())?
            .expect("reverse search must match if forward search does");
        assert_eq!(
            start.pattern(),
            end.pattern(),
            "forward and reverse search must match same pattern",
        );
        assert!(start.offset() <= end.offset());
        Ok(Some(MultiMatch::new(
            end.pattern(),
            start.offset(),
            end.offset(),
        )))
    }
}

impl Regex {
    pub fn forward(&self) -> &DFA {
        &self.forward
    }

    pub fn reverse(&self) -> &DFA {
        &self.reverse
    }

    pub fn pattern_count(&self) -> usize {
        assert_eq!(
            self.forward().pattern_count(),
            self.reverse().pattern_count()
        );
        self.forward().pattern_count()
    }

    pub fn prefilter(&self) -> Option<&dyn Prefilter> {
        self.pre.as_ref().map(|x| &**x)
    }

    pub fn set_prefilter(&mut self, pre: Option<Box<dyn Prefilter>>) {
        self.pre = pre;
    }

    fn scanner(&self) -> Option<prefilter::Scanner> {
        self.prefilter().map(prefilter::Scanner::new)
    }
}

#[derive(Debug)]
pub struct FindEarliestMatches<'r, 'c, 't>(TryFindEarliestMatches<'r, 'c, 't>);

impl<'r, 'c, 't> FindEarliestMatches<'r, 'c, 't> {
    fn new(re: &'r Regex, cache: &'c mut Cache, text: &'t [u8]) -> FindEarliestMatches<'r, 'c, 't> {
        FindEarliestMatches(TryFindEarliestMatches::new(re, cache, text))
    }
}

impl<'r, 'c, 't> Iterator for FindEarliestMatches<'r, 'c, 't> {
    type Item = MultiMatch;

    fn next(&mut self) -> Option<MultiMatch> {
        next_unwrap(self.0.next())
    }
}

#[derive(Debug)]
pub struct FindLeftmostMatches<'r, 'c, 't>(TryFindLeftmostMatches<'r, 'c, 't>);

impl<'r, 'c, 't> FindLeftmostMatches<'r, 'c, 't> {
    fn new(re: &'r Regex, cache: &'c mut Cache, text: &'t [u8]) -> FindLeftmostMatches<'r, 'c, 't> {
        FindLeftmostMatches(TryFindLeftmostMatches::new(re, cache, text))
    }
}

impl<'r, 'c, 't> Iterator for FindLeftmostMatches<'r, 'c, 't> {
    type Item = MultiMatch;

    fn next(&mut self) -> Option<MultiMatch> {
        next_unwrap(self.0.next())
    }
}

#[derive(Debug)]
pub struct FindOverlappingMatches<'r, 'c, 't>(TryFindOverlappingMatches<'r, 'c, 't>);

impl<'r, 'c, 't> FindOverlappingMatches<'r, 'c, 't> {
    fn new(
        re: &'r Regex,
        cache: &'c mut Cache,
        text: &'t [u8],
    ) -> FindOverlappingMatches<'r, 'c, 't> {
        FindOverlappingMatches(TryFindOverlappingMatches::new(re, cache, text))
    }
}

impl<'r, 'c, 't> Iterator for FindOverlappingMatches<'r, 'c, 't> {
    type Item = MultiMatch;

    fn next(&mut self) -> Option<MultiMatch> {
        next_unwrap(self.0.next())
    }
}

#[derive(Debug)]
pub struct TryFindEarliestMatches<'r, 'c, 't> {
    re: &'r Regex,
    cache: &'c mut Cache,
    scanner: Option<prefilter::Scanner<'r>>,
    text: &'t [u8],
    last_end: usize,
    last_match: Option<usize>,
}

impl<'r, 'c, 't> TryFindEarliestMatches<'r, 'c, 't> {
    fn new(
        re: &'r Regex,
        cache: &'c mut Cache,
        text: &'t [u8],
    ) -> TryFindEarliestMatches<'r, 'c, 't> {
        let scanner = re.scanner();
        TryFindEarliestMatches {
            re,
            cache,
            scanner,
            text,
            last_end: 0,
            last_match: None,
        }
    }
}

impl<'r, 'c, 't> Iterator for TryFindEarliestMatches<'r, 'c, 't> {
    type Item = Result<MultiMatch, MatchError>;

    fn next(&mut self) -> Option<Result<MultiMatch, MatchError>> {
        if self.last_end > self.text.len() {
            return None;
        }
        let result = self.re.try_find_earliest_at_imp(
            self.scanner.as_mut(),
            self.cache,
            self.text,
            self.last_end,
            self.text.len(),
        );
        let m = match result {
            Err(err) => return Some(Err(err)),
            Ok(None) => return None,
            Ok(Some(m)) => m,
        };
        if m.is_empty() {
            // This is an empty match. To ensure we make progress, start
            // the next search at the smallest possible starting position
            // of the next match following this one.
            self.last_end = if self.re.utf8 {
                crate::regex_automata::util::next_utf8(self.text, m.end())
            } else {
                m.end() + 1
            };
            // Don't accept empty matches immediately following a match.
            // Just move on to the next match.
            if Some(m.end()) == self.last_match {
                return self.next();
            }
        } else {
            self.last_end = m.end();
        }
        self.last_match = Some(m.end());
        Some(Ok(m))
    }
}

#[derive(Debug)]
pub struct TryFindLeftmostMatches<'r, 'c, 't> {
    re: &'r Regex,
    cache: &'c mut Cache,
    scanner: Option<prefilter::Scanner<'r>>,
    text: &'t [u8],
    last_end: usize,
    last_match: Option<usize>,
}

impl<'r, 'c, 't> TryFindLeftmostMatches<'r, 'c, 't> {
    fn new(
        re: &'r Regex,
        cache: &'c mut Cache,
        text: &'t [u8],
    ) -> TryFindLeftmostMatches<'r, 'c, 't> {
        let scanner = re.scanner();
        TryFindLeftmostMatches {
            re,
            cache,
            scanner,
            text,
            last_end: 0,
            last_match: None,
        }
    }
}

impl<'r, 'c, 't> Iterator for TryFindLeftmostMatches<'r, 'c, 't> {
    type Item = Result<MultiMatch, MatchError>;

    fn next(&mut self) -> Option<Result<MultiMatch, MatchError>> {
        if self.last_end > self.text.len() {
            return None;
        }
        let result = self.re.try_find_leftmost_at_imp(
            self.scanner.as_mut(),
            self.cache,
            self.text,
            self.last_end,
            self.text.len(),
        );
        let m = match result {
            Err(err) => return Some(Err(err)),
            Ok(None) => return None,
            Ok(Some(m)) => m,
        };
        if m.is_empty() {
            // This is an empty match. To ensure we make progress, start
            // the next search at the smallest possible starting position
            // of the next match following this one.
            self.last_end = if self.re.utf8 {
                crate::regex_automata::util::next_utf8(self.text, m.end())
            } else {
                m.end() + 1
            };
            // Don't accept empty matches immediately following a match.
            // Just move on to the next match.
            if Some(m.end()) == self.last_match {
                return self.next();
            }
        } else {
            self.last_end = m.end();
        }
        self.last_match = Some(m.end());
        Some(Ok(m))
    }
}

#[derive(Debug)]
pub struct TryFindOverlappingMatches<'r, 'c, 't> {
    re: &'r Regex,
    cache: &'c mut Cache,
    scanner: Option<prefilter::Scanner<'r>>,
    text: &'t [u8],
    last_end: usize,
    state: OverlappingState,
}

impl<'r, 'c, 't> TryFindOverlappingMatches<'r, 'c, 't> {
    fn new(
        re: &'r Regex,
        cache: &'c mut Cache,
        text: &'t [u8],
    ) -> TryFindOverlappingMatches<'r, 'c, 't> {
        let scanner = re.scanner();
        TryFindOverlappingMatches {
            re,
            cache,
            scanner,
            text,
            last_end: 0,
            state: OverlappingState::start(),
        }
    }
}

impl<'r, 'c, 't> Iterator for TryFindOverlappingMatches<'r, 'c, 't> {
    type Item = Result<MultiMatch, MatchError>;

    fn next(&mut self) -> Option<Result<MultiMatch, MatchError>> {
        if self.last_end > self.text.len() {
            return None;
        }
        let result = self.re.try_find_overlapping_at_imp(
            self.scanner.as_mut(),
            self.cache,
            self.text,
            self.last_end,
            self.text.len(),
            &mut self.state,
        );
        let m = match result {
            Err(err) => return Some(Err(err)),
            Ok(None) => return None,
            Ok(Some(m)) => m,
        };
        // Unlike the non-overlapping case, we're OK with empty matches at this
        // level. In particular, the overlapping search algorithm is itself
        // responsible for ensuring that progress is always made.
        self.last_end = m.end();
        Some(Ok(m))
    }
}

#[derive(Debug, Clone)]
pub struct Cache {
    forward: dfa::Cache,
    reverse: dfa::Cache,
}

impl Cache {
    pub fn new(re: &Regex) -> Cache {
        let forward = dfa::Cache::new(re.forward());
        let reverse = dfa::Cache::new(re.reverse());
        Cache { forward, reverse }
    }

    pub fn reset(&mut self, re: &Regex) {
        self.forward.reset(re.forward());
        self.reverse.reset(re.reverse());
    }

    pub fn memory_usage(&self) -> usize {
        self.forward.memory_usage() + self.reverse.memory_usage()
    }

    pub fn as_parts(&self) -> (&dfa::Cache, &dfa::Cache) {
        (&self.forward, &self.reverse)
    }

    pub fn as_parts_mut(&mut self) -> (&mut dfa::Cache, &mut dfa::Cache) {
        (&mut self.forward, &mut self.reverse)
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Config {
    utf8: Option<bool>,
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }

    pub fn utf8(mut self, yes: bool) -> Config {
        self.utf8 = Some(yes);
        self
    }

    pub fn get_utf8(&self) -> bool {
        self.utf8.unwrap_or(true)
    }

    pub(crate) fn overwrite(self, o: Config) -> Config {
        Config {
            utf8: o.utf8.or(self.utf8),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    dfa: dfa::Builder,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            config: Config::default(),
            dfa: DFA::builder(),
        }
    }

    pub fn build(&self, pattern: &str) -> Result<Regex, BuildError> {
        self.build_many(&[pattern])
    }

    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<Regex, BuildError> {
        let forward = self.dfa.build_many(patterns)?;
        let reverse = self
            .dfa
            .clone()
            .configure(
                DFA::config()
                    .anchored(true)
                    .match_kind(MatchKind::All)
                    .starts_for_each_pattern(true),
            )
            .thompson(thompson::Config::new().reverse(true))
            .build_many(patterns)?;
        Ok(self.build_from_dfas(forward, reverse))
    }

    fn build_from_dfas(&self, forward: DFA, reverse: DFA) -> Regex {
        // The congruous method on DFA-backed regexes is exposed, but it's
        // not clear this builder is useful here since lazy DFAs can't be
        // serialized and there is only one type of them.
        let utf8 = self.config.get_utf8();
        Regex {
            pre: None,
            forward,
            reverse,
            utf8,
        }
    }

    pub fn configure(&mut self, config: Config) -> &mut Builder {
        self.config = self.config.overwrite(config);
        self
    }

    pub fn syntax(
        &mut self,
        config: crate::regex_automata::util::syntax::SyntaxConfig,
    ) -> &mut Builder {
        self.dfa.syntax(config);
        self
    }

    pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {
        self.dfa.thompson(config);
        self
    }

    pub fn dfa(&mut self, config: dfa::Config) -> &mut Builder {
        self.dfa.configure(config);
        self
    }
}

impl Default for Builder {
    fn default() -> Builder {
        Builder::new()
    }
}

#[inline(always)]
fn next_unwrap(item: Option<Result<MultiMatch, MatchError>>) -> Option<MultiMatch> {
    match item {
        None => None,
        Some(Ok(m)) => Some(m),
        Some(Err(err)) => panic!(
            "unexpected regex search error: {}\n\
             to handle search errors, use try_ methods",
            err,
        ),
    }
}
