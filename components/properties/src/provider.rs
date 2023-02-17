// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! 🚧 \[Unstable\] Data provider struct definitions for this ICU4X component.
//!
//! <div class="stab unstable">
//! 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
//! including in SemVer minor releases. While the serde representation of data structs is guaranteed
//! to be stable, their Rust representation might not be. Use with caution.
//! </div>
//!
//! Read more about data providers: [`icu_provider`]

use crate::script::ScriptWithExt;
use crate::Script;

use alloc::boxed::Box;
use core::cmp::Ordering;
use core::fmt;
use core::ops::RangeInclusive;
use core::str;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_collections::codepointinvliststringlist::CodePointInversionListAndStringList;
use icu_collections::codepointtrie::{CodePointMapRange, CodePointTrie, TrieValue};
use icu_provider::prelude::*;
use zerofrom::ZeroFrom;

use zerovec::{
    maps::ZeroMapKV, ule::VarULE, VarZeroSlice, VarZeroVec, ZeroMap, ZeroSlice, ZeroVecError,
};

/// A set of characters which share a particular property value.
///
/// This data enum is extensible, more backends may be added in the future.
/// Old data can be used with newer code but not vice versa.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Eq, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyCodePointSetV1<'data> {
    /// The set of characters, represented as an inversion list
    InversionList(#[cfg_attr(feature = "serde", serde(borrow))] CodePointInversionList<'data>),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

/// A map efficiently storing data about individual characters.
///
/// This data enum is extensible, more backends may be added in the future.
/// Old data can be used with newer code but not vice versa.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Clone, Debug, Eq, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyCodePointMapV1<'data, T: TrieValue> {
    /// A codepoint trie storing the data
    CodePointTrie(#[cfg_attr(feature = "serde", serde(borrow))] CodePointTrie<'data, T>),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

/// A set of characters and strings which share a particular property value.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Eq, PartialEq, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum PropertyUnicodeSetV1<'data> {
    /// A set representing characters in an inversion list, and the strings in a list.
    CPInversionListStrList(
        #[cfg_attr(feature = "serde", serde(borrow))] CodePointInversionListAndStringList<'data>,
    ),
    // new variants should go BELOW existing ones
    // Serde serializes based on variant name and index in the enum
    // https://docs.rs/serde/latest/serde/trait.Serializer.html#tymethod.serialize_unit_variant
}

impl<'data> PropertyUnicodeSetV1<'data> {
    #[inline]
    pub(crate) fn contains(&self, s: &str) -> bool {
        match *self {
            Self::CPInversionListStrList(ref l) => l.contains(s),
        }
    }

    #[inline]
    pub(crate) fn contains32(&self, cp: u32) -> bool {
        match *self {
            Self::CPInversionListStrList(ref l) => l.contains32(cp),
        }
    }

    #[inline]
    pub(crate) fn contains_char(&self, ch: char) -> bool {
        match *self {
            Self::CPInversionListStrList(ref l) => l.contains_char(ch),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_inversion_list_string_list(
        l: CodePointInversionListAndStringList<'static>,
    ) -> Self {
        Self::CPInversionListStrList(l)
    }

    #[inline]
    pub(crate) fn as_code_point_inversion_list_string_list(
        &'_ self,
    ) -> Option<&'_ CodePointInversionListAndStringList<'data>> {
        match *self {
            Self::CPInversionListStrList(ref l) => Some(l),
            // any other backing data structure that cannot return a CPInversionListStrList in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_inversion_list_string_list(
        &self,
    ) -> CodePointInversionListAndStringList<'_> {
        match *self {
            Self::CPInversionListStrList(ref t) => ZeroFrom::zero_from(t),
        }
    }
}

/// A struct that efficiently stores `Script` and `Script_Extensions` property data.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[icu_provider::data_struct(ScriptWithExtensionsPropertyV1Marker = "props/scx@1")]
#[derive(Debug, Eq, PartialEq, Clone)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ScriptWithExtensionsPropertyV1<'data> {
    /// Note: The `ScriptWithExt` values in this array will assume a 12-bit layout. The 2
    /// higher order bits 11..10 will indicate how to deduce the Script value and
    /// Script_Extensions value, nearly matching the representation
    /// [in ICU](https://github.com/unicode-org/icu/blob/main/icu4c/source/common/uprops.h):
    ///
    /// | High order 2 bits value | Script                                                 | Script_Extensions                                              |
    /// |-------------------------|--------------------------------------------------------|----------------------------------------------------------------|
    /// | 3                       | First value in sub-array, index given by lower 10 bits | Sub-array excluding first value, index given by lower 10 bits  |
    /// | 2                       | Script=Inherited                                       | Entire sub-array, index given by lower 10 bits                 |
    /// | 1                       | Script=Common                                          | Entire sub-array, index given by lower 10 bits                 |
    /// | 0                       | Value in lower 10 bits                                 | `[ Script value ]` single-element array                        |
    ///
    /// When the lower 10 bits of the value are used as an index, that index is
    /// used for the outer-level vector of the nested `extensions` structure.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub trie: CodePointTrie<'data, ScriptWithExt>,

    /// This companion structure stores Script_Extensions values, which are
    /// themselves arrays / vectors. This structure only stores the values for
    /// cases in which `scx(cp) != [ sc(cp) ]`. Each sub-vector is distinct. The
    /// sub-vector represents the Script_Extensions array value for a code point,
    /// and may also indicate Script value, as described for the `trie` field.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub extensions: VarZeroVec<'data, ZeroSlice<Script>>,
}

impl<'data> ScriptWithExtensionsPropertyV1<'data> {
    // This method is intended to be used by constructors of deserialized data
    // in a data provider.
    #[doc(hidden)]
    pub fn new(
        trie: CodePointTrie<'data, ScriptWithExt>,
        extensions: VarZeroVec<'data, ZeroSlice<Script>>,
    ) -> ScriptWithExtensionsPropertyV1<'data> {
        ScriptWithExtensionsPropertyV1 { trie, extensions }
    }
}

// See CodePointSetData for documentation of these functions
impl<'data> PropertyCodePointSetV1<'data> {
    #[inline]
    pub(crate) fn contains(&self, ch: char) -> bool {
        match *self {
            Self::InversionList(ref l) => l.contains(ch),
        }
    }

    #[inline]
    pub(crate) fn contains32(&self, ch: u32) -> bool {
        match *self {
            Self::InversionList(ref l) => l.contains32(ch),
        }
    }

    #[inline]
    pub(crate) fn iter_ranges(&self) -> impl Iterator<Item = RangeInclusive<u32>> + '_ {
        match *self {
            Self::InversionList(ref l) => l.iter_ranges(),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_inversion_list(l: CodePointInversionList<'static>) -> Self {
        Self::InversionList(l)
    }

    #[inline]
    pub(crate) fn as_code_point_inversion_list(
        &'_ self,
    ) -> Option<&'_ CodePointInversionList<'data>> {
        match *self {
            Self::InversionList(ref l) => Some(l),
            // any other backing data structure that cannot return a CPInvList in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_inversion_list(&self) -> CodePointInversionList<'_> {
        match *self {
            Self::InversionList(ref t) => ZeroFrom::zero_from(t),
        }
    }
}

// See CodePointMapData for documentation of these functions
impl<'data, T: TrieValue> PropertyCodePointMapV1<'data, T> {
    #[inline]
    pub(crate) fn get32(&self, ch: u32) -> T {
        match *self {
            Self::CodePointTrie(ref t) => t.get32(ch),
        }
    }

    #[inline]
    pub(crate) fn try_into_converted<P>(
        self,
    ) -> Result<PropertyCodePointMapV1<'data, P>, ZeroVecError>
    where
        P: TrieValue,
    {
        match self {
            Self::CodePointTrie(t) => t
                .try_into_converted()
                .map(PropertyCodePointMapV1::CodePointTrie),
        }
    }

    #[inline]
    pub(crate) fn get_set_for_value(&self, value: T) -> CodePointInversionList<'static> {
        match *self {
            Self::CodePointTrie(ref t) => t.get_set_for_value(value),
        }
    }

    #[inline]
    pub(crate) fn iter_ranges(&self) -> impl Iterator<Item = CodePointMapRange<T>> + '_ {
        match *self {
            Self::CodePointTrie(ref t) => t.iter_ranges(),
        }
    }

    #[inline]
    pub(crate) fn from_code_point_trie(trie: CodePointTrie<'static, T>) -> Self {
        Self::CodePointTrie(trie)
    }

    #[inline]
    pub(crate) fn as_code_point_trie(&self) -> Option<&CodePointTrie<'data, T>> {
        match *self {
            Self::CodePointTrie(ref t) => Some(t),
            // any other backing data structure that cannot return a CPT in O(1) time should return None
        }
    }

    #[inline]
    pub(crate) fn to_code_point_trie(&self) -> CodePointTrie<'_, T> {
        match *self {
            Self::CodePointTrie(ref t) => ZeroFrom::zero_from(t),
        }
    }
}

