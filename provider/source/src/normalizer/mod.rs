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
use icu::collections::codepointinvlist::CodePointInversionListBuilder;
use icu::collections::codepointtrie::{CodePointTrie, CodePointTrieBuilder};
use icu::normalizer::provider::*;
use icu::normalizer::{ComposingNormalizer, ComposingNormalizerBorrowed};
use icu::properties::props::{CanonicalCombiningClass, FullCompositionExclusion};
use icu::properties::{
    CodePointMapData, CodePointMapDataBorrowed, CodePointSetData, CodePointSetDataBorrowed,
};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use zerovec::ZeroVec;

const HANGUL_S_BASE: u32 = 0xAC00;
const HANGUL_L_BASE: u32 = 0x1100;
const HANGUL_V_BASE: u32 = 0x1161;
const HANGUL_T_BASE: u32 = 0x11A7;
const HANGUL_T_COUNT: u32 = 28;
const HANGUL_N_COUNT: u32 = 588;
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
    let mut i = 0;
    while i < out.len() {
        if ccc.get(out[i]) == CanonicalCombiningClass::NotReordered {
            i += 1;
            continue;
        }
        let start = i;
        while i < out.len() && ccc.get(out[i]) != CanonicalCombiningClass::NotReordered {
            i += 1;
        }
        out[start..i].sort_by_key(|&c| ccc.get(c));
    }
    out
}

/// Computes the canonical compositions trie and the set of starters that can combine
/// backwards.
fn canonical_compositions(
    nfd_mapping: &Mapping,
    ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
    full_composition_exclusion: CodePointSetDataBorrowed,
) -> (CanonicalCompositions<'static>, HashSet<char>) {
    let mut backward_combining_starters = HashSet::new();
    let mut compositions = Vec::new();
    for composite in (0..=char::MAX as u32).filter_map(char::from_u32) {
        let is_hangul = HANGUL_SYLLABLES.contains(&(composite as u32));
        let (starter, second) = if is_hangul {
            let s = composite as u32 - HANGUL_S_BASE;
            let t = s % HANGUL_T_COUNT;
            if t == 0 {
                // LV syllable
                let l = HANGUL_L_BASE + s / HANGUL_N_COUNT;
                let v = HANGUL_V_BASE + (s % HANGUL_N_COUNT) / HANGUL_T_COUNT;
                (l, v)
            } else {
                // LVT syllable
                (composite as u32 - t, HANGUL_T_BASE + t)
            }
        } else {
            match nfd_mapping(composite) {
                Some(&[starter, second]) if !full_composition_exclusion.contains(composite) => {
                    (starter as u32, second as u32)
                }
                _ => continue,
            }
        };
        let (Some(starter), Some(second)) = (char::from_u32(starter), char::from_u32(second))
        else {
            unreachable!()
        };
        if ccc.get(second) == CanonicalCombiningClass::NotReordered {
            backward_combining_starters.insert(second);
        }
        if is_hangul {
            // Hangul is handled algorithmically at runtime.
            continue;
        }
        compositions.push((starter, second, composite));
    }
    (
        CanonicalCompositions::from_compositions(compositions),
        backward_combining_starters,
    )
}

/// A trie value that might need to be adjusted when the final storage layout is known.
#[derive(Debug, Clone, Copy)]
enum PendingDescriptor {
    Simple(u32),
    Complex {
        round_trips: bool,
        backward_combining: bool,
        supplementary: bool,
        only_non_starters_in_trail: bool,
        len: usize,
        offset: usize,
    },
}

/// Sizes of the 16-bit and 32-bit storage for the NFD tables and the
/// (NFKD and UTS 46) supplementary tables.
#[derive(Debug, Clone, Copy)]
struct StorageLayout {
    base_size_16: usize,
    base_size_32: usize,
    supplement_size_16: usize,
}

