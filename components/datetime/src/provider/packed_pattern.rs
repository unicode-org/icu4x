// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures for packing of datetime patterns.

use crate::{
    helpers::size_test,
    pattern::{
        runtime::{PatternBorrowed, PatternMetadata},
        PatternItem,
    },
    NeoSkeletonLength,
};
use alloc::vec::Vec;
use constants::Q_BIT;
use icu_plurals::{
    provider::{FourBitMetadata, PluralElementsPackedULE},
    PluralElements,
};
use zerovec::{VarZeroVec, ZeroSlice};

use crate::pattern::runtime::Pattern;

/// A field of [`PackedPatternsBuilder`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthPluralElements<T> {
    /// The "long" length pattern plural elements.
    pub long: PluralElements<T>,
    /// The "medium" length pattern plural elements.
    pub medium: PluralElements<T>,
    /// The "short" length pattern plural elements.
    pub short: PluralElements<T>,
}

/// A builder for a [`PackedPatternsV1`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackedPatternsBuilder<'a> {
    /// Patterns always available.
    pub standard: LengthPluralElements<Pattern<'a>>,
    /// Patterns for variant 0. If `None`, falls back to standard.
    pub variant0: Option<LengthPluralElements<Pattern<'a>>>,
    /// Patterns for variant 1. If `None`, falls back to standard.
    pub variant1: Option<LengthPluralElements<Pattern<'a>>>,
}

size_test!(PackedPatternsV1, packed_skeleton_data_size, 32);

/// Main data struct for packed datetime patterns.
///
/// ## Variants
///
/// This supports a set of "standard" patterns plus up to two "variants".
/// The variants are currently used by year formatting:
///
/// - Standard: Year, which could be partial precision (2-digit Gregorain)
/// - Variant 0: Full Year, which is always full precision
/// - Variant 1: Year With Era
///
/// Variants should be used when the pattern could depend on the value being
/// formatted. For example, with [`EraDisplay::Auto`], any of these three
/// patterns could be selected based on the year value.
///
/// ## Representation
///
/// Currently, there are at most 9 patterns that need to be stored together,
/// named according to this table:
///
/// |        | Standard | Variant 0 | Variant 1 |
/// |--------|----------|-----------|-----------|
/// | Long   | La       | Lb        | Lc        |
/// | Medium | Ma       | Mb        | Mc        |
/// | Short  | Sa       | Sb        | Sc        |
///
/// The header byte encodes which pattern in the patterns array corresponds to
/// a particular cell in the table. It contains the following information:
///
/// - Bits 0-1: "LMS" value of the standard column
/// - Bit 2: "Q" value: 1 for directly-indexed variants; 0 for per-cell offsets
/// - Bits 3-20: Packed offset into patterns table for each variant cell
/// - Bits 21-31: unused/reserved
///
/// The LMS value determines which pattern index is used for the first column:
///
/// | LMS Value   | Long Index | Medium Index | Short Index |
/// |-------------|------------|--------------|-------------|
/// | 0 (L=M=S)   | 0          | 0            | 0           |
/// | 1 (L, M=S)  | 0          | 1            | 1           |
/// | 2 (L=M, S)  | 0          | 0            | 1           |
/// | 3 (L, M, S) | 0          | 1            | 2           |
///
/// If bit 2 is 1 (Q=1), it means there is one pattern per table cell,
/// with the index offset by the short index `S` from the table above.
/// However, this requires storing multiple, possibly duplicate, patterns in
/// the packed structure. The more common case is Q=0 and then to store
/// per-cell offsets in chunks of 3 bits per cell:
///
/// - Chunk = 0: Inherit according to the table below
/// - Chunk = 1-7: Use pattern index Chunk - 1
///
/// This is summarized below:
///
/// | Cell in Table | Q=1 Pattern Index | Q=0 Header Bits | Inheritance |
/// |---------------|-------------------|-----------------|-------------|
/// | Lb            | S + 1             | 3-5             | La          |
/// | Mb            | S + 2             | 6-8             | Ma          |
/// | Sb            | S + 3             | 9-11            | Sa          |
/// | Lc            | S + 4             | 12-14           | La          |
/// | Mc            | S + 5             | 15-17           | Ma          |
/// | Sc            | S + 6             | 18-20           | Sa          |
///
/// As a result, if there are no variants, bits 2 and higher will be all zero,
/// making the header int suitable for varint packing, such as that used by
/// postcard and other size-optimized serialization formats.
///
/// [`EraDisplay::Auto`]: crate::neo_skeleton::EraDisplay::Auto
#[doc = packed_skeleton_data_size!()]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackedPatternsV1<'data> {
    /// An encoding of which standard/variant cell corresponds to which entry
    /// in the patterns table. See class docs.
    pub header: u32,
    /// The list of patterns. Length should be between 1 and 9,
    /// depending on the header.
    pub elements: VarZeroVec<'data, PluralElementsPackedULE<ZeroSlice<PatternItem>>>,
}

