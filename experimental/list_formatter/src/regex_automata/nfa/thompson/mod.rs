use core::{fmt, mem, ops::Range};

use alloc::{boxed::Box, format, string::String, sync::Arc, vec, vec::Vec};

use crate::regex_automata::util::{
    alphabet::{self, ByteClassSet},
    decode_last_utf8, decode_utf8,
    id::{IteratorIDExt, PatternID, PatternIDIter, StateID},
    is_word_byte, is_word_char_fwd, is_word_char_rev,
};

pub use self::{
    compiler::{Builder, Config},
    error::Error,
};

mod compiler;
mod error;
mod map;
pub mod pikevm;
mod range_trie;

#[cfg(feature = "std")]
type CaptureNameMap = std::collections::HashMap<(PatternID, Arc<str>), usize>;
#[cfg(not(feature = "std"))]
type CaptureNameMap = alloc::collections::BTreeMap<(PatternID, Arc<str>), usize>;

#[derive(Clone)]
pub struct NFA {
    states: Vec<State>,
    start_anchored: StateID,
    start_unanchored: StateID,
    start_pattern: Vec<StateID>,
    patterns_to_captures: Vec<Range<usize>>,
    slots: usize,
    capture_name_to_index: CaptureNameMap,
    capture_index_to_name: Vec<Option<Arc<str>>>,
    byte_class_set: ByteClassSet,
    facts: Facts,
    memory_states: usize,
}

impl NFA {
    pub fn config() -> Config {
        Config::new()
    }

    pub fn builder() -> Builder {
        Builder::new()
    }

    #[inline]
    pub fn empty() -> NFA {
        NFA {
            states: vec![],
            start_anchored: StateID::ZERO,
            start_unanchored: StateID::ZERO,
            start_pattern: vec![],
            patterns_to_captures: vec![],
            slots: 0,
            capture_name_to_index: CaptureNameMap::new(),
            capture_index_to_name: vec![],
            byte_class_set: ByteClassSet::empty(),
            facts: Facts::default(),
            memory_states: 0,
        }
    }

    #[inline]
    pub fn always_match() -> NFA {
        let mut nfa = NFA::empty();
        nfa.add(State::Match {
            id: PatternID::ZERO,
        });
        nfa
    }

    #[inline]
    pub fn never_match() -> NFA {
        let mut nfa = NFA::empty();
        nfa.add(State::Fail);
        nfa
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.states.len()
    }

    #[inline]
    pub fn match_len(&self) -> usize {
        self.start_pattern.len()
    }

    #[inline]
    pub(crate) fn set_match_len(&mut self, patterns: usize) {
        self.start_pattern.resize(patterns, StateID::ZERO);
    }

    #[inline]
    pub fn capture_len(&self) -> usize {
        let slots = self.capture_slot_len();
        assert_eq!(slots % 2, 0, "capture slots must be divisible by 2");
        slots / 2
    }

    #[inline]
    pub fn capture_slot_len(&self) -> usize {
        self.slots
    }

    #[inline]
    pub(crate) fn set_capture_slot_len(&mut self, slots: usize) {
        self.slots = slots;
    }

    #[inline]
    pub fn patterns(&self) -> PatternIter {
        PatternIter {
            it: PatternID::iter(self.match_len()),
            _marker: core::marker::PhantomData,
        }
    }

    #[inline]
    pub fn start_anchored(&self) -> StateID {
        self.start_anchored
    }

    #[inline]
    pub fn set_start_anchored(&mut self, id: StateID) {
        self.start_anchored = id;
    }

    #[inline]
    pub fn start_unanchored(&self) -> StateID {
        self.start_unanchored
    }

    #[inline]
    pub fn set_start_unanchored(&mut self, id: StateID) {
        self.start_unanchored = id;
    }

    #[inline]
    pub fn start_pattern(&self, pid: PatternID) -> StateID {
        self.start_pattern[pid]
    }

    #[inline]
    pub fn set_start_pattern(&mut self, pid: PatternID, id: StateID) {
        self.start_pattern[pid] = id;
    }

    #[inline]
    pub fn byte_class_set(&self) -> &ByteClassSet {
        &self.byte_class_set
    }

