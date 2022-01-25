#[cfg(feature = "std")]
use core::cmp;
use core::{convert::TryFrom, fmt, iter, mem::size_of, slice};

#[cfg(feature = "std")]
use alloc::{
    collections::{BTreeMap, BTreeSet},
    vec,
    vec::Vec,
};

#[cfg(feature = "std")]
use crate::regex_automata::{
    dfa::{
        accel::Accel, determinize, error::Error, minimize::Minimizer, sparse,
    },
    nfa::thompson,
    util::alphabet::ByteSet,
    MatchKind,
};
use crate::regex_automata::{
    dfa::{
        accel::Accels,
        automaton::{fmt_state_indicator, Automaton},
        special::Special,
        DEAD,
    },
    util::{
        alphabet::{self, ByteClasses},
        bytes::{self, DeserializeError, Endian, SerializeError},
        id::{PatternID, StateID},
        start::Start,
    },
};

const LABEL: &str = "rust-regex-automata-dfa-dense";

const VERSION: u32 = 2;

#[cfg(feature = "std")]
#[derive(Clone, Copy, Debug, Default)]
pub struct Config {
    // As with other configuration types in this crate, we put all our knobs
    // in options so that we can distinguish between "default" and "not set."
    // This makes it possible to easily combine multiple configurations
    // without default values overwriting explicitly specified values. See the
    // 'overwrite' method.
    //
    // For docs on the fields below, see the corresponding method setters.
    anchored: Option<bool>,
    accelerate: Option<bool>,
    minimize: Option<bool>,
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quit: Option<ByteSet>,
    dfa_size_limit: Option<Option<usize>>,
    determinize_size_limit: Option<Option<usize>>,
}

#[cfg(feature = "std")]
impl Config {
        pub fn new() -> Config {
        Config::default()
    }

                                                                                                                                                                                                                                                                                                                                                                                        pub fn anchored(mut self, yes: bool) -> Config {
        self.anchored = Some(yes);
        self
    }

                                                                            pub fn accelerate(mut self, yes: bool) -> Config {
        self.accelerate = Some(yes);
        self
    }

                                                                                                                                                                    pub fn minimize(mut self, yes: bool) -> Config {
        self.minimize = Some(yes);
        self
    }

                                                                                                                                                                                                                                                                                                                                                                                                pub fn match_kind(mut self, kind: MatchKind) -> Config {
        self.match_kind = Some(kind);
        self
    }

                                                                                                                                                                                                                                                                                                        pub fn starts_for_each_pattern(mut self, yes: bool) -> Config {
        self.starts_for_each_pattern = Some(yes);
        self
    }

                                                                                                                    pub fn byte_classes(mut self, yes: bool) -> Config {
        self.byte_classes = Some(yes);
        self
    }

                                                                                                                                                                                                                                                                                        pub fn unicode_word_boundary(mut self, yes: bool) -> Config {
        // We have a separate option for this instead of just setting the
        // appropriate quit bytes here because we don't want to set quit bytes
        // for every regex. We only want to set them when the regex contains a
        // Unicode word boundary.
        self.unicode_word_boundary = Some(yes);
        self
    }

                                                                                                                                                                                                                                                                    pub fn quit(mut self, byte: u8, yes: bool) -> Config {
        if self.get_unicode_word_boundary() && !byte.is_ascii() && !yes {
            panic!(
                "cannot set non-ASCII byte to be non-quit when \
                 Unicode word boundaries are enabled"
            );
        }
        if self.quit.is_none() {
            self.quit = Some(ByteSet::empty());
        }
        if yes {
            self.quit.as_mut().unwrap().add(byte);
        } else {
            self.quit.as_mut().unwrap().remove(byte);
        }
        self
    }

                                                                                                                                                                                                        pub fn dfa_size_limit(mut self, bytes: Option<usize>) -> Config {
        self.dfa_size_limit = Some(bytes);
        self
    }

                                                                                                                                                                                                        pub fn determinize_size_limit(mut self, bytes: Option<usize>) -> Config {
        self.determinize_size_limit = Some(bytes);
        self
    }

        pub fn get_anchored(&self) -> bool {
        self.anchored.unwrap_or(false)
    }

            pub fn get_accelerate(&self) -> bool {
        self.accelerate.unwrap_or(true)
    }

            pub fn get_minimize(&self) -> bool {
        self.minimize.unwrap_or(false)
    }

        pub fn get_match_kind(&self) -> MatchKind {
        self.match_kind.unwrap_or(MatchKind::LeftmostFirst)
    }

            pub fn get_starts_for_each_pattern(&self) -> bool {
        self.starts_for_each_pattern.unwrap_or(false)
    }

                pub fn get_byte_classes(&self) -> bool {
        self.byte_classes.unwrap_or(true)
    }

                pub fn get_unicode_word_boundary(&self) -> bool {
        self.unicode_word_boundary.unwrap_or(false)
    }

                    pub fn get_quit(&self, byte: u8) -> bool {
        self.quit.map_or(false, |q| q.contains(byte))
    }

                    pub fn get_dfa_size_limit(&self) -> Option<usize> {
        self.dfa_size_limit.unwrap_or(None)
    }

                                                    pub fn get_determinize_size_limit(&self) -> Option<usize> {
        self.determinize_size_limit.unwrap_or(None)
    }

                    pub(crate) fn overwrite(self, o: Config) -> Config {
        Config {
            anchored: o.anchored.or(self.anchored),
            accelerate: o.accelerate.or(self.accelerate),
            minimize: o.minimize.or(self.minimize),
            match_kind: o.match_kind.or(self.match_kind),
            starts_for_each_pattern: o
                .starts_for_each_pattern
                .or(self.starts_for_each_pattern),
            byte_classes: o.byte_classes.or(self.byte_classes),
            unicode_word_boundary: o
                .unicode_word_boundary
                .or(self.unicode_word_boundary),
            quit: o.quit.or(self.quit),
            dfa_size_limit: o.dfa_size_limit.or(self.dfa_size_limit),
            determinize_size_limit: o
                .determinize_size_limit
                .or(self.determinize_size_limit),
        }
    }
}

#[cfg(feature = "std")]
#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    thompson: thompson::Builder,
}

#[cfg(feature = "std")]
impl Builder {
        pub fn new() -> Builder {
        Builder {
            config: Config::default(),
            thompson: thompson::Builder::new(),
        }
    }

                    pub fn build(&self, pattern: &str) -> Result<OwnedDFA, Error> {
        self.build_many(&[pattern])
    }

                    pub fn build_many<P: AsRef<str>>(
        &self,
        patterns: &[P],
    ) -> Result<OwnedDFA, Error> {
        let nfa = self.thompson.build_many(patterns).map_err(Error::nfa)?;
        self.build_from_nfa(&nfa)
    }

                                                                                                                pub fn build_from_nfa(
        &self,
        nfa: &thompson::NFA,
    ) -> Result<OwnedDFA, Error> {
        let mut quit = self.config.quit.unwrap_or(ByteSet::empty());
        if self.config.get_unicode_word_boundary()
            && nfa.has_word_boundary_unicode()
        {
            for b in 0x80..=0xFF {
                quit.add(b);
            }
        }
        let classes = if !self.config.get_byte_classes() {
            // DFAs will always use the equivalence class map, but enabling
            // this option is useful for debugging. Namely, this will cause all
            // transitions to be defined over their actual bytes instead of an
            // opaque equivalence class identifier. The former is much easier
            // to grok as a human.
            ByteClasses::singletons()
        } else {
            let mut set = nfa.byte_class_set().clone();
            // It is important to distinguish any "quit" bytes from all other
            // bytes. Otherwise, a non-quit byte may end up in the same class
            // as a quit byte, and thus cause the DFA stop when it shouldn't.
            if !quit.is_empty() {
                set.add_set(&quit);
            }
            set.byte_classes()
        };

        let mut dfa = DFA::initial(
            classes,
            nfa.match_len(),
            self.config.get_starts_for_each_pattern(),
        )?;
        determinize::Config::new()
            .anchored(self.config.get_anchored())
            .match_kind(self.config.get_match_kind())
            .quit(quit)
            .dfa_size_limit(self.config.get_dfa_size_limit())
            .determinize_size_limit(self.config.get_determinize_size_limit())
            .run(nfa, &mut dfa)?;
        if self.config.get_minimize() {
            dfa.minimize();
        }
        if self.config.get_accelerate() {
            dfa.accelerate();
        }
        Ok(dfa)
    }

        pub fn configure(&mut self, config: Config) -> &mut Builder {
        self.config = self.config.overwrite(config);
        self
    }

                                    pub fn syntax(
        &mut self,
        config: crate::regex_automata::util::syntax::SyntaxConfig,
    ) -> &mut Builder {
        self.thompson.syntax(config);
        self
    }

                                        pub fn thompson(&mut self, config: thompson::Config) -> &mut Builder {
        self.thompson.configure(config);
        self
    }
}

#[cfg(feature = "std")]
impl Default for Builder {
    fn default() -> Builder {
        Builder::new()
    }
}

