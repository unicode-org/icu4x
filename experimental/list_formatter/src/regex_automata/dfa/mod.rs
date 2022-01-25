pub use crate::regex_automata::dfa::automaton::{Automaton, OverlappingState};
#[cfg(feature = "std")]
pub use crate::regex_automata::dfa::error::Error;

const DEAD: crate::regex_automata::util::id::StateID =
    crate::regex_automata::util::id::StateID::ZERO;

mod accel;
mod automaton;
pub mod dense;
#[cfg(feature = "std")]
mod determinize;
#[cfg(feature = "std")]
pub(crate) mod error;
#[cfg(feature = "std")]
mod minimize;
pub mod regex;
mod search;
pub mod sparse;
mod special;
#[cfg(feature = "transducer")]
mod transducer;
