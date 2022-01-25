use core::{convert::TryFrom, mem};

use alloc::{sync::Arc, vec::Vec};

use crate::regex_automata::{
    nfa::thompson::LookSet,
    util::{
        bytes::{self, Endian},
        id::{PatternID, StateID},
    },
};

#[derive(Clone, Eq, Hash, PartialEq, PartialOrd, Ord)]
pub(crate) struct State(Arc<[u8]>);

impl core::borrow::Borrow<[u8]> for State {
    fn borrow(&self) -> &[u8] {
        &*self.0
    }
}

impl core::fmt::Debug for State {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("State").field(&self.repr()).finish()
    }
}

impl State {
    pub(crate) fn dead() -> State {
        StateBuilderEmpty::new().into_matches().into_nfa().to_state()
    }

    pub(crate) fn is_match(&self) -> bool {
        self.repr().is_match()
    }

    pub(crate) fn is_from_word(&self) -> bool {
        self.repr().is_from_word()
    }

    pub(crate) fn look_have(&self) -> LookSet {
        self.repr().look_have()
    }

    pub(crate) fn look_need(&self) -> LookSet {
        self.repr().look_need()
    }

    pub(crate) fn match_count(&self) -> usize {
        self.repr().match_count()
    }

    pub(crate) fn match_pattern(&self, index: usize) -> PatternID {
        self.repr().match_pattern(index)
    }

    pub(crate) fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {
        self.repr().match_pattern_ids()
    }

    pub(crate) fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, f: F) {
        self.repr().iter_match_pattern_ids(f)
    }

    pub(crate) fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, f: F) {
        self.repr().iter_nfa_state_ids(f)
    }

    pub(crate) fn memory_usage(&self) -> usize {
        self.0.len()
    }

    fn repr(&self) -> Repr<'_> {
        Repr(&*self.0)
    }
}

#[derive(Clone, Debug)]
pub(crate) struct StateBuilderEmpty(Vec<u8>);

impl StateBuilderEmpty {
    pub(crate) fn new() -> StateBuilderEmpty {
        StateBuilderEmpty(alloc::vec![])
    }

    pub(crate) fn into_matches(mut self) -> StateBuilderMatches {
        self.0.extend_from_slice(&[0, 0, 0]);
        StateBuilderMatches(self.0)
    }

    fn clear(&mut self) {
        self.0.clear();
    }

    pub(crate) fn capacity(&self) -> usize {
        self.0.capacity()
    }
}

#[derive(Clone)]
pub(crate) struct StateBuilderMatches(Vec<u8>);

impl core::fmt::Debug for StateBuilderMatches {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("StateBuilderMatches").field(&self.repr()).finish()
    }
}

impl StateBuilderMatches {
    pub(crate) fn into_nfa(mut self) -> StateBuilderNFA {
        self.repr_vec().close_match_pattern_ids();
        StateBuilderNFA { repr: self.0, prev_nfa_state_id: StateID::ZERO }
    }

    pub(crate) fn clear(self) -> StateBuilderEmpty {
        let mut builder = StateBuilderEmpty(self.0);
        builder.clear();
        builder
    }

    pub(crate) fn is_match(&self) -> bool {
        self.repr().is_match()
    }

    pub(crate) fn is_from_word(&self) -> bool {
        self.repr().is_from_word()
    }

    pub(crate) fn set_is_from_word(&mut self) {
        self.repr_vec().set_is_from_word()
    }

    pub(crate) fn look_have(&mut self) -> &mut LookSet {
        LookSet::from_repr_mut(&mut self.0[1])
    }

    pub(crate) fn look_need(&mut self) -> &mut LookSet {
        LookSet::from_repr_mut(&mut self.0[2])
    }

    pub(crate) fn add_match_pattern_id(&mut self, pid: PatternID) {
        self.repr_vec().add_match_pattern_id(pid)
    }

    fn repr(&self) -> Repr<'_> {
        Repr(&self.0)
    }

    fn repr_vec(&mut self) -> ReprVec<'_> {
        ReprVec(&mut self.0)
    }
}

#[derive(Clone)]
pub(crate) struct StateBuilderNFA {
    repr: Vec<u8>,
    prev_nfa_state_id: StateID,
}

impl core::fmt::Debug for StateBuilderNFA {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_tuple("StateBuilderNFA").field(&self.repr()).finish()
    }
}

impl StateBuilderNFA {
    pub(crate) fn to_state(&self) -> State {
        State(Arc::from(&*self.repr))
    }

    pub(crate) fn clear(self) -> StateBuilderEmpty {
        let mut builder = StateBuilderEmpty(self.repr);
        builder.clear();
        builder
    }

    pub(crate) fn is_match(&self) -> bool {
        self.repr().is_match()
    }

    pub(crate) fn is_from_word(&self) -> bool {
        self.repr().is_from_word()
    }