impl PendingDescriptor {
    fn resolve(self, c: char, layout: StorageLayout) -> Result<u32, DataError> {
        let (
            round_trips,
            backward_combining,
            supplementary,
            only_non_starters_in_trail,
            len,
            offset,
        ) = match self {
            Self::Simple(value) => return Ok(value),
            Self::Complex {
                round_trips,
                backward_combining,
                supplementary,
                only_non_starters_in_trail,
                len,
                offset,
            } => (
                round_trips,
                backward_combining,
                supplementary,
                only_non_starters_in_trail,
                len,
                offset,
            ),
        };
        // Convert the offset into the 16-bit or 32-bit storage into an offset into
        // the logical concatenation of NFD 16-bit, NFD 32-bit, supplementary 16-bit,
        // and supplementary 32-bit storage.
        let offset = match (
            supplementary,
            offset >= layout.base_size_16,
            offset >= layout.base_size_32,
        ) {
            // NFD 16-bit
            (false, false, _) => offset,
            // Supplementary 16-bit (the 16-bit base data length is already part of offset)
            (false, true, _) => offset + layout.base_size_32,
            // NFD 32-bit
            (true, _, false) => offset + layout.base_size_16,
            // Supplementary 32-bit (the 32-bit base data length is already part of offset)
            (true, _, true) => offset + layout.base_size_16 + layout.supplement_size_16,
        };
        DecompositionData::encode_complex(
            offset,
            len,
            supplementary,
            only_non_starters_in_trail,
            round_trips,
            backward_combining,
        )
        .ok_or_else(|| {
            DataError::custom("Complex decomposition cannot be encoded").with_debug_context(&c)
        })
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

#[derive(Debug)]
struct DecompositionsOutput {
    pending: BTreeMap<char, PendingDescriptor>,
    /// The complex decompositions that were not already present in the previous tables.
    tables: DecompositionTables<'static>,
    decomposition_passthrough_bound: u32,
    composition_passthrough_bound: u32,
    decomposition_starts_with_non_starter: BTreeSet<char>,
    non_recursive: BTreeMap<char, u32>,
    non_recursive_32: Vec<char>,
}

/// Computes decomposition data for `flavor`, whose raw decomposition mappings are `mapping`.
///
/// Complex decompositions are looked up in the concatenation of the `previous` tables, and
/// the returned tables contain those that are not already present there. Offsets in the
/// pending descriptors are relative to the concatenation of the `previous` tables and the
/// returned tables.
///
/// If `nfc` is `None`, all decompositions are treated as not round-tripping via canonical
/// composition.
fn compute_decompositions(
    flavor: Flavor,
    mapping: &Mapping,
    nfd_mapping: &Mapping,
    ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
    backward_combining_starters: &HashSet<char>,
    nfc: Option<&ComposingNormalizerBorrowed>,
    previous: &[&DecompositionTables],
) -> Result<DecompositionsOutput, DataError> {
    let mut storage16 = previous
        .iter()
        .flat_map(|t| t.scalars16.iter())
        .collect::<Vec<_>>();
    let mut storage32 = previous
        .iter()
        .flat_map(|t| t.scalars24.iter())
        .collect::<Vec<_>>();
    let previous_len_16 = storage16.len();
    let previous_len_32 = storage32.len();

    let error = |message: &'static str, c: char| {
        DataError::custom(message)
            .with_debug_context(&c)
            .with_debug_context(&flavor)
    };

    let mut output = DecompositionsOutput {
        pending: BTreeMap::new(),
        tables: DecompositionTables {
            scalars16: ZeroVec::new(),
            scalars24: ZeroVec::new(),
        },
        decomposition_passthrough_bound: 0x10FFFF,
        composition_passthrough_bound: 0x10FFFF,
        decomposition_starts_with_non_starter: BTreeSet::new(),
        non_recursive: BTreeMap::new(),
        non_recursive_32: Vec::new(),
    };

    // Iterate over all scalar values excluding Hangul syllables.
    //
    // We go backwards in order to better find overlapping decompositions.
    for c in (0..=char::MAX as u32).rev().filter_map(char::from_u32) {
        if HANGUL_SYLLABLES.contains(&(c as u32)) {
            continue;
        }
        if c == char::REPLACEMENT_CHARACTER {
            // This character is a starter that decomposes to self, so without a special
            // case here it would end up as passthrough-eligible in all normalizations forms.
            // However, in the potentially-ill-formed UTF-8 case UTF-8 errors return U+FFFD
            // from the iterator, and errors need to be treated as ineligible for passthrough
            // on the slice fast path. By giving U+FFFD a trie value whose flags make it
            // ineligible for passthrough avoids a specific U+FFFD branch on the passthrough
            // fast path.
            output.pending.insert(
                c,
                PendingDescriptor::Simple(
                    DecompositionData::NON_ROUND_TRIP_MARKER
                        | DecompositionData::BACKWARD_COMBINING_MARKER,
                ),
            );
            continue;
        }

        let dst = full_decomposition(c, mapping, ccc);
        let decomposes_to_self = dst == [c];
        let round_trips_via_canonical_composition =
            || nfc.is_some_and(|nfc| nfc.normalize_iter(dst.iter().copied()).eq([c]));

        if flavor != Flavor::Uts46
            && (dst.is_empty()
                || (dst == [char::REPLACEMENT_CHARACTER] && c != char::REPLACEMENT_CHARACTER))
        {
            return Err(error("Unexpected empty or U+FFFD decomposition", c));
        }

        let first = dst.first().copied();
        let first_combining_class = first.map(|f| ccc.get(f).0).unwrap_or_default();
        let mut special_non_starter_decomposition = false;
        let mut starts_with_backward_combining_starter = false;
        if first_combining_class != 0 {
            output.decomposition_passthrough_bound = c as u32;
            output.composition_passthrough_bound = c as u32;
            output.decomposition_starts_with_non_starter.insert(c);
            if !decomposes_to_self {
                if DecompositionData::SPECIAL_NON_STARTER_DECOMPOSITIONS
                    .iter()
                    .any(|&(s, d)| s == c && d == dst.as_slice())
                {
                    special_non_starter_decomposition = true;
                } else {
                    // A character whose decomposition starts with a non-starter and isn't the
                    // same as the character itself and isn't already hard-coded into ICU4X.
                    return Err(error("Unexpected non-starter decomposition", c));
                }
            }
        } else if first.is_some_and(|f| backward_combining_starters.contains(&f)) {
            output.composition_passthrough_bound = c as u32;
            starts_with_backward_combining_starter = true;
        }

        if flavor != Flavor::Nfd {
            if dst == full_decomposition(c, nfd_mapping, ccc) {
                continue;
            }
            output.decomposition_passthrough_bound = c as u32;
            output.composition_passthrough_bound = c as u32;
        }

        if first_combining_class != 0 {
            output.pending.insert(
                c,
                PendingDescriptor::Simple(if special_non_starter_decomposition {
                    DecompositionData::encode_special_non_starter(ccc.get(c).0)
                } else {
                    DecompositionData::encode_non_starter(first_combining_class)
                }),
            );
            continue;
        }

        if decomposes_to_self {
            if starts_with_backward_combining_starter {
                output.pending.insert(
                    c,
                    PendingDescriptor::Simple(DecompositionData::BACKWARD_COMBINING_MARKER),
                );
            }
            continue;
        }

        output.decomposition_passthrough_bound = c as u32;

        // ICU4X hard-codes ANGSTROM SIGN
        if c != DecompositionData::ANGSTROM_SIGN && flavor == Flavor::Nfd {
            let raw =
                nfd_mapping(c).ok_or_else(|| error("Missing non-recursive decomposition", c))?;
            // In addition to actual difference, put the whole range that contains characters
            // with oxia into the non-recursive trie in order to catch cases where characters
            // with oxia have singleton decompositions to corresponding characters with tonos.
            // This way, the run-time decision to fall through can be done on the range
            // without checking for individual characters inside the range.
            if raw != dst.as_slice() || NonRecursiveDecompositionSupplement::OXIA_RANGE.contains(&c)
            {
                let value =
                    NonRecursiveDecompositionSupplement::encode(raw, &mut output.non_recursive_32)
                        .ok_or_else(|| error("Non-recursive decomposition cannot be encoded", c))?;
                output.non_recursive.insert(c, value);
            }
        }

        let round_trips = round_trips_via_canonical_composition();
        if !round_trips {
            output.composition_passthrough_bound = c as u32;
        }

        let is_special_in_flavor = |d: char| {
            (d == DecompositionData::IOTA_SUBSCRIPT && flavor == Flavor::Uts46)
                || d == DecompositionData::HALF_WIDTH_VOICED_SOUND_MARK
                || d == DecompositionData::HALF_WIDTH_SEMI_VOICED_SOUND_MARK
        };

        let descriptor = match *dst {
            [] => {
                // Ignorable in UTS 46 (the non-UTS 46 case has errored above).
                PendingDescriptor::Simple(DecompositionData::IGNORABLE_MARKER)
            }
            // Singleton decompositions to conjoining jamo.
            [d] if ('\u{1161}'..='\u{1175}').contains(&d)
                || ('\u{11A8}'..='\u{11C2}').contains(&d) =>
            {
                if flavor == Flavor::Nfd {
                    return Err(error("Unexpected decomposition to conjoining jamo", c));
                }
                PendingDescriptor::Simple(
                    DecompositionData::encode_singleton(d, starts_with_backward_combining_starter)
                        .ok_or_else(|| error("Singleton cannot be encoded", c))?,
                )
            }
            [d] if !starts_with_backward_combining_starter && d as u32 <= 0xFFFF => {
                PendingDescriptor::Simple(
                    DecompositionData::encode_singleton(d, false)
                        .ok_or_else(|| error("Singleton cannot be encoded", c))?,
                )
            }
            // ANGSTROM SIGN is special to make the Harfbuzz case branch less in the more common case.
            [a, b]
                if c != DecompositionData::ANGSTROM_SIGN
                    && !starts_with_backward_combining_starter
                    && ccc.get(a).0 == 0
                    && ccc.get(b).0 != 0
                    && let Some(value) = DecompositionData::encode_pair(a, b, round_trips) =>
            {
                if is_special_in_flavor(a) || is_special_in_flavor(b) {
                    // Iota subscript and half-width voicing marks must never occur in these
                    // expansions in the normalization forms where they are special.
                    return Err(error("Unexpected special character in expansion", c));
                }
                PendingDescriptor::Simple(value)
            }
            _ => {
                if dst.iter().copied().any(is_special_in_flavor) {
                    // Iota subscript and half-width voicing marks must never occur in these
                    // expansions in the normalization forms where they are special.
                    return Err(error("Unexpected special character in expansion", c));
                }
                if dst.contains(&'\0') {
                    return Err(error("Unexpected NUL in expansion", c));
                }
                let non_initial_starter = dst.iter().skip(1).any(|&d| ccc.get(d).0 == 0);
                // The format doesn't allow for length 1 for BMP, so these need to be
                // promoted to wider storage.
                let supplementary = dst.len() == 1 || dst.iter().any(|&d| d as u32 > 0xFFFF);
                // Check whether the length can be encoded (offset 0 is always encodable).
                if DecompositionData::encode_complex(
                    0,
                    dst.len(),
                    supplementary,
                    false,
                    true,
                    false,
                )
                .is_none()
                {
                    if c == '\u{FDFA}'
                        && dst
                            .iter()
                            .skip(1)
                            .map(|&d| d as u32)
                            .eq(DecompositionData::FDFA_NFKD.iter().map(|&d| u32::from(d)))
                    {
                        // Special marker for the one character whose decomposition is too long.
                        output.pending.insert(
                            c,
                            PendingDescriptor::Simple(DecompositionData::encode_fdfa()),
                        );
                        continue;
                    }
                    return Err(error("Decomposition too long", c));
                }
                let offset = if supplementary {
                    find_or_append(&mut storage32, &dst)
                } else {
                    find_or_append(
                        &mut storage16,
                        &dst.iter().map(|&d| d as u16).collect::<Vec<_>>(),
                    )
                };
                PendingDescriptor::Complex {
                    round_trips,
                    backward_combining: starts_with_backward_combining_starter,
                    supplementary,
                    only_non_starters_in_trail: !non_initial_starter,
                    len: dst.len(),
                    offset,
                }
            }
        };
        output.pending.insert(c, descriptor);
    }

    if storage16.len() + storage32.len() > DecompositionTables::MAX_COMBINED_LEN {
        // We actually have 14 bits available, but let's error out so that docs can be
        // updated when taking a reserved bit out of potential future flag usage.
        return Err(
            DataError::custom("Decomposition storage too large").with_debug_context(&flavor)
        );
    }

    output.tables = DecompositionTables {
        scalars16: ZeroVec::alloc_from_slice(&storage16[previous_len_16..]),
        scalars24: ZeroVec::alloc_from_slice(&storage32[previous_len_32..]),
    };
    Ok(output)
}

fn resolve_all(
    pending: &BTreeMap<char, PendingDescriptor>,
    layout: StorageLayout,
    values: &mut BTreeMap<char, u32>,
) -> Result<(), DataError> {
    for (&c, &descriptor) in pending {
        values.insert(c, descriptor.resolve(c, layout)?);
    }
    Ok(())
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

#[derive(Debug)]
pub(crate) struct NormalizerData {
    nfd: DecompositionTrieValues,
    nfkd: DecompositionTrieValues,
    uts46: DecompositionTrieValues,
    nfd_tables: DecompositionTables<'static>,
    nfkd_tables: DecompositionTables<'static>,
    canonical_compositions: CanonicalCompositions<'static>,
    non_recursive: BTreeMap<char, u32>,
    non_recursive_32: Vec<char>,
}

/// Validates the expectations that ICU4X hard-codes about which characters'
/// decompositions start with a non-starter.
fn check_non_starters(
    flavor: Flavor,
    set: &BTreeSet<char>,
    nfd_set: &BTreeSet<char>,
) -> Result<(), DataError> {
    let error = |message: &'static str| DataError::custom(message).with_debug_context(&flavor);
    if nfd_set.contains(&DecompositionData::HALF_WIDTH_VOICED_SOUND_MARK)
        || nfd_set.contains(&DecompositionData::HALF_WIDTH_SEMI_VOICED_SOUND_MARK)
        || !nfd_set.contains(&DecompositionData::IOTA_SUBSCRIPT)
    {
        // NFD expectations don't hold. The set must not contain the half-width
        // kana voicing marks and must contain iota subscript.
        return Err(error("Unexpected NFD non-starters"));
    }
    let half_width_check = set.difference(nfd_set).copied().collect::<Vec<_>>();
    if !half_width_check.is_empty()
        && half_width_check
            != [
                DecompositionData::HALF_WIDTH_VOICED_SOUND_MARK,
                DecompositionData::HALF_WIDTH_SEMI_VOICED_SOUND_MARK,
            ]
    {
        // The result was neither empty nor contained exactly the two half-width voicing
        // marks. The ICU4X normalizer doesn't know how to deal with this case.
        return Err(error("Unexpected additional non-starters"));
    }
    let iota_check = nfd_set.difference(set).copied().collect::<Vec<_>>();
    if !iota_check.is_empty() && iota_check != [DecompositionData::IOTA_SUBSCRIPT] {
        // The result was neither empty nor contained exactly the iota subscript. The
        // ICU4X normalizer doesn't know how to deal with this case.
        return Err(error("Unexpected missing non-starters"));
    }
    Ok(())
}

/// Checks that the passthrough bounds are within the ranges ICU4X expects, and returns
/// the passthrough cap for the data.
fn passthrough_cap(output: &DecompositionsOutput, flavor: Flavor) -> Result<u16, DataError> {
    let error = || DataError::custom("Unexpected passthrough bounds").with_debug_context(&flavor);
    let decomposition_bound = output.decomposition_passthrough_bound;
    let composition_bound = output.composition_passthrough_bound;
    if flavor == Flavor::Nfd {
        if !(decomposition_bound == 0xC0 && composition_bound == 0x300) {
            return Err(error());
        }
    } else if !(decomposition_bound <= 0xC0 && composition_bound <= 0x300)
        || (composition_bound > 0xC0 && decomposition_bound != 0xC0)
        || (composition_bound <= 0xC0 && decomposition_bound != composition_bound)
    {
        return Err(error());
    }
    Ok(composition_bound as u16)
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

        let nfc = Self::bootstrap_nfc(
            &nfd_mapping,
            ccc,
            &backward_combining_starters,
            &canonical_compositions,
        )?;
        let nfc = nfc.as_borrowed();

        let nfd = compute_decompositions(
            Flavor::Nfd,
            &nfd_mapping,
            &nfd_mapping,
            ccc,
            &backward_combining_starters,
            Some(&nfc),
            &[],
        )?;
        let nfd_cap = passthrough_cap(&nfd, Flavor::Nfd)?;

        let nfkd = compute_decompositions(
            Flavor::Nfkd,
            &nfkd_mapping,
            &nfd_mapping,
            ccc,
            &backward_combining_starters,
            Some(&nfc),
            &[&nfd.tables],
        )?;
        let nfkd_cap = passthrough_cap(&nfkd, Flavor::Nfkd)?;
        check_non_starters(
            Flavor::Nfkd,
            &nfkd.decomposition_starts_with_non_starter,
            &nfd.decomposition_starts_with_non_starter,
        )?;

        let uts46 = compute_decompositions(
            Flavor::Uts46,
            &uts46_mapping,
            &nfd_mapping,
            ccc,
            &backward_combining_starters,
            Some(&nfc),
            &[&nfd.tables, &nfkd.tables],
        )?;
        let uts46_cap = passthrough_cap(&uts46, Flavor::Uts46)?;
        check_non_starters(
            Flavor::Uts46,
            &uts46.decomposition_starts_with_non_starter,
            &nfd.decomposition_starts_with_non_starter,
        )?;

        // The UTS 46 decompositions that are not in the NFD or NFKD tables are added
        // to the supplementary tables.
        let nfkd_tables = DecompositionTables {
            scalars16: nfkd
                .tables
                .scalars16
                .iter()
                .chain(uts46.tables.scalars16.iter())
                .collect(),
            scalars24: nfkd
                .tables
                .scalars24
                .iter()
                .chain(uts46.tables.scalars24.iter())
                .collect(),
        };

        let layout = StorageLayout {
            base_size_16: nfd.tables.scalars16.len(),
            base_size_32: nfd.tables.scalars24.len(),
            supplement_size_16: nfkd_tables.scalars16.len(),
        };

        // The NFKD and UTS 46 tries are the NFD trie, partially overwritten.
        let mut nfd_values = BTreeMap::new();
        resolve_all(&nfd.pending, layout, &mut nfd_values)?;
        let mut nfkd_values = nfd_values.clone();
        resolve_all(&nfkd.pending, layout, &mut nfkd_values)?;
        let mut uts46_values = nfd_values.clone();
        resolve_all(&uts46.pending, layout, &mut uts46_values)?;

        Ok(Self {
            nfd: DecompositionTrieValues {
                values: nfd_values,
                surrogates_to_replacement_character: true,
                passthrough_cap: nfd_cap,
            },
            nfkd: DecompositionTrieValues {
                values: nfkd_values,
                surrogates_to_replacement_character: true,
                passthrough_cap: nfkd_cap,
            },
            uts46: DecompositionTrieValues {
                values: uts46_values,
                // UTS 46 is not supported in slice modes
                surrogates_to_replacement_character: false,
                passthrough_cap: uts46_cap,
            },
            nfd_tables: nfd.tables,
            nfkd_tables,
            canonical_compositions,
            non_recursive: nfd.non_recursive,
            non_recursive_32: nfd.non_recursive_32,
        })
    }

    /// Constructs an NFC normalizer from NFD data where all decompositions are marked as
    /// not round-tripping, and where passthrough is disabled.
    fn bootstrap_nfc(
        nfd_mapping: &Mapping,
        ccc: CodePointMapDataBorrowed<CanonicalCombiningClass>,
        backward_combining_starters: &HashSet<char>,
        canonical_compositions: &CanonicalCompositions<'static>,
    ) -> Result<ComposingNormalizer, DataError> {
        let nfd = compute_decompositions(
            Flavor::Nfd,
            nfd_mapping,
            nfd_mapping,
            ccc,
            backward_combining_starters,
            None,
            &[],
        )?;
        let mut values = BTreeMap::new();
        resolve_all(
            &nfd.pending,
            StorageLayout {
                base_size_16: nfd.tables.scalars16.len(),
                base_size_32: nfd.tables.scalars24.len(),
                supplement_size_16: 0,
            },
            &mut values,
        )?;

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
            tables: nfd.tables,
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
normalization_provider!(NormalizerNfdSupplementV1, |p, data| {
    let mut builder = CodePointTrieBuilder::new(0, 0, p.trie_type().into());
    for (&c, &value) in &data.non_recursive {
        builder.set_value(c as u32, value);
    }
    let trie: CodePointTrie<'static, u32> = builder.build();
    NonRecursiveDecompositionSupplement {
        trie,
        scalars24: data.non_recursive_32.iter().copied().collect(),
    }
});

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
