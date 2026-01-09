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

use icu_pattern::SinglePlaceholderPattern;
use icu_plurals::provider::PluralElementsPackedULE;
use icu_provider::prelude::*;
use zerovec::ule::encode_varule_to_box;
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
    CompactDecimalPatternData<'static>,
);
icu_provider::data_marker!(
    /// `ShortCompactDecimalFormatDataV1`
    ShortCompactDecimalFormatDataV1,
    CompactDecimalPatternData<'static>,
);

/// Compact Decimal Pattern data struct.
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
/// of ten is omitted; that is, there is an implicit (0, other) ↦ 0.
///
/// The plural patterns are stored with the 4-bit metadata representing the exponent
/// shift (number of zeros in the pattern minus 1).
#[derive(Debug, Clone, Default, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_experimental::compactdecimal::provider))]
#[yoke(prove_covariance_manually)]
pub struct CompactDecimalPatternData<'data> {
    /// A map keyed on log10 of the CLDR `type` attribute.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns:
        VarZeroVec<'data, VarTupleULE<u8, PluralElementsPackedULE<SinglePlaceholderPattern>>>,
}

impl CompactDecimalPatternData<'_> {
    /// The pattern `0`, which is used for low magnitudes and omitted from the data struct.
    // Safety: the integrity of the VarULE is enforced in validate_plural_pattern_0_map
    pub const PLURAL_PATTERN_0: &'static PluralElementsPackedULE<SinglePlaceholderPattern> =
        unsafe { PluralElementsPackedULE::from_bytes_unchecked(&[0, 1]) };

    pub(crate) fn patterns_and_exponent_for_magnitude(
        &self,
        magnitude: i16,
    ) -> (&PluralElementsPackedULE<SinglePlaceholderPattern>, u8) {
        self.patterns
            .iter()
            .filter(|t| i16::from(t.sized) <= magnitude)
            .last()
            .map(|t| (&t.variable, t.sized - t.variable.get_default().0.get()))
            .unwrap_or((Self::PLURAL_PATTERN_0, 0))
    }
}

#[test]
fn validate_plural_pattern_0_map() {
    use icu_plurals::{provider::FourBitMetadata, PluralElements};

    assert_eq!(
        CompactDecimalPatternData::PLURAL_PATTERN_0,
        &*encode_varule_to_box(&PluralElements::new((
            FourBitMetadata::try_from_byte(0).unwrap(),
            SinglePlaceholderPattern::try_from_str("{0}", Default::default())
                .unwrap()
                .as_ref()
        )))
    );
}

icu_provider::data_struct!(CompactDecimalPatternData<'_>, #[cfg(feature = "datagen")]);