#[cfg(feature = "std")]
pub(crate) type OwnedDFA = DFA<Vec<u32>>;

#[derive(Clone)]
pub struct DFA<T> {
                tt: TransitionTable<T>,
                    st: StartTable<T>,
                                        ms: MatchStates<T>,
                special: Special,
                                            accels: Accels<T>,
}

#[cfg(feature = "std")]
impl OwnedDFA {
                                                                    pub fn new(pattern: &str) -> Result<OwnedDFA, Error> {
        Builder::new().build(pattern)
    }

                                                                    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<OwnedDFA, Error> {
        Builder::new().build_many(patterns)
    }
}

#[cfg(feature = "std")]
impl OwnedDFA {
                                                            pub fn always_match() -> Result<OwnedDFA, Error> {
        let nfa = thompson::NFA::always_match();
        Builder::new().build_from_nfa(&nfa)
    }

                                                    pub fn never_match() -> Result<OwnedDFA, Error> {
        let nfa = thompson::NFA::never_match();
        Builder::new().build_from_nfa(&nfa)
    }

                fn initial(
        classes: ByteClasses,
        pattern_count: usize,
        starts_for_each_pattern: bool,
    ) -> Result<OwnedDFA, Error> {
        let start_pattern_count =
            if starts_for_each_pattern { pattern_count } else { 0 };
        Ok(DFA {
            tt: TransitionTable::minimal(classes),
            st: StartTable::dead(start_pattern_count)?,
            ms: MatchStates::empty(pattern_count),
            special: Special::new(),
            accels: Accels::empty(),
        })
    }
}

impl<T: AsRef<[u32]>> DFA<T> {
            pub fn as_ref(&self) -> DFA<&'_ [u32]> {
        DFA {
            tt: self.tt.as_ref(),
            st: self.st.as_ref(),
            ms: self.ms.as_ref(),
            special: self.special,
            accels: self.accels(),
        }
    }

                        #[cfg(feature = "std")]
    pub fn to_owned(&self) -> OwnedDFA {
        DFA {
            tt: self.tt.to_owned(),
            st: self.st.to_owned(),
            ms: self.ms.to_owned(),
            special: self.special,
            accels: self.accels().to_owned(),
        }
    }

                                        pub fn has_starts_for_each_pattern(&self) -> bool {
        self.st.patterns > 0
    }

                                                                                                    pub fn alphabet_len(&self) -> usize {
        self.tt.alphabet_len()
    }

                                                                                    pub fn stride2(&self) -> usize {
        self.tt.stride2
    }

                                pub fn stride(&self) -> usize {
        self.tt.stride()
    }

                                            pub(crate) fn universal_start_state(&self) -> StateID {
        // We choose 'NonWordByte' for no particular reason, other than
        // the fact that this is the 'main' starting configuration used in
        // determinization. But in essence, it doesn't really matter.
        //
        // Also, we might consider exposing this routine, but it seems
        // a little tricky to use correctly. Maybe if we also expose a
        // 'has_universal_start_state' method?
        self.st.start(Start::NonWordByte, None)
    }

                                pub fn memory_usage(&self) -> usize {
        self.tt.memory_usage()
            + self.st.memory_usage()
            + self.ms.memory_usage()
            + self.accels.memory_usage()
    }
}

impl<T: AsRef<[u32]>> DFA<T> {
                                                                                #[cfg(feature = "std")]
    pub fn to_sparse(&self) -> Result<sparse::DFA<Vec<u8>>, Error> {
        sparse::DFA::from_dense(self)
    }

                                                                                                                                                            #[cfg(feature = "std")]
    pub fn to_bytes_little_endian(&self) -> (Vec<u8>, usize) {
        self.to_bytes::<bytes::LE>()
    }

                                                                                                                                                            #[cfg(feature = "std")]
    pub fn to_bytes_big_endian(&self) -> (Vec<u8>, usize) {
        self.to_bytes::<bytes::BE>()
    }

                                                                                                                                                                                        #[cfg(feature = "std")]
    pub fn to_bytes_native_endian(&self) -> (Vec<u8>, usize) {
        self.to_bytes::<bytes::NE>()
    }

            #[cfg(feature = "std")]
    fn to_bytes<E: Endian>(&self) -> (Vec<u8>, usize) {
        let len = self.write_to_len();
        let (mut buf, padding) = bytes::alloc_aligned_buffer::<u32>(len);
        // This should always succeed since the only possible serialization
        // error is providing a buffer that's too small, but we've ensured that
        // `buf` is big enough here.
        self.as_ref().write_to::<E>(&mut buf[padding..]).unwrap();
        (buf, padding)
    }

                                                                                                                                                                                pub fn write_to_little_endian(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        self.as_ref().write_to::<bytes::LE>(dst)
    }

                                                                                                                                                                                pub fn write_to_big_endian(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        self.as_ref().write_to::<bytes::BE>(dst)
    }

                                                                                                                                                                                                            pub fn write_to_native_endian(
        &self,
        dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        self.as_ref().write_to::<bytes::NE>(dst)
    }

                                                                                                                                                            pub fn write_to_len(&self) -> usize {
        bytes::write_label_len(LABEL)
        + bytes::write_endianness_check_len()
        + bytes::write_version_len()
        + size_of::<u32>() // unused, intended for future flexibility
        + self.tt.write_to_len()
        + self.st.write_to_len()
        + self.ms.write_to_len()
        + self.special.write_to_len()
        + self.accels.write_to_len()
    }
}

impl<'a> DFA<&'a [u32]> {
                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                pub fn from_bytes(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u32]>, usize), DeserializeError> {
        // SAFETY: This is safe because we validate both the transition table,
        // start state ID list and the match states below. If either validation
        // fails, then we return an error.
        let (dfa, nread) = unsafe { DFA::from_bytes_unchecked(slice)? };
        dfa.tt.validate()?;
        dfa.st.validate(&dfa.tt)?;
        dfa.ms.validate(&dfa)?;
        dfa.accels.validate()?;
        // N.B. dfa.special doesn't have a way to do unchecked deserialization,
        // so it has already been validated.
        Ok((dfa, nread))
    }

                                                                                                                pub unsafe fn from_bytes_unchecked(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u32]>, usize), DeserializeError> {
        let mut nr = 0;

        nr += bytes::skip_initial_padding(slice);
        bytes::check_alignment::<StateID>(&slice[nr..])?;
        nr += bytes::read_label(&slice[nr..], LABEL)?;
        nr += bytes::read_endianness_check(&slice[nr..])?;
        nr += bytes::read_version(&slice[nr..], VERSION)?;

        let _unused = bytes::try_read_u32(&slice[nr..], "unused space")?;
        nr += size_of::<u32>();

        let (tt, nread) = TransitionTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (st, nread) = StartTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (ms, nread) = MatchStates::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (special, nread) = Special::from_bytes(&slice[nr..])?;
        nr += nread;
        special.validate_state_count(tt.count(), tt.stride2)?;

        let (accels, nread) = Accels::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        Ok((DFA { tt, st, ms, special, accels }, nr))
    }

                    fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("dense DFA"));
        }
        dst = &mut dst[..nwrite];

        let mut nw = 0;
        nw += bytes::write_label(LABEL, &mut dst[nw..])?;
        nw += bytes::write_endianness_check::<E>(&mut dst[nw..])?;
        nw += bytes::write_version::<E>(VERSION, &mut dst[nw..])?;
        nw += {
            // Currently unused, intended for future flexibility
            E::write_u32(0, &mut dst[nw..]);
            size_of::<u32>()
        };
        nw += self.tt.write_to::<E>(&mut dst[nw..])?;
        nw += self.st.write_to::<E>(&mut dst[nw..])?;
        nw += self.ms.write_to::<E>(&mut dst[nw..])?;
        nw += self.special.write_to::<E>(&mut dst[nw..])?;
        nw += self.accels.write_to::<E>(&mut dst[nw..])?;
        Ok(nw)
    }
}

#[cfg(feature = "std")]
impl OwnedDFA {
        pub(crate) fn set_start_state(
        &mut self,
        index: Start,
        pattern_id: Option<PatternID>,
        id: StateID,
    ) {
        assert!(self.tt.is_valid(id), "invalid start state");
        self.st.set_start(index, pattern_id, id);
    }

            pub(crate) fn set_transition(
        &mut self,
        from: StateID,
        byte: alphabet::Unit,
        to: StateID,
    ) {
        self.tt.set(from, byte, to);
    }

                            pub(crate) fn add_empty_state(&mut self) -> Result<StateID, Error> {
        self.tt.add_empty_state()
    }

                        pub(crate) fn swap_states(&mut self, id1: StateID, id2: StateID) {
        self.tt.swap(id1, id2);
    }

                        pub(crate) fn truncate_states(&mut self, count: usize) {
        self.tt.truncate(count);
    }