mod constants {
    /// Value when standard long, medium, and short are all the same
    pub(super) const LMS: u32 = 0;
    /// Value when standard medium is the same as short but not long
    pub(super) const L_MS: u32 = 1;
    /// Value when standard medium is the same as long but not short
    pub(super) const LM_S: u32 = 2;
    /// Bit that indicates that standard medium differs from standard long
    pub(super) const M_DIFFERS: u32 = 0x1;
    /// Bit that indicates that standard short differs from standard medium
    pub(super) const S_DIFFERS: u32 = 0x2;
    /// Bitmask over all LMS values
    pub(super) const LMS_MASK: u32 = 0x3;
    /// Bit that indicates whether there are per-cell chunks
    pub(super) const Q_BIT: u32 = 0x4;
    /// A mask applied to individual chunks (the largest possible chunk)
    pub(super) const CHUNK_MASK: u32 = 0x7;
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[derive(Default)]
struct UnpackedPatterns<'a> {
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "core::ops::Not::not")
    )]
    pub(super) has_explicit_medium: bool,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "core::ops::Not::not")
    )]
    pub(super) has_explicit_short: bool,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "core::ops::Not::not")
    )]
    pub(super) has_one_pattern_per_variant: bool,
    #[cfg_attr(
        feature = "serde",
        serde(default, skip_serializing_if = "Option::is_none")
    )]
    pub(super) variant_pattern_indices: Option<[u32; 6]>,
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub(super) elements: Vec<PluralElementsWrap<'a>>,
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(transparent))]
struct PluralElementsWrap<'data>(
    #[cfg_attr(feature = "serde", serde(with = "plural_elements_serde"))]
    #[cfg_attr(feature = "serde", serde(borrow))]
    PluralElements<Pattern<'data>>,
);

impl<'data> From<&'data PluralElements<Pattern<'_>>> for PluralElementsWrap<'data> {
    fn from(value: &'data PluralElements<Pattern<'_>>) -> Self {
        Self(
            value
                .as_ref()
                .map(|pattern| pattern.as_borrowed().as_pattern()),
        )
    }
}

#[derive(Debug)]
struct UnpackedPatternsConsistencyError;

