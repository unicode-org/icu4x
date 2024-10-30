// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! UTS 35 classical datetime skeletons.
//! 
//! Semantic skeletons (field sets) use classical skeletons for pattern matching in datagen.
//! 
//! See the [`Skeleton`](reference::Skeleton) struct for more information.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>

mod error;
mod helpers;
mod plural;
pub mod reference;
pub mod runtime;
#[cfg(feature = "serde")]
mod serde;
pub use error::*;
pub use helpers::*;
pub use plural::*;