                pub(crate) fn state_mut(&mut self, id: StateID) -> StateMut<'_> {
        self.tt.state_mut(id)
    }

        pub(crate) fn minimize(&mut self) {
        Minimizer::new(self).run();
    }

                                pub(crate) fn set_pattern_map(
        &mut self,
        map: &BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), Error> {
        self.ms = self.ms.new_with_map(map)?;
        Ok(())
    }

            pub(crate) fn accelerate(&mut self) {
        // dead and quit states can never be accelerated.
        if self.state_count() <= 2 {
            return;
        }

        // Go through every state and record their accelerator, if possible.
        let mut accels = BTreeMap::new();
        // Count the number of accelerated match, start and non-match/start
        // states.
        let (mut cmatch, mut cstart, mut cnormal) = (0, 0, 0);
        for state in self.states() {
            if let Some(accel) = state.accelerate(self.byte_classes()) {
                accels.insert(state.id(), accel);
                if self.is_match_state(state.id()) {
                    cmatch += 1;
                } else if self.is_start_state(state.id()) {
                    cstart += 1;
                } else {
                    assert!(!self.is_dead_state(state.id()));
                    assert!(!self.is_quit_state(state.id()));
                    cnormal += 1;
                }
            }
        }
        // If no states were able to be accelerated, then we're done.
        if accels.is_empty() {
            return;
        }
        let original_accels_len = accels.len();

        // A remapper keeps track of state ID changes. Once we're done
        // shuffling, the remapper is used to rewrite all transitions in the
        // DFA based on the new positions of states.
        let mut remapper = Remapper::from_dfa(self);

        // As we swap states, if they are match states, we need to swap their
        // pattern ID lists too (for multi-regexes). We do this by converting
        // the lists to an easily swappable map, and then convert back to
        // MatchStates once we're done.
        let mut new_matches = self.ms.to_map(self);

        // There is at least one state that gets accelerated, so these are
        // guaranteed to get set to sensible values below.
        self.special.min_accel = StateID::MAX;
        self.special.max_accel = StateID::ZERO;
        let update_special_accel =
            |special: &mut Special, accel_id: StateID| {
                special.min_accel = cmp::min(special.min_accel, accel_id);
                special.max_accel = cmp::max(special.max_accel, accel_id);
            };

        // Start by shuffling match states. Any match states that are
        // accelerated get moved to the end of the match state range.
        if cmatch > 0 && self.special.matches() {
            // N.B. special.{min,max}_match do not need updating, since the
            // range/number of match states does not change. Only the ordering
            // of match states may change.
            let mut next_id = self.special.max_match;
            let mut cur_id = next_id;
            while cur_id >= self.special.min_match {
                if let Some(accel) = accels.remove(&cur_id) {
                    accels.insert(next_id, accel);
                    update_special_accel(&mut self.special, next_id);

                    // No need to do any actual swapping for equivalent IDs.
                    if cur_id != next_id {
                        remapper.swap(self, cur_id, next_id);

                        // Swap pattern IDs for match states.
                        let cur_pids = new_matches.remove(&cur_id).unwrap();
                        let next_pids = new_matches.remove(&next_id).unwrap();
                        new_matches.insert(cur_id, next_pids);
                        new_matches.insert(next_id, cur_pids);
                    }
                    next_id = self.tt.prev_state_id(next_id);
                }
                cur_id = self.tt.prev_state_id(cur_id);
            }
        }

        // This is where it gets tricky. Without acceleration, start states
        // normally come right after match states. But we want accelerated
        // states to be a single contiguous range (to make it very fast
        // to determine whether a state *is* accelerated), while also keeping
        // match and starting states as contiguous ranges for the same reason.
        // So what we do here is shuffle states such that it looks like this:
        //
        //     DQMMMMAAAAASSSSSSNNNNNNN
        //         |         |
        //         |---------|
        //      accelerated states
        //
        // Where:
        //   D - dead state
        //   Q - quit state
        //   M - match state (may be accelerated)
        //   A - normal state that is accelerated
        //   S - start state (may be accelerated)
        //   N - normal state that is NOT accelerated
        //
        // We implement this by shuffling states, which is done by a sequence
        // of pairwise swaps. We start by looking at all normal states to be
        // accelerated. When we find one, we swap it with the earliest starting
        // state, and then swap that with the earliest normal state. This
        // preserves the contiguous property.
        //
        // Once we're done looking for accelerated normal states, now we look
        // for accelerated starting states by moving them to the beginning
        // of the starting state range (just like we moved accelerated match
        // states to the end of the matching state range).
        //
        // For a more detailed/different perspective on this, see the docs
        // in dfa/special.rs.
        if cnormal > 0 {
            // our next available starting and normal states for swapping.
            let mut next_start_id = self.special.min_start;
            let mut cur_id = self.from_index(self.state_count() - 1);
            // This is guaranteed to exist since cnormal > 0.
            let mut next_norm_id =
                self.tt.next_state_id(self.special.max_start);
            while cur_id >= next_norm_id {
                if let Some(accel) = accels.remove(&cur_id) {
                    remapper.swap(self, next_start_id, cur_id);
                    remapper.swap(self, next_norm_id, cur_id);
                    // Keep our accelerator map updated with new IDs if the
                    // states we swapped were also accelerated.
                    if let Some(accel2) = accels.remove(&next_norm_id) {
                        accels.insert(cur_id, accel2);
                    }
                    if let Some(accel2) = accels.remove(&next_start_id) {
                        accels.insert(next_norm_id, accel2);
                    }
                    accels.insert(next_start_id, accel);
                    update_special_accel(&mut self.special, next_start_id);
                    // Our start range shifts one to the right now.
                    self.special.min_start =
                        self.tt.next_state_id(self.special.min_start);
                    self.special.max_start =
                        self.tt.next_state_id(self.special.max_start);
                    next_start_id = self.tt.next_state_id(next_start_id);
                    next_norm_id = self.tt.next_state_id(next_norm_id);
                }
                // This is pretty tricky, but if our 'next_norm_id' state also
                // happened to be accelerated, then the result is that it is
                // now in the position of cur_id, so we need to consider it
                // again. This loop is still guaranteed to terminate though,
                // because when accels contains cur_id, we're guaranteed to
                // increment next_norm_id even if cur_id remains unchanged.
                if !accels.contains_key(&cur_id) {
                    cur_id = self.tt.prev_state_id(cur_id);
                }
            }
        }
        // Just like we did for match states, but we want to move accelerated
        // start states to the beginning of the range instead of the end.
        if cstart > 0 {
            // N.B. special.{min,max}_start do not need updating, since the
            // range/number of start states does not change at this point. Only
            // the ordering of start states may change.
            let mut next_id = self.special.min_start;
            let mut cur_id = next_id;
            while cur_id <= self.special.max_start {
                if let Some(accel) = accels.remove(&cur_id) {
                    remapper.swap(self, cur_id, next_id);
                    accels.insert(next_id, accel);
                    update_special_accel(&mut self.special, next_id);
                    next_id = self.tt.next_state_id(next_id);
                }
                cur_id = self.tt.next_state_id(cur_id);
            }
        }

        // Remap all transitions in our DFA and assert some things.
        remapper.remap(self);
        // This unwrap is OK because acceleration never changes the number of
        // match states or patterns in those match states. Since acceleration
        // runs after the pattern map has been set at least once, we know that
        // our match states cannot error.
        self.set_pattern_map(&new_matches).unwrap();
        self.special.set_max();
        self.special.validate().expect("special state ranges should validate");
        self.special
            .validate_state_count(self.state_count(), self.stride2())
            .expect(
                "special state ranges should be consistent with state count",
            );
        assert_eq!(
            self.special.accel_len(self.stride()),
            // We record the number of accelerated states initially detected
            // since the accels map is itself mutated in the process above.
            // If mutated incorrectly, its size may change, and thus can't be
            // trusted as a source of truth of how many accelerated states we
            // expected there to be.
            original_accels_len,
            "mismatch with expected number of accelerated states",
        );

        // And finally record our accelerators. We kept our accels map updated
        // as we shuffled states above, so the accelerators should now
        // correspond to a contiguous range in the state ID space. (Which we
        // assert.)
        let mut prev: Option<StateID> = None;
        for (id, accel) in accels {
            assert!(prev.map_or(true, |p| self.tt.next_state_id(p) == id));
            prev = Some(id);
            self.accels.add(accel);
        }
    }

                    pub(crate) fn shuffle(
        &mut self,
        mut matches: BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<(), Error> {
        // The determinizer always adds a quit state and it is always second.
        self.special.quit_id = self.from_index(1);
        // If all we have are the dead and quit states, then we're done and
        // the DFA will never produce a match.
        if self.state_count() <= 2 {
            self.special.set_max();
            return Ok(());
        }

        // Collect all our start states into a convenient set and confirm there
        // is no overlap with match states. In the classicl DFA construction,
        // start states can be match states. But because of look-around, we
        // delay all matches by a byte, which prevents start states from being
        // match states.
        let mut is_start: BTreeSet<StateID> = BTreeSet::new();
        for (start_id, _, _) in self.starts() {
            // While there's nothing theoretically wrong with setting a start
            // state to a dead ID (indeed, it could be an optimization!), the
            // shuffling code below assumes that start states aren't dead. If
            // this assumption is violated, the dead state could be shuffled
            // to a new location, which must never happen. So if we do want
            // to allow start states to be dead, then this assert should be
            // removed and the code below fixed.
            //
            // N.B. Minimization can cause start states to be dead, but that
            // happens after states are shuffled, so it's OK. Also, start
            // states are dead for the DFA that never matches anything, but
            // in that case, there are no states to shuffle.
            assert_ne!(start_id, DEAD, "start state cannot be dead");
            assert!(
                !matches.contains_key(&start_id),
                "{:?} is both a start and a match state, which is not allowed",
                start_id,
            );
            is_start.insert(start_id);
        }

        // We implement shuffling by a sequence of pairwise swaps of states.
        // Since we have a number of things referencing states via their
        // IDs and swapping them changes their IDs, we need to record every
        // swap we make so that we can remap IDs. The remapper handles this
        // book-keeping for us.
        let mut remapper = Remapper::from_dfa(self);

        // Shuffle matching states.
        if matches.is_empty() {
            self.special.min_match = DEAD;
            self.special.max_match = DEAD;
        } else {
            // The determinizer guarantees that the first two states are the
            // dead and quit states, respectively. We want our match states to
            // come right after quit.
            let mut next_id = self.from_index(2);
            let mut new_matches = BTreeMap::new();
            self.special.min_match = next_id;
            for (id, pids) in matches {
                remapper.swap(self, next_id, id);
                new_matches.insert(next_id, pids);
                // If we swapped a start state, then update our set.
                if is_start.contains(&next_id) {
                    is_start.remove(&next_id);
                    is_start.insert(id);
                }
                next_id = self.tt.next_state_id(next_id);
            }
            matches = new_matches;
            self.special.max_match = cmp::max(
                self.special.min_match,
                self.tt.prev_state_id(next_id),
            );
        }

        // Shuffle starting states.
        {
            let mut next_id = self.from_index(2);
            if self.special.matches() {
                next_id = self.tt.next_state_id(self.special.max_match);
            }
            self.special.min_start = next_id;
            for id in is_start {
                remapper.swap(self, next_id, id);
                next_id = self.tt.next_state_id(next_id);
            }
            self.special.max_start = cmp::max(
                self.special.min_start,
                self.tt.prev_state_id(next_id),
            );
        }

        // Finally remap all transitions in our DFA.
        remapper.remap(self);
        self.set_pattern_map(&matches)?;
        self.special.set_max();
        self.special.validate().expect("special state ranges should validate");
        self.special
            .validate_state_count(self.state_count(), self.stride2())
            .expect(
                "special state ranges should be consistent with state count",
            );
        Ok(())
    }
}

