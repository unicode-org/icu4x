#[cfg(feature = "std")]
use core::iter;
use core::{
    convert::{TryFrom, TryInto},
    fmt,
    mem::size_of,
};

#[cfg(feature = "std")]
use alloc::{collections::BTreeSet, vec, vec::Vec};

#[cfg(feature = "std")]
use crate::regex_automata::dfa::{dense, error::Error};
use crate::regex_automata::{
    dfa::{
        automaton::{fmt_state_indicator, Automaton},
        special::Special,
        DEAD,
    },
    util::{
        alphabet::ByteClasses,
        bytes::{self, DeserializeError, Endian, SerializeError},
        id::{PatternID, StateID},
        start::Start,
        DebugByte,
    },
};

const LABEL: &str = "rust-regex-automata-dfa-sparse";
const VERSION: u32 = 2;

#[derive(Clone)]
pub struct DFA<T> {
    // When compared to a dense DFA, a sparse DFA *looks* a lot simpler
    // representation-wise. In reality, it is perhaps more complicated. Namely,
    // in a dense DFA, all information needs to be very cheaply accessible
    // using only state IDs. In a sparse DFA however, each state uses a
    // variable amount of space because each state encodes more information
    // than just its transitions. Each state also includes an accelerator if
    // one exists, along with the matching pattern IDs if the state is a match
    // state.
    //
    // That is, a lot of the complexity is pushed down into how each state
    // itself is represented.
    trans: Transitions<T>,
    starts: StartTable<T>,
    special: Special,
}

#[cfg(feature = "std")]
impl DFA<Vec<u8>> {
    pub fn new(pattern: &str) -> Result<DFA<Vec<u8>>, Error> {
        dense::Builder::new()
            .build(pattern)
            .and_then(|dense| dense.to_sparse())
    }

    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA<Vec<u8>>, Error> {
        dense::Builder::new()
            .build_many(patterns)
            .and_then(|dense| dense.to_sparse())
    }
}

#[cfg(feature = "std")]
impl DFA<Vec<u8>> {
    pub fn always_match() -> Result<DFA<Vec<u8>>, Error> {
        dense::DFA::always_match()?.to_sparse()
    }

    pub fn never_match() -> Result<DFA<Vec<u8>>, Error> {
        dense::DFA::never_match()?.to_sparse()
    }

