use core::{borrow::Borrow, iter, mem::size_of};

use alloc::{sync::Arc, vec::Vec};

use crate::regex_automata::{
    hybrid::{
        error::{BuildError, CacheError},
        id::{LazyStateID, LazyStateIDError, OverlappingState},
        search,
    },
    nfa::thompson,
    util::{
        alphabet::{self, ByteClasses, ByteSet},
        determinize::{self, State, StateBuilderEmpty, StateBuilderNFA},
        id::{PatternID, StateID as NFAStateID},
        matchtypes::{HalfMatch, MatchError, MatchKind},
        prefilter,
        sparse_set::SparseSets,
        start::Start,
    },
};

const MIN_STATES: usize = 5;

#[derive(Clone, Debug)]
pub struct DFA {
    nfa: Arc<thompson::NFA>,
    stride2: usize,
    classes: ByteClasses,
    quitset: ByteSet,
    anchored: bool,
    match_kind: MatchKind,
    starts_for_each_pattern: bool,
    cache_capacity: usize,
    minimum_cache_clear_count: Option<usize>,
}

impl DFA {
    pub fn new(pattern: &str) -> Result<DFA, BuildError> {
        DFA::builder().build(pattern)
    }

    pub fn new_many<P: AsRef<str>>(patterns: &[P]) -> Result<DFA, BuildError> {
        DFA::builder().build_many(patterns)
    }

    pub fn always_match() -> Result<DFA, BuildError> {
        let nfa = thompson::NFA::always_match();
        Builder::new().build_from_nfa(Arc::new(nfa))
    }

    pub fn never_match() -> Result<DFA, BuildError> {
        let nfa = thompson::NFA::never_match();
        Builder::new().build_from_nfa(Arc::new(nfa))
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
        Lazy::new(self, cache).reset_cache()
    }

    pub fn pattern_count(&self) -> usize {
        self.nfa.match_len()
    }

    pub fn nfa(&self) -> &Arc<thompson::NFA> {
        &self.nfa
    }

    fn stride2(&self) -> usize {
        self.stride2
    }

    fn stride(&self) -> usize {
        1 << self.stride2()
    }

    fn alphabet_len(&self) -> usize {
        self.classes.alphabet_len()
    }

    pub fn memory_usage(&self) -> usize {
        // Everything else is on the stack.
        self.nfa.memory_usage()
    }
}

impl DFA {
    #[inline]
    pub fn find_earliest_fwd(
        &self,
        cache: &mut Cache,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_earliest_fwd_at(cache, None, None, bytes, 0, bytes.len())
    }

    #[inline]
    pub fn find_earliest_rev(
        &self,
        cache: &mut Cache,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_earliest_rev_at(cache, None, bytes, 0, bytes.len())
    }

    #[inline]
    pub fn find_leftmost_fwd(
        &self,
        cache: &mut Cache,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_leftmost_fwd_at(cache, None, None, bytes, 0, bytes.len())
    }

    #[inline]
    pub fn find_leftmost_rev(
        &self,
        cache: &mut Cache,
        bytes: &[u8],
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_leftmost_rev_at(cache, None, bytes, 0, bytes.len())
    }

    #[inline]
    pub fn find_overlapping_fwd(
        &self,
        cache: &mut Cache,
        bytes: &[u8],
        state: &mut OverlappingState,
    ) -> Result<Option<HalfMatch>, MatchError> {
        self.find_overlapping_fwd_at(cache, None, None, bytes, 0, bytes.len(), state)
    }

    #[inline]
    pub fn find_earliest_fwd_at(
        &self,
        cache: &mut Cache,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_earliest_fwd(pre, self, cache, pattern_id, bytes, start, end)
    }

    #[inline]
    pub fn find_earliest_rev_at(
        &self,
        cache: &mut Cache,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_earliest_rev(self, cache, pattern_id, bytes, start, end)
    }

    #[inline]
    pub fn find_leftmost_fwd_at(
        &self,
        cache: &mut Cache,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_leftmost_fwd(pre, self, cache, pattern_id, bytes, start, end)
    }

    #[inline]
    pub fn find_leftmost_rev_at(
        &self,
        cache: &mut Cache,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_leftmost_rev(self, cache, pattern_id, bytes, start, end)
    }

    #[inline]
    pub fn find_overlapping_fwd_at(
        &self,
        cache: &mut Cache,
        pre: Option<&mut prefilter::Scanner>,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
        state: &mut OverlappingState,
    ) -> Result<Option<HalfMatch>, MatchError> {
        search::find_overlapping_fwd(pre, self, cache, pattern_id, bytes, start, end, state)
    }
}