    #[inline]
    pub fn set_byte_class_set(&mut self, set: ByteClassSet) {
        self.byte_class_set = set;
    }

    #[inline]
    pub fn state(&self, id: StateID) -> &State {
        &self.states[id]
    }

    #[inline]
    pub fn states(&self) -> &[State] {
        &self.states
    }

    pub fn remap(&mut self, old_to_new: &[StateID]) {
        for state in &mut self.states {
            state.remap(old_to_new);
        }
    }

    #[inline]
    pub fn add(&mut self, state: State) -> Result<StateID, Error> {
        match state {
            State::Range { .. } | State::Sparse { .. } | State::Union { .. } | State::Fail => {}
            State::Capture { slot, .. } => {
                let len = slot.checked_add(1).unwrap();
                if len > self.capture_slot_len() {
                    self.set_capture_slot_len(len);
                }
            }
            State::Match { id } => {
                let len = id.one_more();
                if len > self.match_len() {
                    self.set_match_len(len);
                }
            }
            State::Look { look, .. } => {
                self.facts.set_has_any_look(true);
                match look {
                    Look::StartLine | Look::EndLine | Look::StartText | Look::EndText => {
                        self.facts.set_has_any_anchor(true);
                    }
                    Look::WordBoundaryUnicode | Look::WordBoundaryUnicodeNegate => {
                        self.facts.set_has_word_boundary_unicode(true);
                    }
                    Look::WordBoundaryAscii | Look::WordBoundaryAsciiNegate => {
                        self.facts.set_has_word_boundary_ascii(true);
                    }
                }
            }
        }
        let id = StateID::new(self.states.len())
            .map_err(|_| Error::too_many_states(self.states.len()))?;
        self.memory_states += state.memory_usage();
        self.states.push(state);
        Ok(id)
    }

    #[inline]
    pub fn clear(&mut self) {
        self.start_anchored = StateID::ZERO;
        self.start_unanchored = StateID::ZERO;
        self.states.clear();
        self.start_pattern.clear();
        self.slots = 0;
        self.byte_class_set = ByteClassSet::empty();
        // These are directly derived from the states added, so they must also
        // be cleared. They will be regenerated as new states are added.
        self.facts = Facts::default();
        self.memory_states = 0;
    }

    #[inline]
    pub fn is_always_start_anchored(&self) -> bool {
        self.start_anchored() == self.start_unanchored()
    }

    #[inline]
    pub fn has_any_look(&self) -> bool {
        self.facts.has_any_look()
    }

    #[inline]
    pub fn has_any_anchor(&self) -> bool {
        self.facts.has_any_anchor()
    }

    #[inline]
    pub fn has_word_boundary(&self) -> bool {
        self.has_word_boundary_unicode() || self.has_word_boundary_ascii()
    }

    #[inline]
    pub fn has_word_boundary_unicode(&self) -> bool {
        self.facts.has_word_boundary_unicode()
    }

    #[inline]
    pub fn has_word_boundary_ascii(&self) -> bool {
        self.facts.has_word_boundary_ascii()
    }

    #[inline]
    pub fn memory_usage(&self) -> usize {
        self.states.len() * mem::size_of::<State>()
            + self.memory_states
            + self.start_pattern.len() * mem::size_of::<StateID>()
    }
}

impl fmt::Debug for NFA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "thompson::NFA(")?;
        for (sid, state) in self.states.iter().with_state_ids() {
            let status = if sid == self.start_anchored {
                '^'
            } else if sid == self.start_unanchored {
                '>'
            } else {
                ' '
            };
            writeln!(f, "{}{:06?}: {:?}", status, sid.as_usize(), state)?;
        }
        if self.match_len() > 1 {
            writeln!(f, "")?;
            for pid in self.patterns() {
                let sid = self.start_pattern(pid);
                writeln!(f, "START({:06?}): {:?}", pid.as_usize(), sid.as_usize())?;
            }
        }
        writeln!(f, "")?;
        writeln!(
            f,
            "transition equivalence classes: {:?}",
            self.byte_class_set().byte_classes()
        )?;
        writeln!(f, ")")?;
        Ok(())
    }
}