impl<'a> UnpackedPatterns<'a> {
    pub(super) fn build(
        &self,
    ) -> Result<PackedPatternsV1<'static>, UnpackedPatternsConsistencyError> {
        let mut header = 0u32;
        if self.has_explicit_medium {
            header |= constants::M_DIFFERS;
        }
        if self.has_explicit_short {
            header |= constants::S_DIFFERS;
        }
        match (
            self.has_one_pattern_per_variant,
            self.variant_pattern_indices,
        ) {
            (true, None) => {
                header |= constants::Q_BIT;
            }
            (false, Some(chunks)) => {
                let mut shift = 3;
                for chunk in chunks.iter() {
                    if *chunk > constants::CHUNK_MASK {
                        return Err(UnpackedPatternsConsistencyError);
                    }
                    header |= chunk << shift;
                    shift += 3;
                }
            }
            _ => return Err(UnpackedPatternsConsistencyError),
        }
        let elements: Vec<PluralElements<(FourBitMetadata, &ZeroSlice<PatternItem>)>> = self
            .elements
            .iter()
            .map(|plural_elements| {
                plural_elements.0.as_ref().map(|pattern| {
                    (
                        pattern.metadata.to_four_bit_metadata(),
                        pattern.items.as_slice(),
                    )
                })
            })
            .collect();
        Ok(PackedPatternsV1 {
            header,
            elements: elements.as_slice().into(),
        })
    }
}

impl<'data> From<&'data PackedPatternsV1<'_>> for UnpackedPatterns<'data> {
    fn from(packed: &'data PackedPatternsV1<'_>) -> Self {
        let mut unpacked = Self::default();
        unpacked.has_explicit_medium = (packed.header & constants::M_DIFFERS) != 0;
        unpacked.has_explicit_short = (packed.header & constants::S_DIFFERS) != 0;
        unpacked.has_one_pattern_per_variant = (packed.header & Q_BIT) != 0;
        if !unpacked.has_one_pattern_per_variant {
            unpacked.variant_pattern_indices = Some([
                (packed.header >> 3) & constants::CHUNK_MASK,
                (packed.header >> 6) & constants::CHUNK_MASK,
                (packed.header >> 9) & constants::CHUNK_MASK,
                (packed.header >> 12) & constants::CHUNK_MASK,
                (packed.header >> 15) & constants::CHUNK_MASK,
                (packed.header >> 18) & constants::CHUNK_MASK,
            ]);
        }
        unpacked.elements = packed
            .elements
            .iter()
            .map(|plural_elements| {
                PluralElementsWrap(plural_elements.decode().map(|(metadata, items)| {
                    PatternBorrowed {
                        metadata: PatternMetadata::from_u8(metadata.get()),
                        items,
                    }
                    .as_pattern()
                }))
            })
            .collect();
        unpacked
    }
}

impl PackedPatternsBuilder<'_> {
    /// Builds a packed pattern representation from the builder.
    pub fn build(mut self) -> PackedPatternsV1<'static> {
        self.simplify();

        // Initialize the elements vector with the standard patterns.
        let mut unpacked = UnpackedPatterns::default();
        unpacked.elements.push((&self.standard.long).into());
        let mut s_offset = 0;
        if self.standard.medium != self.standard.long {
            unpacked.elements.push((&self.standard.medium).into());
            unpacked.has_explicit_medium = true;
            s_offset += 1;
        }
        if self.standard.short != self.standard.medium {
            unpacked.elements.push((&self.standard.short).into());
            unpacked.has_explicit_short = true;
            s_offset += 1;
        }

        // Fill in the variant patterns
        let variant_patterns = [
            self.variant0.as_ref().map(|v| &v.long),
            self.variant0.as_ref().map(|v| &v.medium),
            self.variant0.as_ref().map(|v| &v.short),
            self.variant1.as_ref().map(|v| &v.long),
            self.variant1.as_ref().map(|v| &v.medium),
            self.variant1.as_ref().map(|v| &v.short),
        ];
        let fallbacks = [
            &self.standard.long,
            &self.standard.medium,
            &self.standard.short,
            &self.standard.long,
            &self.standard.medium,
            &self.standard.short,
        ];
        let mut chunks = [0u32; 6]; // per-cell chunk values
        for ((pattern, fallback), chunk) in variant_patterns
            .iter()
            .zip(fallbacks.iter())
            .zip(chunks.iter_mut())
        {
            if let Some(pattern) = pattern {
                if pattern != fallback {
                    *chunk = match unpacked.elements.iter().position(|p| &p.0 == *pattern) {
                        Some(i) => i as u32 + 1,
                        None => {
                            unpacked.elements.push((*pattern).into());
                            unpacked.elements.len() as u32
                        }
                    }
                }
            }
        }

        // Check to see if we need to switch to Q=1 mode. We need to do this
        // if any of the calculated chunk values is too big (larger than 7).
        //
        // Calculate the max chunk using Iterator::max, which won't fail
        // because the array is length 6 (nonempty), but we need to unwrap:
        // <https://github.com/rust-lang/rust/issues/78504>
        #[allow(clippy::unwrap_used)]
        if *chunks.iter().max().unwrap() > constants::CHUNK_MASK {
            // one pattern per table cell
            unpacked.has_one_pattern_per_variant = true;
            unpacked.elements.truncate(s_offset + 1);
            unpacked.elements.extend(
                variant_patterns
                    .into_iter()
                    .zip(fallbacks.iter())
                    .map(|(pattern, fallback)| pattern.unwrap_or(fallback))
                    .map(Into::into),
            );
        } else {
            // per-cell offsets
            unpacked.variant_pattern_indices = Some(chunks);
        }

        // Now we can build the data representation
        unpacked.build().unwrap()
    }

    fn simplify(&mut self) {
        if self.variant0.as_ref() == Some(&self.standard) {
            self.variant0 = None;
        }
        if self.variant1.as_ref() == Some(&self.standard) {
            self.variant1 = None;
        }
    }
}

