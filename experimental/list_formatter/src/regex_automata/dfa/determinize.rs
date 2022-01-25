use alloc::{
    collections::BTreeMap,
    vec::{self, Vec},
};

use crate::regex_automata::{
    dfa::{dense, Error, DEAD},
    nfa::thompson,
    util::{
        self,
        alphabet::{self, ByteSet},
        determinize::{State, StateBuilderEmpty, StateBuilderNFA},
        id::{PatternID, StateID},
        matchtypes::MatchKind,
        sparse_set::{SparseSet, SparseSets},
        start::Start,
    },
};

#[derive(Clone, Debug)]
pub(crate) struct Config {
    anchored: bool,
    match_kind: MatchKind,
    quit: ByteSet,
    dfa_size_limit: Option<usize>,
    determinize_size_limit: Option<usize>,
}

impl Config {
            pub fn new() -> Config {
        Config {
            anchored: false,
            match_kind: MatchKind::LeftmostFirst,
            quit: ByteSet::empty(),
            dfa_size_limit: None,
            determinize_size_limit: None,
        }
    }

                    pub fn run(
        &self,
        nfa: &thompson::NFA,
        dfa: &mut dense::OwnedDFA,
    ) -> Result<(), Error> {
        let dead = State::dead();
        let quit = State::dead();
        let mut cache = StateMap::default();
        // We only insert the dead state here since its representation is
        // identical to the quit state. And we never want anything pointing
        // to the quit state other than specific transitions derived from the
        // determinizer's configured "quit" bytes.
        //
        // We do put the quit state into 'builder_states' below. This ensures
        // that a proper DFA state ID is allocated for it, and that no other
        // DFA state uses the "location after the DEAD state." That is, it
        // is assumed that the quit state is always the state immediately
        // following the DEAD state.
        cache.insert(dead.clone(), DEAD);

        let runner = Runner {
            config: self.clone(),
            nfa,
            dfa,
            builder_states: alloc::vec![dead, quit],
            cache,
            memory_usage_state: 0,
            sparses: SparseSets::new(nfa.len()),
            stack: alloc::vec![],
            scratch_state_builder: StateBuilderEmpty::new(),
        };
        runner.run()
    }

                pub fn anchored(&mut self, yes: bool) -> &mut Config {
        self.anchored = yes;
        self
    }

                                            pub fn match_kind(&mut self, kind: MatchKind) -> &mut Config {
        self.match_kind = kind;
        self
    }

            pub fn quit(&mut self, set: ByteSet) -> &mut Config {
        self.quit = set;
        self
    }

            pub fn dfa_size_limit(&mut self, bytes: Option<usize>) -> &mut Config {
        self.dfa_size_limit = bytes;
        self
    }

            pub fn determinize_size_limit(
        &mut self,
        bytes: Option<usize>,
    ) -> &mut Config {
        self.determinize_size_limit = bytes;
        self
    }
}

#[derive(Debug)]
struct Runner<'a> {
        config: Config,
        nfa: &'a thompson::NFA,
        dfa: &'a mut dense::OwnedDFA,
                                                                                                                            builder_states: Vec<State>,
                        cache: StateMap,
                    memory_usage_state: usize,
                sparses: SparseSets,
            stack: Vec<StateID>,
                            scratch_state_builder: StateBuilderEmpty,
}

#[cfg(feature = "std")]
type StateMap = std::collections::HashMap<State, StateID>;
#[cfg(not(feature = "std"))]
type StateMap = BTreeMap<State, StateID>;

