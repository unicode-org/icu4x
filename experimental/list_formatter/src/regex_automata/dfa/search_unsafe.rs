use crate::regex_automata::dfa::automaton::{Automaton, State};
use crate::regex_automata::MatchError;

#[inline(always)]
pub fn find_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    start: usize,
    end: usize,
    earliest: bool,
) -> Result<Option<usize>, MatchError> {
    assert!(start <= end);
    assert!(start <= bytes.len());
    assert!(end <= bytes.len());

    let (mut state, mut last_match) = init_fwd(dfa, bytes, start, end)?;
    if earliest && last_match.is_some() {
        return Ok(last_match);
    }

    let mut at = start;
    while at < end {
        let byte = bytes[at];
        state = dfa.next_state(state, byte);
        at += 1;
        if dfa.is_special_state(state) {
            if dfa.is_dead_state(state) {
                return Ok(last_match);
            } else if dfa.is_quit_state(state) {
                return Err(MatchError::Quit { byte, offset: at - 1 });
            }
            last_match = Some(at - dfa.match_offset());
            if earliest {
                return Ok(last_match);
            }
        }
    }
    /*
    unsafe {
        let mut p = bytes.as_ptr().add(start);
        while p < bytes[end..].as_ptr() {
            let byte = *p;
            state = dfa.next_state_unchecked(state, byte);
            p = p.add(1);
            if dfa.is_special_state(state) {
                if dfa.is_dead_state(state) {
                    return Ok(last_match);
                } else if dfa.is_quit_state(state) {
                    return Err(MatchError::Quit {
                        byte,
                        offset: offset(bytes, p) - 1,
                    });
                }
                last_match = Some(offset(bytes, p) - dfa.match_offset());
                if earliest {
                    return Ok(last_match);
                }
            }
        }
    }
    */
    Ok(eof_fwd(dfa, bytes, end, &mut state)?.or(last_match))
}

#[inline(always)]
pub fn find_rev<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    start: usize,
    end: usize,
    earliest: bool,
) -> Result<Option<usize>, MatchError> {
    assert!(start <= end);
    assert!(start <= bytes.len());
    assert!(end <= bytes.len());

    let (mut state, mut last_match) = init_rev(dfa, bytes, start, end)?;
    if earliest && last_match.is_some() {
        return Ok(last_match);
    }

    let mut at = end;
    while at > start {
        at -= 1;
        let byte = bytes[at];
        state = dfa.next_state(state, byte);
        if dfa.is_special_state(state) {
            if dfa.is_dead_state(state) {
                return Ok(last_match);
            } else if dfa.is_quit_state(state) {
                return Err(MatchError::Quit { byte, offset: at });
            }
            last_match = Some(at + dfa.match_offset());
            if earliest {
                return Ok(last_match);
            }
        }
    }
    /*
    unsafe {
        let mut p = bytes.as_ptr().add(end);
        while p > bytes[start..].as_ptr() {
            p = p.sub(1);
            let byte = *p;
            state = dfa.next_state_unchecked(state, byte);
            if dfa.is_special_state(state) {
                if dfa.is_dead_state(state) {
                    return Ok(last_match);
                } else if dfa.is_quit_state(state) {
                    return Err(MatchError::Quit {
                        byte,
                        offset: offset(bytes, p),
                    });
                }
                last_match = Some(offset(bytes, p) + dfa.match_offset());
                if earliest {
                    return Ok(last_match);
                }
            }
        }
    }
    */
    Ok(eof_rev(dfa, state, bytes, start)?.or(last_match))
}