#[derive(Clone, Eq, PartialEq)]
pub enum State {
    Range { range: Transition },
    Sparse(SparseTransitions),
    Look { look: Look, next: StateID },
    Union { alternates: Box<[StateID]> },
    Capture { next: StateID, slot: usize },
    Fail,
    Match { id: PatternID },
}

impl State {
    #[inline]
    pub fn is_epsilon(&self) -> bool {
        match *self {
            State::Range { .. } | State::Sparse { .. } | State::Fail | State::Match { .. } => false,
            State::Look { .. } | State::Union { .. } | State::Capture { .. } => true,
        }
    }

    fn memory_usage(&self) -> usize {
        match *self {
            State::Range { .. }
            | State::Look { .. }
            | State::Capture { .. }
            | State::Match { .. }
            | State::Fail => 0,
            State::Sparse(SparseTransitions { ref ranges }) => {
                ranges.len() * mem::size_of::<Transition>()
            }
            State::Union { ref alternates } => alternates.len() * mem::size_of::<StateID>(),
        }
    }

    fn remap(&mut self, remap: &[StateID]) {
        match *self {
            State::Range { ref mut range } => range.next = remap[range.next],
            State::Sparse(SparseTransitions { ref mut ranges }) => {
                for r in ranges.iter_mut() {
                    r.next = remap[r.next];
                }
            }
            State::Look { ref mut next, .. } => *next = remap[*next],
            State::Union { ref mut alternates } => {
                for alt in alternates.iter_mut() {
                    *alt = remap[*alt];
                }
            }
            State::Capture { ref mut next, .. } => *next = remap[*next],
            State::Fail => {}
            State::Match { .. } => {}
        }
    }
}

impl fmt::Debug for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            State::Range { ref range } => range.fmt(f),
            State::Sparse(SparseTransitions { ref ranges }) => {
                let rs = ranges
                    .iter()
                    .map(|t| format!("{:?}", t))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "sparse({})", rs)
            }
            State::Look { ref look, next } => {
                write!(f, "{:?} => {:?}", look, next.as_usize())
            }
            State::Union { ref alternates } => {
                let alts = alternates
                    .iter()
                    .map(|id| format!("{:?}", id.as_usize()))
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "alt({})", alts)
            }
            State::Capture { next, slot } => {
                write!(f, "capture({:?}) => {:?}", slot, next.as_usize())
            }
            State::Fail => write!(f, "FAIL"),
            State::Match { id } => write!(f, "MATCH({:?})", id.as_usize()),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Facts {
    bools: u16,
}

impl Facts {
    define_bool!(0, has_any_look, set_has_any_look);
    define_bool!(1, has_any_anchor, set_has_any_anchor);
    define_bool!(2, has_word_boundary_unicode, set_has_word_boundary_unicode);
    define_bool!(3, has_word_boundary_ascii, set_has_word_boundary_ascii);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SparseTransitions {
    pub ranges: Box<[Transition]>,
}

impl SparseTransitions {
    pub fn matches(&self, haystack: &[u8], at: usize) -> Option<StateID> {
        haystack.get(at).and_then(|&b| self.matches_byte(b))
    }

    pub fn matches_unit(&self, unit: alphabet::Unit) -> Option<StateID> {
        unit.as_u8().map_or(None, |byte| self.matches_byte(byte))
    }

    pub fn matches_byte(&self, byte: u8) -> Option<StateID> {
        for t in self.ranges.iter() {
            if t.start > byte {
                break;
            } else if t.matches_byte(byte) {
                return Some(t.next);
            }
        }
        None

        /*
        // This is an alternative implementation that uses binary search. In
        // some ad hoc experiments, like
        //
        //   smallishru=OpenSubtitles2018.raw.sample.smallish.ru
        //   regex-cli find nfa thompson pikevm -b "@$smallishru" '\b\w+\b'
        //
        // I could not observe any improvement, and in fact, things seemed to
        // be a bit slower.
        self.ranges
            .binary_search_by(|t| {
                if t.end < byte {
                    core::cmp::Ordering::Less
                } else if t.start > byte {
                    core::cmp::Ordering::Greater
                } else {
                    core::cmp::Ordering::Equal
                }
            })
            .ok()
            .map(|i| self.ranges[i].next)
        */
    }
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct Transition {
    pub start: u8,
    pub end: u8,
    pub next: StateID,
}

impl Transition {
    pub fn matches(&self, haystack: &[u8], at: usize) -> bool {
        haystack.get(at).map_or(false, |&b| self.matches_byte(b))
    }

