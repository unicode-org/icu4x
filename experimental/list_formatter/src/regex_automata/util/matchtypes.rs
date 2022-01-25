use crate::regex_automata::util::id::PatternID;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MatchKind {
        All,
                LeftmostFirst,
                        #[doc(hidden)]
    __Nonexhaustive,
    // There is prior art in RE2 that shows that we should be able to add
    // LeftmostLongest too. The tricky part of it is supporting ungreedy
    // repetitions. Instead of treating all NFA states as having equivalent
    // priority (as in 'All') or treating all NFA states as having distinct
    // priority based on order (as in 'LeftmostFirst'), we instead group NFA
    // states into sets, and treat members of each set as having equivalent
    // priority, but having greater priority than all following members
    // of different sets.
    //
    // However, it's not clear whether it's really worth adding this. After
    // all, leftmost-longest can be emulated when using literals by using
    // leftmost-first and sorting the literals by length in descending order.
    // However, this won't work for arbitrary regexes. e.g., `\w|\w\w` will
    // always match `a` in `ab` when using leftmost-first, but leftmost-longest
    // would match `ab`.
}

impl MatchKind {
    #[cfg(feature = "std")]
    pub(crate) fn continue_past_first_match(&self) -> bool {
        *self == MatchKind::All
    }
}

impl Default for MatchKind {
    fn default() -> MatchKind {
        MatchKind::LeftmostFirst
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Match {
        start: usize,
        end: usize,
}

impl Match {
                        #[inline]
    pub fn new(start: usize, end: usize) -> Match {
        assert!(start <= end);
        Match { start, end }
    }

        #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

        #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

        #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {
        self.start..self.end
    }

                        #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct HalfMatch {
        pub(crate) pattern: PatternID,
                    pub(crate) offset: usize,
}

impl HalfMatch {
        #[inline]
    pub fn new(pattern: PatternID, offset: usize) -> HalfMatch {
        HalfMatch { pattern, offset }
    }

                        #[inline]
    pub fn must(pattern: usize, offset: usize) -> HalfMatch {
        HalfMatch::new(PatternID::new(pattern).unwrap(), offset)
    }

                        #[inline]
    pub fn pattern(&self) -> PatternID {
        self.pattern
    }

                        #[inline]
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct MultiMatch {
        pattern: PatternID,
        start: usize,
        end: usize,
}

impl MultiMatch {
                        #[inline]
    pub fn new(pattern: PatternID, start: usize, end: usize) -> MultiMatch {
        assert!(start <= end);
        MultiMatch { pattern, start, end }
    }

                                        #[inline]
    pub fn must(pattern: usize, start: usize, end: usize) -> MultiMatch {
        MultiMatch::new(PatternID::new(pattern).unwrap(), start, end)
    }

                            #[inline]
    pub fn pattern(&self) -> PatternID {
        self.pattern
    }

        #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

        #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

        #[inline]
    pub fn range(&self) -> core::ops::Range<usize> {
        self.start..self.end
    }

                        #[inline]
    pub fn is_empty(&self) -> bool {
        self.start == self.end
    }
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum MatchError {
    // Note that the first version of this type was called `SearchError` and it
    // included a third `None` variant to indicate that the search completed
    // and no match was found. However, this was problematic for iterator
    // APIs where the `None` sentinel for stopping iteration corresponds
    // precisely to the "match not found" case. The fact that the `None`
    // variant was buried inside this type was in turn quite awkward. So
    // instead, I removed the `None` variant, renamed the type and used
    // `Result<Option<Match>, MatchError>` in non-iterator APIs instead of the
    // conceptually simpler `Result<Match, MatchError>`. However, we "regain"
    // ergonomics by only putting the more complex API in the `try_` variants
    // ("fallible") of search methods. The infallible APIs will instead just
    // return `Option<Match>` and panic on error.
            Quit {
                byte: u8,
                offset: usize,
    },
                                GaveUp {
                        offset: usize,
    },
}

#[cfg(feature = "std")]
impl std::error::Error for MatchError {}

impl core::fmt::Display for MatchError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match *self {
            MatchError::Quit { byte, offset } => write!(
                f,
                "quit search after observing byte \\x{:02X} at offset {}",
                byte, offset,
            ),
            MatchError::GaveUp { offset } => {
                write!(f, "gave up searching at offset {}", offset)
            }
        }
    }
}