pub fn find_overlapping_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    mut start: usize,
    end: usize,
    caller_state: &mut State<A::ID>,
) -> Result<Option<usize>, MatchError> {
    assert!(start <= end);
    assert!(start <= bytes.len());
    assert!(end <= bytes.len());

    let (mut state, mut last_match) = match caller_state.as_option() {
        None => init_fwd(dfa, bytes, start, end)?,
        Some(id) => {
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
            // compensate by bumping `start` up by `match_offset` bytes.
            start += dfa.match_offset();
            // Since match_offset could be any arbitrary value and we use
            // `start` in pointer arithmetic below, we check that we are still
            // in bounds. Otherwise, we could materialize a pointer that is
            // more than one past the end point of `bytes`, which is UB.
            if start > end {
                return Ok(None);
            }
            (id, None)
        }
    };
    if last_match.is_some() {
        caller_state.set(state);
        return Ok(last_match);
    }

    let mut at = start;
    while at < end {
        let byte = bytes[at];
        state = dfa.next_state(state, byte);
        at += 1;
        if dfa.is_special_state(state) {
            caller_state.set(state);
            if dfa.is_dead_state(state) {
                return Ok(None);
            } else if dfa.is_quit_state(state) {
                return Err(MatchError::Quit { byte, offset: at - 1 });
            } else {
                return Ok(Some(at - dfa.match_offset()));
            }
        }
    }
    /*
    // SAFETY: Other than the normal pointer arithmetic happening here, a
    // unique aspect of safety for this function is the fact that the caller
    // can provide the state that the search routine will start with. If this
    // state were invalid, it would be possible to incorrectly index the
    // transition table. We however prevent this from happening by guaranteeing
    // that State is valid. Namely, callers cannot mutate a State. All they can
    // do is create a "start" state or otherwise reuse a previously set state.
    // Since callers can't mutate a state, it follows that a previously set
    // state can only be retrieved by crate internal functions. Therefore, our
    // use of it is safe since this code will only ever set the provided state
    // to a valid state.
    unsafe {
        let mut p = bytes.as_ptr().add(start);
        while p < bytes[end..].as_ptr() {
            let byte = *p;
            state = dfa.next_state_unchecked(state, byte);
            p = p.add(1);
            if dfa.is_special_state(state) {
                caller_state.set(state);
                return if dfa.is_dead_state(state) {
                    Ok(None)
                } else if dfa.is_quit_state(state) {
                    Err(MatchError::Quit { byte, offset: offset(bytes, p) - 1 })
                } else {
                    Ok(Some(offset(bytes, p) - dfa.match_offset()))
                };
            }
        }
    }
    */

    let result = eof_fwd(dfa, bytes, end, &mut state);
    caller_state.set(state);
    result
}

fn init_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<(A::ID, Option<usize>), MatchError> {
    let state = dfa.start_state_forward(bytes, start, end);
    if dfa.is_match_state(state) {
        Ok((state, Some(start - dfa.match_offset())))
    } else {
        Ok((state, None))
    }
}

fn init_rev<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    start: usize,
    end: usize,
) -> Result<(A::ID, Option<usize>), MatchError> {
    let state = dfa.start_state_reverse(bytes, start, end);
    if dfa.is_match_state(state) {
        Ok((state, Some(end + dfa.match_offset())))
    } else {
        Ok((state, None))
    }
}

fn eof_fwd<A: Automaton + ?Sized>(
    dfa: &A,
    bytes: &[u8],
    end: usize,
    state: &mut A::ID,
) -> Result<Option<usize>, MatchError> {
    match bytes.get(end) {
        Some(&b) => {
            *state = dfa.next_state(*state, b);
            if dfa.is_match_state(*state) {
                Ok(Some(end))
            } else {
                Ok(None)
            }
        }
        None => {
            *state = dfa.next_eof_state(*state);
            if dfa.is_match_state(*state) {
                Ok(Some(bytes.len()))
            } else {
                Ok(None)
            }
        }
    }
}

fn eof_rev<A: Automaton + ?Sized>(
    dfa: &A,
    state: A::ID,
    bytes: &[u8],
    start: usize,
) -> Result<Option<usize>, MatchError> {
    if start > 0 {
        if dfa.is_match_state(dfa.next_state(state, bytes[start - 1])) {
            Ok(Some(start))
        } else {
            Ok(None)
        }
    } else {
        if dfa.is_match_state(dfa.next_eof_state(state)) {
            Ok(Some(0))
        } else {
            Ok(None)
        }
    }
}

fn offset(bytes: &[u8], p: *const u8) -> usize {
    debug_assert!(bytes.as_ptr() <= p);
    debug_assert!(bytes[bytes.len()..].as_ptr() >= p);
    ((p as isize) - (bytes.as_ptr() as isize)) as usize
}