impl<'a> Runner<'a> {
                fn run(mut self) -> Result<(), Error> {
        if self.nfa.has_word_boundary_unicode()
            && !self.config.quit.contains_range(0x80, 0xFF)
        {
            return Err(Error::unsupported_dfa_word_boundary_unicode());
        }

        // A sequence of "representative" bytes drawn from each equivalence
        // class. These representative bytes are fed to the NFA to compute
        // state transitions. This allows us to avoid re-computing state
        // transitions for bytes that are guaranteed to produce identical
        // results.
        let representatives: Vec<alphabet::Unit> =
            self.dfa.byte_classes().representatives().collect();
        // The set of all DFA state IDs that still need to have their
        // transitions set. We start by seeding this with all starting states.
        let mut uncompiled = alloc::vec![];
        self.add_all_starts(&mut uncompiled)?;
        while let Some(dfa_id) = uncompiled.pop() {
            for &unit in &representatives {
                if unit.as_u8().map_or(false, |b| self.config.quit.contains(b))
                {
                    continue;
                }
                // In many cases, the state we transition to has already been
                // computed. 'cached_state' will do the minimal amount of work
                // to check this, and if it exists, immediately return an
                // already existing state ID.
                let (next_dfa_id, is_new) = self.cached_state(dfa_id, unit)?;
                self.dfa.set_transition(dfa_id, unit, next_dfa_id);
                // If the state ID we got back is newly created, then we need
                // to compile it, so add it to our uncompiled frontier.
                if is_new {
                    uncompiled.push(next_dfa_id);
                }
            }
        }
        trace!(
            "determinization complete, memory usage: {}, dense DFA size: {}",
            self.memory_usage(),
            self.dfa.memory_usage(),
        );

        // A map from DFA state ID to one or more NFA match IDs. Each NFA match
        // ID corresponds to a distinct regex pattern that matches in the state
        // corresponding to the key.
        let mut matches: BTreeMap<StateID, Vec<PatternID>> = BTreeMap::new();
        self.cache.clear();
        #[allow(unused_variables)]
        let mut total_pat_count = 0;
        for (i, state) in self.builder_states.into_iter().enumerate() {
            if let Some(pat_ids) = state.match_pattern_ids() {
                let id = self.dfa.from_index(i);
                total_pat_count += pat_ids.len();
                matches.insert(id, pat_ids);
            }
        }
        log! {
            use core::mem::size_of;
            let per_elem = size_of::<StateID>() + size_of::<Vec<PatternID>>();
            let pats = total_pat_count * size_of::<PatternID>();
            let mem = (matches.len() * per_elem) + pats;
            log::trace!("matches map built, memory usage: {}", mem);
        }
        // At this point, we shuffle the "special" states in the final DFA.
        // This permits a DFA's match loop to detect a match condition (among
        // other things) by merely inspecting the current state's identifier,
        // and avoids the need for any additional auxiliary storage.
        self.dfa.shuffle(matches)?;
        Ok(())
    }

                                    fn cached_state(
        &mut self,
        dfa_id: StateID,
        unit: alphabet::Unit,
    ) -> Result<(StateID, bool), Error> {
        // Compute the set of all reachable NFA states, including epsilons.
        let empty_builder = self.get_state_builder();
        let builder = util::determinize::next(
            self.nfa,
            self.config.match_kind,
            &mut self.sparses,
            &mut self.stack,
            &self.builder_states[self.dfa.to_index(dfa_id)],
            unit,
            empty_builder,
        );
        self.maybe_add_state(builder)
    }

            fn add_all_starts(
        &mut self,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), Error> {
        // Always add the (possibly unanchored) start states for matching any
        // of the patterns in this DFA.
        self.add_start_group(None, dfa_state_ids)?;
        // We only need to compute anchored start states for each pattern if it
        // was requested to do so.
        if self.dfa.has_starts_for_each_pattern() {
            for pid in PatternID::iter(self.dfa.pattern_count()) {
                self.add_start_group(Some(pid), dfa_state_ids)?;
            }
        }
        Ok(())
    }

                                    fn add_start_group(
        &mut self,
        pattern_id: Option<PatternID>,
        dfa_state_ids: &mut Vec<StateID>,
    ) -> Result<(), Error> {
        let nfa_start = match pattern_id {
            Some(pid) => self.nfa.start_pattern(pid),
            None if self.config.anchored => self.nfa.start_anchored(),
            None => self.nfa.start_unanchored(),
        };

        // When compiling start states, we're careful not to build additional
        // states that aren't necessary. For example, if the NFA has no word
        // boundary assertion, then there's no reason to have distinct start
        // states for 'NonWordByte' and 'WordByte' starting configurations.
        // Instead, the 'WordByte' starting configuration can just point
        // directly to the start state for the 'NonWordByte' config.

        let (id, is_new) =
            self.add_one_start(nfa_start, Start::NonWordByte)?;
        self.dfa.set_start_state(Start::NonWordByte, pattern_id, id);
        if is_new {
            dfa_state_ids.push(id);
        }

        if !self.nfa.has_word_boundary() {
            self.dfa.set_start_state(Start::WordByte, pattern_id, id);
        } else {
            let (id, is_new) =
                self.add_one_start(nfa_start, Start::WordByte)?;
            self.dfa.set_start_state(Start::WordByte, pattern_id, id);
            if is_new {
                dfa_state_ids.push(id);
            }
        }
        if !self.nfa.has_any_anchor() {
            self.dfa.set_start_state(Start::Text, pattern_id, id);
            self.dfa.set_start_state(Start::Line, pattern_id, id);
        } else {
            let (id, is_new) = self.add_one_start(nfa_start, Start::Text)?;
            self.dfa.set_start_state(Start::Text, pattern_id, id);
            if is_new {
                dfa_state_ids.push(id);
            }

            let (id, is_new) = self.add_one_start(nfa_start, Start::Line)?;
            self.dfa.set_start_state(Start::Line, pattern_id, id);
            if is_new {
                dfa_state_ids.push(id);
            }
        }

        Ok(())
    }