    pub(crate) fn from_dense<T: AsRef<[u32]>>(dfa: &dense::DFA<T>) -> Result<DFA<Vec<u8>>, Error> {
        // In order to build the transition table, we need to be able to write
        // state identifiers for each of the "next" transitions in each state.
        // Our state identifiers correspond to the byte offset in the
        // transition table at which the state is encoded. Therefore, we do not
        // actually know what the state identifiers are until we've allocated
        // exactly as much space as we need for each state. Thus, construction
        // of the transition table happens in two passes.
        //
        // In the first pass, we fill out the shell of each state, which
        // includes the transition count, the input byte ranges and zero-filled
        // space for the transitions and accelerators, if present. In this
        // first pass, we also build up a map from the state identifier index
        // of the dense DFA to the state identifier in this sparse DFA.
        //
        // In the second pass, we fill in the transitions based on the map
        // built in the first pass.

        // The capacity given here reflects a minimum. (Well, the true minimum
        // is likely even bigger, but hopefully this saves a few reallocs.)
        let mut sparse = Vec::with_capacity(StateID::SIZE * dfa.state_count());
        // This maps state indices from the dense DFA to StateIDs in the sparse
        // DFA. We build out this map on the first pass, and then use it in the
        // second pass to back-fill our transitions.
        let mut remap: Vec<StateID> = vec![DEAD; dfa.state_count()];
        for state in dfa.states() {
            let pos = sparse.len();

            remap[dfa.to_index(state.id())] =
                StateID::new(pos).map_err(|_| Error::too_many_states())?;
            // zero-filled space for the transition count
            sparse.push(0);
            sparse.push(0);

            let mut transition_count = 0;
            for (unit1, unit2, _) in state.sparse_transitions() {
                match (unit1.as_u8(), unit2.as_u8()) {
                    (Some(b1), Some(b2)) => {
                        transition_count += 1;
                        sparse.push(b1);
                        sparse.push(b2);
                    }
                    (None, None) => {}
                    (Some(_), None) | (None, Some(_)) => {
                        // can never occur because sparse_transitions never
                        // groups EOI with any other transition.
                        unreachable!()
                    }
                }
            }
            // Add dummy EOI transition. This is never actually read while
            // searching, but having space equivalent to the total number
            // of transitions is convenient. Otherwise, we'd need to track
            // a different number of transitions for the byte ranges as for
            // the 'next' states.
            //
            // N.B. The loop above is not guaranteed to yield the EOI
            // transition, since it may point to a DEAD state. By putting
            // it here, we always write the EOI transition, and thus
            // guarantee that our transition count is >0. Why do we always
            // need the EOI transition? Because in order to implement
            // Automaton::next_eoi_state, this lets us just ask for the last
            // transition. There are probably other/better ways to do this.
            transition_count += 1;
            sparse.push(0);
            sparse.push(0);

            // Check some assumptions about transition count.
            assert_ne!(transition_count, 0, "transition count should be non-zero",);
            assert!(
                transition_count <= 257,
                "expected transition count {} to be <= 257",
                transition_count,
            );

            // Fill in the transition count.
            // Since transition count is always <= 257, we use the most
            // significant bit to indicate whether this is a match state or
            // not.
            let ntrans = if dfa.is_match_state(state.id()) {
                transition_count | (1 << 15)
            } else {
                transition_count
            };
            bytes::NE::write_u16(ntrans, &mut sparse[pos..]);

            // zero-fill the actual transitions.
            // Unwraps are OK since transition_count <= 257 and our minimum
            // support usize size is 16-bits.
            let zeros = usize::try_from(transition_count)
                .unwrap()
                .checked_mul(StateID::SIZE)
                .unwrap();
            sparse.extend(iter::repeat(0).take(zeros));

            // If this is a match state, write the pattern IDs matched by this
            // state.
            if dfa.is_match_state(state.id()) {
                let plen = dfa.match_pattern_len(state.id());
                // Write the actual pattern IDs with a u32 length prefix.
                // First, zero-fill space.
                let mut pos = sparse.len();
                // Unwraps are OK since it's guaranteed that plen <=
                // PatternID::LIMIT, which is in turn guaranteed to fit into a
                // u32.
                let zeros = size_of::<u32>()
                    .checked_mul(plen)
                    .unwrap()
                    .checked_add(size_of::<u32>())
                    .unwrap();
                sparse.extend(iter::repeat(0).take(zeros));

                // Now write the length prefix.
                bytes::NE::write_u32(
                    // Will never fail since u32::MAX is invalid pattern ID.
                    // Thus, the number of pattern IDs is representable by a
                    // u32.
                    plen.try_into().expect("pattern ID count fits in u32"),
                    &mut sparse[pos..],
                );
                pos += size_of::<u32>();

                // Now write the pattern IDs.
                for &pid in dfa.pattern_id_slice(state.id()) {
                    pos += bytes::write_pattern_id::<bytes::NE>(pid, &mut sparse[pos..]);
                }
            }

            // And now add the accelerator, if one exists. An accelerator is
            // at most 4 bytes and at least 1 byte. The first byte is the
            // length, N. N bytes follow the length. The set of bytes that
            // follow correspond (exhaustively) to the bytes that must be seen
            // to leave this state.
            let accel = dfa.accelerator(state.id());
            sparse.push(accel.len().try_into().unwrap());
            sparse.extend_from_slice(accel);
        }

        let mut new = DFA {
            trans: Transitions {
                sparse,
                classes: dfa.byte_classes().clone(),
                count: dfa.state_count(),
                patterns: dfa.pattern_count(),
            },
            starts: StartTable::from_dense_dfa(dfa, &remap)?,
            special: dfa.special().remap(|id| remap[dfa.to_index(id)]),
        };
        // And here's our second pass. Iterate over all of the dense states
        // again, and update the transitions in each of the states in the
        // sparse DFA.
        for old_state in dfa.states() {
            let new_id = remap[dfa.to_index(old_state.id())];
            let mut new_state = new.trans.state_mut(new_id);
            let sparse = old_state.sparse_transitions();
            for (i, (_, _, next)) in sparse.enumerate() {
                let next = remap[dfa.to_index(next)];
                new_state.set_next_at(i, next);
            }
        }
        trace!(
            "created sparse DFA, memory usage: {} (dense memory usage: {})",
            new.memory_usage(),
            dfa.memory_usage(),
        );
        Ok(new)
    }
}

impl<T: AsRef<[u8]>> DFA<T> {
    pub fn as_ref<'a>(&'a self) -> DFA<&'a [u8]> {
        DFA {
            trans: self.trans.as_ref(),
            starts: self.starts.as_ref(),
            special: self.special,
        }
    }

    #[cfg(feature = "std")]
    pub fn to_owned(&self) -> DFA<Vec<u8>> {
        DFA {
            trans: self.trans.to_owned(),
            starts: self.starts.to_owned(),
            special: self.special,
        }
    }

    pub fn memory_usage(&self) -> usize {
        self.trans.memory_usage() + self.starts.memory_usage()
    }

    pub fn has_starts_for_each_pattern(&self) -> bool {
        self.starts.patterns > 0
    }
}

impl<T: AsRef<[u8]>> DFA<T> {
    #[cfg(feature = "std")]
    pub fn to_bytes_little_endian(&self) -> Vec<u8> {
        self.to_bytes::<bytes::LE>()
    }

    #[cfg(feature = "std")]
    pub fn to_bytes_big_endian(&self) -> Vec<u8> {
        self.to_bytes::<bytes::BE>()
    }

    #[cfg(feature = "std")]
    pub fn to_bytes_native_endian(&self) -> Vec<u8> {
        self.to_bytes::<bytes::NE>()
    }

