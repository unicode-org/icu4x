// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations for normalization data, computed
//! from the Unicode Character Database and the UTS 46 IDNA mapping table.
//!
//! See `components/normalizer/trie-value-format.md` for documentation of the trie
//! value format.
//!
//! The computation needs to know whether decompositions round-trip through canonical
//! composition. Instead of implementing canonical composition here, we bootstrap: we
//! first compute NFD data with conservative round-trip flags (which disables passthrough
//! fast paths, but is otherwise correct), and use that to construct an NFC normalizer,
//! which is then used to compute the final data.

use crate::SourceDataProvider;
use crate::TrieType;
use crate::properties::ucd_helpers;
use icu::collections::codepointinvlist::{CodePointInversionList, CodePointInversionListBuilder};
use icu::collections::codepointtrie::CodePointTrieBuilder;
use icu::normalizer::provider::*;
use icu::normalizer::{ComposingNormalizer, ComposingNormalizerBorrowed};
use icu::properties::props::{CanonicalCombiningClass, FullCompositionExclusion};
use icu::properties::{
    CodePointMapData, CodePointMapDataBorrowed, CodePointSetData, CodePointSetDataBorrowed,
};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, HashMap, HashSet};
use zerovec::ZeroVec;

// These constants originate from page 143 of Unicode 14.0
/// Syllable base
const HANGUL_S_BASE: u32 = 0xAC00;
/// Lead jamo base
const HANGUL_L_BASE: u32 = 0x1100;
/// Vowel jamo base
const HANGUL_V_BASE: u32 = 0x1161;
/// Trail jamo base (deliberately off by one to account for the absence of a trail)
const HANGUL_T_BASE: u32 = 0x11A7;
/// Trail jamo count (deliberately off by one to account for the absence of a trail)
const HANGUL_T_COUNT: u32 = 28;
/// Vowel jamo count times trail jamo count
const HANGUL_N_COUNT: u32 = 588;
/// Hangul syllables
const HANGUL_SYLLABLES: std::ops::RangeInclusive<u32> = 0xAC00..=0xD7A3;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Flavor {
    Nfd,
    Nfkd,
    Uts46,
}

/// Raw (non-recursive) decomposition mappings of a flavor, excluding Hangul syllables.
type Mapping<'a> = dyn Fn(char) -> Option<&'a [char]> + 'a;

fn push_full_decomposition(c: char, mapping: &Mapping, out: &mut Vec<char>) {
    if HANGUL_SYLLABLES.contains(&(c as u32)) {
        let s = c as u32 - HANGUL_S_BASE;
        let l = HANGUL_L_BASE + s / HANGUL_N_COUNT;
        let v = HANGUL_V_BASE + (s % HANGUL_N_COUNT) / HANGUL_T_COUNT;
        let t = HANGUL_T_BASE + s % HANGUL_T_COUNT;
        out.extend([l, v].into_iter().filter_map(char::from_u32));
        if t != HANGUL_T_BASE {
            out.extend(char::from_u32(t));
        }
    } else if let Some(m) = mapping(c) {
        for &m in m {
            push_full_decomposition(m, mapping, out);
        }
    } else {
        out.push(c);
    }
}

/// The full (recursive, canonically ordered) decomposition of `c` under `mapping`.
fn full_decomposition(
    c: char,
    mapping: &Mapping,
    ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
) -> Vec<char> {
    let mut out = Vec::new();
    push_full_decomposition(c, mapping, &mut out);
    // Canonical ordering: stably sort each run of non-starters by ccc.
    let is_non_starter = |c: char| ccc.get(c) != CanonicalCombiningClass::NotReordered;
    for run in out.chunk_by_mut(|&a, &b| is_non_starter(a) && is_non_starter(b)) {
        run.sort_by_key(|&c| ccc.get(c));
    }
    out
}

/// Computes the canonical compositions trie and the set of starters that can combine
/// backwards.
fn canonical_compositions(
    nfd_mapping: &Mapping,
    ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
    full_composition_exclusion: CodePointSetDataBorrowed,
) -> (
    CanonicalCompositions<'static>,
    CodePointInversionList<'static>,
) {
    let mut backward_combining_starters = CodePointInversionListBuilder::new();
    let mut compositions = Vec::new();
    for composite in (0..=char::MAX as u32).filter_map(char::from_u32) {
        if HANGUL_SYLLABLES.contains(&(composite as u32)) {
            let s = composite as u32 - HANGUL_S_BASE;
            let t = s % HANGUL_T_COUNT;
            if t == 0 {
                // LV syllable
                let v =
                    char::from_u32(HANGUL_V_BASE + (s % HANGUL_N_COUNT) / HANGUL_T_COUNT).unwrap();
                if ccc.get(v) == CanonicalCombiningClass::NotReordered {
                    backward_combining_starters.add_char(v);
                }
            } else {
                // LVT syllable
                let t = char::from_u32(HANGUL_T_BASE + t).unwrap();
                if ccc.get(t) == CanonicalCombiningClass::NotReordered {
                    backward_combining_starters.add_char(t);
                }
            }
        } else {
            match nfd_mapping(composite) {
                Some(&[starter, second]) if !full_composition_exclusion.contains(composite) => {
                    compositions.push((starter, second, composite));

                    if ccc.get(second) == CanonicalCombiningClass::NotReordered {
                        backward_combining_starters.add_char(second);
                    }
                }
                _ => continue,
            }
        };
    }
    (
        CanonicalCompositions::from_compositions(compositions),
        backward_combining_starters.build(),
    )
}