/// This is a property name that can be "loose matched" as according to
/// https://www.unicode.org/Public/UCD/latest/ucd/PropertyValueAliases.txt
///
/// (matched case-insensitively in ASCII, ignoring underscores, whitespace, and hyphens)
///
/// This is expected to be ASCII, but we do not rely on this invariant anywhere except during
/// datagen.
///
/// The Ord impl will sort things using strict equality, but in such a way that all loose-equal items
/// will sort into the same area, such that the map can be searched for both strict and loose equality.
///
/// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(PartialEq, Eq)] // VarULE wants these to be byte equality
#[derive(VarULE)]
#[repr(transparent)]
pub struct NormalizedPropertyNameStr([u8]);

#[cfg(feature = "datagen")]
impl serde::Serialize for NormalizedPropertyNameStr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::Error;
        if serializer.is_human_readable() {
            let s = str::from_utf8(&self.0)
                .map_err(|_| S::Error::custom("Attempted to datagen invalid string property"))?;
            serializer.serialize_str(s)
        } else {
            serializer.serialize_bytes(&self.0)
        }
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Box<NormalizedPropertyNameStr> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use alloc::borrow::Cow;
        let s; // lifetime lengthening
        let b;
        // Can be improved with https://github.com/unicode-org/icu4x/issues/2310
        // the allocations here are fine, in normal ICU4X code they'll only get hit
        // during human-readable deserialization
        let bytes = if deserializer.is_human_readable() {
            s = <Cow<str>>::deserialize(deserializer)?;
            s.as_bytes()
        } else {
            b = <Cow<[u8]>>::deserialize(deserializer)?;
            &b
        };
        Ok(NormalizedPropertyNameStr::boxed_from_bytes(bytes))
    }
}