    #[cfg(feature = "std")]
    fn to_bytes<E: Endian>(&self) -> Vec<u8> {
        let mut buf = vec![0; self.write_to_len()];
        // This should always succeed since the only possible serialization
        // error is providing a buffer that's too small, but we've ensured that
        // `buf` is big enough here.
        self.write_to::<E>(&mut buf).unwrap();
        buf
    }

    pub fn write_to_little_endian(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {
        self.write_to::<bytes::LE>(dst)
    }

    pub fn write_to_big_endian(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {
        self.write_to::<bytes::BE>(dst)
    }

    pub fn write_to_native_endian(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {
        self.write_to::<bytes::NE>(dst)
    }

    fn write_to<E: Endian>(&self, dst: &mut [u8]) -> Result<usize, SerializeError> {
        let mut nw = 0;
        nw += bytes::write_label(LABEL, &mut dst[nw..])?;
        nw += bytes::write_endianness_check::<E>(&mut dst[nw..])?;
        nw += bytes::write_version::<E>(VERSION, &mut dst[nw..])?;
        nw += {
            // Currently unused, intended for future flexibility
            E::write_u32(0, &mut dst[nw..]);
            size_of::<u32>()
        };
        nw += self.trans.write_to::<E>(&mut dst[nw..])?;
        nw += self.starts.write_to::<E>(&mut dst[nw..])?;
        nw += self.special.write_to::<E>(&mut dst[nw..])?;
        Ok(nw)
    }

    pub fn write_to_len(&self) -> usize {
        bytes::write_label_len(LABEL)
        + bytes::write_endianness_check_len()
        + bytes::write_version_len()
        + size_of::<u32>() // unused, intended for future flexibility
        + self.trans.write_to_len()
        + self.starts.write_to_len()
        + self.special.write_to_len()
    }
}

impl<'a> DFA<&'a [u8]> {
    pub fn from_bytes(slice: &'a [u8]) -> Result<(DFA<&'a [u8]>, usize), DeserializeError> {
        // SAFETY: This is safe because we validate both the sparse transitions
        // (by trying to decode every state) and start state ID list below. If
        // either validation fails, then we return an error.
        let (dfa, nread) = unsafe { DFA::from_bytes_unchecked(slice)? };
        dfa.trans.validate()?;
        dfa.starts.validate(&dfa.trans)?;
        // N.B. dfa.special doesn't have a way to do unchecked deserialization,
        // so it has already been validated.
        Ok((dfa, nread))
    }

    pub unsafe fn from_bytes_unchecked(
        slice: &'a [u8],
    ) -> Result<(DFA<&'a [u8]>, usize), DeserializeError> {
        let mut nr = 0;

        nr += bytes::read_label(&slice[nr..], LABEL)?;
        nr += bytes::read_endianness_check(&slice[nr..])?;
        nr += bytes::read_version(&slice[nr..], VERSION)?;

        let _unused = bytes::try_read_u32(&slice[nr..], "unused space")?;
        nr += size_of::<u32>();

        let (trans, nread) = Transitions::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (starts, nread) = StartTable::from_bytes_unchecked(&slice[nr..])?;
        nr += nread;

        let (special, nread) = Special::from_bytes(&slice[nr..])?;
        nr += nread;
        if special.max.as_usize() >= trans.sparse().len() {
            return Err(DeserializeError::generic(
                "max should not be greater than or equal to sparse bytes",
            ));
        }

        Ok((
            DFA {
                trans,
                starts,
                special,
            },
            nr,
        ))
    }
}

impl<T: AsRef<[u8]>> fmt::Debug for DFA<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "sparse::DFA(")?;
        for state in self.trans.states() {
            fmt_state_indicator(f, self, state.id())?;
            writeln!(f, "{:06?}: {:?}", state.id(), state)?;
        }
        writeln!(f, "")?;
        for (i, (start_id, sty, pid)) in self.starts.iter().enumerate() {
            if i % self.starts.stride == 0 {
                match pid {
                    None => writeln!(f, "START-GROUP(ALL)")?,
                    Some(pid) => writeln!(f, "START_GROUP(pattern: {:?})", pid)?,
                }
            }
            writeln!(f, "  {:?} => {:06?}", sty, start_id.as_usize())?;
        }
        writeln!(f, "state count: {:?}", self.trans.count)?;
        writeln!(f, ")")?;
        Ok(())
    }
}

unsafe impl<T: AsRef<[u8]>> Automaton for DFA<T> {
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

    // This is marked as inline to help dramatically boost sparse searching,
    // which decodes each state it enters to follow the next transition.
    #[inline(always)]
    fn next_state(&self, current: StateID, input: u8) -> StateID {
        let input = self.trans.classes.get(input);
        self.trans.state(current).next(input)
    }

    #[inline]
    unsafe fn next_state_unchecked(&self, current: StateID, input: u8) -> StateID {
        self.next_state(current, input)
    }

    #[inline]
    fn next_eoi_state(&self, current: StateID) -> StateID {
        self.trans.state(current).next_eoi()
    }