pub(crate) enum PackedSkeletonVariant {
    Standard,
    Variant0,
    Variant1,
}

impl PackedPatternsV1<'_> {
    pub(crate) fn get(
        &self,
        length: NeoSkeletonLength,
        variant: PackedSkeletonVariant,
    ) -> PatternBorrowed {
        use NeoSkeletonLength::*;
        use PackedSkeletonVariant::*;
        let lms = self.header & constants::LMS_MASK;
        let pattern_index = if matches!(variant, Standard) {
            // Standard pattern (first column)
            match (length, lms) {
                (Long, _) => 0,
                (Medium, constants::LMS | constants::LM_S) => 0,
                (Medium, _) => 1,
                (Short, constants::LMS) => 0,
                (Short, constants::L_MS | constants::LM_S) => 1,
                (Short, _) => 2,
            }
        } else {
            let s_offset = match lms {
                constants::LMS => 0,
                constants::L_MS | constants::LM_S => 1,
                _ => 2,
            };
            let q = self.header & constants::Q_BIT;
            if q == 0 {
                // per-cell offsets
                let chunk_in_low_bits = match (length, variant) {
                    (Long, Variant0) => self.header >> 3,
                    (Medium, Variant0) => self.header >> 6,
                    (Short, Variant0) => self.header >> 9,
                    (Long, Variant1) => self.header >> 12,
                    (Medium, Variant1) => self.header >> 15,
                    (Short, Variant1) => self.header >> 18,
                    (_, Standard) => {
                        debug_assert!(false, "unreachable");
                        return PatternBorrowed::DEFAULT;
                    }
                };
                let chunk = chunk_in_low_bits & constants::CHUNK_MASK;
                if chunk == 0 {
                    // Fall back to standard with the same length
                    return self.get(length, Standard);
                }
                chunk - 1
            } else {
                // one pattern per table cell
                let additional_offset = match (length, variant) {
                    (Long, Variant0) => 1,
                    (Medium, Variant0) => 2,
                    (Short, Variant0) => 3,
                    (Long, Variant1) => 4,
                    (Medium, Variant1) => 5,
                    (Short, Variant1) => 6,
                    (_, Standard) => {
                        debug_assert!(false, "unreachable");
                        return PatternBorrowed::DEFAULT;
                    }
                };
                s_offset + additional_offset
            }
        };
        let Some(plural_elements) = self.elements.get(pattern_index as usize) else {
            debug_assert!(false, "unreachable");
            return PatternBorrowed::DEFAULT;
        };
        let (metadata, items) = plural_elements.get_default();
        PatternBorrowed {
            metadata: PatternMetadata::from_u8(metadata.get()),
            items,
        }
    }

    fn get_as_plural_elements(
        &self,
        length: NeoSkeletonLength,
        variant: PackedSkeletonVariant,
    ) -> PluralElements<Pattern> {
        PluralElements::new(self.get(length, variant).as_pattern())
    }

    /// Converts this packed data to a builder that can be mutated.
    pub fn to_builder(&self) -> PackedPatternsBuilder {
        use NeoSkeletonLength::*;
        use PackedSkeletonVariant::*;
        let mut builder = PackedPatternsBuilder {
            standard: LengthPluralElements {
                long: self.get_as_plural_elements(Long, Standard),
                medium: self.get_as_plural_elements(Medium, Standard),
                short: self.get_as_plural_elements(Short, Standard),
            },
            variant0: Some(LengthPluralElements {
                long: self.get_as_plural_elements(Long, Variant0),
                medium: self.get_as_plural_elements(Medium, Variant0),
                short: self.get_as_plural_elements(Short, Variant0),
            }),
            variant1: Some(LengthPluralElements {
                long: self.get_as_plural_elements(Long, Variant1),
                medium: self.get_as_plural_elements(Medium, Variant1),
                short: self.get_as_plural_elements(Short, Variant1),
            }),
        };
        builder.simplify();
        builder
    }
}

