use crate::regex_automata::{
    dfa::{automaton::Automaton, dense, sparse},
    util::id::StateID,
};

impl<T: AsRef<[u32]>> fst::Automaton for dense::DFA<T> {
    type State = StateID;

    #[inline]
    fn start(&self) -> StateID {
        self.start_state_forward(None, &[], 0, 0)
    }

    #[inline]
    fn is_match(&self, state: &StateID) -> bool {
        self.is_match_state(*state)
    }

    #[inline]
    fn accept(&self, state: &StateID, byte: u8) -> StateID {
        if fst::Automaton::is_match(self, state) {
            return *state;
        }
        self.next_state(*state, byte)
    }

    #[inline]
    fn accept_eof(&self, state: &StateID) -> Option<StateID> {
        if fst::Automaton::is_match(self, state) {
            return Some(*state);
        }
        Some(self.next_eoi_state(*state))
    }

    #[inline]
    fn can_match(&self, state: &StateID) -> bool {
        !self.is_dead_state(*state)
    }
}

impl<T: AsRef<[u8]>> fst::Automaton for sparse::DFA<T> {
    type State = StateID;

    #[inline]
    fn start(&self) -> StateID {
        self.start_state_forward(None, &[], 0, 0)
    }

    #[inline]
    fn is_match(&self, state: &StateID) -> bool {
        self.is_match_state(*state)
    }

    #[inline]
    fn accept(&self, state: &StateID, byte: u8) -> StateID {
        if fst::Automaton::is_match(self, state) {
            return *state;
        }
        self.next_state(*state, byte)
    }

    #[inline]
    fn accept_eof(&self, state: &StateID) -> Option<StateID> {
        if fst::Automaton::is_match(self, state) {
            return Some(*state);
        }
        Some(self.next_eoi_state(*state))
    }

    #[inline]
    fn can_match(&self, state: &StateID) -> bool {
        !self.is_dead_state(*state)
    }
}