    #[inline]
    fn pattern_count(&self) -> usize {
        self.trans.patterns
    }

    #[inline]
    fn match_count(&self, id: StateID) -> usize {
        self.trans.state(id).pattern_count()
    }

    #[inline]
    fn match_pattern(&self, id: StateID, match_index: usize) -> PatternID {
        // This is an optimization for the very common case of a DFA with a
        // single pattern. This conditional avoids a somewhat more costly path
        // that finds the pattern ID from the state machine, which requires
        // a bit of slicing/pointer-chasing. This optimization tends to only
        // matter when matches are frequent.
        if self.trans.patterns == 1 {
            return PatternID::ZERO;
        }
        self.trans.state(id).pattern_id(match_index)
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
        self.starts.start(index, pattern_id)
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
        self.starts.start(index, pattern_id)
    }

    #[inline]
    fn accelerator(&self, id: StateID) -> &[u8] {
        self.trans.state(id).accelerator()
    }
}

#[derive(Clone)]
struct Transitions<T> {
    sparse: T,
    classes: ByteClasses,
    count: usize,
    patterns: usize,
}

impl<'a> Transitions<&'a [u8]> {
    unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(Transitions<&'a [u8]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr() as usize;

        let (state_count, nr) = bytes::try_read_u32_as_usize(&slice, "state count")?;
        slice = &slice[nr..];

        let (pattern_count, nr) = bytes::try_read_u32_as_usize(&slice, "pattern count")?;
        slice = &slice[nr..];

        let (classes, nr) = ByteClasses::from_bytes(&slice)?;
        slice = &slice[nr..];

        let (len, nr) = bytes::try_read_u32_as_usize(&slice, "sparse transitions length")?;
        slice = &slice[nr..];

        bytes::check_slice_len(slice, len, "sparse states byte length")?;
        let sparse = &slice[..len];
        slice = &slice[len..];

        let trans = Transitions {
            sparse,
            classes,
            count: state_count,
            patterns: pattern_count,
        };
        Ok((trans, slice.as_ptr() as usize - slice_start))
    }
}