    pub(crate) fn look_have(&mut self) -> &mut LookSet {
        LookSet::from_repr_mut(&mut self.repr[1])
    }

    pub(crate) fn look_need(&mut self) -> &mut LookSet {
        LookSet::from_repr_mut(&mut self.repr[2])
    }

    pub(crate) fn add_nfa_state_id(&mut self, sid: StateID) {
        ReprVec(&mut self.repr)
            .add_nfa_state_id(&mut self.prev_nfa_state_id, sid)
    }

    pub(crate) fn memory_usage(&self) -> usize {
        self.repr.len()
    }

    pub(crate) fn as_bytes(&self) -> &[u8] {
        &self.repr
    }

    fn repr(&self) -> Repr<'_> {
        Repr(&self.repr)
    }

    fn repr_vec(&mut self) -> ReprVec<'_> {
        ReprVec(&mut self.repr)
    }
}

struct Repr<'a>(&'a [u8]);

impl<'a> Repr<'a> {
                                fn is_match(&self) -> bool {
        self.0[0] & (1 << 0) > 0
    }

                                fn has_pattern_ids(&self) -> bool {
        self.0[0] & (1 << 1) > 0
    }

                                                fn is_from_word(&self) -> bool {
        self.0[0] & (1 << 2) > 0
    }

                                            fn look_have(&self) -> LookSet {
        LookSet::from_repr(self.0[1])
    }

                                fn look_need(&self) -> LookSet {
        LookSet::from_repr(self.0[2])
    }

                fn match_count(&self) -> usize {
        if !self.is_match() {
            return 0;
        } else if !self.has_pattern_ids() {
            1
        } else {
            self.encoded_pattern_count()
        }
    }

                    fn match_pattern(&self, index: usize) -> PatternID {
        if !self.has_pattern_ids() {
            PatternID::ZERO
        } else {
            let offset = 7 + index * PatternID::SIZE;
            // This is OK since we only ever serialize valid PatternIDs to
            // states.
            bytes::read_pattern_id_unchecked(&self.0[offset..]).0
        }
    }

            fn match_pattern_ids(&self) -> Option<Vec<PatternID>> {
        if !self.is_match() {
            return None;
        }
        let mut pids = alloc::vec![];
        self.iter_match_pattern_ids(|pid| pids.push(pid));
        Some(pids)
    }

        fn iter_match_pattern_ids<F: FnMut(PatternID)>(&self, mut f: F) {
        if !self.is_match() {
            return;
        }
        // As an optimization for a very common case, when this is a match
        // state for an NFA with only one pattern, we don't actually write the
        // pattern ID to the state representation. Instead, we know it must
        // be there since it is the only possible choice.
        if !self.has_pattern_ids() {
            f(PatternID::ZERO);
            return;
        }
        let mut pids = &self.0[7..self.pattern_offset_end()];
        while !pids.is_empty() {
            let pid = bytes::read_u32(pids);
            pids = &pids[PatternID::SIZE..];
            // This is OK since we only ever serialize valid PatternIDs to
            // states. And since pattern IDs can never exceed a usize, the
            // unwrap is OK.
            f(PatternID::new_unchecked(usize::try_from(pid).unwrap()));
        }
    }

        fn iter_nfa_state_ids<F: FnMut(StateID)>(&self, mut f: F) {
        let mut sids = &self.0[self.pattern_offset_end()..];
        let mut prev = 0i32;
        while !sids.is_empty() {
            let (delta, nr) = read_vari32(sids);
            sids = &sids[nr..];
            let sid = prev + delta;
            prev = sid;
            // This is OK since we only ever serialize valid StateIDs to
            // states. And since state IDs can never exceed an isize, they must
            // always be able to fit into a usize, and thus cast is OK.
            f(StateID::new_unchecked(sid as usize))
        }
    }

            fn pattern_offset_end(&self) -> usize {
        let encoded = self.encoded_pattern_count();
        if encoded == 0 {
            return 3;
        }
        // This arithmetic is OK since we were able to address this many bytes
        // when writing to the state, thus, it must fit into a usize.
        encoded.checked_mul(4).unwrap().checked_add(7).unwrap()
    }

                        fn encoded_pattern_count(&self) -> usize {
        if !self.has_pattern_ids() {
            return 0;
        }
        // This unwrap is OK since the total number of patterns is always
        // guaranteed to fit into a usize.
        usize::try_from(bytes::read_u32(&self.0[3..7])).unwrap()
    }
}

impl<'a> core::fmt::Debug for Repr<'a> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut nfa_ids = alloc::vec![];
        self.iter_nfa_state_ids(|sid| nfa_ids.push(sid));
        f.debug_struct("Repr")
            .field("is_match", &self.is_match())
            .field("is_from_word", &self.is_from_word())
            .field("look_have", &self.look_have())
            .field("look_need", &self.look_need())
            .field("match_pattern_ids", &self.match_pattern_ids())
            .field("nfa_state_ids", &nfa_ids)
            .finish()
    }
}