impl DFA {
    #[inline]
    pub fn next_state(
        &self,
        cache: &mut Cache,
        current: LazyStateID,
        input: u8,
    ) -> Result<LazyStateID, CacheError> {
        let class = usize::from(self.classes.get(input));
        let offset = current.as_usize_untagged() + class;
        let sid = cache.trans[offset];
        if !sid.is_unknown() {
            return Ok(sid);
        }
        let unit = alphabet::Unit::u8(input);
        Lazy::new(self, cache).cache_next_state(current, unit)
    }

    #[inline]
    pub fn next_state_untagged(
        &self,
        cache: &Cache,
        current: LazyStateID,
        input: u8,
    ) -> LazyStateID {
        debug_assert!(!current.is_tagged());
        let class = usize::from(self.classes.get(input));
        let offset = current.as_usize_unchecked() + class;
        cache.trans[offset]
    }

    #[inline]
    pub unsafe fn next_state_untagged_unchecked(
        &self,
        cache: &Cache,
        current: LazyStateID,
        input: u8,
    ) -> LazyStateID {
        debug_assert!(!current.is_tagged());
        let class = usize::from(self.classes.get(input));
        let offset = current.as_usize_unchecked() + class;
        *cache.trans.get_unchecked(offset)
    }

    #[inline]
    pub fn next_eoi_state(
        &self,
        cache: &mut Cache,
        current: LazyStateID,
    ) -> Result<LazyStateID, CacheError> {
        let eoi = self.classes.eoi().as_usize();
        let offset = current.as_usize_untagged() + eoi;
        let sid = cache.trans[offset];
        if !sid.is_unknown() {
            return Ok(sid);
        }
        let unit = self.classes.eoi();
        Lazy::new(self, cache).cache_next_state(current, unit)
    }

    #[inline]
    pub fn start_state_forward(
        &self,
        cache: &mut Cache,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<LazyStateID, CacheError> {
        let mut lazy = Lazy::new(self, cache);
        let start_type = Start::from_position_fwd(bytes, start, end);
        let sid = lazy.as_ref().get_cached_start_id(pattern_id, start_type);
        if !sid.is_unknown() {
            return Ok(sid);
        }
        lazy.cache_start_group(pattern_id, start_type)
    }

    #[inline]
    pub fn start_state_reverse(
        &self,
        cache: &mut Cache,
        pattern_id: Option<PatternID>,
        bytes: &[u8],
        start: usize,
        end: usize,
    ) -> Result<LazyStateID, CacheError> {
        let mut lazy = Lazy::new(self, cache);
        let start_type = Start::from_position_rev(bytes, start, end);
        let sid = lazy.as_ref().get_cached_start_id(pattern_id, start_type);
        if !sid.is_unknown() {
            return Ok(sid);
        }
        lazy.cache_start_group(pattern_id, start_type)
    }

    #[inline]
    pub fn match_count(&self, cache: &Cache, id: LazyStateID) -> usize {
        assert!(id.is_match());
        LazyRef::new(self, cache).get_cached_state(id).match_count()
    }

    #[inline]
    pub fn match_pattern(&self, cache: &Cache, id: LazyStateID, match_index: usize) -> PatternID {
        // This is an optimization for the very common case of a DFA with a
        // single pattern. This conditional avoids a somewhat more costly path
        // that finds the pattern ID from the corresponding `State`, which
        // requires a bit of slicing/pointer-chasing. This optimization tends
        // to only matter when matches are frequent.
        if self.pattern_count() == 1 {
            return PatternID::ZERO;
        }
        LazyRef::new(self, cache)
            .get_cached_state(id)
            .match_pattern(match_index)
    }
}

#[derive(Clone, Debug)]
pub struct Cache {
    // N.B. If you're looking to understand how determinization works, it
    // is probably simpler to first grok src/dfa/determinize.rs, since that
    // doesn't have the "laziness" component.
    trans: Vec<LazyStateID>,
    starts: Vec<LazyStateID>,
    states: Vec<State>,
    states_to_id: StateMap,
    sparses: SparseSets,
    stack: Vec<NFAStateID>,
    scratch_state_builder: StateBuilderEmpty,
    state_saver: StateSaver,
    memory_usage_state: usize,
    clear_count: usize,
}

impl Cache {
    pub fn new(dfa: &DFA) -> Cache {
        let mut cache = Cache {
            trans: alloc::vec![],
            starts: alloc::vec![],
            states: alloc::vec![],
            states_to_id: StateMap::new(),
            sparses: SparseSets::new(dfa.nfa.len()),
            stack: alloc::vec![],
            scratch_state_builder: StateBuilderEmpty::new(),
            state_saver: StateSaver::none(),
            memory_usage_state: 0,
            clear_count: 0,
        };
        Lazy {
            dfa,
            cache: &mut cache,
        }
        .init_cache();
        cache
    }