impl<T: AsRef<[u8]>> Transitions<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small("sparse transition table"));
        }
        dst = &mut dst[..nwrite];

        // write state count
        E::write_u32(u32::try_from(self.count).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write pattern count
        E::write_u32(u32::try_from(self.patterns).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write byte class map
        let n = self.classes.write_to(dst)?;
        dst = &mut dst[n..];

        // write number of bytes in sparse transitions
        E::write_u32(u32::try_from(self.sparse().len()).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];

        // write actual transitions
        dst.copy_from_slice(self.sparse());
        Ok(nwrite)
    }

    fn write_to_len(&self) -> usize {
        size_of::<u32>()   // state count
        + size_of::<u32>() // pattern count
        + self.classes.write_to_len()
        + size_of::<u32>() // sparse transitions length
        + self.sparse().len()
    }

    fn validate(&self) -> Result<(), DeserializeError> {
        // In order to validate everything, we not only need to make sure we
        // can decode every state, but that every transition in every state
        // points to a valid state. There are many duplicative transitions, so
        // we record state IDs that we've verified so that we don't redo the
        // decoding work.
        //
        // Except, when in no_std mode, we don't have dynamic memory allocation
        // available to us, so we skip this optimization. It's not clear
        // whether doing something more clever is worth it just yet. If you're
        // profiling this code and need it to run faster, please file an issue.
        //
        // ---AG
        struct Seen {
            #[cfg(feature = "std")]
            set: BTreeSet<StateID>,
            #[cfg(not(feature = "std"))]
            set: core::marker::PhantomData<StateID>,
        }

        #[cfg(feature = "std")]
        impl Seen {
            fn new() -> Seen {
                Seen {
                    set: BTreeSet::new(),
                }
            }
            fn insert(&mut self, id: StateID) {
                self.set.insert(id);
            }
            fn contains(&self, id: &StateID) -> bool {
                self.set.contains(id)
            }
        }

        #[cfg(not(feature = "std"))]
        impl Seen {
            fn new() -> Seen {
                Seen {
                    set: core::marker::PhantomData,
                }
            }
            fn insert(&mut self, _id: StateID) {}
            fn contains(&self, _id: &StateID) -> bool {
                false
            }
        }

        let mut verified: Seen = Seen::new();
        // We need to make sure that we decode the correct number of states.
        // Otherwise, an empty set of transitions would validate even if the
        // recorded state count is non-empty.
        let mut count = 0;
        // We can't use the self.states() iterator because it assumes the state
        // encodings are valid. It could panic if they aren't.
        let mut id = DEAD;
        while id.as_usize() < self.sparse().len() {
            let state = self.try_state(id)?;
            verified.insert(id);
            // The next ID should be the offset immediately following `state`.
            id = StateID::new(bytes::add(
                id.as_usize(),
                state.bytes_len(),
                "next state ID offset",
            )?)
            .map_err(|err| DeserializeError::state_id_error(err, "next state ID offset"))?;
            count += 1;

            // Now check that all transitions in this state are correct.
            for i in 0..state.ntrans {
                let to = state.next_at(i);
                if verified.contains(&to) {
                    continue;
                }
                let _ = self.try_state(to)?;
                verified.insert(id);
            }
        }
        if count != self.count {
            return Err(DeserializeError::generic("mismatching sparse state count"));
        }
        Ok(())
    }

    fn as_ref(&self) -> Transitions<&'_ [u8]> {
        Transitions {
            sparse: self.sparse(),
            classes: self.classes.clone(),
            count: self.count,
            patterns: self.patterns,
        }
    }

    #[cfg(feature = "std")]
    fn to_owned(&self) -> Transitions<Vec<u8>> {
        Transitions {
            sparse: self.sparse().to_vec(),
            classes: self.classes.clone(),
            count: self.count,
            patterns: self.patterns,
        }
    }

    #[inline(always)]
    fn state(&self, id: StateID) -> State<'_> {
        let mut state = &self.sparse()[id.as_usize()..];
        let mut ntrans = bytes::read_u16(&state) as usize;
        let is_match = (1 << 15) & ntrans != 0;
        ntrans &= !(1 << 15);
        state = &state[2..];

        let (input_ranges, state) = state.split_at(ntrans * 2);
        let (next, state) = state.split_at(ntrans * StateID::SIZE);
        let (pattern_ids, state) = if is_match {
            let npats = bytes::read_u32(&state) as usize;
            state[4..].split_at(npats * 4)
        } else {
            (&[][..], state)
        };

        let accel_len = state[0] as usize;
        let accel = &state[1..accel_len + 1];
        State {
            id,
            is_match,
            ntrans,
            input_ranges,
            next,
            pattern_ids,
            accel,
        }
    }

    fn try_state(&self, id: StateID) -> Result<State<'_>, DeserializeError> {
        if id.as_usize() > self.sparse().len() {
            return Err(DeserializeError::generic("invalid sparse state ID"));
        }
        let mut state = &self.sparse()[id.as_usize()..];
        // Encoding format starts with a u16 that stores the total number of
        // transitions in this state.
        let (mut ntrans, _) = bytes::try_read_u16_as_usize(state, "state transition count")?;
        let is_match = ((1 << 15) & ntrans) != 0;
        ntrans &= !(1 << 15);
        state = &state[2..];
        if ntrans > 257 || ntrans == 0 {
            return Err(DeserializeError::generic("invalid transition count"));
        }

        // Each transition has two pieces: an inclusive range of bytes on which
        // it is defined, and the state ID that those bytes transition to. The
        // pairs come first, followed by a corresponding sequence of state IDs.
        let input_ranges_len = ntrans.checked_mul(2).unwrap();
        bytes::check_slice_len(state, input_ranges_len, "sparse byte pairs")?;
        let (input_ranges, state) = state.split_at(input_ranges_len);
        // Every range should be of the form A-B, where A<=B.
        for pair in input_ranges.chunks(2) {
            let (start, end) = (pair[0], pair[1]);
            if start > end {
                return Err(DeserializeError::generic("invalid input range"));
            }
        }

        // And now extract the corresponding sequence of state IDs. We leave
        // this sequence as a &[u8] instead of a &[S] because sparse DFAs do
        // not have any alignment requirements.
        let next_len = ntrans
            .checked_mul(self.id_len())
            .expect("state size * #trans should always fit in a usize");
        bytes::check_slice_len(state, next_len, "sparse trans state IDs")?;
        let (next, state) = state.split_at(next_len);
        // We can at least verify that every state ID is in bounds.
        for idbytes in next.chunks(self.id_len()) {
            let (id, _) = bytes::read_state_id(idbytes, "sparse state ID in try_state")?;
            bytes::check_slice_len(self.sparse(), id.as_usize(), "invalid sparse state ID")?;
        }

        // If this is a match state, then read the pattern IDs for this state.
        // Pattern IDs is a u32-length prefixed sequence of native endian
        // encoded 32-bit integers.
        let (pattern_ids, state) = if is_match {
            let (npats, nr) = bytes::try_read_u32_as_usize(state, "pattern ID count")?;
            let state = &state[nr..];

            let pattern_ids_len = bytes::mul(npats, 4, "sparse pattern ID byte length")?;
            bytes::check_slice_len(state, pattern_ids_len, "sparse pattern IDs")?;
            let (pattern_ids, state) = state.split_at(pattern_ids_len);
            for patbytes in pattern_ids.chunks(PatternID::SIZE) {
                bytes::read_pattern_id(patbytes, "sparse pattern ID in try_state")?;
            }
            (pattern_ids, state)
        } else {
            (&[][..], state)
        };

        // Now read this state's accelerator info. The first byte is the length
        // of the accelerator, which is typically 0 (for no acceleration) but
        // is no bigger than 3. The length indicates the number of bytes that
        // follow, where each byte corresponds to a transition out of this
        // state.
        if state.is_empty() {
            return Err(DeserializeError::generic("no accelerator length"));
        }
        let (accel_len, state) = (state[0] as usize, &state[1..]);

        if accel_len > 3 {
            return Err(DeserializeError::generic(
                "sparse invalid accelerator length",
            ));
        }
        bytes::check_slice_len(state, accel_len, "sparse corrupt accelerator length")?;
        let (accel, _) = (&state[..accel_len], &state[accel_len..]);

        Ok(State {
            id,
            is_match,
            ntrans,
            input_ranges,
            next,
            pattern_ids,
            accel,
        })
    }

    fn states(&self) -> StateIter<'_, T> {
        StateIter {
            trans: self,
            id: DEAD.as_usize(),
        }
    }

    fn sparse(&self) -> &[u8] {
        self.sparse.as_ref()
    }

    fn id_len(&self) -> usize {
        StateID::SIZE
    }

    fn memory_usage(&self) -> usize {
        self.sparse().len()
    }
}