struct ReprVec<'a>(&'a mut Vec<u8>);

impl<'a> ReprVec<'a> {
                    fn set_is_match(&mut self) {
        self.0[0] |= 1 << 0;
    }

                            fn set_has_pattern_ids(&mut self) {
        self.0[0] |= 1 << 1;
    }

                        fn set_is_from_word(&mut self) {
        self.0[0] |= 1 << 2;
    }

        fn look_have_mut(&mut self) -> &mut LookSet {
        LookSet::from_repr_mut(&mut self.0[1])
    }

        fn look_need_mut(&mut self) -> &mut LookSet {
        LookSet::from_repr_mut(&mut self.0[2])
    }

                                fn add_match_pattern_id(&mut self, pid: PatternID) {
        // As a (somewhat small) space saving optimization, in the case where
        // a matching state has exactly one pattern ID, PatternID::ZERO, we do
        // not write either the pattern ID or the number of patterns encoded.
        // Instead, all we do is set the 'is_match' bit on this state. Overall,
        // this saves 8 bytes per match state for the overwhelming majority of
        // match states.
        //
        // In order to know whether pattern IDs need to be explicitly read or
        // not, we use another internal-only bit, 'has_pattern_ids', to
        // indicate whether they have been explicitly written or not.
        if !self.repr().has_pattern_ids() {
            if pid == PatternID::ZERO {
                self.set_is_match();
                return;
            }
            // Make room for 'close_match_pattern_ids' to write the total
            // number of pattern IDs written.
            self.0.extend(core::iter::repeat(0).take(PatternID::SIZE));
            self.set_has_pattern_ids();
            // If this was already a match state, then the only way that's
            // possible when the state doesn't have pattern IDs is if
            // PatternID::ZERO was added by the caller previously. In this
            // case, we are now adding a non-ZERO pattern ID after it, in
            // which case, we want to make sure to represent ZERO explicitly
            // now.
            if self.repr().is_match() {
                write_u32(self.0, 0)
            } else {
                // Otherwise, just make sure the 'is_match' bit is set.
                self.set_is_match();
            }
        }
        write_u32(self.0, pid.as_u32());
    }

                                    fn close_match_pattern_ids(&mut self) {
        // If we never wrote any pattern IDs, then there's nothing to do here.
        if !self.repr().has_pattern_ids() {
            return;
        }
        let patsize = PatternID::SIZE;
        let pattern_bytes = self.0.len() - 7;
        // Every pattern ID uses 4 bytes, so number of bytes should be
        // divisible by 4.
        assert_eq!(pattern_bytes % patsize, 0);
        // This unwrap is OK since we are guaranteed that the maximum number
        // of possible patterns fits into a u32.
        let count32 = u32::try_from(pattern_bytes / patsize).unwrap();
        bytes::NE::write_u32(count32, &mut self.0[3..7]);
    }

                fn add_nfa_state_id(&mut self, prev: &mut StateID, sid: StateID) {
        let delta = sid.as_i32() - prev.as_i32();
        write_vari32(self.0, delta);
        *prev = sid;
    }

        fn repr(&self) -> Repr<'_> {
        Repr(self.0.as_slice())
    }
}

fn write_vari32(data: &mut Vec<u8>, n: i32) {
    let mut un = (n as u32) << 1;
    if n < 0 {
        un = !un;
    }
    write_varu32(data, un)
}

fn read_vari32(data: &[u8]) -> (i32, usize) {
    let (un, i) = read_varu32(data);
    let mut n = (un >> 1) as i32;
    if un & 1 != 0 {
        n = !n;
    }
    (n, i)
}

fn write_varu32(data: &mut Vec<u8>, mut n: u32) {
    while n >= 0b1000_0000 {
        data.push((n as u8) | 0b1000_0000);
        n >>= 7;
    }
    data.push(n as u8);
}

fn read_varu32(data: &[u8]) -> (u32, usize) {
    // N.B. We can assume correctness here since we know that all varuints are
    // written with write_varu32. Hence, the 'as' uses and unchecked arithmetic
    // is all okay.
    let mut n: u32 = 0;
    let mut shift: u32 = 0;
    for (i, &b) in data.iter().enumerate() {
        if b < 0b1000_0000 {
            return (n | ((b as u32) << shift), i + 1);
        }
        n |= ((b as u32) & 0b0111_1111) << shift;
        shift += 7;
    }
    (0, 0)
}

fn write_u32(dst: &mut Vec<u8>, n: u32) {
    use crate::regex_automata::util::bytes::{Endian, NE};

    let start = dst.len();
    dst.extend(core::iter::repeat(0).take(mem::size_of::<u32>()));
    NE::write_u32(n, &mut dst[start..]);
}
