use crate::regex_automata::{
    dfa::{
        accel,
        automaton::{Automaton, OverlappingState, StateMatch},
    },
    util::{
        id::{PatternID, StateID},
        matchtypes::HalfMatch,
        prefilter, MATCH_OFFSET,
    },
    MatchError,
};

#[inline(never)]
pub fn find_earliest_fwd<A: Automaton + ?Sized>(
    pre: Option<&mut prefilter::Scanner>,
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<Option<HalfMatch>, MatchError> {
    // Searching with a pattern ID is always anchored, so we should never use
    // a prefilter.
    if pre.is_some() && pattern_id.is_none() {
        find_fwd(pre, true, dfa, pattern_id, bytes, start, end)
    } else {
        find_fwd(None, true, dfa, pattern_id, bytes, start, end)
    }
}

#[inline(never)]
pub fn find_leftmost_fwd<A: Automaton + ?Sized>(
    pre: Option<&mut prefilter::Scanner>,
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<Option<HalfMatch>, MatchError> {
    // Searching with a pattern ID is always anchored, so we should never use
    // a prefilter.
    if pre.is_some() && pattern_id.is_none() {
        find_fwd(pre, false, dfa, pattern_id, bytes, start, end)
    } else {
        find_fwd(None, false, dfa, pattern_id, bytes, start, end)
    }
}

#[inline(always)]
fn find_fwd<A: Automaton + ?Sized>(
    mut pre: Option<&mut prefilter::Scanner>,
    earliest: bool,
    dfa: &A,
    pattern_id: Option<PatternID>,
    haystack: &[u8],
    start: usize,
    end: usize,
) -> Result<Option<HalfMatch>, MatchError> {
    assert!(start <= end);
    assert!(start <= haystack.len());
    assert!(end <= haystack.len());

    // Why do this? This lets 'bytes[at]' work without bounds checks below.
    // It seems the assert on 'end <= haystack.len()' above is otherwise
    // not enough. Why not just make 'bytes' scoped this way anyway? Well,
    // 'eoi_fwd' (below) might actually want to try to access the byte at 'end'
    // for resolving look-ahead.
    let bytes = &haystack[..end];

    let mut state = init_fwd(dfa, pattern_id, haystack, start, end)?;
    let mut last_match = None;
    let mut at = start;
    if let Some(ref mut pre) = pre {
        // If a prefilter doesn't report false positives, then we don't need to
        // touch the DFA at all. However, since all matches include the pattern
        // ID, and the prefilter infrastructure doesn't report pattern IDs, we
        // limit this optimization to cases where there is exactly one pattern.
        // In that case, any match must be the 0th pattern.
        if dfa.pattern_count() == 1 && !pre.reports_false_positives() {
            return Ok(pre
                .next_candidate(bytes, at)
                .into_option()
                .map(|offset| HalfMatch {
                    pattern: PatternID::ZERO,
                    offset,
                }));
        } else if pre.is_effective(at) {
            match pre.next_candidate(bytes, at).into_option() {
                None => return Ok(None),
                Some(i) => {
                    at = i;
                }
            }
        }
    }
    while at < end {
        let byte = bytes[at];
        state = dfa.next_state(state, byte);
        at += 1;
        if dfa.is_special_state(state) {
            if dfa.is_start_state(state) {
                if let Some(ref mut pre) = pre {
                    if pre.is_effective(at) {
                        match pre.next_candidate(bytes, at).into_option() {
                            None => return Ok(None),
                            Some(i) => {
                                at = i;
                            }
                        }
                    }
                } else if dfa.is_accel_state(state) {
                    let needles = dfa.accelerator(state);
                    at = accel::find_fwd(needles, bytes, at).unwrap_or(bytes.len());
                }
            } else if dfa.is_match_state(state) {
                last_match = Some(HalfMatch {
                    pattern: dfa.match_pattern(state, 0),
                    offset: at - MATCH_OFFSET,
                });
                if earliest {
                    return Ok(last_match);
                }
                if dfa.is_accel_state(state) {
                    let needles = dfa.accelerator(state);
                    at = accel::find_fwd(needles, bytes, at).unwrap_or(bytes.len());
                }
            } else if dfa.is_accel_state(state) {
                let needs = dfa.accelerator(state);
                at = accel::find_fwd(needs, bytes, at).unwrap_or(bytes.len());
            } else if dfa.is_dead_state(state) {
                return Ok(last_match);
            } else {
                debug_assert!(dfa.is_quit_state(state));
                if last_match.is_some() {
                    return Ok(last_match);
                }
                return Err(MatchError::Quit {
                    byte,
                    offset: at - 1,
                });
            }
        }
        while at < end && dfa.next_state(state, bytes[at]) == state {
            at += 1;
        }
    }
    Ok(eoi_fwd(dfa, haystack, end, &mut state)?.or(last_match))
}

#[inline(never)]
pub fn find_earliest_rev<A: Automaton + ?Sized>(
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<Option<HalfMatch>, MatchError> {
    find_rev(true, dfa, pattern_id, bytes, start, end)
}

#[inline(never)]
pub fn find_leftmost_rev<A: Automaton + ?Sized>(
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<Option<HalfMatch>, MatchError> {
    find_rev(false, dfa, pattern_id, bytes, start, end)
}

#[inline(always)]
fn find_rev<A: Automaton + ?Sized>(
    earliest: bool,
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<Option<HalfMatch>, MatchError> {
    assert!(start <= end);
    assert!(start <= bytes.len());
    assert!(end <= bytes.len());

    let mut state = init_rev(dfa, pattern_id, bytes, start, end)?;
    let mut last_match = None;
    let mut at = end;
    while at > start {
        at -= 1;
        while at > start && dfa.next_state(state, bytes[at]) == state {
            at -= 1;
        }

        let byte = bytes[at];
        state = dfa.next_state(state, byte);
        if dfa.is_special_state(state) {
            if dfa.is_start_state(state) {
                if dfa.is_accel_state(state) {
                    let needles = dfa.accelerator(state);
                    at = accel::find_rev(needles, bytes, at)
                        .map(|i| i + 1)
                        .unwrap_or(0);
                }
            } else if dfa.is_match_state(state) {
                last_match = Some(HalfMatch {
                    pattern: dfa.match_pattern(state, 0),
                    offset: at + MATCH_OFFSET,
                });
                if earliest {
                    return Ok(last_match);
                }
                if dfa.is_accel_state(state) {
                    let needles = dfa.accelerator(state);
                    at = accel::find_rev(needles, bytes, at)
                        .map(|i| i + 1)
                        .unwrap_or(0);
                }
            } else if dfa.is_accel_state(state) {
                let needles = dfa.accelerator(state);
                at = accel::find_rev(needles, bytes, at)
                    .map(|i| i + 1)
                    .unwrap_or(0);
            } else if dfa.is_dead_state(state) {
                return Ok(last_match);
            } else {
                debug_assert!(dfa.is_quit_state(state));
                if last_match.is_some() {
                    return Ok(last_match);
                }
                return Err(MatchError::Quit { byte, offset: at });
            }
        }
    }
    Ok(eoi_rev(dfa, bytes, start, state)?.or(last_match))
}

#[inline(never)]
pub fn find_overlapping_fwd<A: Automaton + ?Sized>(
    pre: Option<&mut prefilter::Scanner>,
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
    caller_state: &mut OverlappingState,
) -> Result<Option<HalfMatch>, MatchError> {
    // Searching with a pattern ID is always anchored, so we should only ever
    // use a prefilter when no pattern ID is given.
    if pre.is_some() && pattern_id.is_none() {
        find_overlapping_fwd_imp(pre, dfa, pattern_id, bytes, start, end, caller_state)
    } else {
        find_overlapping_fwd_imp(None, dfa, pattern_id, bytes, start, end, caller_state)
    }
}

#[inline(always)]
fn find_overlapping_fwd_imp<A: Automaton + ?Sized>(
    mut pre: Option<&mut prefilter::Scanner>,
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    mut start: usize,
    end: usize,
    caller_state: &mut OverlappingState,
) -> Result<Option<HalfMatch>, MatchError> {
    assert!(start <= end);
    assert!(start <= bytes.len());
    assert!(end <= bytes.len());

    let mut state = match caller_state.id() {
        None => init_fwd(dfa, pattern_id, bytes, start, end)?,
        Some(id) => {
            if let Some(last) = caller_state.last_match() {
                let match_count = dfa.match_count(id);
                if last.match_index < match_count {
                    let m = HalfMatch {
                        pattern: dfa.match_pattern(id, last.match_index),
                        offset: last.offset,
                    };
                    last.match_index += 1;
                    return Ok(Some(m));
                }
            }

            // This is a subtle but critical detail. If the caller provides a
            // non-None state ID, then it must be the case that the state ID
            // corresponds to one set by this function. The state ID therefore
            // corresponds to a match state, a dead state or some other state.
            // However, "some other" state _only_ occurs when the input has
            // been exhausted because the only way to stop before then is to
            // see a match or a dead/quit state.
            //
            // If the input is exhausted or if it's a dead state, then
            // incrementing the starting position has no relevance on
            // correctness, since the loop below will either not execute
            // at all or will immediately stop due to being in a dead state.
            // (Once in a dead state it is impossible to leave it.)
            //
            // Therefore, the only case we need to consider is when
            // caller_state is a match state. In this case, since our machines
            // support the ability to delay a match by a certain number of
            // bytes (to support look-around), it follows that we actually
            // consumed that many additional bytes on our previous search. When
            // the caller resumes their search to find subsequent matches, they
            // will use the ending location from the previous match as the next
            // starting point, which is `match_offset` bytes PRIOR to where
            // we scanned to on the previous search. Therefore, we need to
            // compensate by bumping `start` up by `MATCH_OFFSET` bytes.
            //
            // Incidentally, since MATCH_OFFSET is non-zero, this also makes
            // dealing with empty matches convenient. Namely, callers needn't
            // special case them when implementing an iterator. Instead, this
            // ensures that forward progress is always made.
            start += MATCH_OFFSET;
            id
        }
    };

    let mut at = start;
    while at < end {
        let byte = bytes[at];
        state = dfa.next_state(state, byte);
        at += 1;
        if dfa.is_special_state(state) {
            caller_state.set_id(state);
            if dfa.is_start_state(state) {
                if let Some(ref mut pre) = pre {
                    if pre.is_effective(at) {
                        match pre.next_candidate(bytes, at).into_option() {
                            None => return Ok(None),
                            Some(i) => {
                                at = i;
                            }
                        }
                    }
                } else if dfa.is_accel_state(state) {
                    let needles = dfa.accelerator(state);
                    at = accel::find_fwd(needles, bytes, at).unwrap_or(bytes.len());
                }
            } else if dfa.is_match_state(state) {
                let offset = at - MATCH_OFFSET;
                caller_state.set_last_match(StateMatch {
                    match_index: 1,
                    offset,
                });
                return Ok(Some(HalfMatch {
                    pattern: dfa.match_pattern(state, 0),
                    offset,
                }));
            } else if dfa.is_accel_state(state) {
                let needs = dfa.accelerator(state);
                at = accel::find_fwd(needs, bytes, at).unwrap_or(bytes.len());
            } else if dfa.is_dead_state(state) {
                return Ok(None);
            } else {
                debug_assert!(dfa.is_quit_state(state));
                return Err(MatchError::Quit {
                    byte,
                    offset: at - 1,
                });
            }
        }
    }

    let result = eoi_fwd(dfa, bytes, end, &mut state);
    caller_state.set_id(state);
    if let Ok(Some(ref last_match)) = result {
        caller_state.set_last_match(StateMatch {
            match_index: 1,
            offset: last_match.offset(),
        });
    }
    result
}

fn init_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<StateID, MatchError> {
    let state = dfa.start_state_forward(pattern_id, bytes, start, end);
    // Start states can never be match states, since all matches are delayed
    // by 1 byte.
    assert!(!dfa.is_match_state(state));
    Ok(state)
}

fn init_rev<A: Automaton + ?Sized>(
    dfa: &A,
    pattern_id: Option<PatternID>,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<StateID, MatchError> {
    let state = dfa.start_state_reverse(pattern_id, bytes, start, end);
    // Start states can never be match states, since all matches are delayed
    // by 1 byte.
    assert!(!dfa.is_match_state(state));
    Ok(state)
}

fn eoi_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    end: usize,
    state: &mut StateID,
) -> Result<Option<HalfMatch>, MatchError> {
    match bytes.get(end) {
        Some(&b) => {
            *state = dfa.next_state(*state, b);
            if dfa.is_match_state(*state) {
                Ok(Some(HalfMatch {
                    pattern: dfa.match_pattern(*state, 0),
                    offset: end,
                }))
            } else {
                Ok(None)
            }
        }
        None => {
            *state = dfa.next_eoi_state(*state);
            if dfa.is_match_state(*state) {
                Ok(Some(HalfMatch {
                    pattern: dfa.match_pattern(*state, 0),
                    offset: bytes.len(),
                }))
            } else {
                Ok(None)
            }
        }
    }
}

fn eoi_rev<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    start: usize,
    state: StateID,
) -> Result<Option<HalfMatch>, MatchError> {
    if start > 0 {
        let state = dfa.next_state(state, bytes[start - 1]);
        if dfa.is_match_state(state) {
            Ok(Some(HalfMatch {
                pattern: dfa.match_pattern(state, 0),
                offset: start,
            }))
        } else {
            Ok(None)
        }
    } else {
        let state = dfa.next_eoi_state(state);
        if dfa.is_match_state(state) {
            Ok(Some(HalfMatch {
                pattern: dfa.match_pattern(state, 0),
                offset: 0,
            }))
        } else {
            Ok(None)
        }
    }
}

// Currently unused, but is useful to keep around. This was originally used
// when the code above used raw pointers for its main loop.
// // // // fn offset(bytes: &[u8], p: *const u8) -> usize {
// debug_assert!(bytes.as_ptr() <= p);
// debug_assert!(bytes[bytes.len()..].as_ptr() >= p);
// ((p as isize) - (bytes.as_ptr() as isize)) as usize
// }