#[cfg(feature = "serde")]
mod plural_elements_serde {
    //! Currently this always serializes the plural elements as a single pattern.
    //! When multi-plural patterns are added back, this code will need to change.

    use super::*;
    use crate::pattern::reference;
    use serde::{ser::Error as _, Deserialize, Deserializer, Serialize, Serializer};

    pub(super) fn deserialize<'data, 'de, D>(
        deserializer: D,
    ) -> Result<PluralElements<Pattern<'data>>, D::Error>
    where
        'de: 'data,
        D: Deserializer<'de>,
    {
        debug_assert!(deserializer.is_human_readable());
        let reference_pattern = reference::Pattern::deserialize(deserializer)?;
        Ok(PluralElements::new(reference_pattern.to_runtime_pattern()))
    }

    pub(super) fn serialize<S>(
        elements: &PluralElements<Pattern<'_>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        debug_assert!(serializer.is_human_readable());
        let pattern = elements
            .as_ref()
            .try_into_other()
            .ok_or_else(|| S::Error::custom("cannot yet serialize multi-plural patterns"))?;
        let reference_pattern = reference::Pattern::from(pattern);
        reference_pattern.serialize(serializer)
    }
}

#[cfg(feature = "serde")]
mod _serde {
    use super::*;
    use crate::pattern::reference;
    use serde::de::Error;
    use zerovec::VarZeroSlice;

