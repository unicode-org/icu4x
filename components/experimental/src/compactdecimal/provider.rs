// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable.
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]
// Suppress a warning on zerovec::makevarule.
#![allow(missing_docs)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use icu_pattern::{Pattern, PatternBackend, SinglePlaceholder};
use icu_plurals::provider::PluralElementsPackedULE;
use icu_provider::prelude::*;
use zerovec::ule::vartuple::VarTupleULE;
use zerovec::VarZeroVec;

#[cfg(feature = "compiled_data")]
/// Baked data
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. In particular, the `DataProvider` implementations are only
/// guaranteed to match with this version's `*_unstable` providers. Use with caution.
/// </div>
pub use crate::provider::Baked;

icu_provider::data_marker!(
    /// `LongCompactDecimalFormatDataV1`
    LongCompactDecimalFormatDataV1,
    CompactPatterns<'static, SinglePlaceholder>,
);
icu_provider::data_marker!(
    /// `ShortCompactDecimalFormatDataV1`
    ShortCompactDecimalFormatDataV1,
    CompactPatterns<'static, SinglePlaceholder>,
);

/// Compact pattern data struct.
///
/// As in CLDR, this is a mapping from type (a power of ten, corresponding to
/// the magnitude of the number being formatted) to a plural pattern.
///
/// If all plural patterns are compatible across consecutive types, the
/// larger types are omitted, thus given
/// > (3, other) ↦ 0K, (4, other) ↦ 00K, (5, other) ↦ 000K
///
/// only
/// > (3, other) ↦ 0K
///
/// is stored.
///
/// Finally, the pattern indicating noncompact notation for the first few powers
/// of ten might be omitted; that is, there is an implicit (0, other) ↦ 0.
///
/// The plural patterns are stored with the 4-bit metadata representing the exponent
/// shift (number of zeros in the pattern minus 1).
#[derive(Debug, Clone, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::compactdecimal::provider))]
pub struct CompactPatterns<'a, P: PatternBackend>(
    pub VarZeroVec<'a, VarTupleULE<u8, PluralElementsPackedULE<Pattern<P>>>>,
);

impl<P: PatternBackend<Store = str>> Default for CompactPatterns<'_, P> {
    fn default() -> Self {
        Self(VarZeroVec::new())
    }
}

#[cfg(feature = "datagen")]
impl<'data, P: PatternBackend> serde::Serialize for CompactPatterns<'data, P>
where
    Pattern<P>: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
impl<'de, 'data, P: PatternBackend> serde::Deserialize<'de> for CompactPatterns<'data, P>
where
    'de: 'data,
    alloc::boxed::Box<Pattern<P>>: serde::Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        VarZeroVec::<VarTupleULE<u8, PluralElementsPackedULE<Pattern<P>>>>::deserialize(
            deserializer,
        )
        .map(Self)
    }
}

impl<P: PatternBackend> icu_provider::ule::MaybeAsVarULE for CompactPatterns<'_, P> {
    type EncodedStruct = [()];
}

#[cfg(feature = "datagen")]
impl<P: PatternBackend> icu_provider::ule::MaybeEncodeAsVarULE for CompactPatterns<'_, P> {
    type EncodeableStruct<'b>
        = &'b [()]
    where
        Self: 'b;
    fn maybe_as_encodeable<'b>(&'b self) -> Option<Self::EncodeableStruct<'b>> {
        None
    }
}

#[cfg(feature = "datagen")]
impl<P: PatternBackend> CompactPatterns<'static, P> {
    #[allow(clippy::type_complexity)]
    pub fn new(
        patterns: alloc::collections::BTreeMap<
            u8,
            (
                u8,
                icu_plurals::PluralElements<alloc::boxed::Box<Pattern<P>>>,
            ),
        >,
        zero_magnitude: Option<&icu_plurals::PluralElements<&Pattern<P>>>,
    ) -> Result<Self, alloc::string::String> {
        use alloc::boxed::Box;
        use alloc::vec::Vec;
        use icu_plurals::provider::FourBitMetadata;
        use icu_plurals::PluralElements;
        use zerovec::ule::encode_varule_to_box;
        use zerovec::ule::vartuple::VarTuple;
        use zerovec::vecs::VarZeroVecOwned;

        if !patterns
            .values()
            .zip(patterns.values().skip(1))
            .all(|(low, high)| low.0 <= high.0)
        {
            Err(alloc::format!(
                "Compact exponents should be nondecreasing: {:?}",
                patterns
                    .values()
                    .map(|(exponent, _)| exponent)
                    .collect::<Vec<_>>(),
            ))?;
        }

        let mut deduplicated_patterns: Vec<(
            u8,
            PluralElements<(FourBitMetadata, Box<Pattern<P>>)>,
        )> = Vec::new();

        // Deduplicate sequences of types that have the same plural map, keeping the lowest type.
        for (log10_type, (exponent, map)) in patterns
            .into_iter()
            // Skip leading 0 patterns
            .skip_while(|(_, (_, pattern))| Some(&pattern.as_ref().map(|p| &**p)) == zero_magnitude)
        {
            if let Some(prev) = deduplicated_patterns.last() {
                // The higher pattern can never be exactly one of the low pattern, so we can ignore that value
                if prev
                    .1
                    .as_ref()
                    .with_explicit_one_value(None)
                    .map(|(_, p)| p)
                    == map.as_ref()
                {
                    continue;
                }
            }

            // Store the exponent as a difference from the log10_type, i.e. the number of zeros
            // in the pattern, minus 1. No pattern should have more than 16 zeros.
            let Some(metadata) = FourBitMetadata::try_from_byte(log10_type - exponent) else {
                return Err(alloc::format!(
                    "Pattern has too many zeros {}",
                    log10_type - exponent
                ));
            };

            deduplicated_patterns.push((log10_type, map.map(|p| (metadata, p))))
        }

        #[allow(clippy::unwrap_used)] // keyed by u8, so it cannot exceed usize/2
        Ok(Self(
            VarZeroVecOwned::try_from_elements(
                &deduplicated_patterns
                    .into_iter()
                    .map(|(log10_type, plural_map)| {
                        encode_varule_to_box(&VarTuple {
                            sized: log10_type,
                            variable: plural_map,
                        })
                    })
                    .collect::<Vec<Box<VarTupleULE<u8, PluralElementsPackedULE<Pattern<P>>>>>>(),
            )
            .unwrap()
            .into(),
        ))
    }
}