impl<T: AsRef<[u32]>> DFA<T> {
        pub(crate) fn byte_classes(&self) -> &ByteClasses {
        &self.tt.classes
    }

        pub(crate) fn special(&self) -> &Special {
        &self.special
    }

        #[cfg(feature = "std")]
    pub(crate) fn special_mut(&mut self) -> &mut Special {
        &mut self.special
    }

                        pub(crate) fn states(&self) -> StateIter<'_, T> {
        self.tt.states()
    }

            pub(crate) fn state_count(&self) -> usize {
        self.tt.count()
    }

                #[cfg(feature = "std")]
    pub(crate) fn pattern_id_slice(&self, id: StateID) -> &[PatternID] {
        assert!(self.is_match_state(id));
        self.ms.pattern_id_slice(self.match_state_index(id))
    }

                pub(crate) fn match_pattern_len(&self, id: StateID) -> usize {
        assert!(self.is_match_state(id));
        self.ms.pattern_len(self.match_state_index(id))
    }

        pub(crate) fn pattern_count(&self) -> usize {
        self.ms.patterns
    }

            #[cfg(feature = "std")]
    pub(crate) fn pattern_map(&self) -> BTreeMap<StateID, Vec<PatternID>> {
        self.ms.to_map(self)
    }

        #[cfg(feature = "std")]
    pub(crate) fn quit_id(&self) -> StateID {
        self.from_index(1)
    }

                        pub(crate) fn to_index(&self, id: StateID) -> usize {
        self.tt.to_index(id)
    }

                        #[cfg(feature = "std")]
    pub(crate) fn from_index(&self, index: usize) -> StateID {
        self.tt.from_index(index)
    }

        pub(crate) fn starts(&self) -> StartStateIter<'_> {
        self.st.iter()
    }

                fn match_state_index(&self, id: StateID) -> usize {
        debug_assert!(self.is_match_state(id));
        // This is one of the places where we rely on the fact that match
        // states are contiguous in the transition table. Namely, that the
        // first match state ID always corresponds to dfa.special.min_start.
        // From there, since we know the stride, we can compute the overall
        // index of any match state given the match state's ID.
        let min = self.special().min_match.as_usize();
        // CORRECTNESS: We're allowed to produce an incorrect result or panic,
        // so both the subtraction and the unchecked StateID construction is
        // OK.
        self.to_index(StateID::new_unchecked(id.as_usize() - min))
    }

                fn accelerator_index(&self, id: StateID) -> usize {
        let min = self.special().min_accel.as_usize();
        // CORRECTNESS: We're allowed to produce an incorrect result or panic,
        // so both the subtraction and the unchecked StateID construction is
        // OK.
        self.to_index(StateID::new_unchecked(id.as_usize() - min))
    }

        fn accels(&self) -> Accels<&[u32]> {
        self.accels.as_ref()
    }

        fn trans(&self) -> &[StateID] {
        self.tt.table()
    }
}

impl<T: AsRef<[u32]>> fmt::Debug for DFA<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "dense::DFA(")?;
        for state in self.states() {
            fmt_state_indicator(f, self, state.id())?;
            let id = if f.alternate() {
                state.id().as_usize()
            } else {
                self.to_index(state.id())
            };
            write!(f, "{:06?}: ", id)?;
            state.fmt(f)?;
            write!(f, "\n")?;
        }
        writeln!(f, "")?;
        for (i, (start_id, sty, pid)) in self.starts().enumerate() {
            let id = if f.alternate() {
                start_id.as_usize()
            } else {
                self.to_index(start_id)
            };
            if i % self.st.stride == 0 {
                match pid {
                    None => writeln!(f, "START-GROUP(ALL)")?,
                    Some(pid) => {
                        writeln!(f, "START_GROUP(pattern: {:?})", pid)?
                    }
                }
            }
            writeln!(f, "  {:?} => {:06?}", sty, id)?;
        }
        if self.pattern_count() > 1 {
            writeln!(f, "")?;
            for i in 0..self.ms.count() {
                let id = self.ms.match_state_id(self, i);
                let id = if f.alternate() {
                    id.as_usize()
                } else {
                    self.to_index(id)
                };
                write!(f, "MATCH({:06?}): ", id)?;
                for (i, &pid) in self.ms.pattern_id_slice(i).iter().enumerate()
                {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{:?}", pid)?;
                }
                writeln!(f, "")?;
            }
        }
        writeln!(f, "state count: {:?}", self.state_count())?;
        writeln!(f, "pattern count: {:?}", self.pattern_count())?;
        writeln!(f, ")")?;
        Ok(())
    }
}

unsafe impl<T: AsRef<[u32]>> Automaton for DFA<T> {
    #[inline]
    fn is_special_state(&self, id: StateID) -> bool {
        self.special.is_special_state(id)
    }

    #[inline]
    fn is_dead_state(&self, id: StateID) -> bool {
        self.special.is_dead_state(id)
    }

    #[inline]
    fn is_quit_state(&self, id: StateID) -> bool {
        self.special.is_quit_state(id)
    }

    #[inline]
    fn is_match_state(&self, id: StateID) -> bool {
        self.special.is_match_state(id)
    }

    #[inline]
    fn is_start_state(&self, id: StateID) -> bool {
        self.special.is_start_state(id)
    }

    #[inline]
    fn is_accel_state(&self, id: StateID) -> bool {
        self.special.is_accel_state(id)
    }

    #[inline]
    fn next_state(&self, current: StateID, input: u8) -> StateID {
        let input = self.byte_classes().get(input);
        let o = current.as_usize() + usize::from(input);
        self.trans()[o]
    }

    #[inline]
    unsafe fn next_state_unchecked(
        &self,
        current: StateID,
        input: u8,
    ) -> StateID {
        let input = self.byte_classes().get_unchecked(input);
        let o = current.as_usize() + usize::from(input);
        *self.trans().get_unchecked(o)
    }

    #[inline]
    fn next_eoi_state(&self, current: StateID) -> StateID {
        let eoi = self.byte_classes().eoi().as_usize();
        let o = current.as_usize() + eoi;
        self.trans()[o]
    }

    #[inline]
    fn pattern_count(&self) -> usize {
        self.ms.patterns
    }

