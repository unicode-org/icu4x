// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::internals::CaseMappingInternals;
use icu_provider::{yoke, zerofrom};

#[icu_provider::data_struct(CaseMappingV1Marker = "props/casemap@1")]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "serialize", derive(serde::Serialize, serde::Deserialize))]
#[yoke(prove_covariance_manually)]
/// CaseMapping provides low-level access to the data necessary to
/// convert characters and strings to upper, lower, or title case.
pub struct CaseMappingV1<'data> {
    /// Case mapping data
    #[cfg_attr(feature = "serialize", serde(borrow))]
    pub casemap: CaseMappingInternals<'data>,
}