    #[cfg(feature = "serde")]
    #[derive(serde::Deserialize)]
    #[cfg_attr(feature = "datagen", derive(serde::Serialize))]
    struct PackedPatternsMachine<'data> {
        pub header: u32,
        #[serde(borrow)]
        pub elements: &'data VarZeroSlice<PluralElementsPackedULE<ZeroSlice<PatternItem>>>,
    }

    #[cfg(feature = "serde")]
    #[derive(serde::Deserialize)]
    #[cfg_attr(feature = "datagen", derive(serde::Serialize))]
    struct PackedPatternsHuman {
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "core::ops::Not::not"))]
        pub has_explicit_medium: bool,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "core::ops::Not::not"))]
        pub has_explicit_short: bool,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "core::ops::Not::not"))]
        pub has_one_pattern_per_variant: bool,
        #[cfg_attr(feature = "serde", serde(skip_serializing_if = "Vec::is_empty"))]
        pub variant_pattern_indices: Vec<u32>,
        pub patterns: Vec<reference::Pattern>,
    }

    impl<'de, 'data> serde::Deserialize<'de> for PackedPatternsV1<'data>
    where
        'de: 'data,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                let human = <UnpackedPatterns>::deserialize(deserializer)?;
                human
                    .build()
                    .map_err(|_| D::Error::custom("invalid packed data"))
            } else {
                let machine = <PackedPatternsMachine>::deserialize(deserializer)?;
                Ok(Self {
                    header: machine.header,
                    elements: machine.elements.as_varzerovec(),
                })
            }
        }
    }

    #[cfg(feature = "datagen")]
    impl serde::Serialize for PackedPatternsV1<'_> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            if serializer.is_human_readable() {
                let human = UnpackedPatterns::from(self);
                human.serialize(serializer)
            } else {
                let machine = PackedPatternsMachine {
                    header: self.header,
                    elements: &self.elements,
                };
                machine.serialize(serializer)
            }
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::pattern::reference;

    const PATTERN_STRS: &[&str] = &[
        "M/d/y",
        "HH:mm",
        "MMM d y G",
        "E",
        "E MMM d",
        "dd.MM.yy",
        "h a",
        "hh:mm:ss B",
        "y MMMM",
    ];

    fn get_patterns() -> Vec<Pattern<'static>> {
        PATTERN_STRS
            .iter()
            .map(|s| {
                s.parse::<reference::Pattern>()
                    .unwrap()
                    .to_runtime_pattern()
            })
            .collect::<Vec<_>>()
    }

    #[test]
    fn test_basic() {
        let patterns = get_patterns();
        let mut it = patterns.iter().cloned();
        let lms0 = LengthPluralElements {
            long: PluralElements::new(it.next().unwrap()),
            medium: PluralElements::new(it.next().unwrap()),
            short: PluralElements::new(it.next().unwrap()),
        };
        let lms1 = LengthPluralElements {
            long: PluralElements::new(it.next().unwrap()),
            medium: PluralElements::new(it.next().unwrap()),
            short: PluralElements::new(it.next().unwrap()),
        };
        let lms2 = LengthPluralElements {
            long: PluralElements::new(it.next().unwrap()),
            medium: PluralElements::new(it.next().unwrap()),
            short: PluralElements::new(it.next().unwrap()),
        };
        let lms0a = LengthPluralElements {
            long: PluralElements::new(patterns[0].clone()),
            medium: PluralElements::new(patterns[0].clone()),
            short: PluralElements::new(patterns[1].clone()),
        };
        let lms1a = LengthPluralElements {
            long: PluralElements::new(patterns[3].clone()),
            medium: PluralElements::new(patterns[4].clone()),
            short: PluralElements::new(patterns[4].clone()),
        };

        {
            // Q = 1
            let builder = PackedPatternsBuilder {
                standard: lms0.clone(),
                variant0: Some(lms1.clone()),
                variant1: Some(lms2.clone()),
            };
            let packed = builder.clone().build();
            assert_eq!(packed.header, 7);
            assert_eq!(packed.elements.len(), 9);
            for (pattern_elements, expected) in packed.elements.iter().zip(patterns.iter()) {
                assert_eq!(pattern_elements.get_default().1, &expected.items);
            }
            let recovered_builder = packed.to_builder();
            assert_eq!(builder, recovered_builder);
        }
        {
            // Q = 0
            let builder = PackedPatternsBuilder {
                standard: lms0.clone(),
                variant0: Some(lms0.clone()),
                variant1: Some(lms2.clone()),
            };
            let packed = builder.clone().build();
            assert_eq!(packed.header, 0x1AC003);
            assert_eq!(packed.elements.len(), 6);
            let recovered_builder = packed.to_builder();
            assert_ne!(builder, recovered_builder);
            let mut builder = builder;
            builder.simplify();
            assert_eq!(builder, recovered_builder);
        }
        {
            // No variants
            let builder = PackedPatternsBuilder {
                standard: lms0.clone(),
                variant0: None,
                variant1: None,
            };
            let packed = builder.clone().build();
            assert_eq!(packed.header, 3);
            assert_eq!(packed.elements.len(), 3);
            let recovered_builder = packed.to_builder();
            assert_eq!(builder, recovered_builder);
        }
        {
            // Some duplicate patterns and inheritance
            let builder = PackedPatternsBuilder {
                standard: lms0a.clone(),
                variant0: Some(lms0.clone()),
                variant1: Some(lms1.clone()),
            };
            let packed = builder.clone().build();
            assert_eq!(packed.header, 0x1AC682);
            assert_eq!(packed.elements.len(), 6);
            let recovered_builder = packed.to_builder();
            assert_eq!(builder, recovered_builder);
        }
        {
            // Q = 1 with 8 patterns (min for Q = 1)
            let builder = PackedPatternsBuilder {
                standard: lms0a.clone(),
                variant0: Some(lms1.clone()),
                variant1: Some(lms2.clone()),
            };
            let packed = builder.clone().build();
            assert_eq!(packed.header, 6);
            assert_eq!(packed.elements.len(), 8);
            let recovered_builder = packed.to_builder();
            assert_eq!(builder, recovered_builder);
        }
        {
            // Q = 0 with 7 patterns (max for Q = 0)
            let builder = PackedPatternsBuilder {
                standard: lms1a.clone(),
                variant0: Some(lms0a.clone()),
                variant1: Some(lms2.clone()),
            };
            let packed = builder.clone().build();
            assert_eq!(packed.header, 0x1F58D9);
            assert_eq!(packed.elements.len(), 7);
            let recovered_builder = packed.to_builder();
            assert_eq!(builder, recovered_builder);
        }
    }

    #[cfg(feature = "datagen")]
    #[test]
    fn test_serde() {
        let patterns = get_patterns();
        let lms0a = LengthPluralElements {
            long: PluralElements::new(patterns[0].clone()),
            medium: PluralElements::new(patterns[0].clone()),
            short: PluralElements::new(patterns[1].clone()),
        };
        let lms1 = LengthPluralElements {
            long: PluralElements::new(patterns[3].clone()),
            medium: PluralElements::new(patterns[4].clone()),
            short: PluralElements::new(patterns[5].clone()),
        };

        let builder = PackedPatternsBuilder {
            standard: lms0a,
            variant0: Some(lms1),
            variant1: None,
        };
        let packed = builder.clone().build();

        let bincode_bytes = bincode::serialize(&packed).unwrap();
        assert_eq!(
            bincode_bytes.as_slice(),
            &[
                26, 11, 0, 0, 74, 0, 0, 0, 0, 0, 0, 0, 5, 0, 0, 0, 16, 0, 26, 0, 30, 0, 46, 0, 0,
                128, 32, 1, 0, 0, 47, 128, 64, 1, 0, 0, 47, 128, 16, 1, 2, 128, 114, 2, 0, 0, 58,
                128, 128, 2, 0, 128, 80, 1, 0, 128, 80, 1, 0, 0, 32, 128, 32, 3, 0, 0, 32, 128, 64,
                1, 0, 128, 64, 2, 0, 0, 46, 128, 32, 2, 0, 0, 46, 128, 16, 2
            ][..]
        );
        let bincode_recovered = bincode::deserialize::<PackedPatternsV1>(&bincode_bytes).unwrap();
        assert_eq!(builder, bincode_recovered.to_builder());

        let json_str = serde_json::to_string(&packed).unwrap();
        assert_eq!(json_str, "{\"has_explicit_short\":true,\"variant_pattern_indices\":[3,4,5,0,0,0],\"elements\":[\"M/d/y\",\"HH:mm\",\"E\",\"E MMM d\",\"dd.MM.yy\"]}");
        let json_recovered = serde_json::from_str::<PackedPatternsV1>(&json_str).unwrap();
        assert_eq!(builder, json_recovered.to_builder());
    }
}