/// How a character decomposes in a normalization flavor. Each variant corresponds to one
/// of the trie value types described in `trie-value-format.md`.
///
/// Starters that decompose to themselves and cannot combine backwards are not represented.
#[derive(Debug, Clone, PartialEq)]
enum Decomposition {
    /// A starter that decomposes to itself and can combine backwards.
    BackwardCombiningStarter,
    /// U+FFFD, a starter that decomposes to itself. In the potentially-ill-formed UTF-8
    /// case, UTF-8 errors return U+FFFD from the iterator, and errors need to be treated
    /// as ineligible for passthrough on the slice fast path. Giving U+FFFD a trie value
    /// whose flags make it ineligible for passthrough avoids a specific U+FFFD branch on
    /// the passthrough fast path.
    ReplacementCharacter,
    /// A non-starter that decomposes to itself.
    NonStarter(CanonicalCombiningClass),
    /// A character in [`DecompositionData::SPECIAL_NON_STARTER_DECOMPOSITIONS`], with the
    /// canonical combining class of the undecomposed character.
    SpecialNonStarter(CanonicalCombiningClass),
    /// A UTS 46 ignorable.
    Ignorable,
    /// A singleton decomposition to a BMP starter.
    Singleton { c: char, backward_combining: bool },
    /// A decomposition to a starter followed by a non-starter, stored inline.
    Pair {
        starter: char,
        non_starter: char,
        round_trips: bool,
    },
    /// A decomposition stored in the tables.
    Complex {
        chars: Vec<char>,
        supplementary: bool,
        only_non_starters_in_trail: bool,
        round_trips: bool,
        backward_combining: bool,
    },
    /// The NFKD decomposition of U+FDFA, which is too long to be stored in the tables
    /// and is hard-coded instead.
    Fdfa,
}

impl Decomposition {
    /// Whether characters with this decomposition can be passed through by the decomposing
    /// normalizer without looking at their trie value.
    fn is_decomposition_passthrough(&self) -> bool {
        matches!(self, Self::BackwardCombiningStarter)
    }

    /// Whether characters with this decomposition can be passed through by the composing
    /// normalizer without looking at their trie value.
    fn is_composition_passthrough(&self) -> bool {
        matches!(
            self,
            Self::Pair {
                round_trips: true,
                ..
            } | Self::Complex {
                round_trips: true,
                backward_combining: false,
                ..
            }
        )
    }

    fn starts_with_non_starter(&self) -> bool {
        matches!(self, Self::NonStarter(_) | Self::SpecialNonStarter(_))
    }

    /// Encodes the trie value. `offset` is the offset of a complex decomposition into the
    /// logical concatenation of the tables, see [`DecompositionData::encode_complex`].
    fn encode(&self, offset: Option<usize>) -> Option<u32> {
        match *self {
            Self::BackwardCombiningStarter => Some(DecompositionData::BACKWARD_COMBINING_MARKER),
            Self::ReplacementCharacter => Some(
                DecompositionData::NON_ROUND_TRIP_MARKER
                    | DecompositionData::BACKWARD_COMBINING_MARKER,
            ),
            Self::NonStarter(ccc) => Some(DecompositionData::encode_non_starter(ccc)),
            Self::SpecialNonStarter(ccc) => {
                Some(DecompositionData::encode_special_non_starter(ccc))
            }
            Self::Ignorable => Some(DecompositionData::IGNORABLE_MARKER),
            Self::Singleton {
                c,
                backward_combining,
            } => DecompositionData::encode_singleton(c, backward_combining),
            Self::Pair {
                starter,
                non_starter,
                round_trips,
            } => DecompositionData::encode_pair(starter, non_starter, round_trips),
            Self::Complex {
                ref chars,
                supplementary,
                only_non_starters_in_trail,
                round_trips,
                backward_combining,
            } => DecompositionData::encode_complex(
                offset?,
                chars.len(),
                supplementary,
                only_non_starters_in_trail,
                round_trips,
                backward_combining,
            ),
            Self::Fdfa => Some(DecompositionData::encode_fdfa()),
        }
    }
}

/// Computes the decompositions of characters.
struct Classifier<'a, 'm> {
    ccc: CodePointMapDataBorrowed<'a, CanonicalCombiningClass>,
    nfd_mapping: &'a Mapping<'m>,
    backward_combining_starters: &'a CodePointInversionList<'a>,
    /// If `None`, all decompositions are treated as not round-tripping via canonical
    /// composition.
    nfc: Option<&'a ComposingNormalizerBorrowed<'a>>,
}

impl Classifier<'_, '_> {
    /// Computes the decompositions of `candidates` in `flavor`, whose raw decomposition
    /// mappings are `mapping`. For flavors other than NFD, only the decompositions that
    /// differ from NFD are returned.
    ///
    /// `candidates` must contain all characters that don't trivially decompose to
    /// themselves: characters with a decomposition mapping, non-starters, and starters
    /// that can combine backwards.
    fn classify_all(
        &self,
        flavor: Flavor,
        mapping: &Mapping,
        candidates: &CodePointInversionList,
    ) -> Result<BTreeMap<char, Decomposition>, DataError> {
        candidates
            .iter_chars()
            .filter_map(|c| {
                self.classify(c, flavor, mapping)
                    .transpose()
                    .map(|d| d.map(|d| (c, d)))
            })
            .collect()
    }

    fn classify(
        &self,
        c: char,
        flavor: Flavor,
        mapping: &Mapping,
    ) -> Result<Option<Decomposition>, DataError> {
        let error = |message: &'static str| {
            DataError::custom(message)
                .with_debug_context(&c)
                .with_debug_context(&flavor)
        };
        let ccc = |c: char| self.ccc.get(c);

        if HANGUL_SYLLABLES.contains(&(c as u32)) {
            // Hangul syllables are decomposed algorithmically.
            return Ok(None);
        }

        if c == char::REPLACEMENT_CHARACTER {
            return Ok(Some(Decomposition::ReplacementCharacter));
        }

        let dst = full_decomposition(c, mapping, self.ccc);
        if flavor != Flavor::Nfd && dst == full_decomposition(c, self.nfd_mapping, self.ccc) {
            return Ok(None);
        }

        if flavor != Flavor::Uts46 && (dst.is_empty() || dst == [char::REPLACEMENT_CHARACTER]) {
            return Err(error("Unexpected empty or U+FFFD decomposition"));
        }