    #[inline]
    fn match_count(&self, id: StateID) -> usize {
        self.match_pattern_len(id)
    }

    #[inline]
    fn match_pattern(&self, id: StateID, match_index: usize) -> PatternID {
        // This is an optimization for the very common case of a DFA with a
        // single pattern. This conditional avoids a somewhat more costly path
        // that finds the pattern ID from the state machine, which requires
        // a bit of slicing/pointer-chasing. This optimization tends to only
        // matter when matches are frequent.
        if self.ms.patterns == 1 {
            return PatternID::ZERO;
        }
        let state_index = self.match_state_index(id);
        self.ms.pattern_id(state_index, match_index)
    }

    #[inline]
    fn start_state_forward(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> StateID {
        let index = Start::from_position_fwd(bytes, start, end);
        self.st.start(index, pattern_id)
    }

    #[inline]
    fn start_state_reverse(
        &self,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> StateID {
        let index = Start::from_position_rev(bytes, start, end);
        self.st.start(index, pattern_id)
    }

    #[inline(always)]
    fn accelerator(&self, id: StateID) -> &[u8] {
        if !self.is_accel_state(id) {
            return &[];
        }
        self.accels.needles(self.accelerator_index(id))
    }
}

#[derive(Clone)]
pub(crate) struct TransitionTable<T> {
                                        table: T,
                                                                                classes: ByteClasses,
                                                                                stride2: usize,
}

impl<'a> TransitionTable<&'a [u32]> {
                                                                                                unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(TransitionTable<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr() as usize;

        let (count, nr) = bytes::try_read_u32_as_usize(slice, "state count")?;
        slice = &slice[nr..];

        let (stride2, nr) = bytes::try_read_u32_as_usize(slice, "stride2")?;
        slice = &slice[nr..];

        let (classes, nr) = ByteClasses::from_bytes(slice)?;
        slice = &slice[nr..];

        // The alphabet length (determined by the byte class map) cannot be
        // bigger than the stride (total space used by each DFA state).
        if stride2 > 9 {
            return Err(DeserializeError::generic(
                "dense DFA has invalid stride2 (too big)",
            ));
        }
        // It also cannot be zero, since even a DFA that never matches anything
        // has a non-zero number of states with at least two equivalence
        // classes: one for all 256 byte values and another for the EOI
        // sentinel.
        if stride2 < 1 {
            return Err(DeserializeError::generic(
                "dense DFA has invalid stride2 (too small)",
            ));
        }
        // This is OK since 1 <= stride2 <= 9.
        let stride =
            1usize.checked_shl(u32::try_from(stride2).unwrap()).unwrap();
        if classes.alphabet_len() > stride {
            return Err(DeserializeError::generic(
                "alphabet size cannot be bigger than transition table stride",
            ));
        }

        let trans_count =
            bytes::shl(count, stride2, "dense table transition count")?;
        let table_bytes_len = bytes::mul(
            trans_count,
            StateID::SIZE,
            "dense table state byte count",
        )?;
        bytes::check_slice_len(slice, table_bytes_len, "transition table")?;
        bytes::check_alignment::<StateID>(slice)?;
        let table_bytes = &slice[..table_bytes_len];
        slice = &slice[table_bytes_len..];
        // SAFETY: Since StateID is always representable as a u32, all we need
        // to do is ensure that we have the proper length and alignment. We've
        // checked both above, so the cast below is safe.
        //
        // N.B. This is the only not-safe code in this function, so we mark
        // it explicitly to call it out, even though it is technically
        // superfluous.
        #[allow(unused_unsafe)]
        let table = unsafe {
            core::slice::from_raw_parts(
                table_bytes.as_ptr() as *const u32,
                trans_count,
            )
        };
        let tt = TransitionTable { table, classes, stride2 };
        Ok((tt, slice.as_ptr() as usize - slice_start))
    }
}

#[cfg(feature = "std")]
impl TransitionTable<Vec<u32>> {
                fn minimal(classes: ByteClasses) -> TransitionTable<Vec<u32>> {
        let mut tt = TransitionTable {
            table: vec![],
            classes,
            stride2: classes.stride2(),
        };
        // Two states, regardless of alphabet size, can always fit into u32.
        tt.add_empty_state().unwrap(); // dead state
        tt.add_empty_state().unwrap(); // quit state
        tt
    }

                fn set(&mut self, from: StateID, unit: alphabet::Unit, to: StateID) {
        assert!(self.is_valid(from), "invalid 'from' state");
        assert!(self.is_valid(to), "invalid 'to' state");
        self.table[from.as_usize() + self.classes.get_by_unit(unit)] =
            to.as_u32();
    }

                            fn add_empty_state(&mut self) -> Result<StateID, Error> {
        // Normally, to get a fresh state identifier, we would just
        // take the index of the next state added to the transition
        // table. However, we actually perform an optimization here
        // that premultiplies state IDs by the stride, such that they
        // point immediately at the beginning of their transitions in
        // the transition table. This avoids an extra multiplication
        // instruction for state lookup at search time.
        //
        // Premultiplied identifiers means that instead of your matching
        // loop looking something like this:
        //
        //   state = dfa.start
        //   for byte in haystack:
        //       next = dfa.transitions[state * stride + byte]
        //       if dfa.is_match(next):
        //           return true
        //   return false
        //
        // it can instead look like this:
        //
        //   state = dfa.start
        //   for byte in haystack:
        //       next = dfa.transitions[state + byte]
        //       if dfa.is_match(next):
        //           return true
        //   return false
        //
        // In other words, we save a multiplication instruction in the
        // critical path. This turns out to be a decent performance win.
        // The cost of using premultiplied state ids is that they can
        // require a bigger state id representation. (And they also make
        // the code a bit more complex, especially during minimization and
        // when reshuffling states, as one needs to convert back and forth
        // between state IDs and state indices.)
        //
        // To do this, we simply take the index of the state into the
        // entire transition table, rather than the index of the state
        // itself. e.g., If the stride is 64, then the ID of the 3rd state
        // is 192, not 2.
        let next = self.table.len();
        let id = StateID::new(next).map_err(|_| Error::too_many_states())?;
        self.table.extend(iter::repeat(0).take(self.stride()));
        Ok(id)
    }

                                fn swap(&mut self, id1: StateID, id2: StateID) {
        assert!(self.is_valid(id1), "invalid 'id1' state: {:?}", id1);
        assert!(self.is_valid(id2), "invalid 'id2' state: {:?}", id2);
        // We only need to swap the parts of the state that are used. So if the
        // stride is 64, but the alphabet length is only 33, then we save a lot
        // of work.
        for b in 0..self.classes.alphabet_len() {
            self.table.swap(id1.as_usize() + b, id2.as_usize() + b);
        }
    }

                        fn truncate(&mut self, count: usize) {
        self.table.truncate(count << self.stride2);
    }

                fn state_mut(&mut self, id: StateID) -> StateMut<'_> {
        let alphabet_len = self.alphabet_len();
        let i = id.as_usize();
        StateMut {
            id,
            stride2: self.stride2,
            transitions: &mut self.table_mut()[i..i + alphabet_len],
        }
    }
}

impl<T: AsRef<[u32]>> TransitionTable<T> {
                fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("transition table"));
        }
        dst = &mut dst[..nwrite];

        // write state count
        // Unwrap is OK since number of states is guaranteed to fit in a u32.
        E::write_u32(u32::try_from(self.count()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write state stride (as power of 2)
        // Unwrap is OK since stride2 is guaranteed to be <= 9.
        E::write_u32(u32::try_from(self.stride2).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write byte class map
        let n = self.classes.write_to(dst)?;
        dst = &mut dst[n..];

        // write actual transitions
        for &sid in self.table() {
            let n = bytes::write_state_id::<E>(sid, &mut dst);
            dst = &mut dst[n..];
        }
        Ok(nwrite)
    }

            fn write_to_len(&self) -> usize {
        size_of::<u32>()   // state count
        + size_of::<u32>() // stride2
        + self.classes.write_to_len()
        + (self.table().len() * StateID::SIZE)
    }

                    fn validate(&self) -> Result<(), DeserializeError> {
        for state in self.states() {
            for (_, to) in state.transitions() {
                if !self.is_valid(to) {
                    return Err(DeserializeError::generic(
                        "found invalid state ID in transition table",
                    ));
                }
            }
        }
        Ok(())
    }

        fn as_ref(&self) -> TransitionTable<&'_ [u32]> {
        TransitionTable {
            table: self.table.as_ref(),
            classes: self.classes.clone(),
            stride2: self.stride2,
        }
    }

        #[cfg(feature = "std")]
    fn to_owned(&self) -> TransitionTable<Vec<u32>> {
        TransitionTable {
            table: self.table.as_ref().to_vec(),
            classes: self.classes.clone(),
            stride2: self.stride2,
        }
    }

            fn state(&self, id: StateID) -> State<'_> {
        assert!(self.is_valid(id));

        let i = id.as_usize();
        State {
            id,
            stride2: self.stride2,
            transitions: &self.table()[i..i + self.alphabet_len()],
        }
    }

                        fn states(&self) -> StateIter<'_, T> {
        StateIter {
            tt: self,
            it: self.table().chunks(self.stride()).enumerate(),
        }
    }

                                    fn to_index(&self, id: StateID) -> usize {
        id.as_usize() >> self.stride2
    }

                                    fn from_index(&self, index: usize) -> StateID {
        // CORRECTNESS: If the given index is not valid, then it is not
        // required for this to panic or return a valid state ID.
        StateID::new_unchecked(index << self.stride2)
    }

                        #[cfg(feature = "std")]
    fn next_state_id(&self, id: StateID) -> StateID {
        self.from_index(self.to_index(id).checked_add(1).unwrap())
    }

                #[cfg(feature = "std")]
    fn prev_state_id(&self, id: StateID) -> StateID {
        self.from_index(self.to_index(id).checked_sub(1).unwrap())
    }

        fn table(&self) -> &[StateID] {
        let integers = self.table.as_ref();
        // SAFETY: This is safe because StateID is guaranteed to be
        // representable as a u32.
        unsafe {
            core::slice::from_raw_parts(
                integers.as_ptr() as *const StateID,
                integers.len(),
            )
        }
    }

                            fn count(&self) -> usize {
        self.table().len() >> self.stride2
    }

                fn stride(&self) -> usize {
        1 << self.stride2
    }

                    fn alphabet_len(&self) -> usize {
        self.classes.alphabet_len()
    }

                    fn is_valid(&self, id: StateID) -> bool {
        let id = id.as_usize();
        id < self.table().len() && id % self.stride() == 0
    }

                fn memory_usage(&self) -> usize {
        self.table().len() * StateID::SIZE
    }
}

