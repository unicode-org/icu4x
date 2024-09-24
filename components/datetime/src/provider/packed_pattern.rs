// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures for packing of datetime patterns.

use std::collections::HashSet;

use crate::{
    helpers::size_test,
    pattern::{
        runtime::{PatternBorrowed, PatternMetadata},
        PatternItem,
    },
    NeoSkeletonLength,
};
use icu_plurals::{provider::PluralElementsPackedULE, PluralElements, PluralOperands};
use zerovec::{VarZeroVec, ZeroSlice};

use crate::pattern::runtime::{Pattern, PatternULE};

#[derive(Debug, PartialEq, Eq)]
pub struct LengthPluralElements<'a> {
    pub long: PluralElements<Pattern<'a>>,
    pub medium: PluralElements<Pattern<'a>>,
    pub short: PluralElements<Pattern<'a>>,
}

/// A builder for a [`PackedSkeletonDataV2`].
#[derive(Debug, PartialEq, Eq)]
pub struct PackedSkeletonDataBuilder<'a> {
    pub standard: LengthPluralElements<'a>,
    /// Patterns for variant 0. If `None`, falls back to standard.
    pub variant0: Option<LengthPluralElements<'a>>,
    /// Patterns for variant 1. If `None`, falls back to standard.
    pub variant1: Option<LengthPluralElements<'a>>,
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
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::neo))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PackedSkeletonDataV2<'data> {
    /// An encoding of which standard/variant cell corresponds to which entry
    /// in the patterns table. See class docs.
    pub header: u32,
    /// The list of patterns.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, PluralElementsPackedULE<ZeroSlice<PatternItem>>>,
}

impl<'a> LengthPluralElements<'a> {
    fn iter_patterns(&self) -> impl Iterator<Item = &'_ PluralElements<Pattern<'a>>> + '_ {
        [&self.long, &self.medium, &self.short].into_iter()
    }
}

impl PackedSkeletonDataBuilder<'_> {
    /// Builds a packed pattern representation from the builder.
    pub fn build(mut self) -> PackedSkeletonDataV2<'static> {
        self.simplify();

        // Initialize the elements vector with the standard patterns.
        let mut elements = Vec::new();
        elements.push(self.standard.long);
        let mut lms = 0;
        let mut s_offset = 0;
        if self.standard.medium != self.standard.long {
            elements.push(self.standard.medium);
            lms |= 0x1;
            s_offset += 1;
        }
        if self.standard.short != self.standard.medium {
            elements.push(self.standard.short);
            lms |= 0x2;
            s_offset += 1;
        }

        // Collect all patterns that could occur.
        let mut all_elements_set = HashSet::new();
        all_elements_set.extend(elements.iter());
        if let Some(variant0) = self.variant0.as_ref() {
            all_elements_set.extend(variant0.iter_patterns());
        }
        if let Some(variant1) = self.variant1.as_ref() {
            all_elements_set.extend(variant1.iter_patterns());
        }

        todo!()

        // let mut chunks = [0; 6];
        // let [variant0_long, variant0_medium, variant0_short, variant1_long, variant1_medium, variant1_short] =
        //     &mut chunks;
        // if let Some(variant0) = self.variant0 {
        //     if variant0.long != self.standard.long {
        //         elements.push(variant0.long);
        //         *variant0_long = elements.len();
        //     }
        //     if variant0.medium != self.standard.medium {
        //         elements.push(variant0.medium);
        //         *variant0_medium = elements.len();
        //     }
        //     if variant0.short != self.standard.short {
        //         elements.push(variant0.short);
        //         *variant0_short = elements.len();
        //     }
        // }
        // if let Some(variant1) = self.variant1 {
        //     if variant1.long != self.standard.long {
        //         elements.push(variant1.long);
        //         *variant1_long = elements.len();
        //     }
        //     if variant1.medium != self.standard.medium {
        //         elements.push(variant1.medium);
        //         *variant1_medium = elements.len();
        //     }
        //     if variant1.short != self.standard.short {
        //         elements.push(variant1.short);
        //         *variant1_short = elements.len();
        //     }
        // }
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
    ) -> Option<PatternBorrowed> {
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
                _ => {
                    debug_assert!(false, "unreachable");
                    return None;
                }
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
                        return None;
                    }
                };
                let chunk = chunk_in_low_bits & 0x7;
                if chunk == 0 {
                    // Fall back to standard with the same length
                    return self.get(length, Standard);
                }
                s_offset + chunk - 1
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
                        return None;
                    }
                };
                s_offset + additional_offset
            }
        };
        let packed_plurals = self.patterns.get(pattern_index as usize)?;
        let (metadata, items) = packed_plurals.get_default();
        Some(PatternBorrowed {
            metadata: PatternMetadata::from_u8(metadata.get()),
            items,
        })
    }

    fn get_as_plural_elements(
        &self,
        length: NeoSkeletonLength,
        variant: PackedSkeletonVariant,
    ) -> PluralElements<Pattern> {
        PluralElements::new(self.get(length, variant).unwrap().as_pattern())
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
