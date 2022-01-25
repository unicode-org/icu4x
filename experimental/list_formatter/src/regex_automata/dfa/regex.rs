#[cfg(feature = "std")]
use alloc::vec::Vec;

use crate::regex_automata::{
    dfa::automaton::{Automaton, OverlappingState},
    util::prefilter::{self, Prefilter},
    MatchError, MultiMatch,
};
#[cfg(feature = "std")]
use crate::regex_automata::{
    dfa::{dense, error::Error, sparse},
    nfa::thompson,
    util::matchtypes::MatchKind,
};

// When the alloc feature is enabled, the regex type sets its A type parameter
// to default to an owned dense DFA. But without alloc, we set no default. This
// makes things a lot more convenient in the common case, since writing out the
// DFA types is pretty annoying.
//
// Since we have two different definitions but only want to write one doc
// string, we use a macro to capture the doc and other attributes once and then
// repeat them for each definition.
macro_rules! define_regex_type {
    ($(#[$doc:meta])*) => {
        #[cfg(feature = "std")]
        $(#[$doc])*
        pub struct Regex<A = dense::OwnedDFA, P = prefilter::None> {
            prefilter: Option<P>,
            forward: A,
            reverse: A,
            utf8: bool,
        }

        #[cfg(not(feature = "std"))]
        $(#[$doc])*
        pub struct Regex<A, P = prefilter::None> {
            prefilter: Option<P>,
            forward: A,
            reverse: A,
            utf8: bool,
        }
    };
}

define_regex_type!(
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                #[derive(Clone, Debug)]
);

#[cfg(feature = "std")]
impl Regex {
                                                                            pub fn new(pattern: &str) -> Result<Regex, Error> {
        Builder::new().build(pattern)
    }

                                                                                    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<Regex, Error> {
        Builder::new().build_many(patterns)
    }
}

#[cfg(feature = "std")]
impl Regex<sparse::DFA<Vec<u8>>> {
                                                                            pub fn new_sparse(
        pattern: &str,
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, Error> {
        Builder::new().build_sparse(pattern)
    }

                                                                                        pub fn new_many_sparse<P: AsRef<str>>(
        patterns: &[P],
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, Error> {
        Builder::new().build_many_sparse(patterns)
    }
}

#[cfg(feature = "std")]
impl Regex {
                                                                                                                            pub fn config() -> Config {
        Config::new()
    }

                                                                                                                        pub fn builder() -> Builder {
        Builder::new()
    }
}

impl<A: Automaton, P: Prefilter> Regex<A, P> {
                                                                                                            pub fn is_match(&self, haystack: &[u8]) -> bool {
        self.is_match_at(haystack, 0, haystack.len())
    }

                                                                                                                                                    pub fn find_earliest(&self, haystack: &[u8]) -> Option<MultiMatch> {
        self.find_earliest_at(haystack, 0, haystack.len())
    }

                                                                                                                                    pub fn find_leftmost(&self, haystack: &[u8]) -> Option<MultiMatch> {
        self.find_leftmost_at(haystack, 0, haystack.len())
    }

                                                                                                                                                                                            pub fn find_overlapping(
        &self,
        haystack: &[u8],
        state: &mut OverlappingState,
    ) -> Option<MultiMatch> {
        self.find_overlapping_at(haystack, 0, haystack.len(), state)
    }

                                                                                                                                                pub fn find_earliest_iter<'r, 't>(
        &'r self,
        haystack: &'t [u8],
    ) -> FindEarliestMatches<'r, 't, A, P> {
        FindEarliestMatches::new(self, haystack)
    }

                                                                                                                        pub fn find_leftmost_iter<'r, 't>(
        &'r self,
        haystack: &'t [u8],
    ) -> FindLeftmostMatches<'r, 't, A, P> {
        FindLeftmostMatches::new(self, haystack)
    }

                                                                                                                                                    pub fn find_overlapping_iter<'r, 't>(
        &'r self,
        haystack: &'t [u8],
    ) -> FindOverlappingMatches<'r, 't, A, P> {
        FindOverlappingMatches::new(self, haystack)
    }
}

impl<A: Automaton, P: Prefilter> Regex<A, P> {
                                                                                                    pub fn is_match_at(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> bool {
        self.try_is_match_at(haystack, start, end).unwrap()
    }

                                                                                                                    pub fn find_earliest_at(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Option<MultiMatch> {
        self.try_find_earliest_at(haystack, start, end).unwrap()
    }

                                                                                                                    pub fn find_leftmost_at(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Option<MultiMatch> {
        self.try_find_leftmost_at(haystack, start, end).unwrap()
    }

                                                                                                                            pub fn find_overlapping_at(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Option<MultiMatch> {
        self.try_find_overlapping_at(haystack, start, end, state).unwrap()
    }
}

impl<A: Automaton, P: Prefilter> Regex<A, P> {
                                                                                pub fn try_is_match(&self, haystack: &[u8]) -> Result<bool, MatchError> {
        self.try_is_match_at(haystack, 0, haystack.len())
    }

                                                                                pub fn try_find_earliest(
        &self,
        haystack: &[u8],
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_earliest_at(haystack, 0, haystack.len())
    }

                                                                pub fn try_find_leftmost(
        &self,
        haystack: &[u8],
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_leftmost_at(haystack, 0, haystack.len())
    }

                                                                                    pub fn try_find_overlapping(
        &self,
        haystack: &[u8],
        state: &mut OverlappingState,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_overlapping_at(haystack, 0, haystack.len(), state)
    }

                                                                        pub fn try_find_earliest_iter<'r, 't>(
        &'r self,
        haystack: &'t [u8],
    ) -> TryFindEarliestMatches<'r, 't, A, P> {
        TryFindEarliestMatches::new(self, haystack)
    }

                                                                        pub fn try_find_leftmost_iter<'r, 't>(
        &'r self,
        haystack: &'t [u8],
    ) -> TryFindLeftmostMatches<'r, 't, A, P> {
        TryFindLeftmostMatches::new(self, haystack)
    }

                                                                                pub fn try_find_overlapping_iter<'r, 't>(
        &'r self,
        haystack: &'t [u8],
    ) -> TryFindOverlappingMatches<'r, 't, A, P> {
        TryFindOverlappingMatches::new(self, haystack)
    }
}

impl<A: Automaton, P: Prefilter> Regex<A, P> {
                                                                                                                        pub fn try_is_match_at(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<bool, MatchError> {
        self.forward()
            .find_earliest_fwd_at(
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
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_earliest_at_imp(
            self.scanner().as_mut(),
            haystack,
            start,
            end,
        )
    }

            fn try_find_earliest_at_imp(
        &self,
        pre: Option<&mut prefilter::Scanner>,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        // N.B. We use `&&A` here to call `Automaton` methods, which ensures
        // that we always use the `impl Automaton for &A` for calling methods.
        // Since this is the usual way that automata are used, this helps
        // reduce the number of monomorphized copies of the search code.
        let (fwd, rev) = (self.forward(), self.reverse());
        let end = match (&fwd)
            .find_earliest_fwd_at(pre, None, haystack, start, end)?
        {
            None => return Ok(None),
            Some(end) => end,
        };
        // N.B. The only time we need to tell the reverse searcher the pattern
        // to match is in the overlapping case, since it's ambiguous. In the
        // leftmost case, I have tentatively convinced myself that it isn't
        // necessary and the reverse search will always find the same pattern
        // to match as the forward search. But I lack a rigorous proof.
        let start = (&rev)
            .find_earliest_rev_at(None, haystack, start, end.offset())?
            .expect("reverse search must match if forward search does");
        assert_eq!(
            start.pattern(),
            end.pattern(),
            "forward and reverse search must match same pattern"
        );
        assert!(start.offset() <= end.offset());
        Ok(Some(MultiMatch::new(end.pattern(), start.offset(), end.offset())))
    }

                                                                                                                    pub fn try_find_leftmost_at(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_leftmost_at_imp(
            self.scanner().as_mut(),
            haystack,
            start,
            end,
        )
    }

            fn try_find_leftmost_at_imp(
        &self,
        scanner: Option<&mut prefilter::Scanner>,
        haystack: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<MultiMatch>, MatchError> {
        // N.B. We use `&&A` here to call `Automaton` methods, which ensures
        // that we always use the `impl Automaton for &A` for calling methods.
        // Since this is the usual way that automata are used, this helps
        // reduce the number of monomorphized copies of the search code.
        let (fwd, rev) = (self.forward(), self.reverse());
        let end = match (&fwd)
            .find_leftmost_fwd_at(scanner, None, haystack, start, end)?
        {
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
        let start = (&rev)
            .find_leftmost_rev_at(None, haystack, start, end.offset())?
            .expect("reverse search must match if forward search does");
        assert_eq!(
            start.pattern(),
            end.pattern(),
            "forward and reverse search must match same pattern",
        );
        assert!(start.offset() <= end.offset());
        Ok(Some(MultiMatch::new(end.pattern(), start.offset(), end.offset())))
    }

                                                                                                                                            pub fn try_find_overlapping_at(
        &self,
        haystack: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Result<Option<MultiMatch>, MatchError> {
        self.try_find_overlapping_at_imp(
            self.scanner().as_mut(),
            haystack,
            start,
            end,
            state,
        )
    }

                fn try_find_overlapping_at_imp(
        &self,
        scanner: Option<&mut prefilter::Scanner>,
        haystack: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Result<Option<MultiMatch>, MatchError> {
        // N.B. We use `&&A` here to call `Automaton` methods, which ensures
        // that we always use the `impl Automaton for &A` for calling methods.
        // Since this is the usual way that automata are used, this helps
        // reduce the number of monomorphized copies of the search code.
        let (fwd, rev) = (self.forward(), self.reverse());
        // TODO: Decide whether it's worth making this assert work. It doesn't
        // work currently because 'has_starts_for_each_pattern' isn't on the
        // Automaton trait. Without this assert, we still get a panic, but it's
        // a bit more inscrutable.
        // assert!(
        // rev.has_starts_for_each_pattern(),
        // "overlapping searches require that the reverse DFA is \
        // compiled with the 'starts_for_each_pattern' option",
        // );
        let end = match (&fwd).find_overlapping_fwd_at(
            scanner, None, haystack, start, end, state,
        )? {
            None => return Ok(None),
            Some(end) => end,
        };
        // Unlike the leftmost cases, the reverse overlapping search may match
        // a different pattern than the forward search. See test failures when
        // using `None` instead of `Some(end.pattern())` below. Thus, we must
        // run our reverse search using the pattern that matched in the forward
        // direction.
        let start = (&rev)
            .find_leftmost_rev_at(
                Some(end.pattern()),
                haystack,
                0,
                end.offset(),
            )?
            .expect("reverse search must match if forward search does");
        assert!(start.offset() <= end.offset());
        assert_eq!(start.pattern(), end.pattern());
        Ok(Some(MultiMatch::new(end.pattern(), start.offset(), end.offset())))
    }
}

impl<A: Automaton, P: Prefilter> Regex<A, P> {
        pub fn with_prefilter<Q: Prefilter>(self, prefilter: Q) -> Regex<A, Q> {
        Regex {
            prefilter: Some(prefilter),
            forward: self.forward,
            reverse: self.reverse,
            utf8: self.utf8,
        }
    }

        pub fn without_prefilter(self) -> Regex<A> {
        Regex {
            prefilter: None,
            forward: self.forward,
            reverse: self.reverse,
            utf8: self.utf8,
        }
    }

                        pub fn forward(&self) -> &A {
        &self.forward
    }

                        pub fn reverse(&self) -> &A {
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
        match self.prefilter {
            None => None,
            Some(ref x) => Some(&*x),
        }
    }

        fn scanner(&self) -> Option<prefilter::Scanner> {
        self.prefilter().map(prefilter::Scanner::new)
    }
}

#[derive(Clone, Debug)]
pub struct FindEarliestMatches<'r, 't, A, P>(
    TryFindEarliestMatches<'r, 't, A, P>,
);

impl<'r, 't, A: Automaton, P: Prefilter> FindEarliestMatches<'r, 't, A, P> {
    fn new(
        re: &'r Regex<A, P>,
        text: &'t [u8],
    ) -> FindEarliestMatches<'r, 't, A, P> {
        FindEarliestMatches(TryFindEarliestMatches::new(re, text))
    }
}

impl<'r, 't, A: Automaton, P: Prefilter> Iterator
    for FindEarliestMatches<'r, 't, A, P>
{
    type Item = MultiMatch;

    fn next(&mut self) -> Option<MultiMatch> {
        next_unwrap(self.0.next())
    }
}

#[derive(Clone, Debug)]
pub struct FindLeftmostMatches<'r, 't, A, P>(
    TryFindLeftmostMatches<'r, 't, A, P>,
);

impl<'r, 't, A: Automaton, P: Prefilter> FindLeftmostMatches<'r, 't, A, P> {
    fn new(
        re: &'r Regex<A, P>,
        text: &'t [u8],
    ) -> FindLeftmostMatches<'r, 't, A, P> {
        FindLeftmostMatches(TryFindLeftmostMatches::new(re, text))
    }
}

impl<'r, 't, A: Automaton, P: Prefilter> Iterator
    for FindLeftmostMatches<'r, 't, A, P>
{
    type Item = MultiMatch;

    fn next(&mut self) -> Option<MultiMatch> {
        next_unwrap(self.0.next())
    }
}

#[derive(Clone, Debug)]
pub struct FindOverlappingMatches<'r, 't, A: Automaton, P>(
    TryFindOverlappingMatches<'r, 't, A, P>,
);

impl<'r, 't, A: Automaton, P: Prefilter> FindOverlappingMatches<'r, 't, A, P> {
    fn new(
        re: &'r Regex<A, P>,
        text: &'t [u8],
    ) -> FindOverlappingMatches<'r, 't, A, P> {
        FindOverlappingMatches(TryFindOverlappingMatches::new(re, text))
    }
}

impl<'r, 't, A: Automaton, P: Prefilter> Iterator
    for FindOverlappingMatches<'r, 't, A, P>
{
    type Item = MultiMatch;

    fn next(&mut self) -> Option<MultiMatch> {
        next_unwrap(self.0.next())
    }
}

#[derive(Clone, Debug)]
pub struct TryFindEarliestMatches<'r, 't, A, P> {
    re: &'r Regex<A, P>,
    scanner: Option<prefilter::Scanner<'r>>,
    text: &'t [u8],
    last_end: usize,
    last_match: Option<usize>,
}

impl<'r, 't, A: Automaton, P: Prefilter> TryFindEarliestMatches<'r, 't, A, P> {
    fn new(
        re: &'r Regex<A, P>,
        text: &'t [u8],
    ) -> TryFindEarliestMatches<'r, 't, A, P> {
        let scanner = re.scanner();
        TryFindEarliestMatches {
            re,
            scanner,
            text,
            last_end: 0,
            last_match: None,
        }
    }
}

impl<'r, 't, A: Automaton, P: Prefilter> Iterator
    for TryFindEarliestMatches<'r, 't, A, P>
{
    type Item = Result<MultiMatch, MatchError>;

    fn next(&mut self) -> Option<Result<MultiMatch, MatchError>> {
        if self.last_end > self.text.len() {
            return None;
        }
        let result = self.re.try_find_earliest_at_imp(
            self.scanner.as_mut(),
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

#[derive(Clone, Debug)]
pub struct TryFindLeftmostMatches<'r, 't, A, P> {
    re: &'r Regex<A, P>,
    scanner: Option<prefilter::Scanner<'r>>,
    text: &'t [u8],
    last_end: usize,
    last_match: Option<usize>,
}

impl<'r, 't, A: Automaton, P: Prefilter> TryFindLeftmostMatches<'r, 't, A, P> {
    fn new(
        re: &'r Regex<A, P>,
        text: &'t [u8],
    ) -> TryFindLeftmostMatches<'r, 't, A, P> {
        let scanner = re.scanner();
        TryFindLeftmostMatches {
            re,
            scanner,
            text,
            last_end: 0,
            last_match: None,
        }
    }
}

impl<'r, 't, A: Automaton, P: Prefilter> Iterator
    for TryFindLeftmostMatches<'r, 't, A, P>
{
    type Item = Result<MultiMatch, MatchError>;

    fn next(&mut self) -> Option<Result<MultiMatch, MatchError>> {
        if self.last_end > self.text.len() {
            return None;
        }
        let result = self.re.try_find_leftmost_at_imp(
            self.scanner.as_mut(),
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

#[derive(Clone, Debug)]
pub struct TryFindOverlappingMatches<'r, 't, A: Automaton, P> {
    re: &'r Regex<A, P>,
    scanner: Option<prefilter::Scanner<'r>>,
    text: &'t [u8],
    last_end: usize,
    state: OverlappingState,
}

impl<'r, 't, A: Automaton, P: Prefilter>
    TryFindOverlappingMatches<'r, 't, A, P>
{
    fn new(
        re: &'r Regex<A, P>,
        text: &'t [u8],
    ) -> TryFindOverlappingMatches<'r, 't, A, P> {
        let scanner = re.scanner();
        TryFindOverlappingMatches {
            re,
            scanner,
            text,
            last_end: 0,
            state: OverlappingState::start(),
        }
    }
}

impl<'r, 't, A: Automaton, P: Prefilter> Iterator
    for TryFindOverlappingMatches<'r, 't, A, P>
{
    type Item = Result<MultiMatch, MatchError>;

    fn next(&mut self) -> Option<Result<MultiMatch, MatchError>> {
        if self.last_end > self.text.len() {
            return None;
        }
        let result = self.re.try_find_overlapping_at_imp(
            self.scanner.as_mut(),
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

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Default)]
pub struct Config {
    utf8: Option<bool>,
}

#[cfg(feature = "std")]
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
        Config { utf8: o.utf8.or(self.utf8) }
    }
}

#[cfg(feature = "std")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    dfa: dense::Builder,
}

#[cfg(feature = "std")]
impl Builder {
        pub fn new() -> Builder {
        Builder { config: Config::default(), dfa: dense::Builder::new() }
    }

                    pub fn build(&self, pattern: &str) -> Result<Regex, Error> {
        self.build_many(&[pattern])
    }

                    pub fn build_sparse(
        &self,
        pattern: &str,
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, Error> {
        self.build_many_sparse(&[pattern])
    }

        pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex, Error> {
        let forward = self.dfa.build_many(patterns)?;
        let reverse = self
            .dfa
            .clone()
            .configure(
                dense::Config::new()
                    .anchored(true)
                    .match_kind(MatchKind::All)
                    .starts_for_each_pattern(true),
            )
            .thompson(thompson::Config::new().reverse(true))
            .build_many(patterns)?;
        Ok(self.build_from_dfas(forward, reverse))
    }

        pub fn build_many_sparse<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<Regex<sparse::DFA<Vec<u8>>>, Error> {
        let re = self.build_many(patterns)?;
        let forward = re.forward().to_sparse()?;
        let reverse = re.reverse().to_sparse()?;
        Ok(self.build_from_dfas(forward, reverse))
    }

                                                                                                                                                                                                                                                pub fn build_from_dfas<A: Automaton>(
        &self,
        forward: A,
        reverse: A,
    ) -> Regex<A> {
        let utf8 = self.config.get_utf8();
        Regex { prefilter: None, forward, reverse, utf8 }
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

                        pub fn dense(&mut self, config: dense::Config) -> &mut Builder {
        self.dfa.configure(config);
        self
    }
}

#[cfg(feature = "std")]
impl Default for Builder {
    fn default() -> Builder {
        Builder::new()
    }
}

#[inline(always)]
fn next_unwrap(
    item: Option<Result<MultiMatch, MatchError>>,
) -> Option<MultiMatch> {
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