#[cfg(feature = "std")]
impl<T: AsMut<[u32]>> TransitionTable<T> {
        fn table_mut(&mut self) -> &mut [StateID] {
        let integers = self.table.as_mut();
        // SAFETY: This is safe because StateID is guaranteed to be
        // representable as a u32.
        unsafe {
            core::slice::from_raw_parts_mut(
                integers.as_mut_ptr() as *mut StateID,
                integers.len(),
            )
        }
    }
}

#[derive(Clone)]
pub(crate) struct StartTable<T> {
                                        table: T,
        stride: usize,
                        patterns: usize,
}

#[cfg(feature = "std")]
impl StartTable<Vec<u32>> {
                                            fn dead(patterns: usize) -> Result<StartTable<Vec<u32>>, Error> {
        assert!(patterns <= PatternID::LIMIT);
        let stride = Start::count();
        let pattern_starts_len = match stride.checked_mul(patterns) {
            Some(x) => x,
            None => return Err(Error::too_many_start_states()),
        };
        let table_len = match stride.checked_add(pattern_starts_len) {
            Some(x) => x,
            None => return Err(Error::too_many_start_states()),
        };
        if table_len > core::isize::MAX as usize {
            return Err(Error::too_many_start_states());
        }
        let table = vec![DEAD.as_u32(); table_len];
        Ok(StartTable { table, stride, patterns })
    }
}

impl<'a> StartTable<&'a [u32]> {
                                                                                                    unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(StartTable<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr() as usize;

        let (stride, nr) =
            bytes::try_read_u32_as_usize(slice, "start table stride")?;
        slice = &slice[nr..];

        let (patterns, nr) =
            bytes::try_read_u32_as_usize(slice, "start table patterns")?;
        slice = &slice[nr..];

        if stride != Start::count() {
            return Err(DeserializeError::generic(
                "invalid starting table stride",
            ));
        }
        if patterns > PatternID::LIMIT {
            return Err(DeserializeError::generic(
                "invalid number of patterns",
            ));
        }
        let pattern_table_size =
            bytes::mul(stride, patterns, "invalid pattern count")?;
        // Our start states always start with a single stride of start states
        // for the entire automaton which permit it to match any pattern. What
        // follows it are an optional set of start states for each pattern.
        let start_state_count = bytes::add(
            stride,
            pattern_table_size,
            "invalid 'any' pattern starts size",
        )?;
        let table_bytes_len = bytes::mul(
            start_state_count,
            StateID::SIZE,
            "pattern table bytes length",
        )?;
        bytes::check_slice_len(slice, table_bytes_len, "start ID table")?;
        bytes::check_alignment::<StateID>(slice)?;
        let table_bytes = &slice[..table_bytes_len];
        slice = &slice[table_bytes_len..];
        // SAFETY: Since StateID is always representable as a u32, all we need
        // to do is ensure that we have the proper length and alignment. We've
        // checked both above, so the cast below is safe.
        //
        // N.B. This is the only not-safe code in this function, so we mark
        // it explicitly to call it out, even though it is technically
        // superfluous.
        #[allow(unused_unsafe)]
        let table = unsafe {
            core::slice::from_raw_parts(
                table_bytes.as_ptr() as *const u32,
                start_state_count,
            )
        };
        let st = StartTable { table, stride, patterns };
        Ok((st, slice.as_ptr() as usize - slice_start))
    }
}

impl<T: AsRef<[u32]>> StartTable<T> {
                fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small(
                "starting table ids",
            ));
        }
        dst = &mut dst[..nwrite];

        // write stride
        // Unwrap is OK since the stride is always 4 (currently).
        E::write_u32(u32::try_from(self.stride).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        // write pattern count
        // Unwrap is OK since number of patterns is guaranteed to fit in a u32.
        E::write_u32(u32::try_from(self.patterns).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        // write start IDs
        for &sid in self.table() {
            let n = bytes::write_state_id::<E>(sid, &mut dst);
            dst = &mut dst[n..];
        }
        Ok(nwrite)
    }

            fn write_to_len(&self) -> usize {
        size_of::<u32>()   // stride
        + size_of::<u32>() // # patterns
        + (self.table().len() * StateID::SIZE)
    }

                    fn validate(
        &self,
        tt: &TransitionTable<T>,
    ) -> Result<(), DeserializeError> {
        for &id in self.table() {
            if !tt.is_valid(id) {
                return Err(DeserializeError::generic(
                    "found invalid starting state ID",
                ));
            }
        }
        Ok(())
    }

        fn as_ref(&self) -> StartTable<&'_ [u32]> {
        StartTable {
            table: self.table.as_ref(),
            stride: self.stride,
            patterns: self.patterns,
        }
    }

        #[cfg(feature = "std")]
    fn to_owned(&self) -> StartTable<Vec<u32>> {
        StartTable {
            table: self.table.as_ref().to_vec(),
            stride: self.stride,
            patterns: self.patterns,
        }
    }

                            fn start(&self, index: Start, pattern_id: Option<PatternID>) -> StateID {
        let start_index = index.as_usize();
        let index = match pattern_id {
            None => start_index,
            Some(pid) => {
                let pid = pid.as_usize();
                assert!(pid < self.patterns, "invalid pattern ID {:?}", pid);
                self.stride + (self.stride * pid) + start_index
            }
        };
        self.table()[index]
    }

                    fn iter(&self) -> StartStateIter<'_> {
        StartStateIter { st: self.as_ref(), i: 0 }
    }

        fn table(&self) -> &[StateID] {
        let integers = self.table.as_ref();
        // SAFETY: This is safe because StateID is guaranteed to be
        // representable as a u32.
        unsafe {
            core::slice::from_raw_parts(
                integers.as_ptr() as *const StateID,
                integers.len(),
            )
        }
    }

                fn memory_usage(&self) -> usize {
        self.table().len() * StateID::SIZE
    }
}

#[cfg(feature = "std")]
impl<T: AsMut<[u32]>> StartTable<T> {
                fn set_start(
        &mut self,
        index: Start,
        pattern_id: Option<PatternID>,
        id: StateID,
    ) {
        let start_index = index.as_usize();
        let index = match pattern_id {
            None => start_index,
            Some(pid) => self
                .stride
                .checked_mul(pid.as_usize())
                .unwrap()
                .checked_add(self.stride)
                .unwrap()
                .checked_add(start_index)
                .unwrap(),
        };
        self.table_mut()[index] = id;
    }

        fn table_mut(&mut self) -> &mut [StateID] {
        let integers = self.table.as_mut();
        // SAFETY: This is safe because StateID is guaranteed to be
        // representable as a u32.
        unsafe {
            core::slice::from_raw_parts_mut(
                integers.as_mut_ptr() as *mut StateID,
                integers.len(),
            )
        }
    }
}

pub(crate) struct StartStateIter<'a> {
    st: StartTable<&'a [u32]>,
    i: usize,
}

impl<'a> Iterator for StartStateIter<'a> {
    type Item = (StateID, Start, Option<PatternID>);

    fn next(&mut self) -> Option<(StateID, Start, Option<PatternID>)> {
        let i = self.i;
        let table = self.st.table();
        if i >= table.len() {
            return None;
        }
        self.i += 1;

        // This unwrap is okay since the stride of the starting state table
        // must always match the number of start state types.
        let start_type = Start::from_usize(i % self.st.stride).unwrap();
        let pid = if i < self.st.stride {
            None
        } else {
            Some(
                PatternID::new((i - self.st.stride) / self.st.stride).unwrap(),
            )
        };
        Some((table[i], start_type, pid))
    }
}