impl<'a> ZeroMapKV<'a> for NormalizedPropertyNameStr {
    type Container = VarZeroVec<'a, NormalizedPropertyNameStr>;
    type Slice = VarZeroSlice<NormalizedPropertyNameStr>;
    type GetType = NormalizedPropertyNameStr;
    type OwnedType = Box<NormalizedPropertyNameStr>;
}

impl PartialOrd for NormalizedPropertyNameStr {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Normalize a character based on the "loose matching" described in PropertyValueAliases.txt,
/// returning None for skippable characters
///
/// ICU has [code for this][1] (and [during property lookup][2]) which we emulate.
/// In particular, ICU only does normalization within ASCII, which makes sense since character names
/// seem to be only ASCII.
///
/// [1]: https://github.com/unicode-org/icu/blob/288c4c7555915ce7b1fb675d94ddd495058fc039/icu4c/source/common/propname.cpp#L35
/// [2]: https://github.com/unicode-org/icu/blob/288c4c7555915ce7b1fb675d94ddd495058fc039/icu4c/source/common/propname.cpp#L226-L230
fn normalize_char(ch: u8) -> Option<u8> {
    match ch {
        // all ascii whitespace
        ch if ch.is_ascii_whitespace() => None,
        // underscores, hyphens
        b'_' | b'-' | b' ' | 0x09..=0x0d => None,
        // ignore case by lowercasing
        ch => Some(ch.to_ascii_lowercase()),
    }
}

impl Ord for NormalizedPropertyNameStr {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self.cmp_loose(other);
        // When loose equality holds, fall back to strict equality
        if cmp == Ordering::Equal {
            self.0.cmp(&other.0)
        } else {
            cmp
        }
    }
}

impl fmt::Debug for NormalizedPropertyNameStr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Ok(s) = str::from_utf8(&self.0) {
            f.write_str(s)
        } else {
            f.write_str("(invalid utf8)")
        }
    }
}

impl NormalizedPropertyNameStr {
    pub(crate) fn cmp_loose(&self, other: &Self) -> Ordering {
        let self_iter = self.0.iter().copied().filter_map(normalize_char);
        let other_iter = other.0.iter().copied().filter_map(normalize_char);
        self_iter.cmp(other_iter)
    }
    #[cfg(feature = "serde")]
    /// Get a Box<NormalizedPropertyName> from a byte slice
    pub fn boxed_from_bytes(b: &[u8]) -> Box<Self> {
        #[allow(clippy::expect_used)] // Self has no invariants
        // can be cleaned up with https://github.com/unicode-org/icu4x/issues/2310
        let this = Self::parse_byte_slice(b).expect("NormalizedPropertyName has no invariants");

        zerovec::ule::encode_varule_to_box(&this)
    }
}

