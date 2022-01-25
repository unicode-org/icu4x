#![allow(warnings)]
// #![deny(missing_docs)]
#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(any(
    target_pointer_width = "16",
    target_pointer_width = "32",
    target_pointer_width = "64"
)))]
compile_error!("regex-automata currently not supported on non-{16,32,64}");

#[cfg(feature = "std")]
extern crate alloc;

#[doc(inline)]
pub use crate::regex_automata::util::id::PatternID;
#[cfg(feature = "std")]
pub use crate::regex_automata::util::syntax::SyntaxConfig;
pub use crate::regex_automata::util::{
    bytes::{DeserializeError, SerializeError},
    matchtypes::{HalfMatch, Match, MatchError, MatchKind, MultiMatch},
};

#[macro_use]
mod macros;

pub mod dfa;
#[cfg(feature = "std")]
pub mod hybrid;
#[cfg(feature = "std")]
pub mod nfa;
pub mod util;
