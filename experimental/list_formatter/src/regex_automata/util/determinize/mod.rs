use alloc::vec::Vec;

pub(crate) use self::state::{
    State, StateBuilderEmpty, StateBuilderMatches, StateBuilderNFA,
};

use crate::regex_automata::{
    nfa::thompson::{self, Look, LookSet},
    util::{
        alphabet,
        id::StateID,
        matchtypes::MatchKind,
        sparse_set::{SparseSet, SparseSets},
        start::Start,
    },
};

mod state;

pub(crate) fn next(
    nfa: &thompson::NFA,
    match_kind: MatchKind,
    sparses: &mut SparseSets,
    stack: &mut Vec<StateID>,
    state: &State,
    unit: alphabet::Unit,
    empty_builder: StateBuilderEmpty,
) -> StateBuilderNFA {
    sparses.clear();

    // Put the NFA state IDs into a sparse set in case we need to
    // re-compute their epsilon closure.
    //
    // Doing this state shuffling is technically not necessary unless some
    // kind of look-around is used in the DFA. Some ad hoc experiments
    // suggested that avoiding this didn't lead to much of an improvement,
    // but perhaps more rigorous experimentation should be done. And in
    // particular, avoiding this check requires some light refactoring of
    // the code below.
    state.iter_nfa_state_ids(|nfa_id| {
        sparses.set1.insert(nfa_id);
    });

    // Compute look-ahead assertions originating from the current state.
    // Based on the input unit we're transitioning over, some additional
    // set of assertions may be true. Thus, we re-compute this state's
    // epsilon closure (but only if necessary).
    if !state.look_need().is_empty() {
        // Add look-ahead assertions that are now true based on the current
        // input unit.
        let mut look_have = state.look_have().clone();
        match unit.as_u8() {
            Some(b'\n') => {
                look_have.insert(Look::EndLine);
            }
            Some(_) => {}
            None => {
                look_have.insert(Look::EndText);
                look_have.insert(Look::EndLine);
            }
        }
        if state.is_from_word() == unit.is_word_byte() {
            look_have.insert(Look::WordBoundaryUnicodeNegate);
            look_have.insert(Look::WordBoundaryAsciiNegate);
        } else {
            look_have.insert(Look::WordBoundaryUnicode);
            look_have.insert(Look::WordBoundaryAscii);
        }
        // If we have new assertions satisfied that are among the set of
        // assertions that exist in this state (that is, just because
        // we added an EndLine assertion above doesn't mean there is an
        // EndLine conditional epsilon transition in this state), then we
        // re-compute this state's epsilon closure using the updated set of
        // assertions.
        if !look_have
            .subtract(state.look_have())
            .intersect(state.look_need())
            .is_empty()
        {
            for nfa_id in &sparses.set1 {
                epsilon_closure(
                    nfa,
                    nfa_id,
                    look_have,
                    stack,
                    &mut sparses.set2,
                );
            }
            sparses.swap();
            sparses.set2.clear();
        }
    }

    // Convert our empty builder into one that can record assertions and match
    // pattern IDs.
    let mut builder = empty_builder.into_matches();
    // Set whether the StartLine look-behind assertion is true for this
    // transition or not. The look-behind assertion for ASCII word boundaries
    // is handled below.
    if nfa.has_any_anchor() {
        if unit.as_u8().map_or(false, |b| b == b'\n') {
            // Why only handle StartLine here and not StartText? That's
            // because StartText can only impact the starting state, which
            // is speical cased in start state handling.
            builder.look_have().insert(Look::StartLine);
        }
    }
    for nfa_id in &sparses.set1 {
        match *nfa.state(nfa_id) {
            thompson::State::Union { .. }
            | thompson::State::Fail
            | thompson::State::Look { .. }
            | thompson::State::Capture { .. } => {}
            thompson::State::Match { id } => {
                // Notice here that we are calling the NEW state a match
                // state if the OLD state we are transitioning from
                // contains an NFA match state. This is precisely how we
                // delay all matches by one byte and also what therefore
                // guarantees that starting states cannot be match states.
                //
                // If we didn't delay matches by one byte, then whether
                // a DFA is a matching state or not would be determined
                // by whether one of its own constituent NFA states
                // was a match state. (And that would be done in
                // 'add_nfa_states'.)
                //
                // Also, 'add_match_pattern_id' requires that callers never
                // pass duplicative pattern IDs. We do in fact uphold that
                // guarantee here, but it's subtle. In particular, a Thompson
                // NFA guarantees that each pattern has exactly one match
                // state. Moreover, since we're iterating over the NFA state
                // IDs in a set, we are guarateed not to have any duplicative
                // match states. Thus, it is impossible to add the same pattern
                // ID more than once.
                builder.add_match_pattern_id(id);
                if !match_kind.continue_past_first_match() {
                    break;
                }
            }
            thompson::State::Range { range: ref r } => {
                if r.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        r.next,
                        *builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
            thompson::State::Sparse(ref sparse) => {
                if let Some(next) = sparse.matches_unit(unit) {
                    epsilon_closure(
                        nfa,
                        next,
                        *builder.look_have(),
                        stack,
                        &mut sparses.set2,
                    );
                }
            }
        }
    }
    // We only set the word byte if there's a word boundary look-around
    // anywhere in this regex. Otherwise, there's no point in bloating the
    // number of states if we don't have one.
    //
    // We also only set it when the state has a non-zero number of NFA states.
    // Otherwise, we could wind up with states that *should* be DEAD states
    // but are otherwise distinct from DEAD states because of this look-behind
    // assertion being set. While this can't technically impact correctness *in
    // theory*, it can create pathological DFAs that consume input until EOI or
    // a quit byte is seen. Consuming until EOI isn't a correctness problem,
    // but a (serious) perf problem. Hitting a quit byte, however, could be a
    // correctness problem since it could cause search routines to report an
    // error instead of a detected match once the quit state is entered. (The
    // search routine could be made to be a bit smarter by reporting a match
    // if one was detected once it enters a quit state (and indeed, the search
    // routines in this crate do just that), but it seems better to prevent
    // these things by construction if possible.)
    if nfa.has_word_boundary()
        && unit.is_word_byte()
        && !sparses.set2.is_empty()
    {
        builder.set_is_from_word();
    }
    let mut builder_nfa = builder.into_nfa();
    add_nfa_states(nfa, &sparses.set2, &mut builder_nfa);
    builder_nfa
}

pub(crate) fn epsilon_closure(
    nfa: &thompson::NFA,
    start_nfa_id: StateID,
    look_have: LookSet,
    stack: &mut Vec<StateID>,
    set: &mut SparseSet,
) {
    assert!(stack.is_empty());
    // If this isn't an epsilon state, then the epsilon closure is always just
    // itself, so there's no need to spin up the machinery below to handle it.
    if !nfa.state(start_nfa_id).is_epsilon() {
        set.insert(start_nfa_id);
        return;
    }

    stack.push(start_nfa_id);
    while let Some(mut id) = stack.pop() {
        // In many cases, we can avoid stack operations when an NFA state only
        // adds one new state to visit. In that case, we just set our ID to
        // that state and mush on. We only use the stack when an NFA state
        // introduces multiple new states to visit.
        loop {
            // Insert this NFA state, and if it's already in the set and thus
            // already visited, then we can move on to the next one.
            if !set.insert(id) {
                break;
            }
            match *nfa.state(id) {
                thompson::State::Range { .. }
                | thompson::State::Sparse { .. }
                | thompson::State::Fail
                | thompson::State::Match { .. } => break,
                thompson::State::Look { look, next } => {
                    if !look_have.contains(look) {
                        break;
                    }
                    id = next;
                }
                thompson::State::Union { ref alternates } => {
                    id = match alternates.get(0) {
                        None => break,
                        Some(&id) => id,
                    };
                    // We need to process our alternates in order to preserve
                    // match preferences, so put the earliest alternates closer
                    // to the top of the stack.
                    stack.extend(alternates[1..].iter().rev());
                }
                thompson::State::Capture { next, .. } => {
                    id = next;
                }
            }
        }
    }
}

pub(crate) fn add_nfa_states(
    nfa: &thompson::NFA,
    set: &SparseSet,
    builder: &mut StateBuilderNFA,
) {
    for nfa_id in set {
        match *nfa.state(nfa_id) {
            thompson::State::Range { .. } => {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Sparse { .. } => {
                builder.add_nfa_state_id(nfa_id);
            }
            thompson::State::Look { look, .. } => {
                builder.add_nfa_state_id(nfa_id);
                builder.look_need().insert(look);
            }
            thompson::State::Union { .. }
            | thompson::State::Capture { .. } => {
                // Pure epsilon transitions don't need to be tracked
                // as part of the DFA state. Tracking them is actually
                // superfluous; they won't cause any harm other than making
                // determinization slower.
                //
                // Why aren't these needed? Well, in an NFA, epsilon
                // transitions are really just jumping points to other
                // states. So once you hit an epsilon transition, the same
                // set of resulting states always appears. Therefore,
                // putting them in a DFA's set of ordered NFA states is
                // strictly redundant.
                //
                // Look-around states are also epsilon transitions, but
                // they are *conditional*. So their presence could be
                // discriminatory, and thus, they are tracked above.
                //
                // But wait... why are epsilon states in our `set` in the
                // first place? Why not just leave them out? They're in
                // our `set` because it was generated by computing an
                // epsilon closure, and we want to keep track of all states
                // we visited to avoid re-visiting them. In exchange, we
                // have to do this second iteration over our collected
                // states to finalize our DFA state.
                //
                // Note that this optimization requires that we re-compute
                // the epsilon closure to account for look-ahead in 'next'
                // *only when necessary*. Namely, only when the set of
                // look-around assertions changes and only when those
                // changes are within the set of assertions that are
                // needed in order to step through the closure correctly.
                // Otherwise, if we re-do the epsilon closure needlessly,
                // it could change based on the fact that we are omitting
                // epsilon states here.
            }
            thompson::State::Fail => {
                break;
            }
            thompson::State::Match { .. } => {
                // Normally, the NFA match state doesn't actually need to
                // be inside the DFA state. But since we delay matches by
                // one byte, the matching DFA state corresponds to states
                // that transition from the one we're building here. And
                // the way we detect those cases is by looking for an NFA
                // match state. See 'next' for how this is handled.
                builder.add_nfa_state_id(nfa_id);
            }
        }
    }
    // If we know this state contains no look-around assertions, then
    // there's no reason to track which look-around assertions were
    // satisfied when this state was created.
    if builder.look_need().is_empty() {
        builder.look_have().clear();
    }
}

pub(crate) fn set_lookbehind_from_start(
    start: &Start,
    builder: &mut StateBuilderMatches,
) {
    match *start {
        Start::NonWordByte => {}
        Start::WordByte => {
            builder.set_is_from_word();
        }
        Start::Text => {
            builder.look_have().insert(Look::StartText);
            builder.look_have().insert(Look::StartLine);
        }
        Start::Line => {
            builder.look_have().insert(Look::StartLine);
        }
    }
}