#[cfg(feature = "std")]
impl<T: AsMut<[u8]>> Transitions<T> {
    fn state_mut(&mut self, id: StateID) -> StateMut<'_> {
        let mut state = &mut self.sparse_mut()[id.as_usize()..];
        let mut ntrans = bytes::read_u16(&state) as usize;
        let is_match = (1 << 15) & ntrans != 0;
        ntrans &= !(1 << 15);
        state = &mut state[2..];

        let (input_ranges, state) = state.split_at_mut(ntrans * 2);
        let (next, state) = state.split_at_mut(ntrans * StateID::SIZE);
        let (pattern_ids, state) = if is_match {
            let npats = bytes::read_u32(&state) as usize;
            state[4..].split_at_mut(npats * 4)
        } else {
            (&mut [][..], state)
        };

        let accel_len = state[0] as usize;
        let accel = &mut state[1..accel_len + 1];
        StateMut {
            id,
            is_match,
            ntrans,
            input_ranges,
            next,
            pattern_ids,
            accel,
        }
    }

    fn sparse_mut(&mut self) -> &mut [u8] {
        self.sparse.as_mut()
    }
}

#[derive(Clone)]
struct StartTable<T> {
    table: T,
    stride: usize,
    patterns: usize,
}

#[cfg(feature = "std")]
impl StartTable<Vec<u8>> {
    fn new(patterns: usize) -> StartTable<Vec<u8>> {
        let stride = Start::count();
        // This is OK since the only way we're here is if a dense DFA could be
        // constructed successfully, which uses the same space.
        let len = stride
            .checked_mul(patterns)
            .unwrap()
            .checked_add(stride)
            .unwrap()
            .checked_mul(StateID::SIZE)
            .unwrap();
        StartTable {
            table: vec![0; len],
            stride,
            patterns,
        }
    }

    fn from_dense_dfa<T: AsRef<[u32]>>(
        dfa: &dense::DFA<T>,
        remap: &[StateID],
    ) -> Result<StartTable<Vec<u8>>, Error> {
        // Unless the DFA has start states compiled for each pattern, then
        // as far as the starting state table is concerned, there are zero
        // patterns to account for. It will instead only store starting states
        // for the entire DFA.
        let start_pattern_count = if dfa.has_starts_for_each_pattern() {
            dfa.pattern_count()
        } else {
            0
        };
        let mut sl = StartTable::new(start_pattern_count);
        for (old_start_id, sty, pid) in dfa.starts() {
            let new_start_id = remap[dfa.to_index(old_start_id)];
            sl.set_start(sty, pid, new_start_id);
        }
        Ok(sl)
    }
}

impl<'a> StartTable<&'a [u8]> {
    unsafe fn from_bytes_unchecked(
        mut slice: &'a [u8],
    ) -> Result<(StartTable<&'a [u8]>, usize), DeserializeError> {
        let slice_start = slice.as_ptr() as usize;

        let (stride, nr) = bytes::try_read_u32_as_usize(slice, "sparse start table stride")?;
        slice = &slice[nr..];

        let (patterns, nr) = bytes::try_read_u32_as_usize(slice, "sparse start table patterns")?;
        slice = &slice[nr..];

        if stride != Start::count() {
            return Err(DeserializeError::generic(
                "invalid sparse starting table stride",
            ));
        }
        if patterns > PatternID::LIMIT {
            return Err(DeserializeError::generic(
                "sparse invalid number of patterns",
            ));
        }
        let pattern_table_size = bytes::mul(stride, patterns, "sparse invalid pattern count")?;
        // Our start states always start with a single stride of start states
        // for the entire automaton which permit it to match any pattern. What
        // follows it are an optional set of start states for each pattern.
        let start_state_count = bytes::add(
            stride,
            pattern_table_size,
            "sparse invalid 'any' pattern starts size",
        )?;
        let table_bytes_len = bytes::mul(
            start_state_count,
            StateID::SIZE,
            "sparse pattern table bytes length",
        )?;
        bytes::check_slice_len(slice, table_bytes_len, "sparse start ID table")?;
        let table_bytes = &slice[..table_bytes_len];
        slice = &slice[table_bytes_len..];

        let sl = StartTable {
            table: table_bytes,
            stride,
            patterns,
        };
        Ok((sl, slice.as_ptr() as usize - slice_start))
    }
}