        let Some(&first) = dst.first() else {
            // Ignorable in UTS 46 (the non-UTS 46 case has errored above).
            return Ok(Some(Decomposition::Ignorable));
        };

        if ccc(first) != CanonicalCombiningClass::NotReordered {
            return if dst == [c] {
                Ok(Some(Decomposition::NonStarter(ccc(first))))
            } else if DecompositionData::SPECIAL_NON_STARTER_DECOMPOSITIONS
                .iter()
                .any(|&(s, d)| s == c && d == dst.as_slice())
            {
                Ok(Some(Decomposition::SpecialNonStarter(ccc(c))))
            } else {
                // A character whose decomposition starts with a non-starter and isn't the
                // same as the character itself and isn't already hard-coded into ICU4X.
                Err(error("Unexpected non-starter decomposition"))
            };
        }

        let backward_combining = self.backward_combining_starters.contains(first);

        if dst == [c] {
            return Ok(backward_combining.then_some(Decomposition::BackwardCombiningStarter));
        }

        let round_trips = self
            .nfc
            .is_some_and(|nfc| nfc.normalize_iter(dst.iter().copied()).eq([c]));

        // Iota subscript and half-width voicing marks must never occur in expansions in
        // the normalization forms where they are special.
        let is_special_in_flavor = |d: char| {
            (d == DecompositionData::IOTA_SUBSCRIPT && flavor == Flavor::Uts46)
                || d == DecompositionData::HALF_WIDTH_VOICED_SOUND_MARK
                || d == DecompositionData::HALF_WIDTH_SEMI_VOICED_SOUND_MARK
        };

        Ok(Some(match *dst {
            // Singleton decompositions to conjoining jamo.
            [d] if ('\u{1161}'..='\u{1175}').contains(&d)
                || ('\u{11A8}'..='\u{11C2}').contains(&d) =>
            {
                if flavor == Flavor::Nfd {
                    return Err(error("Unexpected decomposition to conjoining jamo"));
                }
                Decomposition::Singleton {
                    c: d,
                    backward_combining,
                }
            }
            [d] if !backward_combining && d as u32 <= 0xFFFF => Decomposition::Singleton {
                c: d,
                backward_combining: false,
            },
            // ANGSTROM SIGN is special to make the Harfbuzz case branch less in the more common case.
            [a, b]
                if c != DecompositionData::ANGSTROM_SIGN
                    && !backward_combining
                    && ccc(a) == CanonicalCombiningClass::NotReordered
                    && ccc(b) != CanonicalCombiningClass::NotReordered
                    && DecompositionData::encode_pair(a, b, round_trips).is_some() =>
            {
                if is_special_in_flavor(a) || is_special_in_flavor(b) {
                    return Err(error("Unexpected special character in expansion"));
                }
                Decomposition::Pair {
                    starter: a,
                    non_starter: b,
                    round_trips,
                }
            }
            _ => {
                if dst.iter().copied().any(is_special_in_flavor) {
                    return Err(error("Unexpected special character in expansion"));
                }
                if dst.contains(&'\0') {
                    return Err(error("Unexpected NUL in expansion"));
                }
                // The format doesn't allow for length 1 for BMP, so these need to be
                // promoted to wider storage.
                let supplementary = dst.len() == 1 || dst.iter().any(|&d| d as u32 > 0xFFFF);
                let max_len = if supplementary {
                    DecompositionData::COMPLEX_MAX_LEN_24
                } else {
                    DecompositionData::COMPLEX_MAX_LEN_16
                };
                if dst.len() > max_len {
                    if c == '\u{FDFA}'
                        && dst
                            .iter()
                            .skip(1)
                            .map(|&d| d as u32)
                            .eq(DecompositionData::FDFA_NFKD.iter().map(|&d| u32::from(d)))
                    {
                        return Ok(Some(Decomposition::Fdfa));
                    }
                    return Err(error("Decomposition too long"));
                }
                Decomposition::Complex {
                    only_non_starters_in_trail: dst[1..]
                        .iter()
                        .all(|&d| ccc(d) != CanonicalCombiningClass::NotReordered),
                    chars: dst,
                    supplementary,
                    round_trips,
                    backward_combining,
                }
            }
        }))
    }
}

/// Find `needle` within `storage` and return its index, failing which, append
/// `needle` to `storage` and return the index of it at the end.
fn find_or_append<T: Copy + PartialEq>(storage: &mut Vec<T>, needle: &[T]) -> usize {
    if let Some(index) = storage.windows(needle.len()).position(|w| w == needle) {
        return index;
    }
    let index = storage.len();
    storage.extend_from_slice(needle);
    index
}

/// The tables storing complex decompositions.
struct ComplexDecompositionTables {
    nfd: DecompositionTables<'static>,
    /// The supplementary (NFKD and UTS 46) tables.
    supplementary: DecompositionTables<'static>,
    /// For each flavor, the offset of each complex decomposition into the logical
    /// concatenation of NFD 16-bit, NFD 32-bit, supplementary 16-bit, and supplementary
    /// 32-bit storage.
    offsets: Vec<BTreeMap<char, usize>>,
}