#[derive(Clone, Debug)]
struct MatchStates<T> {
                                        slices: T,
                    pattern_ids: T,
        patterns: usize,
}

impl<'a> MatchStates<&'a [u32]> {
    unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(MatchStates<&'a [u32]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr() as usize;

        // Read the total number of match states.
        let (count, nr) =
            bytes::try_read_u32_as_usize(slice, "match state count")?;
        slice = &slice[nr..];

        // Read the slice start/length pairs.
        let pair_count = bytes::mul(2, count, "match state offset pairs")?;
        let slices_bytes_len = bytes::mul(
            pair_count,
            PatternID::SIZE,
            "match state slice offset byte length",
        )?;
        bytes::check_slice_len(slice, slices_bytes_len, "match state slices")?;
        bytes::check_alignment::<PatternID>(slice)?;
        let slices_bytes = &slice[..slices_bytes_len];
        slice = &slice[slices_bytes_len..];
        // SAFETY: Since PatternID is always representable as a u32, all we
        // need to do is ensure that we have the proper length and alignment.
        // We've checked both above, so the cast below is safe.
        //
        // N.B. This is one of the few not-safe snippets in this function, so
        // we mark it explicitly to call it out, even though it is technically
        // superfluous.
        #[allow(unused_unsafe)]
        let slices = unsafe {
            core::slice::from_raw_parts(
                slices_bytes.as_ptr() as *const u32,
                pair_count,
            )
        };

        // Read the total number of unique pattern IDs (which is always 1 more
        // than the maximum pattern ID in this automaton, since pattern IDs are
        // handed out contiguously starting at 0).
        let (patterns, nr) =
            bytes::try_read_u32_as_usize(slice, "pattern count")?;
        slice = &slice[nr..];

        // Now read the pattern ID count. We don't need to store this
        // explicitly, but we need it to know how many pattern IDs to read.
        let (idcount, nr) =
            bytes::try_read_u32_as_usize(slice, "pattern ID count")?;
        slice = &slice[nr..];

        // Read the actual pattern IDs.
        let pattern_ids_len =
            bytes::mul(idcount, PatternID::SIZE, "pattern ID byte length")?;
        bytes::check_slice_len(slice, pattern_ids_len, "match pattern IDs")?;
        bytes::check_alignment::<PatternID>(slice)?;
        let pattern_ids_bytes = &slice[..pattern_ids_len];
        slice = &slice[pattern_ids_len..];
        // SAFETY: Since PatternID is always representable as a u32, all we
        // need to do is ensure that we have the proper length and alignment.
        // We've checked both above, so the cast below is safe.
        //
        // N.B. This is one of the few not-safe snippets in this function, so
        // we mark it explicitly to call it out, even though it is technically
        // superfluous.
        #[allow(unused_unsafe)]
        let pattern_ids = unsafe {
            core::slice::from_raw_parts(
                pattern_ids_bytes.as_ptr() as *const u32,
                idcount,
            )
        };

        let ms = MatchStates { slices, pattern_ids, patterns };
        Ok((ms, slice.as_ptr() as usize - slice_start))
    }
}

#[cfg(feature = "std")]
impl MatchStates<Vec<u32>> {
    fn empty(pattern_count: usize) -> MatchStates<Vec<u32>> {
        assert!(pattern_count <= PatternID::LIMIT);
        MatchStates {
            slices: vec![],
            pattern_ids: vec![],
            patterns: pattern_count,
        }
    }

    fn new(
        matches: &BTreeMap<StateID, Vec<PatternID>>,
        pattern_count: usize,
    ) -> Result<MatchStates<Vec<u32>>, Error> {
        let mut m = MatchStates::empty(pattern_count);
        for (_, pids) in matches.iter() {
            let start = PatternID::new(m.pattern_ids.len())
                .map_err(|_| Error::too_many_match_pattern_ids())?;
            m.slices.push(start.as_u32());
            // This is always correct since the number of patterns in a single
            // match state can never exceed maximum number of allowable
            // patterns. Why? Because a pattern can only appear once in a
            // particular match state, by construction. (And since our pattern
            // ID limit is one less than u32::MAX, we're guaranteed that the
            // length fits in a u32.)
            m.slices.push(u32::try_from(pids.len()).unwrap());
            for &pid in pids {
                m.pattern_ids.push(pid.as_u32());
            }
        }
        m.patterns = pattern_count;
        Ok(m)
    }

    fn new_with_map(
        &self,
        matches: &BTreeMap<StateID, Vec<PatternID>>,
    ) -> Result<MatchStates<Vec<u32>>, Error> {
        MatchStates::new(matches, self.patterns)
    }
}

impl<T: AsRef<[u32]>> MatchStates<T> {
                fn write_to<E: Endian>(
        &self,
        mut dst: &mut [u8],
    ) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("match states"));
        }
        dst = &mut dst[..nwrite];

        // write state ID count
        // Unwrap is OK since number of states is guaranteed to fit in a u32.
        E::write_u32(u32::try_from(self.count()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write slice offset pairs
        for &pid in self.slices() {
            let n = bytes::write_pattern_id::<E>(pid, &mut dst);
            dst = &mut dst[n..];
        }

        // write unique pattern ID count
        // Unwrap is OK since number of patterns is guaranteed to fit in a u32.
        E::write_u32(u32::try_from(self.patterns).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write pattern ID count
        // Unwrap is OK since we check at construction (and deserialization)
        // that the number of patterns is representable as a u32.
        E::write_u32(u32::try_from(self.pattern_ids().len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write pattern IDs
        for &pid in self.pattern_ids() {
            let n = bytes::write_pattern_id::<E>(pid, &mut dst);
            dst = &mut dst[n..];
        }

        Ok(nwrite)
    }

            fn write_to_len(&self) -> usize {
        size_of::<u32>()   // match state count
        + (self.slices().len() * PatternID::SIZE)
        + size_of::<u32>() // unique pattern ID count
        + size_of::<u32>() // pattern ID count
        + (self.pattern_ids().len() * PatternID::SIZE)
    }

            fn validate(&self, dfa: &DFA<T>) -> Result<(), DeserializeError> {
        if self.count() != dfa.special.match_len(dfa.stride()) {
            return Err(DeserializeError::generic(
                "match state count mismatch",
            ));
        }
        for si in 0..self.count() {
            let start = self.slices()[si * 2].as_usize();
            let len = self.slices()[si * 2 + 1].as_usize();
            if start >= self.pattern_ids().len() {
                return Err(DeserializeError::generic(
                    "invalid pattern ID start offset",
                ));
            }
            if start + len > self.pattern_ids().len() {
                return Err(DeserializeError::generic(
                    "invalid pattern ID length",
                ));
            }
            for mi in 0..len {
                let pid = self.pattern_id(si, mi);
                if pid.as_usize() >= self.patterns {
                    return Err(DeserializeError::generic(
                        "invalid pattern ID",
                    ));
                }
            }
        }
        Ok(())
    }

                                            #[cfg(feature = "std")]
    fn to_map(&self, dfa: &DFA<T>) -> BTreeMap<StateID, Vec<PatternID>> {
        let mut map = BTreeMap::new();
        for i in 0..self.count() {
            let mut pids = vec![];
            for j in 0..self.pattern_len(i) {
                pids.push(self.pattern_id(i, j));
            }
            map.insert(self.match_state_id(dfa, i), pids);
        }
        map
    }

        fn as_ref(&self) -> MatchStates<&'_ [u32]> {
        MatchStates {
            slices: self.slices.as_ref(),
            pattern_ids: self.pattern_ids.as_ref(),
            patterns: self.patterns,
        }
    }

        #[cfg(feature = "std")]
    fn to_owned(&self) -> MatchStates<Vec<u32>> {
        MatchStates {
            slices: self.slices.as_ref().to_vec(),
            pattern_ids: self.pattern_ids.as_ref().to_vec(),
            patterns: self.patterns,
        }
    }

                    fn match_state_id(&self, dfa: &DFA<T>, index: usize) -> StateID {
        assert!(dfa.special.matches(), "no match states to index");
        // This is one of the places where we rely on the fact that match
        // states are contiguous in the transition table. Namely, that the
        // first match state ID always corresponds to dfa.special.min_start.
        // From there, since we know the stride, we can compute the ID of any
        // match state given its index.
        let stride2 = u32::try_from(dfa.stride2()).unwrap();
        let offset = index.checked_shl(stride2).unwrap();
        let id = dfa.special.min_match.as_usize().checked_add(offset).unwrap();
        let sid = StateID::new(id).unwrap();
        assert!(dfa.is_match_state(sid));
        sid
    }

                                    fn pattern_id(&self, state_index: usize, match_index: usize) -> PatternID {
        self.pattern_id_slice(state_index)[match_index]
    }

                    fn pattern_len(&self, state_index: usize) -> usize {
        self.slices()[state_index * 2 + 1].as_usize()
    }

                    fn pattern_id_slice(&self, state_index: usize) -> &[PatternID] {
        let start = self.slices()[state_index * 2].as_usize();
        let len = self.pattern_len(state_index);
        &self.pattern_ids()[start..start + len]
    }

        fn slices(&self) -> &[PatternID] {
        let integers = self.slices.as_ref();
        // SAFETY: This is safe because PatternID is guaranteed to be
        // representable as a u32.
        unsafe {
            core::slice::from_raw_parts(
                integers.as_ptr() as *const PatternID,
                integers.len(),
            )
        }
    }

        fn count(&self) -> usize {
        assert_eq!(0, self.slices().len() % 2);
        self.slices().len() / 2
    }

        fn pattern_ids(&self) -> &[PatternID] {
        let integers = self.pattern_ids.as_ref();
        // SAFETY: This is safe because PatternID is guaranteed to be
        // representable as a u32.
        unsafe {
            core::slice::from_raw_parts(
                integers.as_ptr() as *const PatternID,
                integers.len(),
            )
        }
    }

        fn memory_usage(&self) -> usize {
        (self.slices().len() + self.pattern_ids().len()) * PatternID::SIZE
    }
}

pub(crate) struct StateIter<'a, T> {
    tt: &'a TransitionTable<T>,
    it: iter::Enumerate<slice::Chunks<'a, StateID>>,
}

impl<'a, T: AsRef<[u32]>> Iterator for StateIter<'a, T> {
    type Item = State<'a>;

    fn next(&mut self) -> Option<State<'a>> {
        self.it.next().map(|(index, _)| {
            let id = self.tt.from_index(index);
            self.tt.state(id)
        })
    }
}

