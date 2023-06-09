// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_provider::prelude::*;

pub use crate::internals::CaseMappingInternals;
pub use crate::internals::CaseMappingUnfoldData;

#[allow(missing_docs)] // TBD, temporary
pub mod data;

#[allow(missing_docs)] // TBD, temporary
pub mod exception_header;

#[allow(missing_docs)] // TBD, temporary
pub mod exceptions;

#[icu_provider::data_struct(CaseMappingV1Marker = "props/casemap@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_casemapping::provider),
)]
#[yoke(prove_covariance_manually)]
/// CaseMapping provides low-level access to the data necessary to
/// convert characters and strings to upper, lower, or title case.
pub struct CaseMappingV1<'data> {
    /// Case mapping data
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub casemap: CaseMappingInternals<'data>,
}