impl ComplexDecompositionTables {
    /// Stores the complex decompositions of `flavors`. The first flavor is stored in the
    /// NFD tables, all others in the supplementary tables.
    fn new(flavors: &[&BTreeMap<char, Decomposition>]) -> Result<Self, DataError> {
        let mut storage16 = Vec::<u16>::new();
        let mut storage32 = Vec::<char>::new();
        let mut nfd_len = (0, 0);
        let mut offsets = Vec::new();
        for (i, decompositions) in flavors.iter().enumerate() {
            // Iterate backwards in order to better find overlapping decompositions.
            offsets.push(
                decompositions
                    .iter()
                    .rev()
                    .filter_map(|(&c, d)| match d {
                        Decomposition::Complex {
                            chars,
                            supplementary: true,
                            ..
                        } => Some((c, (true, find_or_append(&mut storage32, chars)))),
                        Decomposition::Complex { chars, .. } => {
                            let chars = chars.iter().map(|&c| c as u16).collect::<Vec<_>>();
                            Some((c, (false, find_or_append(&mut storage16, &chars))))
                        }
                        _ => None,
                    })
                    .collect::<Vec<_>>(),
            );
            if i == 0 {
                nfd_len = (storage16.len(), storage32.len());
            }
        }

        if storage16.len() + storage32.len() > DecompositionTables::MAX_COMBINED_LEN {
            // We actually have 14 bits available, but let's error out so that docs can be
            // updated when taking a reserved bit out of potential future flag usage.
            return Err(DataError::custom("Decomposition storage too large"));
        }

        let (nfd_len_16, nfd_len_32) = nfd_len;
        let total_len_16 = storage16.len();
        let logical_offset = |(supplementary, offset)| match (supplementary, offset) {
            // NFD 16-bit
            (false, o) if o < nfd_len_16 => o,
            // Supplementary 16-bit (the NFD 16-bit length is already part of the offset)
            (false, o) => o + nfd_len_32,
            // NFD 32-bit
            (true, o) if o < nfd_len_32 => nfd_len_16 + o,
            // Supplementary 32-bit (the NFD 32-bit length is already part of the offset)
            (true, o) => total_len_16 + o,
        };

        Ok(Self {
            nfd: DecompositionTables {
                scalars16: ZeroVec::alloc_from_slice(&storage16[..nfd_len_16]),
                scalars24: ZeroVec::alloc_from_slice(&storage32[..nfd_len_32]),
            },
            supplementary: DecompositionTables {
                scalars16: ZeroVec::alloc_from_slice(&storage16[nfd_len_16..]),
                scalars24: ZeroVec::alloc_from_slice(&storage32[nfd_len_32..]),
            },
            offsets: offsets
                .into_iter()
                .map(|offsets| {
                    offsets
                        .into_iter()
                        .map(|(c, offset)| (c, logical_offset(offset)))
                        .collect()
                })
                .collect(),
        })
    }
}

/// Encodes `decompositions` into `values`, using the complex decomposition `offsets`.
fn encode_all(
    decompositions: &BTreeMap<char, Decomposition>,
    offsets: &BTreeMap<char, usize>,
    values: &mut BTreeMap<char, u32>,
) -> Result<(), DataError> {
    for (&c, decomposition) in decompositions {
        let value = decomposition
            .encode(offsets.get(&c).copied())
            .ok_or_else(|| {
                DataError::custom("Decomposition cannot be encoded").with_debug_context(&c)
            })?;
        values.insert(c, value);
    }
    Ok(())
}

/// The decompositions of a flavor: the NFD decompositions, overridden by the flavor's own.
fn merged<'a>(
    nfd: &'a BTreeMap<char, Decomposition>,
    flavor: &'a BTreeMap<char, Decomposition>,
) -> BTreeMap<char, &'a Decomposition> {
    nfd.iter().chain(flavor).map(|(&c, d)| (c, d)).collect()
}

/// The characters whose decompositions start with a non-starter.
fn non_starters(
    decompositions: &BTreeMap<char, &Decomposition>,
) -> CodePointInversionList<'static> {
    decompositions
        .iter()
        .filter(|(_, d)| d.starts_with_non_starter())
        .map(|(&c, _)| c as u32..=c as u32)
        .collect()
}

/// Validates the expectations that ICU4X hard-codes about which characters'
/// decompositions start with a non-starter.
fn check_non_starters(
    flavor: Flavor,
    set: &CodePointInversionList,
    nfd_set: &CodePointInversionList,
) -> Result<(), DataError> {
    let error = |message: &'static str| DataError::custom(message).with_debug_context(&flavor);
    let set_of = |chars: &[char]| {
        chars
            .iter()
            .map(|&c| c as u32..=c as u32)
            .collect::<CodePointInversionList>()
    };
    let difference = |a: &CodePointInversionList, b: &CodePointInversionList| {
        let mut builder = CodePointInversionListBuilder::new();
        builder.add_set(a);
        builder.remove_set(b);
        builder.build()
    };

    let half_width = set_of(&[
        DecompositionData::HALF_WIDTH_VOICED_SOUND_MARK,
        DecompositionData::HALF_WIDTH_SEMI_VOICED_SOUND_MARK,
    ]);
    let iota = set_of(&[DecompositionData::IOTA_SUBSCRIPT]);

    if nfd_set.contains_set(&half_width) || !nfd_set.contains_set(&iota) {
        // NFD expectations don't hold. The set must not contain the half-width
        // kana voicing marks and must contain iota subscript.
        return Err(error("Unexpected NFD non-starters"));
    }
    let additional = difference(set, nfd_set);
    if !additional.is_empty() && additional != half_width {
        // The result was neither empty nor contained exactly the two half-width voicing
        // marks. The ICU4X normalizer doesn't know how to deal with this case.
        return Err(error("Unexpected additional non-starters"));
    }
    let missing = difference(nfd_set, set);
    if !missing.is_empty() && missing != iota {
        // The result was neither empty nor contained exactly the iota subscript. The
        // ICU4X normalizer doesn't know how to deal with this case.
        return Err(error("Unexpected missing non-starters"));
    }
    Ok(())
}

/// Computes the passthrough bounds, checks that they are within the ranges ICU4X expects,
/// and returns the passthrough cap for the data.
fn passthrough_cap(
    flavor: Flavor,
    decompositions: &BTreeMap<char, &Decomposition>,
) -> Result<u16, DataError> {
    let bound = |is_passthrough: fn(&Decomposition) -> bool| {
        decompositions
            .iter()
            .find(|(_, d)| !is_passthrough(d))
            .map_or(0x10FFFF, |(&c, _)| c as u32)
    };
    let decomposition_bound = bound(Decomposition::is_decomposition_passthrough);
    let composition_bound = bound(Decomposition::is_composition_passthrough);

    let valid = if flavor == Flavor::Nfd {
        decomposition_bound == 0xC0 && composition_bound == 0x300
    } else {
        decomposition_bound <= 0xC0
            && composition_bound <= 0x300
            && if composition_bound > 0xC0 {
                decomposition_bound == 0xC0
            } else {
                decomposition_bound == composition_bound
            }
    };
    if !valid {
        return Err(DataError::custom("Unexpected passthrough bounds").with_debug_context(&flavor));
    }
    Ok(composition_bound as u16)
}