    pub fn reset(&mut self, dfa: &DFA) {
        Lazy::new(dfa, self).reset_cache()
    }

    pub fn clear_count(&self) -> usize {
        self.clear_count
    }

    pub fn memory_usage(&self) -> usize {
        const ID_SIZE: usize = size_of::<LazyStateID>();
        const STATE_SIZE: usize = size_of::<State>();

        self.trans.len() * ID_SIZE
        + self.starts.len() * ID_SIZE
        + self.states.len() * STATE_SIZE
        // Maps likely use more memory than this, but it's probably close.
        + self.states_to_id.len() * (STATE_SIZE + ID_SIZE)
        + self.sparses.memory_usage()
        + self.stack.capacity() * ID_SIZE
        + self.scratch_state_builder.capacity()
        // Heap memory used by 'State' in both 'states' and 'states_to_id'.
        + self.memory_usage_state
    }
}

#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, LazyStateID>;
#[cfg(not(feature = "std"))]
type StateMap = alloc::collections::BTreeMap<State, LazyStateID>;

#[derive(Debug)]
struct Lazy<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c mut Cache,
}

impl<'i, 'c> Lazy<'i, 'c> {
    fn new(dfa: &'i DFA, cache: &'c mut Cache) -> Lazy<'i, 'c> {
        Lazy { dfa, cache }
    }

