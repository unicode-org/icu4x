pub use self::{
    error::{BuildError, CacheError},
    id::{LazyStateID, OverlappingState},
};

pub mod dfa;
mod error;
mod id;
pub mod regex;
mod search;