/// The trie values of a decomposition trie.
#[derive(Debug)]
struct DecompositionTrieValues {
    values: BTreeMap<char, u32>,
    surrogates_to_replacement_character: bool,
    passthrough_cap: u16,
}

impl DecompositionTrieValues {
    fn build(&self, trie_type: TrieType) -> DecompositionData<'static> {
        // Zero is a magic number that means the character decomposes to itself.
        let mut builder = CodePointTrieBuilder::new(0, 0, trie_type.into());
        if self.surrogates_to_replacement_character {
            // Make surrogates decompose to U+FFFD. This optimization is only used by the
            // UTF-16 slice mode, and UTS 46 is not supported in slice modes (which do not
            // support ignorables).
            // Mark these as potentially backward-combining, to make lead surrogates for
            // non-BMP characters that are backward-combining count as backward-combining
            // just in case, though the backward-combiningness is not actually being looked
            // at today.
            builder.set_range_value(
                0xD800..=0xDFFF,
                DecompositionData::NON_ROUND_TRIP_MARKER
                    | DecompositionData::BACKWARD_COMBINING_MARKER
                    | char::REPLACEMENT_CHARACTER as u32,
            );
        }
        builder.set_range_value(HANGUL_SYLLABLES, DecompositionData::HANGUL_SYLLABLE_MARKER);
        for (&c, &value) in &self.values {
            builder.set_value(c as u32, value);
        }
        DecompositionData {
            trie: builder.build(),
            passthrough_cap: self.passthrough_cap,
        }
    }
}

/// The values of the non-recursive decomposition supplement.
#[derive(Debug, Default)]
struct NonRecursiveValues {
    values: BTreeMap<char, u32>,
    scalars24: Vec<char>,
}

impl NonRecursiveValues {
    /// Computes the non-recursive canonical decompositions that differ from the (recursive)
    /// NFD decompositions.
    fn new(
        nfd: &BTreeMap<char, Decomposition>,
        nfd_mapping: &Mapping,
        ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
    ) -> Result<Self, DataError> {
        let mut result = Self::default();
        // Iterate backwards. The iteration order determines the layout of `scalars24`.
        for (&c, decomposition) in nfd.iter().rev() {
            if !matches!(
                decomposition,
                Decomposition::Singleton { .. }
                    | Decomposition::Pair { .. }
                    | Decomposition::Complex { .. }
            ) || c == DecompositionData::ANGSTROM_SIGN
            {
                // Only starters that decompose. ICU4X hard-codes ANGSTROM SIGN.
                continue;
            }
            let error = |message: &'static str| DataError::custom(message).with_debug_context(&c);
            let raw = nfd_mapping(c).ok_or_else(|| error("Missing non-recursive decomposition"))?;
            // In addition to actual difference, put the whole range that contains characters
            // with oxia into the non-recursive trie in order to catch cases where characters
            // with oxia have singleton decompositions to corresponding characters with tonos.
            // This way, the run-time decision to fall through can be done on the range
            // without checking for individual characters inside the range.
            if raw != full_decomposition(c, nfd_mapping, ccc)
                || NonRecursiveDecompositionSupplement::OXIA_RANGE.contains(&c)
            {
                let value = NonRecursiveDecompositionSupplement::encode(raw, &mut result.scalars24)
                    .ok_or_else(|| error("Non-recursive decomposition cannot be encoded"))?;
                result.values.insert(c, value);
            }
        }
        Ok(result)
    }

    fn build(&self, trie_type: TrieType) -> NonRecursiveDecompositionSupplement<'static> {
        let mut builder = CodePointTrieBuilder::new(0, 0, trie_type.into());
        for (&c, &value) in &self.values {
            builder.set_value(c as u32, value);
        }
        NonRecursiveDecompositionSupplement {
            trie: builder.build(),
            scalars24: ZeroVec::alloc_from_slice(&self.scalars24),
        }
    }
}

#[derive(Debug)]
pub(crate) struct NormalizerData {
    nfd: DecompositionTrieValues,
    nfkd: DecompositionTrieValues,
    uts46: DecompositionTrieValues,
    nfd_tables: DecompositionTables<'static>,
    nfkd_tables: DecompositionTables<'static>,
    canonical_compositions: CanonicalCompositions<'static>,
    non_recursive: NonRecursiveValues,
}