    fn as_ref<'a>(&'a self) -> LazyRef<'i, 'a> {
        LazyRef::new(self.dfa, self.cache)
    }

    #[inline(never)]
    fn cache_next_state(
        &mut self,
        mut current: LazyStateID,
        unit: alphabet::Unit,
    ) -> Result<LazyStateID, CacheError> {
        let stride2 = self.dfa.stride2();
        let empty_builder = self.get_state_builder();
        let builder = determinize::next(
            &self.dfa.nfa,
            self.dfa.match_kind,
            &mut self.cache.sparses,
            &mut self.cache.stack,
            &self.cache.states[current.as_usize_untagged() >> stride2],
            unit,
            empty_builder,
        );
        let save_state = !self.as_ref().state_builder_fits_in_cache(&builder);
        if save_state {
            self.save_state(current);
        }
        let next = self.add_builder_state(builder, |sid| sid)?;
        if save_state {
            current = self.saved_state_id();
        }
        // This is the payoff. The next time 'next_state' is called with this
        // state and alphabet unit, it will find this transition and avoid
        // having to re-determinize this transition.
        self.set_transition(current, unit, next);
        Ok(next)
    }

    fn cache_start_group(
        &mut self,
        pattern_id: Option<PatternID>,
        start: Start,
    ) -> Result<LazyStateID, CacheError> {
        let nfa_start_id = match pattern_id {
            Some(pid) => {
                assert!(
                    self.dfa.starts_for_each_pattern,
                    "attempted to search for a specific pattern \
                     without enabling starts_for_each_pattern",
                );
                self.dfa.nfa.start_pattern(pid)
            }
            None if self.dfa.anchored => self.dfa.nfa.start_anchored(),
            None => self.dfa.nfa.start_unanchored(),
        };

        let id = self.cache_start_one(nfa_start_id, start)?;
        self.set_start_state(pattern_id, start, id);
        Ok(id)
    }

    fn cache_start_one(
        &mut self,
        nfa_start_id: NFAStateID,
        start: Start,
    ) -> Result<LazyStateID, CacheError> {
        let mut builder_matches = self.get_state_builder().into_matches();
        determinize::set_lookbehind_from_start(&start, &mut builder_matches);
        self.cache.sparses.set1.clear();
        determinize::epsilon_closure(
            self.dfa.nfa.borrow(),
            nfa_start_id,
            *builder_matches.look_have(),
            &mut self.cache.stack,
            &mut self.cache.sparses.set1,
        );
        let mut builder = builder_matches.into_nfa();
        determinize::add_nfa_states(
            self.dfa.nfa.borrow(),
            &self.cache.sparses.set1,
            &mut builder,
        );
        self.add_builder_state(builder, |id| id.to_start())
    }

    fn add_builder_state(
        &mut self,
        builder: StateBuilderNFA,
        idmap: impl Fn(LazyStateID) -> LazyStateID,
    ) -> Result<LazyStateID, CacheError> {
        if let Some(&cached_id) = self.cache.states_to_id.get(builder.as_bytes()) {
            // Since we have a cached state, put the constructed state's
            // memory back into our scratch space, so that it can be reused.
            self.put_state_builder(builder);
            return Ok(cached_id);
        }
        let result = self.add_state(builder.to_state(), idmap);
        self.put_state_builder(builder);
        result
    }

    fn add_state(
        &mut self,
        state: State,
        idmap: impl Fn(LazyStateID) -> LazyStateID,
    ) -> Result<LazyStateID, CacheError> {
        if !self.as_ref().state_fits_in_cache(&state) {
            self.try_clear_cache()?;
        }
        // It's important for this to come second, since the above may clear
        // the cache. If we clear the cache after ID generation, then the ID
        // is likely bunk since it would have been generated based on a larger
        // transition table.
        let mut id = idmap(self.next_state_id()?);
        if state.is_match() {
            id = id.to_match();
        }
        // Add room in the transition table. Since this is a fresh state, all
        // of its transitions are unknown.
        self.cache
            .trans
            .extend(iter::repeat(self.as_ref().unknown_id()).take(self.dfa.stride()));
        // When we add a sentinel state, we never want to set any quit
        // transitions. Technically, this is harmless, since sentinel states
        // have all of their transitions set to loop back to themselves. But
        // when creating sentinel states before the quit sentinel state,
        // this will try to call 'set_transition' on a state ID that doesn't
        // actually exist yet, which isn't allowed. So we just skip doing so
        // entirely.
        if !self.dfa.quitset.is_empty() && !self.as_ref().is_sentinel(id) {
            let quit_id = self.as_ref().quit_id();
            for b in self.dfa.quitset.iter() {
                self.set_transition(id, alphabet::Unit::u8(b), quit_id);
            }
        }
        self.cache.memory_usage_state += state.memory_usage();
        self.cache.states.push(state.clone());
        self.cache.states_to_id.insert(state, id);
        Ok(id)
    }

    fn next_state_id(&mut self) -> Result<LazyStateID, CacheError> {
        let sid = match LazyStateID::new(self.cache.trans.len()) {
            Ok(sid) => sid,
            Err(_) => {
                self.try_clear_cache()?;
                // This has to pass since we check that ID capacity at
                // construction time can fit at least MIN_STATES states.
                LazyStateID::new(self.cache.trans.len()).unwrap()
            }
        };
        Ok(sid)
    }

    fn try_clear_cache(&mut self) -> Result<(), CacheError> {
        // Currently, the only heuristic we use is the minimum cache clear
        // count. If we pass that minimum, then we give up.
        //
        // It would be good to also add a heuristic based on "bytes searched
        // per generated state," but this requires API design work. Namely,
        // we really do not want to add a counter increment to the transition
        // function, which implies we need to expose APIs to update the number
        // of bytes searched by implementers of the search routines. And that
        // doesn't seem great... But we should do it if this heuristic isn't
        // enough. (The original lazy DFA implementation in the 'regex' crate
        // had this heuristic, since the lazy DFA was coupled with the search
        // routines.)
        if let Some(min_count) = self.dfa.minimum_cache_clear_count {
            if self.cache.clear_count >= min_count {
                return Err(CacheError::too_many_cache_clears());
            }
        }
        self.clear_cache();
        Ok(())
    }

    fn reset_cache(&mut self) {
        self.cache.state_saver = StateSaver::none();
        self.clear_cache();
        // If a new DFA is used, it might have a different number of NFA
        // states, so we need to make sure our sparse sets have the appropriate
        // size.
        self.cache.sparses.resize(self.dfa.nfa.len());
        self.cache.clear_count = 0;
    }

    fn clear_cache(&mut self) {
        self.cache.trans.clear();
        self.cache.starts.clear();
        self.cache.states.clear();
        self.cache.states_to_id.clear();
        self.cache.memory_usage_state = 0;
        self.cache.clear_count += 1;
        trace!(
            "lazy DFA cache has been cleared (count: {})",
            self.cache.clear_count
        );
        self.init_cache();
        // If the state we want to save is one of the sentinel
        // (unknown/dead/quit) states, then 'init_cache' adds those back, and
        // their identifier values remains invariant. So there's no need to add
        // it again. (And indeed, doing so would be incorrect!)
        if let Some((old_id, state)) = self.cache.state_saver.take_to_save() {
            // If the state is one of the special sentinel states, then it is
            // automatically added by cache initialization and its ID always
            // remains the same. With that said, this should never occur since
            // the sentinel states are all loop states back to themselves. So
            // we should never be in a position where we're attempting to save
            // a sentinel state since we never compute transitions out of a
            // sentinel state.
            assert!(
                !self.as_ref().is_sentinel(old_id),
                "cannot save sentinel state"
            );
            let new_id = self
                .add_state(
                    state,
                    |id| {
                        if old_id.is_start() {
                            id.to_start()
                        } else {
                            id
                        }
                    },
                )
                // The unwrap here is OK because lazy DFA creation ensures that
                // we have room in the cache to add MIN_STATES states. Since
                // 'init_cache' above adds 3, this adds a 4th.
                .expect("adding one state after cache clear must work");
            self.cache.state_saver = StateSaver::Saved(new_id);
        }
    }

    fn init_cache(&mut self) {
        let mut starts_len = Start::count();
        if self.dfa.starts_for_each_pattern {
            starts_len += Start::count() * self.dfa.pattern_count();
        }
        self.cache
            .starts
            .extend(iter::repeat(self.as_ref().unknown_id()).take(starts_len));
        // This is the set of NFA states that corresponds to each of our three
        // sentinel states: the empty set.
        let dead = State::dead();
        // This sets up some states that we use as sentinels that are present
        // in every DFA. While it would be technically possible to implement
        // this DFA without explicitly putting these states in the transition
        // table, this is convenient to do to make `next_state` correct for all
        // valid state IDs without needing explicit conditionals to special
        // case these sentinel states.
        //
        // All three of these states are "dead" states. That is, all of
        // them transition only to themselves. So once you enter one of
        // these states, it's impossible to leave them. Thus, any correct
        // search routine must explicitly check for these state types. (Sans
        // `unknown`, since that is only used internally to represent missing
        // states.)
        let unk_id = self.add_state(dead.clone(), |id| id.to_unknown()).unwrap();
        let dead_id = self.add_state(dead.clone(), |id| id.to_dead()).unwrap();
        let quit_id = self.add_state(dead.clone(), |id| id.to_quit()).unwrap();
        assert_eq!(unk_id, self.as_ref().unknown_id());
        assert_eq!(dead_id, self.as_ref().dead_id());
        assert_eq!(quit_id, self.as_ref().quit_id());
        // The idea here is that if you start in an unknown/dead/quit state and
        // try to transition on them, then you should end up where you started.
        self.set_all_transitions(unk_id, unk_id);
        self.set_all_transitions(dead_id, dead_id);
        self.set_all_transitions(quit_id, quit_id);
        // All of these states are technically equivalent from the FSM
        // perspective, so putting all three of them in the cache isn't
        // possible. (They are distinct merely because we use their
        // identifiers as sentinels to mean something, as indicated by the
        // names.) Moreover, we wouldn't want to do that. Unknown and quit
        // states are special in that they are artificial constructions
        // this implementation. But dead states are a natural part of
        // determinization. When you reach a point in the NFA where you cannot
        // go anywhere else, a dead state will naturally arise and we MUST
        // reuse the canonical dead state that we've created here. Why? Because
        // it is the state ID that tells the search routine whether a state is
        // dead or not, and thus, whether to stop the search. Having a bunch of
        // distinct dead states would be quite wasteful!
        self.cache.states_to_id.insert(dead, dead_id);
    }

    fn save_state(&mut self, id: LazyStateID) {
        let state = self.as_ref().get_cached_state(id).clone();
        self.cache.state_saver = StateSaver::ToSave { id, state };
    }

    fn saved_state_id(&mut self) -> LazyStateID {
        self.cache
            .state_saver
            .take_saved()
            .expect("state saver does not have saved state ID")
    }

    fn set_all_transitions(&mut self, from: LazyStateID, to: LazyStateID) {
        for unit in self.dfa.classes.representatives() {
            self.set_transition(from, unit, to);
        }
    }

    fn set_transition(&mut self, from: LazyStateID, unit: alphabet::Unit, to: LazyStateID) {
        assert!(
            self.as_ref().is_valid(from),
            "invalid 'from' id: {:?}",
            from
        );
        assert!(self.as_ref().is_valid(to), "invalid 'to' id: {:?}", to);
        let offset = from.as_usize_untagged() + self.dfa.classes.get_by_unit(unit);
        self.cache.trans[offset] = to;
    }

    fn set_start_state(&mut self, pattern_id: Option<PatternID>, start: Start, id: LazyStateID) {
        assert!(self.as_ref().is_valid(id));
        let start_index = start.as_usize();
        let index = match pattern_id {
            None => start_index,
            Some(pid) => {
                assert!(
                    self.dfa.starts_for_each_pattern,
                    "attempted to search for a specific pattern \
                     without enabling starts_for_each_pattern",
                );
                let pid = pid.as_usize();
                Start::count() + (Start::count() * pid) + start_index
            }
        };
        self.cache.starts[index] = id;
    }

    fn get_state_builder(&mut self) -> StateBuilderEmpty {
        core::mem::replace(
            &mut self.cache.scratch_state_builder,
            StateBuilderEmpty::new(),
        )
    }

    fn put_state_builder(&mut self, builder: StateBuilderNFA) {
        let _ = core::mem::replace(&mut self.cache.scratch_state_builder, builder.clear());
    }
}

