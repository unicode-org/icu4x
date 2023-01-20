// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use regex_automata::dfa::sparse::DFA;
use regex_automata::dfa::Automaton;
use regex_automata::util::id::StateID;
use writeable::Writeable;

pub trait LazyAutomaton: Automaton {
    // Like Automaton::find_earliest_fwd, but doesn't require a materialized string.
    fn matches_earliest_fwd_lazy<S: Writeable + ?Sized>(&self, haystack: &S) -> bool;
}

impl<T: AsRef<[u8]>> LazyAutomaton for DFA<T> {
    fn matches_earliest_fwd_lazy<S: Writeable + ?Sized>(&self, haystack: &S) -> bool {
        struct DFAStepper<'a> {
            dfa: &'a DFA<&'a [u8]>,
            state: StateID,
        }

        impl core::fmt::Write for DFAStepper<'_> {
            fn write_str(&mut self, s: &str) -> core::fmt::Result {
                for &byte in s.as_bytes() {
                    self.state = self.dfa.next_state(self.state, byte);
                    if self.dfa.is_match_state(self.state) || self.dfa.is_dead_state(self.state) {
                        // We matched or are in a no-match-cycle, return early
                        return Err(core::fmt::Error);
                    }
                }
                Ok(())
            }
        }

        let mut stepper = DFAStepper {
            // If start == 0 the start state does not depend on the actual string, so
            // we can just pass an empty slice.
            state: self.start_state_forward(None, &[], 0, 0),
            dfa: &self.as_ref(),
        };

        if haystack.write_to(&mut stepper).is_ok() {
            stepper.state = self.next_eoi_state(stepper.state);
        }

        self.is_match_state(stepper.state)
    }
}

#[cfg(test)]
#[test]
fn test() {
    use crate::provider::SerdeDFA;
    use alloc::borrow::Cow;

    let matcher = SerdeDFA::new(Cow::Borrowed("11(000)*$")).unwrap();

    for writeable in [1i32, 11, 110, 11000, 211000] {
        assert_eq!(
            matcher
                .deref()
                .find_earliest_fwd(writeable.write_to_string().as_bytes())
                .unwrap()
                .is_some(),
            matcher.deref().matches_earliest_fwd_lazy(&writeable)
        );
    }

    struct ExitEarlyTest;

    impl writeable::Writeable for ExitEarlyTest {
        fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
            sink.write_str("12")?;
            unreachable!()
        }
    }

    assert!(!matcher.deref().matches_earliest_fwd_lazy(&ExitEarlyTest));
}
