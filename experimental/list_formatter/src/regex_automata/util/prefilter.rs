use crate::regex_automata::Match;

#[derive(Clone, Debug)]
pub enum Candidate {
    None,
    Match(Match),
    PossibleStartOfMatch(usize),
}

impl Candidate {
    pub fn into_option(self) -> Option<usize> {
        match self {
            Candidate::None => None,
            Candidate::Match(ref m) => Some(m.start()),
            Candidate::PossibleStartOfMatch(start) => Some(start),
        }
    }
}

pub trait Prefilter: core::fmt::Debug {
    fn next_candidate(&self, state: &mut State, haystack: &[u8], at: usize) -> Candidate;

    fn heap_bytes(&self) -> usize;

    fn reports_false_positives(&self) -> bool {
        true
    }
}

impl<'a, P: Prefilter + ?Sized> Prefilter for &'a P {
    #[inline]
    fn next_candidate(&self, state: &mut State, haystack: &[u8], at: usize) -> Candidate {
        (**self).next_candidate(state, haystack, at)
    }

    fn heap_bytes(&self) -> usize {
        (**self).heap_bytes()
    }

    fn reports_false_positives(&self) -> bool {
        (**self).reports_false_positives()
    }
}

#[derive(Clone)]
pub struct Scanner<'p> {
    prefilter: &'p dyn Prefilter,
    state: State,
}

impl<'p> Scanner<'p> {
    pub fn new(prefilter: &'p dyn Prefilter) -> Scanner<'p> {
        Scanner {
            prefilter,
            state: State::new(),
        }
    }

    pub(crate) fn is_effective(&mut self, at: usize) -> bool {
        self.state.is_effective(at)
    }

    pub(crate) fn reports_false_positives(&self) -> bool {
        self.prefilter.reports_false_positives()
    }

    pub(crate) fn next_candidate(&mut self, bytes: &[u8], at: usize) -> Candidate {
        let cand = self.prefilter.next_candidate(&mut self.state, bytes, at);
        match cand {
            Candidate::None => {
                self.state.update_skipped_bytes(bytes.len() - at);
            }
            Candidate::Match(ref m) => {
                self.state.update_skipped_bytes(m.start() - at);
            }
            Candidate::PossibleStartOfMatch(i) => {
                self.state.update_skipped_bytes(i - at);
            }
        }
        cand
    }
}

impl<'p> core::fmt::Debug for Scanner<'p> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Scanner")
            .field("state", &self.state)
            .finish()
    }
}

#[derive(Clone, Debug)]
pub struct State {
    skips: usize,
    skipped: usize,
    inert: bool,
    last_scan_at: usize,
}

impl State {
    const MIN_SKIPS: usize = 40;

    const MIN_AVG_SKIP: usize = 16;

    pub fn new() -> State {
        State {
            skips: 0,
            skipped: 0,
            inert: false,
            last_scan_at: 0,
        }
    }

    pub fn update_last_scan(&mut self, at: usize) {
        if at > self.last_scan_at {
            self.last_scan_at = at;
        }
    }

    fn is_effective(&mut self, at: usize) -> bool {
        if self.inert {
            return false;
        }
        if at < self.last_scan_at {
            return false;
        }
        if self.skips < State::MIN_SKIPS {
            return true;
        }

        if self.skipped >= State::MIN_AVG_SKIP * self.skips {
            return true;
        }

        // We're inert.
        self.inert = true;
        false
    }

    fn update_skipped_bytes(&mut self, skipped: usize) {
        self.skips += 1;
        self.skipped += skipped;
    }
}

#[derive(Clone, Debug)]
pub struct None {
    _priv: (),
}

impl Prefilter for None {
    fn next_candidate(&self, _: &mut State, _: &[u8], at: usize) -> Candidate {
        Candidate::PossibleStartOfMatch(at)
    }

    fn heap_bytes(&self) -> usize {
        0
    }
}