#[derive(Debug)]
struct LazyRef<'i, 'c> {
    dfa: &'i DFA,
    cache: &'c Cache,
}

impl<'i, 'c> LazyRef<'i, 'c> {
    fn new(dfa: &'i DFA, cache: &'c Cache) -> LazyRef<'i, 'c> {
        LazyRef { dfa, cache }
    }

    fn get_cached_start_id(&self, pattern_id: Option<PatternID>, start: Start) -> LazyStateID {
        let start_index = start.as_usize();
        let index = match pattern_id {
            None => start_index,
            Some(pid) => {
                let pid = pid.as_usize();
                assert!(
                    pid < self.dfa.pattern_count(),
                    "invalid pattern ID: {:?}",
                    pid
                );
                Start::count() + (Start::count() * pid) + start_index
            }
        };
        self.cache.starts[index]
    }

    fn get_cached_state(&self, sid: LazyStateID) -> &State {
        let index = sid.as_usize_untagged() >> self.dfa.stride2();
        &self.cache.states[index]
    }

    fn is_sentinel(&self, id: LazyStateID) -> bool {
        id == self.unknown_id() || id == self.dead_id() || id == self.quit_id()
    }

    fn unknown_id(&self) -> LazyStateID {
        // This unwrap is OK since 0 is always a valid state ID.
        LazyStateID::new(0).unwrap().to_unknown()
    }