/// A set of characters and strings which share a particular property value.
///
//// <div class="stab unstable">
/// 🚧 This code is considered unstable; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. While the serde representation of data structs is guaranteed
/// to be stable, their Rust representation might not be. Use with caution.
/// </div>
#[derive(Debug, Clone, yoke::Yokeable, zerofrom::ZeroFrom)]
#[cfg_attr(
    feature = "datagen", 
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[yoke(prove_covariance_manually)]
pub struct PropertyValueNameToEnumMapV1<'data> {
    /// A map from names to their value discriminant
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub map: ZeroMap<'data, NormalizedPropertyNameStr, u16>,
}

macro_rules! expand {
    (
        ($(($code_point_set_marker:ident, $bin_cp_s:literal),)+),
        ($(($unicode_set_marker:ident, $bin_us_s:literal),)+),
        ($(($code_point_map_marker:ident, $value_name_marker:ident, $enum_s:literal, $value_ty:ident),)+)
    ) => {

            // Data keys that return code point sets (represented as CodePointSetData).
            // For now, synonymous with binary properties of code points only.
            $(
                #[doc = core::concat!("Data marker for the '", $bin_cp_s, "' Unicode property")]
                pub struct $code_point_set_marker;

                impl DataMarker for $code_point_set_marker {
                    type Yokeable = PropertyCodePointSetV1<'static>;
                }
                impl KeyedDataMarker for $code_point_set_marker {
                    const KEY: DataKey = data_key!(concat!("props/", $bin_cp_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $code_point_set_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $code_point_set_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$code_point_set_marker }
                    }
                }
            )+

            // Data keys that return sets of strings + code points (represented as UnicodeSetData).
            // Includes:
            //   - binary properties of strings + code points
            //   - exemplar characters
            $(
                #[doc = core::concat!("Data marker for the '", $bin_us_s, "' Unicode property")]
                pub struct $unicode_set_marker;

                impl DataMarker for $unicode_set_marker {
                    type Yokeable = PropertyUnicodeSetV1<'static>;
                }
                impl KeyedDataMarker for $unicode_set_marker {
                    const KEY: DataKey = data_key!(concat!("props/", $bin_us_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $unicode_set_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $unicode_set_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$unicode_set_marker }
                    }
                }
            )+

            // Data keys that return code point map (represented as CodePointMapData).
            // For now, synonymous with enumerated properties [of code points only].
            $(
                #[doc = core::concat!("Data marker for the '", $enum_s, "' Unicode property")]
                pub struct $code_point_map_marker;

                impl DataMarker for $code_point_map_marker {
                    type Yokeable = PropertyCodePointMapV1<'static, crate::$value_ty>;
                }

                impl KeyedDataMarker for $code_point_map_marker {
                    const KEY: DataKey = data_key!(concat!("props/", $enum_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $code_point_map_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $code_point_map_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$code_point_map_marker }
                    }
                }

                #[doc = core::concat!("Data marker for the names of the values of the '", $enum_s, "' Unicode property")]
                pub struct $value_name_marker;

                impl DataMarker for $value_name_marker {
                    type Yokeable = PropertyValueNameToEnumMapV1<'static>;
                }

                impl KeyedDataMarker for $value_name_marker {
                    const KEY: DataKey = data_key!(concat!("props/names/", $enum_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $value_name_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $value_name_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$value_name_marker }
                    }
                }
            )+
    };
}