pub(crate) struct State<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a [StateID],
}

impl<'a> State<'a> {
                                pub(crate) fn transitions(&self) -> StateTransitionIter<'_> {
        StateTransitionIter {
            len: self.transitions.len(),
            it: self.transitions.iter().enumerate(),
        }
    }

                                                    pub(crate) fn sparse_transitions(&self) -> StateSparseTransitionIter<'_> {
        StateSparseTransitionIter { dense: self.transitions(), cur: None }
    }

        pub(crate) fn id(&self) -> StateID {
        self.id
    }

            #[cfg(feature = "std")]
    fn accelerate(&self, classes: &ByteClasses) -> Option<Accel> {
        // We just try to add bytes to our accelerator. Once adding fails
        // (because we've added too many bytes), then give up.
        let mut accel = Accel::new();
        for (class, id) in self.transitions() {
            if id == self.id() {
                continue;
            }
            for unit in classes.elements(class) {
                if let Some(byte) = unit.as_u8() {
                    if !accel.add(byte) {
                        return None;
                    }
                }
            }
        }
        if accel.is_empty() {
            None
        } else {
            Some(accel)
        }
    }
}

impl<'a> fmt::Debug for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, (start, end, id)) in self.sparse_transitions().enumerate() {
            let index = if f.alternate() {
                id.as_usize()
            } else {
                id.as_usize() >> self.stride2
            };
            if i > 0 {
                write!(f, ", ")?;
            }
            if start == end {
                write!(f, "{:?} => {:?}", start, index)?;
            } else {
                write!(f, "{:?}-{:?} => {:?}", start, end, index)?;
            }
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
pub(crate) struct StateMut<'a> {
    id: StateID,
    stride2: usize,
    transitions: &'a mut [StateID],
}

#[cfg(feature = "std")]
impl<'a> StateMut<'a> {
                                pub(crate) fn iter_mut(&mut self) -> StateTransitionIterMut<'_> {
        StateTransitionIterMut {
            len: self.transitions.len(),
            it: self.transitions.iter_mut().enumerate(),
        }
    }
}

#[cfg(feature = "std")]
impl<'a> fmt::Debug for StateMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(
            &State {
                id: self.id,
                stride2: self.stride2,
                transitions: self.transitions,
            },
            f,
        )
    }
}

#[derive(Debug)]
pub(crate) struct StateTransitionIter<'a> {
    len: usize,
    it: iter::Enumerate<slice::Iter<'a, StateID>>,
}

impl<'a> Iterator for StateTransitionIter<'a> {
    type Item = (alphabet::Unit, StateID);

    fn next(&mut self) -> Option<(alphabet::Unit, StateID)> {
        self.it.next().map(|(i, &id)| {
            let unit = if i + 1 == self.len {
                alphabet::Unit::eoi(i)
            } else {
                let b = u8::try_from(i)
                    .expect("raw byte alphabet is never exceeded");
                alphabet::Unit::u8(b)
            };
            (unit, id)
        })
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
pub(crate) struct StateTransitionIterMut<'a> {
    len: usize,
    it: iter::Enumerate<slice::IterMut<'a, StateID>>,
}

#[cfg(feature = "std")]
impl<'a> Iterator for StateTransitionIterMut<'a> {
    type Item = (alphabet::Unit, &'a mut StateID);

    fn next(&mut self) -> Option<(alphabet::Unit, &'a mut StateID)> {
        self.it.next().map(|(i, id)| {
            let unit = if i + 1 == self.len {
                alphabet::Unit::eoi(i)
            } else {
                let b = u8::try_from(i)
                    .expect("raw byte alphabet is never exceeded");
                alphabet::Unit::u8(b)
            };
            (unit, id)
        })
    }
}

#[derive(Debug)]
pub(crate) struct StateSparseTransitionIter<'a> {
    dense: StateTransitionIter<'a>,
    cur: Option<(alphabet::Unit, alphabet::Unit, StateID)>,
}

impl<'a> Iterator for StateSparseTransitionIter<'a> {
    type Item = (alphabet::Unit, alphabet::Unit, StateID);

    fn next(&mut self) -> Option<(alphabet::Unit, alphabet::Unit, StateID)> {
        while let Some((unit, next)) = self.dense.next() {
            let (prev_start, prev_end, prev_next) = match self.cur {
                Some(t) => t,
                None => {
                    self.cur = Some((unit, unit, next));
                    continue;
                }
            };
            if prev_next == next && !unit.is_eoi() {
                self.cur = Some((prev_start, unit, prev_next));
            } else {
                self.cur = Some((unit, unit, next));
                if prev_next != DEAD {
                    return Some((prev_start, prev_end, prev_next));
                }
            }
        }
        if let Some((start, end, next)) = self.cur.take() {
            if next != DEAD {
                return Some((start, end, next));
            }
        }
        None
    }
}

#[derive(Debug)]
pub(crate) struct PatternIDIter<'a>(slice::Iter<'a, PatternID>);

impl<'a> Iterator for PatternIDIter<'a> {
    type Item = PatternID;

    fn next(&mut self) -> Option<PatternID> {
        self.0.next().copied()
    }
}

#[cfg(feature = "std")]
#[derive(Debug)]
struct Remapper {
                                                        map: Vec<StateID>,
}

#[cfg(feature = "std")]
impl Remapper {
    fn from_dfa(dfa: &OwnedDFA) -> Remapper {
        Remapper {
            map: (0..dfa.state_count()).map(|i| dfa.from_index(i)).collect(),
        }
    }

    fn swap(&mut self, dfa: &mut OwnedDFA, id1: StateID, id2: StateID) {
        dfa.swap_states(id1, id2);
        self.map.swap(dfa.to_index(id1), dfa.to_index(id2));
    }

    fn remap(mut self, dfa: &mut OwnedDFA) {
        // Update the map to account for states that have been swapped
        // multiple times. For example, if (A, C) and (C, G) are swapped, then
        // transitions previously pointing to A should now point to G. But if
        // we don't update our map, they will erroneously be set to C. All we
        // do is follow the swaps in our map until we see our original state
        // ID.
        let oldmap = self.map.clone();
        for i in 0..dfa.state_count() {
            let cur_id = dfa.from_index(i);
            let mut new = oldmap[i];
            if cur_id == new {
                continue;
            }
            loop {
                let id = oldmap[dfa.to_index(new)];
                if cur_id == id {
                    self.map[i] = new;
                    break;
                }
                new = id;
            }
        }

        // To work around the borrow checker for converting state IDs to
        // indices. We cannot borrow self while mutably iterating over a
        // state's transitions. Otherwise, we'd just use dfa.to_index(..).
        let stride2 = dfa.stride2();
        let to_index = |id: StateID| -> usize { id.as_usize() >> stride2 };

        // Now that we've finished shuffling, we need to remap all of our
        // transitions. We don't need to handle re-mapping accelerated states
        // since `accels` is only populated after shuffling.
        for &id in self.map.iter() {
            for (_, next_id) in dfa.state_mut(id).iter_mut() {
                *next_id = self.map[to_index(*next_id)];
            }
        }
        for start_id in dfa.st.table_mut().iter_mut() {
            *start_id = self.map[to_index(*start_id)];
        }
    }
}