impl NormalizerData {
    fn try_new(provider: &SourceDataProvider) -> Result<Self, DataError> {
        let rscd = provider.rscd()?;
        let ccc = CodePointMapData::<CanonicalCombiningClass>::try_new_unstable(provider)?;
        let ccc = ccc.as_borrowed();
        let full_composition_exclusion =
            CodePointSetData::try_new_unstable::<FullCompositionExclusion>(provider)?;
        // Canonical and compatibility decomposition mappings
        let mut canonical = HashMap::<char, Vec<char>>::new();
        let mut compatibility = HashMap::<char, Vec<char>>::new();
        for line in rscd.parse_ucd_lines("ucd/UnicodeData.txt")? {
            let Some(line) = line.skip_missing_rule() else {
                continue;
            };
            let fields = line.fields().collect::<Vec<_>>();
            // 0: Code_Point, 5: Decomposition_Type and Decomposition_Mapping
            let (Some(cp), Some(decomposition)) = (fields.first(), fields.get(5)) else {
                return Err(DataError::custom("Invalid UnicodeData.txt line")
                    .with_display_context(&fields.join(";")));
            };
            let Ok(c) = char::try_from(ucd_helpers::parse_cp(cp)) else {
                continue;
            };
            if decomposition.is_empty() {
                continue;
            }
            if let Some((_tag, mapping)) = decomposition.split_once('>') {
                compatibility.insert(c, ucd_helpers::parse_cps(mapping).chars().collect());
            } else {
                canonical.insert(c, ucd_helpers::parse_cps(decomposition).chars().collect());
            }
        }

        // Mappings of the "mapped" and "ignored" IDNA statuses, and the set of characters
        // with the "disallowed" status. This mirrors the transformation that ICU4C's
        // `preparseucd.py` applies to `IdnaMappingTable.txt`.
        let mut uts46 = HashMap::<char, Vec<char>>::new();
        let mut uts46_disallowed = CodePointInversionListBuilder::new();
        for line in rscd.parse_ucd_lines("idna/IdnaMappingTable.txt")? {
            let Some(line) = line.skip_missing_rule() else {
                continue;
            };
            let mut fields = line.fields();
            let (Some(range), Some(status)) = (fields.next(), fields.next()) else {
                return Err(DataError::custom("Invalid IdnaMappingTable.txt line")
                    .with_display_context(&line.fields().collect::<Vec<_>>().join(";")));
            };
            let range = ucd_helpers::parse_range(range);
            match status {
                // Deviation characters are handled in code. Characters disallowed under STD3
                // rules (older versions of UTS 46) are treated like their regular siblings,
                // as STD3 rules are also handled in code.
                "valid" | "deviation" | "disallowed_STD3_valid" => {}
                "mapped" | "disallowed_STD3_mapped" | "ignored" => {
                    let mapping = ucd_helpers::parse_cps(fields.next().unwrap_or_default())
                        .chars()
                        .collect::<Vec<_>>();
                    for c in range.filter_map(char::from_u32) {
                        uts46.insert(c, mapping.clone());
                    }
                }
                "disallowed" => uts46_disallowed.add_range32(range),
                _ => {
                    return Err(
                        DataError::custom("Unknown IDNA status").with_display_context(status)
                    );
                }
            }
        }

        let nfd_mapping = |c: char| canonical.get(&c).map(Vec::as_slice);
        let nfkd_mapping = |c: char| {
            compatibility
                .get(&c)
                .or_else(|| canonical.get(&c))
                .map(Vec::as_slice)
        };
        // Avoid a circular mapping
        uts46_disallowed.remove_char(char::REPLACEMENT_CHARACTER);
        let uts46_disallowed = uts46_disallowed.build();
        let uts46_mapping = |c: char| {
            if uts46_disallowed.contains(c) {
                Some(&[char::REPLACEMENT_CHARACTER][..])
            } else {
                uts46
                    .get(&c)
                    .or_else(|| canonical.get(&c))
                    .map(Vec::as_slice)
            }
        };

        let (canonical_compositions, backward_combining_starters) =
            canonical_compositions(&nfd_mapping, ccc, full_composition_exclusion.as_borrowed());

        // The characters that don't trivially decompose to themselves in each flavor:
        // U+FFFD, non-starters, starters that can combine backwards, and characters with
        // a decomposition mapping.
        let mut candidates = CodePointInversionListBuilder::new();
        candidates.add_char(char::REPLACEMENT_CHARACTER);
        for range in ccc.iter_ranges_for_value_complemented(CanonicalCombiningClass::NotReordered) {
            candidates.add_range32(range);
        }
        candidates.add_set(&backward_combining_starters);
        canonical.keys().for_each(|&c| candidates.add_char(c));
        let nfd_candidates = candidates.clone().build();
        compatibility.keys().for_each(|&c| candidates.add_char(c));
        let nfkd_candidates = candidates.clone().build();
        uts46.keys().for_each(|&c| candidates.add_char(c));
        candidates.add_set(&uts46_disallowed);
        let uts46_candidates = candidates.build();

        let nfc = Self::bootstrap_nfc(
            &nfd_mapping,
            ccc,
            &backward_combining_starters,
            &nfd_candidates,
            &canonical_compositions,
        )?;
        let nfc = nfc.as_borrowed();
        let classifier = Classifier {
            ccc,
            nfd_mapping: &nfd_mapping,
            backward_combining_starters: &backward_combining_starters,
            nfc: Some(&nfc),
        };

        let nfd = classifier.classify_all(Flavor::Nfd, &nfd_mapping, &nfd_candidates)?;
        let nfkd = classifier.classify_all(Flavor::Nfkd, &nfkd_mapping, &nfkd_candidates)?;
        let uts46 = classifier.classify_all(Flavor::Uts46, &uts46_mapping, &uts46_candidates)?;

        let no_overrides = BTreeMap::new();
        let nfd_merged = merged(&nfd, &no_overrides);
        let nfkd_merged = merged(&nfd, &nfkd);
        let uts46_merged = merged(&nfd, &uts46);

        let nfd_non_starters = non_starters(&nfd_merged);
        check_non_starters(Flavor::Nfkd, &non_starters(&nfkd_merged), &nfd_non_starters)?;
        check_non_starters(
            Flavor::Uts46,
            &non_starters(&uts46_merged),
            &nfd_non_starters,
        )?;

        let ComplexDecompositionTables {
            nfd: nfd_tables,
            supplementary: nfkd_tables,
            offsets,
        } = ComplexDecompositionTables::new(&[&nfd, &nfkd, &uts46])?;

        // The NFKD and UTS 46 tries are the NFD trie, partially overwritten.
        let mut nfd_values = BTreeMap::new();
        encode_all(&nfd, &offsets[0], &mut nfd_values)?;
        let mut nfkd_values = nfd_values.clone();
        encode_all(&nfkd, &offsets[1], &mut nfkd_values)?;
        let mut uts46_values = nfd_values.clone();
        encode_all(&uts46, &offsets[2], &mut uts46_values)?;

        Ok(Self {
            nfd: DecompositionTrieValues {
                values: nfd_values,
                surrogates_to_replacement_character: true,
                passthrough_cap: passthrough_cap(Flavor::Nfd, &nfd_merged)?,
            },
            nfkd: DecompositionTrieValues {
                values: nfkd_values,
                surrogates_to_replacement_character: true,
                passthrough_cap: passthrough_cap(Flavor::Nfkd, &nfkd_merged)?,
            },
            uts46: DecompositionTrieValues {
                values: uts46_values,
                // UTS 46 is not supported in slice modes
                surrogates_to_replacement_character: false,
                passthrough_cap: passthrough_cap(Flavor::Uts46, &uts46_merged)?,
            },
            nfd_tables,
            // The supplementary tables also contain the UTS 46 decompositions that are not
            // in the NFD or NFKD tables.
            nfkd_tables,
            canonical_compositions,
            non_recursive: NonRecursiveValues::new(&nfd, &nfd_mapping, ccc)?,
        })
    }

