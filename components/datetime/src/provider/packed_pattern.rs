// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data structures for packing of datetime patterns.

use crate::helpers::size_test;
use icu_plurals::PluralElements;
use zerovec::VarZeroVec;

use crate::pattern::runtime::{Pattern, PatternULE};

#[derive(Debug)]
pub struct LengthPluralElements<'a> {
    pub long: PluralElements<&'a Pattern<'a>>,
    pub medium: PluralElements<&'a Pattern<'a>>,
    pub short: PluralElements<&'a Pattern<'a>>,
}

/// A builder for a [`PackedSkeletonDataV2`].
#[derive(Debug)]
pub struct PackedSkeletonDataBuilder<'a> {
    pub standard: LengthPluralElements<'a>,
    /// Patterns for variant 0. If `None`, falls back to standard.
    pub variant0: Option<LengthPluralElements<'a>>,
    /// Patterns for variant 1. If `None`, falls back to variant 0.
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
/// If bit 2 is 1 (Q=1), it means there is one pattern per variant index,
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
/// | Lc            | S + 4             | 12-14           | Lb          |
/// | Mc            | S + 5             | 15-17           | Mb          |
/// | Sc            | S + 6             | 18-20           | Mc          |
/// 
/// [`EraDisplay::Auto`]: crate::neo_skeleton::EraDisplay::Auto
#[doc = packed_skeleton_data_size!()]
#[derive(Debug, PartialEq, Clone)]
#[cfg_attr(feature = "datagen", derive(serde::Serialize, databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_datetime::provider::neo))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct PackedSkeletonDataV2<'data> {
    /// An encoding of which standard/variant cell corresponds to which entry
    /// in the patterns table. See class docs.
    pub header: u32,
    /// The list of patterns.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub patterns: VarZeroVec<'data, PatternULE>,
}