impl<T: AsRef<[u8]>> StartTable<T> {
    fn write_to<E: Endian>(&self, mut dst: &mut [u8]) -> Result<usize, SerializeError> {
        let nwrite = self.write_to_len();
        if dst.len() < nwrite {
            return Err(SerializeError::buffer_too_small(
                "sparse starting table ids",
            ));
        }
        dst = &mut dst[..nwrite];

        // write stride
        E::write_u32(u32::try_from(self.stride).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        // write pattern count
        E::write_u32(u32::try_from(self.patterns).unwrap(), dst);
        dst = &mut dst[size_of::<u32>()..];
        // write start IDs
        dst.copy_from_slice(self.table());
        Ok(nwrite)
    }

    fn write_to_len(&self) -> usize {
        size_of::<u32>() // stride
        + size_of::<u32>() // # patterns
        + self.table().len()
    }

    fn validate(&self, trans: &Transitions<T>) -> Result<(), DeserializeError> {
        for (id, _, _) in self.iter() {
            let _ = trans.try_state(id)?;
        }
        Ok(())
    }

    fn as_ref(&self) -> StartTable<&'_ [u8]> {
        StartTable {
            table: self.table(),
            stride: self.stride,
            patterns: self.patterns,
        }
    }

    #[cfg(feature = "std")]
    fn to_owned(&self) -> StartTable<Vec<u8>> {
        StartTable {
            table: self.table().to_vec(),
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
                self.stride
                    .checked_mul(pid)
                    .unwrap()
                    .checked_add(self.stride)
                    .unwrap()
                    .checked_add(start_index)
                    .unwrap()
            }
        };
        let start = index * StateID::SIZE;
        // This OK since we're allowed to assume that the start table contains
        // valid StateIDs.
        bytes::read_state_id_unchecked(&self.table()[start..]).0
    }

    fn iter(&self) -> StartStateIter<'_, T> {
        StartStateIter { st: self, i: 0 }
    }

    fn len(&self) -> usize {
        self.table().len() / StateID::SIZE
    }

    fn table(&self) -> &[u8] {
        self.table.as_ref()
    }

    fn memory_usage(&self) -> usize {
        self.table().len()
    }
}

#[cfg(feature = "std")]
impl<T: AsMut<[u8]>> StartTable<T> {
    fn set_start(&mut self, index: Start, pattern_id: Option<PatternID>, id: StateID) {
        let start_index = index.as_usize();
        let index = match pattern_id {
            None => start_index,
            Some(pid) => {
                let pid = pid.as_usize();
                assert!(pid < self.patterns, "invalid pattern ID {:?}", pid);
                self.stride
                    .checked_mul(pid)
                    .unwrap()
                    .checked_add(self.stride)
                    .unwrap()
                    .checked_add(start_index)
                    .unwrap()
            }
        };
        let start = index * StateID::SIZE;
        let end = start + StateID::SIZE;
        bytes::write_state_id::<bytes::NE>(id, &mut self.table.as_mut()[start..end]);
    }
}

struct StartStateIter<'a, T> {
    st: &'a StartTable<T>,
    i: usize,
}

impl<'a, T: AsRef<[u8]>> Iterator for StartStateIter<'a, T> {
    type Item = (StateID, Start, Option<PatternID>);

    fn next(&mut self) -> Option<(StateID, Start, Option<PatternID>)> {
        let i = self.i;
        if i >= self.st.len() {
            return None;
        }
        self.i += 1;

        // This unwrap is okay since the stride of any DFA must always match
        // the number of start state types.
        let start_type = Start::from_usize(i % self.st.stride).unwrap();
        let pid = if i < self.st.stride {
            // This means we don't have start states for each pattern.
            None
        } else {
            // These unwraps are OK since we may assume our table and stride
            // is correct.
            let pid = i
                .checked_sub(self.st.stride)
                .unwrap()
                .checked_div(self.st.stride)
                .unwrap();
            Some(PatternID::new(pid).unwrap())
        };
        let start = i * StateID::SIZE;
        let end = start + StateID::SIZE;
        let bytes = self.st.table()[start..end].try_into().unwrap();
        // This is OK since we're allowed to assume that any IDs in this start
        // table are correct and valid for this DFA.
        let id = StateID::from_ne_bytes_unchecked(bytes);
        Some((id, start_type, pid))
    }
}

impl<'a, T> fmt::Debug for StartStateIter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StartStateIter")
            .field("i", &self.i)
            .finish()
    }
}

struct StateIter<'a, T> {
    trans: &'a Transitions<T>,
    id: usize,
}

impl<'a, T: AsRef<[u8]>> Iterator for StateIter<'a, T> {
    type Item = State<'a>;

