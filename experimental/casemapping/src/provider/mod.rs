// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

use icu_provider::prelude::*;

#[cfg(any(feature = "serde", feature = "datagen"))]
use crate::error::Error;
use crate::provider::data::CaseMappingData;
use crate::provider::exceptions::CaseMappingExceptions;
use crate::provider::unfold::CaseMappingUnfoldData;

use icu_collections::codepointtrie::CodePointTrie;
#[cfg(feature = "datagen")]
use icu_collections::codepointtrie::CodePointTrieHeader;

pub mod data;
pub mod exception_helpers;
pub mod exceptions;
#[cfg(feature = "datagen")]
mod exceptions_builder;
pub mod unfold;

#[cfg(feature = "data")]
#[derive(Debug)]
/// Baked data
pub struct Baked;

#[cfg(feature = "data")]
const _: () = {
    use crate as icu_casemapping;
    icu_casemapping_data::impl_props_casemap_v1!(Baked);
};

/// This type contains all of the casemapping data
///
/// The methods in the provider module are primarily about accessing its data,
/// however the full algorithms are also implemented as methods on this type in
/// the `internals` module of this crate.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(CaseMappingV1Marker = "props/casemap@1")]
#[derive(Debug, PartialEq, Clone)]
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
    pub trie: CodePointTrie<'data, CaseMappingData>,
    /// Exceptions to the case mapping data
    pub exceptions: CaseMappingExceptions<'data>,
    /// Reverse case folding data.
    pub unfold: CaseMappingUnfoldData<'data>,
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for CaseMappingV1<'de> {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(serde::Deserialize)]
        pub struct Raw<'data> {
            #[serde(borrow)]
            pub trie: CodePointTrie<'data, CaseMappingData>,
            #[serde(borrow)]
            pub exceptions: CaseMappingExceptions<'data>,
            #[serde(borrow)]
            pub unfold: CaseMappingUnfoldData<'data>,
        }

        let Raw {
            trie,
            exceptions,
            unfold,
        } = Raw::deserialize(deserializer)?;
        let result = Self {
            trie,
            exceptions,
            unfold,
        };
        debug_assert!(result.validate().is_ok());
        Ok(result)
    }
}

impl<'data> CaseMappingV1<'data> {
    /// Creates a new CaseMappingV1 using data exported by the
    // `icuexportdata` tool in ICU4C. Validates that the data is
    // consistent.
    #[cfg(feature = "datagen")]
    pub fn try_from_icu(
        trie_header: CodePointTrieHeader,
        trie_index: &[u16],
        trie_data: &[u16],
        exceptions: &[u16],
        unfold: &[u16],
    ) -> Result<Self, Error> {
        use self::exceptions_builder::CaseMappingExceptionsBuilder;
        use zerovec::ZeroVec;
        let exceptions_builder = CaseMappingExceptionsBuilder::new(exceptions);
        let (exceptions, idx_map) = exceptions_builder.build()?;

        let trie_index = ZeroVec::alloc_from_slice(trie_index);

        #[allow(clippy::unwrap_used)] // datagen only
        let trie_data = trie_data
            .iter()
            .map(|&i| {
                CaseMappingData::try_from_icu_integer(i)
                    .unwrap()
                    .with_updated_exception(&idx_map)
            })
            .collect::<ZeroVec<_>>();

        let trie = CodePointTrie::try_new(trie_header, trie_index, trie_data)?;

        let unfold = CaseMappingUnfoldData::try_from_icu(unfold)?;

        let result = Self {
            trie,
            exceptions,
            unfold,
        };
        result.validate()?;
        Ok(result)
    }

    /// Given an existing CaseMapping, validates that the data is
    /// consistent. A CaseMapping created by the ICU transformer has
    /// already been validated. Calling this function is only
    /// necessary if you are concerned about data corruption after
    /// deserializing.
    #[cfg(any(feature = "serde", feature = "datagen"))]
    #[allow(unused)] // is only used in debug mode for serde
    pub(crate) fn validate(&self) -> Result<(), crate::error::Error> {
        // First, validate that exception data is well-formed.
        let valid_exception_indices = self.exceptions.validate()?;

        let validate_delta = |c: char, delta: i32| -> Result<(), Error> {
            let new_c = u32::try_from(c as i32 + delta)
                .map_err(|_| Error::Validation("Delta larger than character"))?;
            char::from_u32(new_c).ok_or(Error::Validation("Invalid delta"))?;
            Ok(())
        };

        for i in 0..char::MAX as u32 {
            if let Some(c) = char::from_u32(i) {
                let data = self.lookup_data(c);
                if data.has_exception() {
                    let idx = data.exception_index();
                    let exception = self.exceptions.get(idx);
                    // Verify that the exception index points to a valid exception header.
                    if !valid_exception_indices.contains(&idx) {
                        return Error::invalid("Invalid exception index in trie data");
                    }
                    exception.validate()?;
                } else {
                    validate_delta(c, data.delta() as i32)?;
                }
            }
        }

        // The unfold data is structurally guaranteed to be valid,
        // so there is nothing left to check.
        Ok(())
    }

    pub(crate) fn lookup_data(&self, c: char) -> CaseMappingData {
        self.trie.get32(c as u32)
    }
}
