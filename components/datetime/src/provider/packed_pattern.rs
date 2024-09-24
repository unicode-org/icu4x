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
use icu_plurals::{provider::PluralElementsPackedULE, PluralElements};
use zerovec::{VarZeroVec, ZeroSlice};

use crate::pattern::runtime::Pattern;

/// A field of [`PackedSkeletonDataBuilder`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LengthPluralElements<T> {
    /// The "long" length pattern plural elements.
    pub long: PluralElements<T>,
    /// The "medium" length pattern plural elements.
    pub medium: PluralElements<T>,
    /// The "short" length pattern plural elements.
    pub short: PluralElements<T>,
}

/// A builder for a [`PackedSkeletonDataV2`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackedSkeletonDataBuilder<'a> {
    /// Patterns always available.
    pub standard: LengthPluralElements<Pattern<'a>>,
    /// Patterns for variant 0. If `None`, falls back to standard.
    pub variant0: Option<LengthPluralElements<Pattern<'a>>>,
    /// Patterns for variant 1. If `None`, falls back to standard.
    pub variant1: Option<LengthPluralElements<Pattern<'a>>>,
}

size_test!(PackedSkeletonDataV2, packed_skeleton_data_size, 32);

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
/// | LMS Value | Long Index | Medium Index | Short Index |
/// |-----------|------------|--------------|-------------|
/// | 0         | 0          | 0            | 0           |
/// | 1         | 0          | 1            | 1           |
/// | 2         | 0          | 0            | 1           |
/// | 3         | 0          | 1            | 2           |
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
/// making the header int suitable for varint packing.
///
/// [`EraDisplay::Auto`]: crate::neo_skeleton::EraDisplay::Auto
#[doc = packed_skeleton_data_size!()]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PackedSkeletonDataV2<'data> {
    /// An encoding of which standard/variant cell corresponds to which entry
    /// in the patterns table. See class docs.
    pub header: u32,
    /// The list of patterns.
    pub elements: VarZeroVec<'data, PluralElementsPackedULE<ZeroSlice<PatternItem>>>,
}

impl PackedSkeletonDataBuilder<'_> {
    /// Builds a packed pattern representation from the builder.
    pub fn build(mut self) -> PackedSkeletonDataV2<'static> {
        self.simplify();

        // Initialize the elements vector with the standard patterns.
        let mut elements = Vec::new();
        elements.push(&self.standard.long);
        let mut header = 0;
        let mut s_offset = 0;
        if self.standard.medium != self.standard.long {
            elements.push(&self.standard.medium);
            header |= 0x1;
            s_offset += 1;
        }
        if self.standard.short != self.standard.medium {
            elements.push(&self.standard.short);
            header |= 0x2;
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
        let mut chunks = [0; 6];
        for ((pattern, fallback), chunk) in variant_patterns
            .iter()
            .zip(fallbacks.iter())
            .zip(chunks.iter_mut())
        {
            if let Some(pattern) = pattern {
                if pattern != fallback {
                    *chunk = match elements.iter().position(|p| p == pattern) {
                        Some(i) => i + 1,
                        None => {
                            elements.push(pattern);
                            elements.len()
                        }
                    }
                }
            }
        }

        // Check to see if we need to switch to Q=1 mode
        #[allow(clippy::unwrap_used)] // the array is nonempty
        if chunks.iter().max().unwrap() >= &0x8 {
            // one pattern per table cell
            header |= 0x4;
            elements.truncate(s_offset + 1);
            elements.extend(
                variant_patterns
                    .into_iter()
                    .zip(fallbacks.iter())
                    .map(|(pattern, fallback)| pattern.unwrap_or(fallback)),
            );
        } else {
            // per-cell offsets
            let mut shift = 3;
            for chunk in chunks.iter() {
                header |= chunk << shift;
                shift += 3;
            }
        }

        // Now we can build the data representation
        let elements = elements
            .iter()
            .map(|plural_elements| {
                plural_elements
                    .as_ref()
                    .map(|pattern| (pattern.metadata.to_four_bit_metadata(), &*pattern.items))
            })
            .collect::<Vec<_>>();
        PackedSkeletonDataV2 {
            #[allow(clippy::unwrap_used)] // the header fits in 21 bits
            header: u32::try_from(header).unwrap(),
            elements: VarZeroVec::from(&elements),
        }
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

impl PackedSkeletonDataV2<'_> {
    pub(crate) fn get(
        &self,
        length: NeoSkeletonLength,
        variant: PackedSkeletonVariant,
    ) -> PatternBorrowed {
        use NeoSkeletonLength::*;
        use PackedSkeletonVariant::*;
        let lms = self.header & 0x3;
        let pattern_index = if matches!(variant, Standard) {
            // Standard pattern (first column)
            match (length, lms) {
                (Long, _) => 0,
                (Medium, 0 | 2) => 0,
                (Medium, _) => 1,
                (Short, 0) => 0,
                (Short, 1 | 2) => 1,
                (Short, _) => 2,
            }
        } else {
            let s_offset = match lms {
                0 => 0,
                1 | 2 => 1,
                _ => 2,
            };
            let q = self.header & 0x4;
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
                let chunk = chunk_in_low_bits & 0x7;
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
    pub fn to_builder(&self) -> PackedSkeletonDataBuilder {
        use NeoSkeletonLength::*;
        use PackedSkeletonVariant::*;
        let mut builder = PackedSkeletonDataBuilder {
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

    #[test]
    fn test_basic() {
        let patterns = PATTERN_STRS
            .iter()
            .map(|s| {
                s.parse::<reference::Pattern>()
                    .unwrap()
                    .to_runtime_pattern()
            })
            .collect::<Vec<_>>();
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
            let builder = PackedSkeletonDataBuilder {
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
            let builder = PackedSkeletonDataBuilder {
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
            let builder = PackedSkeletonDataBuilder {
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
            let builder = PackedSkeletonDataBuilder {
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
            let builder = PackedSkeletonDataBuilder {
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
            let builder = PackedSkeletonDataBuilder {
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
}