    pub fn matches_unit(&self, unit: alphabet::Unit) -> bool {
        unit.as_u8().map_or(false, |byte| self.matches_byte(byte))
    }

    pub fn matches_byte(&self, byte: u8) -> bool {
        self.start <= byte && byte <= self.end
    }
}

impl fmt::Debug for Transition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use crate::regex_automata::util::DebugByte;

        let Transition { start, end, next } = *self;
        if self.start == self.end {
            write!(f, "{:?} => {:?}", DebugByte(start), next.as_usize())
        } else {
            write!(
                f,
                "{:?}-{:?} => {:?}",
                DebugByte(start),
                DebugByte(end),
                next.as_usize(),
            )
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Look {
    StartLine = 1 << 0,
    EndLine = 1 << 1,
    StartText = 1 << 2,
    EndText = 1 << 3,
    WordBoundaryUnicode = 1 << 4,
    WordBoundaryUnicodeNegate = 1 << 5,
    WordBoundaryAscii = 1 << 6,
    WordBoundaryAsciiNegate = 1 << 7,
}

impl Look {
    #[inline(always)]
    pub fn matches(&self, bytes: &[u8], at: usize) -> bool {
        match *self {
            Look::StartLine => at == 0 || bytes[at - 1] == b'\n',
            Look::EndLine => at == bytes.len() || bytes[at] == b'\n',
            Look::StartText => at == 0,
            Look::EndText => at == bytes.len(),
            Look::WordBoundaryUnicode => {
                let word_before = is_word_char_rev(bytes, at);
                let word_after = is_word_char_fwd(bytes, at);
                word_before != word_after
            }
            Look::WordBoundaryUnicodeNegate => {
                // This is pretty subtle. Why do we need to do UTF-8 decoding
                // here? Well... at time of writing, the is_word_char_{fwd,rev}
                // routines will only return true if there is a valid UTF-8
                // encoding of a "word" codepoint, and false in every other
                // case (including invalid UTF-8). This means that in regions
                // of invalid UTF-8 (which might be a subset of valid UTF-8!),
                // it would result in \B matching. While this would be
                // questionable in the context of truly invalid UTF-8, it is
                // *certainly* wrong to report match boundaries that split the
                // encoding of a codepoint. So to work around this, we ensure
                // that we can decode a codepoint on either side of `at`. If
                // either direction fails, then we don't permit \B to match at
                // all.
                //
                // Now, this isn't exactly optimal from a perf perspective. We
                // could try and detect this in is_word_char_{fwd,rev}, but
                // it's not clear if it's worth it. \B is, after all, rarely
                // used.
                //
                // And in particular, we do *not* have to do this with \b,
                // because \b *requires* that at least one side of `at` be a
                // "word" codepoint, which in turn implies one side of `at`
                // must be valid UTF-8. This in turn implies that \b can never
                // split a valid UTF-8 encoding of a codepoint. In the case
                // where one side of `at` is truly invalid UTF-8 and the other
                // side IS a word codepoint, then we want \b to match since it
                // represents a valid UTF-8 boundary. It also makes sense. For
                // example, you'd want \b\w+\b to match 'abc' in '\xFFabc\xFF'.
                let word_before = at > 0
                    && match decode_last_utf8(&bytes[..at]) {
                        None | Some(Err(_)) => return false,
                        Some(Ok(_)) => is_word_char_rev(bytes, at),
                    };
                let word_after = at < bytes.len()
                    && match decode_utf8(&bytes[at..]) {
                        None | Some(Err(_)) => return false,
                        Some(Ok(_)) => is_word_char_fwd(bytes, at),
                    };
                word_before == word_after
            }
            Look::WordBoundaryAscii => {
                let word_before = at > 0 && is_word_byte(bytes[at - 1]);
                let word_after = at < bytes.len() && is_word_byte(bytes[at]);
                word_before != word_after
            }
            Look::WordBoundaryAsciiNegate => {
                let word_before = at > 0 && is_word_byte(bytes[at - 1]);
                let word_after = at < bytes.len() && is_word_byte(bytes[at]);
                word_before == word_after
            }
        }
    }

    fn from_int(n: u8) -> Option<Look> {
        match n {
            0b0000_0001 => Some(Look::StartLine),
            0b0000_0010 => Some(Look::EndLine),
            0b0000_0100 => Some(Look::StartText),
            0b0000_1000 => Some(Look::EndText),
            0b0001_0000 => Some(Look::WordBoundaryUnicode),
            0b0010_0000 => Some(Look::WordBoundaryUnicodeNegate),
            0b0100_0000 => Some(Look::WordBoundaryAscii),
            0b1000_0000 => Some(Look::WordBoundaryAsciiNegate),
            _ => None,
        }
    }

    fn reversed(&self) -> Look {
        match *self {
            Look::StartLine => Look::EndLine,
            Look::EndLine => Look::StartLine,
            Look::StartText => Look::EndText,
            Look::EndText => Look::StartText,
            Look::WordBoundaryUnicode => Look::WordBoundaryUnicode,
            Look::WordBoundaryUnicodeNegate => Look::WordBoundaryUnicodeNegate,
            Look::WordBoundaryAscii => Look::WordBoundaryAscii,
            Look::WordBoundaryAsciiNegate => Look::WordBoundaryAsciiNegate,
        }
    }

    fn add_to_byteset(&self, set: &mut ByteClassSet) {
        match *self {
            Look::StartText | Look::EndText => {}
            Look::StartLine | Look::EndLine => {
                set.set_range(b'\n', b'\n');
            }
            Look::WordBoundaryUnicode
            | Look::WordBoundaryUnicodeNegate
            | Look::WordBoundaryAscii
            | Look::WordBoundaryAsciiNegate => {
                // We need to mark all ranges of bytes whose pairs result in
                // evaluating \b differently. This isn't technically correct
                // for Unicode word boundaries, but DFAs can't handle those
                // anyway, and thus, the byte classes don't need to either
                // since they are themselves only used in DFAs.
                let iswb = regex_syntax::is_word_byte;
                let mut b1: u16 = 0;
                let mut b2: u16;
                while b1 <= 255 {
                    b2 = b1 + 1;
                    while b2 <= 255 && iswb(b1 as u8) == iswb(b2 as u8) {
                        b2 += 1;
                    }
                    set.set_range(b1 as u8, (b2 - 1) as u8);
                    b1 = b2;
                }
            }
        }
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct LookSet {
    set: u8,
}

impl LookSet {
    pub(crate) fn from_repr(repr: u8) -> LookSet {
        LookSet { set: repr }
    }

    pub(crate) fn from_repr_mut(repr: &mut u8) -> &mut LookSet {
        // SAFETY: This is safe since a LookSet is repr(transparent) where its
        // repr is a u8.
        unsafe { core::mem::transmute::<&mut u8, &mut LookSet>(repr) }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.set == 0
    }

    pub(crate) fn clear(&mut self) {
        self.set = 0;
    }

    pub(crate) fn insert(&mut self, look: Look) {
        self.set |= look as u8;
    }

    pub(crate) fn contains(&self, look: Look) -> bool {
        (look as u8) & self.set != 0
    }

    pub(crate) fn subtract(&self, other: LookSet) -> LookSet {
        LookSet {
            set: self.set & !other.set,
        }
    }

    pub(crate) fn intersect(&self, other: LookSet) -> LookSet {
        LookSet {
            set: self.set & other.set,
        }
    }
}

impl core::fmt::Debug for LookSet {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut members = vec![];
        for i in 0..8 {
            let look = match Look::from_int(1 << i) {
                None => continue,
                Some(look) => look,
            };
            if self.contains(look) {
                members.push(look);
            }
        }
        f.debug_tuple("LookSet").field(&members).finish()
    }
}

pub struct PatternIter<'a> {
    it: PatternIDIter,
    _marker: core::marker::PhantomData<&'a ()>,
}

impl<'a> Iterator for PatternIter<'a> {
    type Item = PatternID;

    fn next(&mut self) -> Option<PatternID> {
        self.it.next()
    }
}