expand!(
    (
        // code point sets
        (AsciiHexDigitV1Marker, "AHex"),
        (AlnumV1Marker, "alnum"),
        (AlphabeticV1Marker, "Alpha"),
        (BidiControlV1Marker, "Bidi_C"),
        (BidiMirroredV1Marker, "Bidi_M"),
        (BlankV1Marker, "blank"),
        (CasedV1Marker, "Cased"),
        (CaseIgnorableV1Marker, "CI"),
        (FullCompositionExclusionV1Marker, "Comp_Ex"),
        (ChangesWhenCasefoldedV1Marker, "CWCF"),
        (ChangesWhenCasemappedV1Marker, "CWCM"),
        (ChangesWhenNfkcCasefoldedV1Marker, "CWKCF"),
        (ChangesWhenLowercasedV1Marker, "CWL"),
        (ChangesWhenTitlecasedV1Marker, "CWT"),
        (ChangesWhenUppercasedV1Marker, "CWU"),
        (DashV1Marker, "Dash"),
        (DeprecatedV1Marker, "Dep"),
        (DefaultIgnorableCodePointV1Marker, "DI"),
        (DiacriticV1Marker, "Dia"),
        (EmojiModifierBaseV1Marker, "EBase"),
        (EmojiComponentV1Marker, "EComp"),
        (EmojiModifierV1Marker, "EMod"),
        (EmojiV1Marker, "Emoji"),
        (EmojiPresentationV1Marker, "EPres"),
        (ExtenderV1Marker, "Ext"),
        (ExtendedPictographicV1Marker, "ExtPict"),
        (GraphV1Marker, "graph"),
        (GraphemeBaseV1Marker, "Gr_Base"),
        (GraphemeExtendV1Marker, "Gr_Ext"),
        (GraphemeLinkV1Marker, "Gr_Link"),
        (HexDigitV1Marker, "Hex"),
        (HyphenV1Marker, "Hyphen"),
        (IdContinueV1Marker, "IDC"),
        (IdeographicV1Marker, "Ideo"),
        (IdStartV1Marker, "IDS"),
        (IdsBinaryOperatorV1Marker, "IDSB"),
        (IdsTrinaryOperatorV1Marker, "IDST"),
        (JoinControlV1Marker, "Join_C"),
        (LogicalOrderExceptionV1Marker, "LOE"),
        (LowercaseV1Marker, "Lower"),
        (MathV1Marker, "Math"),
        (NoncharacterCodePointV1Marker, "NChar"),
        (NfcInertV1Marker, "nfcinert"),
        (NfdInertV1Marker, "nfdinert"),
        (NfkcInertV1Marker, "nfkcinert"),
        (NfkdInertV1Marker, "nfkdinert"),
        (PatternSyntaxV1Marker, "Pat_Syn"),
        (PatternWhiteSpaceV1Marker, "Pat_WS"),
        (PrependedConcatenationMarkV1Marker, "PCM"),
        (PrintV1Marker, "print"),
        (QuotationMarkV1Marker, "QMark"),
        (RadicalV1Marker, "Radical"),
        (RegionalIndicatorV1Marker, "RI"),
        (SoftDottedV1Marker, "SD"),
        (SegmentStarterV1Marker, "segstart"),
        (CaseSensitiveV1Marker, "Sensitive"),
        (SentenceTerminalV1Marker, "STerm"),
        (TerminalPunctuationV1Marker, "Term"),
        (UnifiedIdeographV1Marker, "UIdeo"),
        (UppercaseV1Marker, "Upper"),
        (VariationSelectorV1Marker, "VS"),
        (WhiteSpaceV1Marker, "WSpace"),
        (XdigitV1Marker, "xdigit"),
        (XidContinueV1Marker, "XIDC"),
        (XidStartV1Marker, "XIDS"),
    ),
    (
        // UnicodeSets (code points + strings)
        (BasicEmojiV1Marker, "Basic_Emoji"),
        (ExemplarCharactersMainV1Marker, "exemplarchars/main"),
        (
            ExemplarCharactersAuxiliaryV1Marker,
            "exemplarchars/auxiliary"
        ),
        (
            ExemplarCharactersPunctuationV1Marker,
            "exemplarchars/punctuation"
        ),
        (ExemplarCharactersNumbersV1Marker, "exemplarchars/numbers"),
        (ExemplarCharactersIndexV1Marker, "exemplarchars/index"),
    ),
    (
        // code point maps
        (
            CanonicalCombiningClassV1Marker,
            CanonicalCombiningClassNameToValueV1Marker,
            "ccc",
            CanonicalCombiningClass
        ),
        (
            GeneralCategoryV1Marker,
            GeneralCategoryNameToValueV1Marker,
            "gc",
            GeneralCategory
        ),
        (
            BidiClassV1Marker,
            BidiClassNameToValueV1Marker,
            "bc",
            BidiClass
        ),
        (ScriptV1Marker, ScriptNameToValueV1Marker, "sc", Script),
        (
            EastAsianWidthV1Marker,
            EastAsianWidthNameToValueV1Marker,
            "ea",
            EastAsianWidth
        ),
        (
            LineBreakV1Marker,
            LineBreakNameToValueV1Marker,
            "lb",
            LineBreak
        ),
        (
            GraphemeClusterBreakV1Marker,
            GraphemeClusterBreakNameToValueV1Marker,
            "GCB",
            GraphemeClusterBreak
        ),
        (
            WordBreakV1Marker,
            WordBreakNameToValueV1Marker,
            "WB",
            WordBreak
        ),
        (
            SentenceBreakV1Marker,
            SentenceBreakNameToValueV1Marker,
            "SB",
            SentenceBreak
        ),
    )
);