                                fn add_one_start(
        &mut self,
        nfa_start: StateID,
        start: Start,
    ) -> Result<(StateID, bool), Error> {
        // Compute the look-behind assertions that are true in this starting
        // configuration, and the determine the epsilon closure. While
        // computing the epsilon closure, we only follow condiional epsilon
        // transitions that satisfy the look-behind assertions in 'facts'.
        let mut builder_matches = self.get_state_builder().into_matches();
        util::determinize::set_lookbehind_from_start(
            &start,
            &mut builder_matches,
        );
        self.sparses.set1.clear();
        util::determinize::epsilon_closure(
            self.nfa,
            nfa_start,
            *builder_matches.look_have(),
            &mut self.stack,
            &mut self.sparses.set1,
        );
        let mut builder = builder_matches.into_nfa();
        util::determinize::add_nfa_states(
            &self.nfa,
            &self.sparses.set1,
            &mut builder,
        );
        self.maybe_add_state(builder)
    }

                                                fn maybe_add_state(
        &mut self,
        builder: StateBuilderNFA,
    ) -> Result<(StateID, bool), Error> {
        if let Some(&cached_id) = self.cache.get(builder.as_bytes()) {
            // Since we have a cached state, put the constructed state's
            // memory back into our scratch space, so that it can be reused.
            self.put_state_builder(builder);
            return Ok((cached_id, false));
        }
        self.add_state(builder).map(|sid| (sid, true))
    }

                                    fn add_state(
        &mut self,
        builder: StateBuilderNFA,
    ) -> Result<StateID, Error> {
        let id = self.dfa.add_empty_state()?;
        if !self.config.quit.is_empty() {
            for b in self.config.quit.iter() {
                self.dfa.set_transition(
                    id,
                    alphabet::Unit::u8(b),
                    self.dfa.quit_id(),
                );
            }
        }
        let state = builder.to_state();
        // States use reference counting internally, so we only need to count
        // their memroy usage once.
        self.memory_usage_state += state.memory_usage();
        self.builder_states.push(state.clone());
        self.cache.insert(state, id);
        self.put_state_builder(builder);
        if let Some(limit) = self.config.dfa_size_limit {
            if self.dfa.memory_usage() > limit {
                return Err(Error::dfa_exceeded_size_limit(limit));
            }
        }
        if let Some(limit) = self.config.determinize_size_limit {
            if self.memory_usage() > limit {
                return Err(Error::determinize_exceeded_size_limit(limit));
            }
        }
        Ok(id)
    }

                            fn get_state_builder(&mut self) -> StateBuilderEmpty {
        core::mem::replace(
            &mut self.scratch_state_builder,
            StateBuilderEmpty::new(),
        )
    }

                    fn put_state_builder(&mut self, builder: StateBuilderNFA) {
        let _ = core::mem::replace(
            &mut self.scratch_state_builder,
            builder.clear(),
        );
    }

                fn memory_usage(&self) -> usize {
        use core::mem::size_of;

        self.builder_states.len() * size_of::<State>()
        // Maps likely use more memory than this, but it's probably close.
        + self.cache.len() * (size_of::<State>() + size_of::<StateID>())
        + self.memory_usage_state
        + self.stack.capacity() * size_of::<StateID>()
        + self.scratch_state_builder.capacity()
    }
}