    /// Constructs an NFC normalizer from NFD data where all decompositions are marked as
    /// not round-tripping, and where passthrough is disabled.
    fn bootstrap_nfc(
        nfd_mapping: &Mapping,
        ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
        backward_combining_starters: &CodePointInversionList,
        candidates: &CodePointInversionList,
        canonical_compositions: &CanonicalCompositions<'static>,
    ) -> Result<ComposingNormalizer, DataError> {
        let nfd = Classifier {
            ccc,
            nfd_mapping,
            backward_combining_starters,
            nfc: None,
        }
        .classify_all(Flavor::Nfd, nfd_mapping, candidates)?;
        let tables = ComplexDecompositionTables::new(&[&nfd])?;
        let mut values = BTreeMap::new();
        encode_all(&nfd, &tables.offsets[0], &mut values)?;

        struct BootstrapProvider {
            data: DecompositionData<'static>,
            tables: DecompositionTables<'static>,
            canonical_compositions: CanonicalCompositions<'static>,
        }

        impl DataProvider<NormalizerNfdDataV1> for BootstrapProvider {
            fn load(&self, _: DataRequest) -> Result<DataResponse<NormalizerNfdDataV1>, DataError> {
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(self.data.clone()),
                })
            }
        }

        impl DataProvider<NormalizerNfdTablesV1> for BootstrapProvider {
            fn load(
                &self,
                _: DataRequest,
            ) -> Result<DataResponse<NormalizerNfdTablesV1>, DataError> {
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(self.tables.clone()),
                })
            }
        }

        impl DataProvider<NormalizerNfcV1> for BootstrapProvider {
            fn load(&self, _: DataRequest) -> Result<DataResponse<NormalizerNfcV1>, DataError> {
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(self.canonical_compositions.clone()),
                })
            }
        }

        ComposingNormalizer::try_new_nfc_unstable(&BootstrapProvider {
            data: DecompositionTrieValues {
                values,
                surrogates_to_replacement_character: true,
                passthrough_cap: 0,
            }
            .build(TrieType::Fast),
            tables: tables.nfd,
            canonical_compositions: canonical_compositions.clone(),
        })
    }
}

impl SourceDataProvider {
    fn normalizer_data(&self) -> Result<&NormalizerData, DataError> {
        let rscd = self.rscd()?;
        rscd.normalizer_cache
            .get_or_init(|| NormalizerData::try_new(self))
            .as_ref()
            .map_err(|&e| e)
    }
}

macro_rules! normalization_provider {
    ($marker:ident, |$self:ident, $data:ident| $conversion:expr) => {
        impl DataProvider<$marker> for SourceDataProvider {
            fn load(&self, req: DataRequest) -> Result<DataResponse<$marker>, DataError> {
                self.check_req::<$marker>(req)?;
                let $self = self;
                let $data = self.normalizer_data()?;
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned($conversion),
                })
            }
        }

        impl crate::IterableDataProviderCached<$marker> for SourceDataProvider {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    };
}

// Always use fast tries for NFD and NFKD to unblock optimizations that depend being able
// to assume the fast trie type at compile time.
// See https://github.com/unicode-org/icu4x/pull/7222#issuecomment-3531679175
normalization_provider!(NormalizerNfdDataV1, |_p, data| data
    .nfd
    .build(TrieType::Fast));
normalization_provider!(NormalizerNfkdDataV1, |_p, data| data
    .nfkd
    .build(TrieType::Fast));
normalization_provider!(NormalizerUts46DataV1, |p, data| data
    .uts46
    .build(p.trie_type()));
normalization_provider!(NormalizerNfdTablesV1, |_p, data| data.nfd_tables.clone());
// No UTS 46 tables, because that data is also in the NFKD tables.
normalization_provider!(NormalizerNfkdTablesV1, |_p, data| data.nfkd_tables.clone());
normalization_provider!(NormalizerNfcV1, |_p, data| data
    .canonical_compositions
    .clone());
normalization_provider!(NormalizerNfdSupplementV1, |p, data| data
    .non_recursive
    .build(p.trie_type()));

// These macros implement ICU4C-internal properties that we have accidentally exposed (#7892).
// They are slated for removal.

macro_rules! impl_decomposition_inert_property {
    ($marker:ident, $try_new_decomp:ident) => {
        impl DataProvider<icu::properties::provider::$marker> for SourceDataProvider {
            fn load(
                &self,
                _req: DataRequest,
            ) -> Result<DataResponse<icu::properties::provider::$marker>, DataError> {
                use icu::collections::codepointinvlist::CodePointInversionListBuilder;
                use icu::normalizer::DecomposingNormalizer;
                use icu::properties::{
                    CodePointMapData, props::CanonicalCombiningClass,
                    provider::PropertyCodePointSet,
                };

                let decomp = DecomposingNormalizer::$try_new_decomp(self)?;
                let decomp = decomp.as_borrowed();
                let ccc = CodePointMapData::<CanonicalCombiningClass>::try_new_unstable(self)?;
                let ccc = ccc.as_borrowed();

                let mut builder = CodePointInversionListBuilder::new();
                // Add all code points that are starters and are not decomposable,
                // including surrogates.
                for cp in 0..=(char::MAX as u32) {
                    let Some(ch) = char::from_u32(cp) else {
                        builder.add32(cp);
                        continue;
                    };

                    if ccc.get(ch) == CanonicalCombiningClass::NotReordered
                        && decomp.is_normalized(ch.encode_utf8(&mut [0; 4]))
                    {
                        builder.add32(cp);
                    }
                }
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(
                        builder.build(),
                    )),
                })
            }
        }
        impl crate::IterableDataProviderCached<icu::properties::provider::$marker>
            for SourceDataProvider
        {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    };
}