    fn dead_id(&self) -> LazyStateID {
        // This unwrap is OK since the maximum value here is 1 * 512 = 512,
        // which is <= 2047 (the maximum state ID on 16-bit systems). Where
        // 512 is the worst case for our equivalence classes (every byte is a
        // distinct class).
        LazyStateID::new(1 << self.dfa.stride2()).unwrap().to_dead()
    }

    fn quit_id(&self) -> LazyStateID {
        // This unwrap is OK since the maximum value here is 2 * 512 = 1024,
        // which is <= 2047 (the maximum state ID on 16-bit systems). Where
        // 512 is the worst case for our equivalence classes (every byte is a
        // distinct class).
        LazyStateID::new(2 << self.dfa.stride2()).unwrap().to_quit()
    }

    fn is_valid(&self, id: LazyStateID) -> bool {
        let id = id.as_usize_untagged();
        id < self.cache.trans.len() && id % self.dfa.stride() == 0
    }

    fn state_fits_in_cache(&self, state: &State) -> bool {
        let needed =
            self.cache.memory_usage() + self.memory_usage_for_one_more_state(state.memory_usage());
        needed <= self.dfa.cache_capacity
    }

    fn state_builder_fits_in_cache(&self, state: &StateBuilderNFA) -> bool {
        let needed = self.cache.memory_usage()
            + self.memory_usage_for_one_more_state(state.as_bytes().len());
        needed <= self.dfa.cache_capacity
    }

    fn memory_usage_for_one_more_state(&self, state_heap_size: usize) -> usize {
        const ID_SIZE: usize = size_of::<LazyStateID>();
        const STATE_SIZE: usize = size_of::<State>();

        self.dfa.stride() * ID_SIZE // additional space needed in trans table
        + STATE_SIZE // space in cache.states
        + (STATE_SIZE + ID_SIZE) // space in cache.states_to_id
        + state_heap_size // heap memory used by state itself
    }
}

#[derive(Clone, Debug)]
enum StateSaver {
    None,
    ToSave { id: LazyStateID, state: State },
    Saved(LazyStateID),
}

impl StateSaver {
    fn none() -> StateSaver {
        StateSaver::None
    }

    fn take_to_save(&mut self) -> Option<(LazyStateID, State)> {
        match core::mem::replace(self, StateSaver::None) {
            StateSaver::None | StateSaver::Saved(_) => None,
            StateSaver::ToSave { id, state } => Some((id, state)),
        }
    }

    fn take_saved(&mut self) -> Option<LazyStateID> {
        match core::mem::replace(self, StateSaver::None) {
            StateSaver::None => None,
            StateSaver::Saved(id) | StateSaver::ToSave { id, .. } => Some(id),
        }
    }
}

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
    match_kind: Option<MatchKind>,
    starts_for_each_pattern: Option<bool>,
    byte_classes: Option<bool>,
    unicode_word_boundary: Option<bool>,
    quitset: Option<ByteSet>,
    cache_capacity: Option<usize>,
    skip_cache_capacity_check: Option<bool>,
    minimum_cache_clear_count: Option<Option<usize>>,
}