    fn next(&mut self) -> Option<State<'a>> {
        if self.id >= self.trans.sparse().len() {
            return None;
        }
        let state = self.trans.state(StateID::new_unchecked(self.id));
        self.id = self.id + state.bytes_len();
        Some(state)
    }
}

impl<'a, T> fmt::Debug for StateIter<'a, T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("StateIter").field("id", &self.id).finish()
    }
}

#[derive(Clone)]
struct State<'a> {
    id: StateID,
    is_match: bool,
    ntrans: usize,
    input_ranges: &'a [u8],
    next: &'a [u8],
    pattern_ids: &'a [u8],
    accel: &'a [u8],
}

impl<'a> State<'a> {
    #[inline(always)]
    fn next(&self, input: u8) -> StateID {
        // This straight linear search was observed to be much better than
        // binary search on ASCII haystacks, likely because a binary search
        // visits the ASCII case last but a linear search sees it first. A
        // binary search does do a little better on non-ASCII haystacks, but
        // not by much. There might be a better trade off lurking here.
        for i in 0..(self.ntrans - 1) {
            let (start, end) = self.range(i);
            if start <= input && input <= end {
                return self.next_at(i);
            }
            // We could bail early with an extra branch: if input < b1, then
            // we know we'll never find a matching transition. Interestingly,
            // this extra branch seems to not help performance, or will even
            // hurt it. It's likely very dependent on the DFA itself and what
            // is being searched.
        }
        DEAD
    }

    fn next_eoi(&self) -> StateID {
        self.next_at(self.ntrans - 1)
    }

    fn id(&self) -> StateID {
        self.id
    }

    fn range(&self, i: usize) -> (u8, u8) {
        (self.input_ranges[i * 2], self.input_ranges[i * 2 + 1])
    }

    fn next_at(&self, i: usize) -> StateID {
        let start = i * StateID::SIZE;
        let end = start + StateID::SIZE;
        let bytes = self.next[start..end].try_into().unwrap();
        StateID::from_ne_bytes_unchecked(bytes)
    }

    fn pattern_id(&self, match_index: usize) -> PatternID {
        let start = match_index * PatternID::SIZE;
        bytes::read_pattern_id_unchecked(&self.pattern_ids[start..]).0
    }

    fn pattern_count(&self) -> usize {
        assert_eq!(0, self.pattern_ids.len() % 4);
        self.pattern_ids.len() / 4
    }

    fn bytes_len(&self) -> usize {
        let mut len =
            2 + (self.ntrans * 2) + (self.ntrans * StateID::SIZE) + (1 + self.accel.len());
        if self.is_match {
            len += size_of::<u32>() + self.pattern_ids.len();
        }
        len
    }

    fn accelerator(&self) -> &'a [u8] {
        self.accel
    }
}

impl<'a> fmt::Debug for State<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printed = false;
        for i in 0..(self.ntrans - 1) {
            let next = self.next_at(i);
            if next == DEAD {
                continue;
            }

            if printed {
                write!(f, ", ")?;
            }
            let (start, end) = self.range(i);
            if start == end {
                write!(f, "{:?} => {:?}", DebugByte(start), next)?;
            } else {
                write!(
                    f,
                    "{:?}-{:?} => {:?}",
                    DebugByte(start),
                    DebugByte(end),
                    next,
                )?;
            }
            printed = true;
        }
        let eoi = self.next_at(self.ntrans - 1);
        if eoi != DEAD {
            if printed {
                write!(f, ", ")?;
            }
            write!(f, "EOI => {:?}", eoi)?;
        }
        Ok(())
    }
}

#[cfg(feature = "std")]
struct StateMut<'a> {
    id: StateID,
    is_match: bool,
    ntrans: usize,
    input_ranges: &'a mut [u8],
    next: &'a mut [u8],
    pattern_ids: &'a [u8],
    accel: &'a mut [u8],
}

#[cfg(feature = "std")]
impl<'a> StateMut<'a> {
    fn set_next_at(&mut self, i: usize, next: StateID) {
        let start = i * StateID::SIZE;
        let end = start + StateID::SIZE;
        bytes::write_state_id::<bytes::NE>(next, &mut self.next[start..end]);
    }
}

#[cfg(feature = "std")]
impl<'a> fmt::Debug for StateMut<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = State {
            id: self.id,
            is_match: self.is_match,
            ntrans: self.ntrans,
            input_ranges: self.input_ranges,
            next: self.next,
            pattern_ids: self.pattern_ids,
            accel: self.accel,
        };
        fmt::Debug::fmt(&state, f)
    }
}

#[allow(dead_code)]
#[inline(always)]
fn binary_search_ranges(ranges: &[u8], needle: u8) -> Option<usize> {
    debug_assert!(ranges.len() % 2 == 0, "ranges must have even length");
    debug_assert!(ranges.len() <= 512, "ranges should be short");

    let (mut left, mut right) = (0, ranges.len() / 2);
    while left < right {
        let mid = (left + right) / 2;
        let (b1, b2) = (ranges[mid * 2], ranges[mid * 2 + 1]);
        if needle < b1 {
            right = mid;
        } else if needle > b2 {
            left = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}