macro_rules! impl_composition_inert_property {
    ($marker:ident, $try_new_decomp:ident, $try_new_comp:ident) => {
        impl DataProvider<icu::properties::provider::$marker> for SourceDataProvider {
            fn load(
                &self,
                _req: DataRequest,
            ) -> Result<DataResponse<icu::properties::provider::$marker>, DataError> {
                use icu::collections::codepointinvlist::CodePointInversionListBuilder;
                use icu::normalizer::properties::{
                    CanonicalComposition, CanonicalDecomposition, Decomposed,
                };
                use icu::normalizer::{ComposingNormalizer, DecomposingNormalizer};
                use icu::properties::{
                    CodePointMapData, props::CanonicalCombiningClass,
                    provider::PropertyCodePointSet,
                };

                let composing_normalizer = ComposingNormalizer::$try_new_comp(self)?;
                let composing_normalizer = composing_normalizer.as_borrowed();
                let decomposing_normalizer = DecomposingNormalizer::$try_new_decomp(self)?;
                let decomposing_normalizer = decomposing_normalizer.as_borrowed();
                let nfd = DecomposingNormalizer::try_new_nfd_unstable(self)?;
                let nfd = nfd.as_borrowed();

                let canonical_comp = CanonicalComposition::try_new_unstable(self)?;
                let canonical_comp = canonical_comp.as_borrowed();
                let canonical_decomp = CanonicalDecomposition::try_new_unstable(self)?;
                let canonical_decomp = canonical_decomp.as_borrowed();
                let ccc = CodePointMapData::<CanonicalCombiningClass>::try_new_unstable(self)?;
                let ccc = ccc.as_borrowed();

                let mut combines_forwards = HashSet::new();
                let mut canonical_comp_seconds = HashSet::new();
                let mut composes_with_lowest_reordered_ccc = HashMap::new();

                // Compute `combines_forwards`, `canonical_comp_seconds`, and
                // `composes_with_lowest_reordered_ccc` from the canonical
                // decompositions of all primary composites.
                for ch in (0..=char::MAX as u32).filter_map(char::from_u32) {
                    if let Decomposed::Expansion(starter, second) = canonical_decomp.decompose(ch)
                        && canonical_comp.compose(starter, second) == Some(ch)
                    {
                        combines_forwards.insert(starter);
                        canonical_comp_seconds.insert(second);
                        let ccc = ccc.get(second);
                        if ccc > CanonicalCombiningClass::NotReordered {
                            composes_with_lowest_reordered_ccc
                                .entry(starter)
                                .and_modify(|c| *c = std::cmp::min(*c, ccc))
                                .or_insert(ccc);
                        }
                    }
                }

                let mut combines_backwards = HashSet::new();
                for ch in (0..=char::MAX as u32).filter_map(char::from_u32) {
                    let nfd_first = nfd.normalize_iter([ch].into_iter()).next().unwrap();
                    if canonical_comp_seconds.contains(&nfd_first) {
                        combines_backwards.insert(ch);
                    }
                }

                let mut builder = CodePointInversionListBuilder::new();
                'cp: for cp in 0..=(char::MAX as u32) {
                    let Some(ch) = char::from_u32(cp) else {
                        builder.add32(cp);
                        continue;
                    };

                    if ccc.get(ch) != CanonicalCombiningClass::NotReordered {
                        continue;
                    }

                    if combines_forwards.contains(&ch) {
                        continue;
                    }

                    if !composing_normalizer.is_normalized(ch.encode_utf8(&mut [0; 4])) {
                        continue;
                    }

                    let mut decomposed = decomposing_normalizer.normalize_iter([ch].into_iter());

                    let mut starter = decomposed.next().unwrap();

                    if combines_backwards.contains(&starter) {
                        continue;
                    }

                    for follow in decomposed {
                        if let Some(&lowest_ccc) = composes_with_lowest_reordered_ccc.get(&starter)
                            && lowest_ccc < ccc.get(follow)
                        {
                            // There exists a character C (with
                            // ccc=`lowest_ccc`) which composes with `starter`
                            // and which is not blocked from `starter` in the
                            // sequence <`starter`, `follow`, C>, nor therefore in
                            // <`starter`, (rest of `decomposed`), C>,
                            // thus the decomposition of <`ch`, C> would compose
                            // to something different and `ch` is not inert.
                            continue 'cp;
                        }

                        starter = canonical_comp.compose(starter, follow).unwrap();
                    }

                    builder.add32(cp);
                }
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(PropertyCodePointSet::InversionList(
                        builder.build(),
                    )),
                })
            }
        }
        impl crate::IterableDataProviderCached<icu::properties::provider::$marker>
            for SourceDataProvider
        {
            fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
                Ok(HashSet::from_iter([Default::default()]))
            }
        }
    };
}

impl_decomposition_inert_property!(PropertyBinaryNfdInertV1, try_new_nfd_unstable);
impl_decomposition_inert_property!(PropertyBinaryNfkdInertV1, try_new_nfkd_unstable);
impl_composition_inert_property!(
    PropertyBinaryNfcInertV1,
    try_new_nfd_unstable,
    try_new_nfc_unstable
);
impl_composition_inert_property!(
    PropertyBinaryNfkcInertV1,
    try_new_nfkd_unstable,
    try_new_nfkc_unstable
);