impl Config {
    pub fn new() -> Config {
        Config::default()
    }

    pub fn anchored(mut self, yes: bool) -> Config {
        self.anchored = Some(yes);
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
        if self.quitset.is_none() {
            self.quitset = Some(ByteSet::empty());
        }
        if yes {
            self.quitset.as_mut().unwrap().add(byte);
        } else {
            self.quitset.as_mut().unwrap().remove(byte);
        }
        self
    }

    pub fn cache_capacity(mut self, bytes: usize) -> Config {
        self.cache_capacity = Some(bytes);
        self
    }

    pub fn skip_cache_capacity_check(mut self, yes: bool) -> Config {
        self.skip_cache_capacity_check = Some(yes);
        self
    }

    pub fn minimum_cache_clear_count(mut self, min: Option<usize>) -> Config {
        self.minimum_cache_clear_count = Some(min);
        self
    }

    pub fn get_anchored(&self) -> bool {
        self.anchored.unwrap_or(false)
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
        self.quitset.map_or(false, |q| q.contains(byte))
    }

    pub fn get_cache_capacity(&self) -> usize {
        self.cache_capacity.unwrap_or(2 * (1 << 20))
    }

    pub fn get_skip_cache_capacity_check(&self) -> bool {
        self.skip_cache_capacity_check.unwrap_or(false)
    }

    pub fn get_minimum_cache_clear_count(&self) -> Option<usize> {
        self.minimum_cache_clear_count.unwrap_or(None)
    }

    pub fn get_minimum_cache_capacity(&self, nfa: &thompson::NFA) -> Result<usize, BuildError> {
        let quitset = self.quit_set_from_nfa(nfa)?;
        let classes = self.byte_classes_from_nfa(nfa, &quitset);
        let starts = self.get_starts_for_each_pattern();
        Ok(minimum_cache_capacity(nfa, &classes, starts))
    }

    fn byte_classes_from_nfa(&self, nfa: &thompson::NFA, quit: &ByteSet) -> ByteClasses {
        if !self.get_byte_classes() {
            // The lazy DFA will always use the equivalence class map, but
            // enabling this option is useful for debugging. Namely, this will
            // cause all transitions to be defined over their actual bytes
            // instead of an opaque equivalence class identifier. The former is
            // much easier to grok as a human.
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
        }
    }

    fn quit_set_from_nfa(&self, nfa: &thompson::NFA) -> Result<ByteSet, BuildError> {
        let mut quit = self.quitset.unwrap_or(ByteSet::empty());
        if nfa.has_word_boundary_unicode() {
            if self.get_unicode_word_boundary() {
                for b in 0x80..=0xFF {
                    quit.add(b);
                }
            } else {
                // If heuristic support for Unicode word boundaries wasn't
                // enabled, then we can still check if our quit set is correct.
                // If the caller set their quit bytes in a way that causes the
                // DFA to quit on at least all non-ASCII bytes, then that's all
                // we need for heuristic support to work.
                if !quit.contains_range(0x80, 0xFF) {
                    return Err(BuildError::unsupported_dfa_word_boundary_unicode());
                }
            }
        }
        Ok(quit)
    }

    fn overwrite(self, o: Config) -> Config {
        Config {
            anchored: o.anchored.or(self.anchored),
            match_kind: o.match_kind.or(self.match_kind),
            starts_for_each_pattern: o.starts_for_each_pattern.or(self.starts_for_each_pattern),
            byte_classes: o.byte_classes.or(self.byte_classes),
            unicode_word_boundary: o.unicode_word_boundary.or(self.unicode_word_boundary),
            quitset: o.quitset.or(self.quitset),
            cache_capacity: o.cache_capacity.or(self.cache_capacity),
            skip_cache_capacity_check: o
                .skip_cache_capacity_check
                .or(self.skip_cache_capacity_check),
            minimum_cache_clear_count: o
                .minimum_cache_clear_count
                .or(self.minimum_cache_clear_count),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Builder {
    config: Config,
    thompson: thompson::Builder,
}

impl Builder {
    pub fn new() -> Builder {
        Builder {
            config: Config::default(),
            thompson: thompson::Builder::new(),
        }
    }

    pub fn build(&self, pattern: &str) -> Result<DFA, BuildError> {
        self.build_many(&[pattern])
    }

    pub fn build_many<P: AsRef<str>>(&self, patterns: &[P]) -> Result<DFA, BuildError> {
        let nfa = self
            .thompson
            .build_many(patterns)
            .map_err(BuildError::nfa)?;
        self.build_from_nfa(Arc::new(nfa))
    }

    pub fn build_from_nfa(&self, nfa: Arc<thompson::NFA>) -> Result<DFA, BuildError> {
        let quitset = self.config.quit_set_from_nfa(&nfa)?;
        let classes = self.config.byte_classes_from_nfa(&nfa, &quitset);
        // Check that we can fit at least a few states into our cache,
        // otherwise it's pretty senseless to use the lazy DFA. This does have
        // a possible failure mode though. This assumes the maximum size of a
        // state in powerset space (so, the total number of NFA states), which
        // may never actually materialize, and could be quite a bit larger
        // than the actual biggest state. If this turns out to be a problem,
        // we could expose a knob that disables this check. But if so, we have
        // to be careful not to panic in other areas of the code (the cache
        // clearing and init code) that tend to assume some minimum useful
        // cache capacity.
        let min_cache =
            minimum_cache_capacity(&nfa, &classes, self.config.get_starts_for_each_pattern());
        let mut cache_capacity = self.config.get_cache_capacity();
        if cache_capacity < min_cache {
            // When the caller has asked us to skip the cache capacity check,
            // then we simply force the cache capacity to its minimum amount
            // and mush on.
            if self.config.get_skip_cache_capacity_check() {
                trace!(
                    "given capacity ({}) is too small, \
                     since skip_cache_capacity_check is enabled, \
                     setting cache capacity to minimum ({})",
                    cache_capacity,
                    min_cache,
                );
                cache_capacity = min_cache;
            } else {
                return Err(BuildError::insufficient_cache_capacity(
                    min_cache,
                    cache_capacity,
                ));
            }
        }
        // We also need to check that we can fit at least some small number
        // of states in our state ID space. This is unlikely to trigger in
        // >=32-bit systems, but 16-bit systems have a pretty small state ID
        // space since a number of bits are used up as sentinels.
        if let Err(err) = minimum_lazy_state_id(&nfa, &classes) {
            return Err(BuildError::insufficient_state_id_capacity(err));
        }
        let stride2 = classes.stride2();
        Ok(DFA {
            nfa,
            stride2,
            classes,
            quitset,
            anchored: self.config.get_anchored(),
            match_kind: self.config.get_match_kind(),
            starts_for_each_pattern: self.config.get_starts_for_each_pattern(),
            cache_capacity,
            minimum_cache_clear_count: self.config.get_minimum_cache_clear_count(),
        })
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

fn minimum_lazy_state_id(
    nfa: &thompson::NFA,
    classes: &ByteClasses,
) -> Result<LazyStateID, LazyStateIDError> {
    let stride = 1 << classes.stride2();
    let min_state_index = MIN_STATES.checked_sub(1).unwrap();
    LazyStateID::new(min_state_index * stride)
}

fn minimum_cache_capacity(
    nfa: &thompson::NFA,
    classes: &ByteClasses,
    starts_for_each_pattern: bool,
) -> usize {
    const ID_SIZE: usize = size_of::<LazyStateID>();
    let stride = 1 << classes.stride2();

    let sparses = 2 * nfa.len() * NFAStateID::SIZE;
    let trans = MIN_STATES * stride * ID_SIZE;

    let mut starts = Start::count() * ID_SIZE;
    if starts_for_each_pattern {
        starts += (Start::count() * nfa.match_len()) * ID_SIZE;
    }

    // Every `State` has three bytes for flags, 4 bytes (max) for the number
    // of patterns, followed by 32-bit encodings of patterns and then delta
    // varint encodings of NFA state IDs. We use the worst case (which isn't
    // technically possible) of 5 bytes for each NFA state ID.
    //
    // HOWEVER, three of the states needed by a lazy DFA are just the sentinel
    // unknown, dead and quit states. Those states have a known size and it is
    // small.
    assert!(
        MIN_STATES >= 3,
        "minimum number of states has to be at least 3"
    );
    let dead_state_size = State::dead().memory_usage();
    let max_state_size = 3 + 4 + (nfa.match_len() * 4) + (nfa.len() * 5);
    let states = (3 * (size_of::<State>() + dead_state_size))
        + ((MIN_STATES - 3) * (size_of::<State>() + max_state_size));
    let states_to_sid = states + (MIN_STATES * ID_SIZE);
    let stack = nfa.len() * NFAStateID::SIZE;
    let scratch_state_builder = max_state_size;

    trans + starts + states + states_to_sid + sparses + stack + scratch_state_builder
}
