// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// Provider structs must be stable
#![allow(clippy::exhaustive_structs, clippy::exhaustive_enums)]

//! Data provider struct definitions for this ICU4X component.
//!
//! Read more about data providers: [`icu_provider`]

use crate::script::{ScriptExtensionsSet, ScriptWithExt, SCRIPT_X_SCRIPT_VAL};
use crate::Script;
use core::iter::FromIterator;
use core::ops::RangeInclusive;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_collections::codepointtrie::{CodePointMapRange, CodePointTrie, TrieValue};
use icu_provider::prelude::*;
use zerofrom::ZeroFrom;
use zerovec::ule::AsULE;
use zerovec::{VarZeroVec, ZeroSlice, ZeroVecError};

/// A set of characters with a particular property.
///
/// This data enum is extensible, more backends may be added in the future.
/// Old data can be used with newer code but not vice versa.
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

/// A struct that efficiently stores `Script` and `Script_Extensions` property data.
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
    #[doc(hidden)] // #2417
    pub trie: CodePointTrie<'data, ScriptWithExt>,

    /// This companion structure stores Script_Extensions values, which are
    /// themselves arrays / vectors. This structure only stores the values for
    /// cases in which `scx(cp) != [ sc(cp) ]`. Each sub-vector is distinct. The
    /// sub-vector represents the Script_Extensions array value for a code point,
    /// and may also indicate Script value, as described for the `trie` field.
    #[cfg_attr(feature = "serde", serde(borrow))]
    #[doc(hidden)] // #2417
    pub extensions: VarZeroVec<'data, ZeroSlice<Script>>,
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

// See ScriptWithExtensions for documentation of these functions
#[allow(missing_docs)]
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

    pub fn get_script_val(&self, code_point: u32) -> Script {
        let sc_with_ext = self.trie.get32(code_point);

        if sc_with_ext.is_other() {
            let ext_idx = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            let scx_val = self.extensions.get(ext_idx as usize);
            let scx_first_sc = scx_val.and_then(|scx| scx.get(0));

            let default_sc_val = Script::Unknown;

            scx_first_sc.unwrap_or(default_sc_val)
        } else if sc_with_ext.is_common() {
            Script::Common
        } else if sc_with_ext.is_inherited() {
            Script::Inherited
        } else {
            let script_val = sc_with_ext.0;
            Script(script_val)
        }
    }

    // Returns the Script_Extensions value for a code_point when the trie value
    // is already known.
    // This private helper method exists to prevent code duplication in callers like
    // `get_script_extensions_val`, `get_script_extensions_set`, and `has_script`.
    fn get_scx_val_using_trie_val<'a>(
        &'a self,
        sc_with_ext_ule: &'a <ScriptWithExt as AsULE>::ULE,
    ) -> &'a ZeroSlice<Script> {
        let sc_with_ext = ScriptWithExt::from_unaligned(*sc_with_ext_ule);
        if sc_with_ext.is_other() {
            let ext_idx = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            let ext_subarray = self.extensions.get(ext_idx as usize);
            // In the OTHER case, where the 2 higher-order bits of the
            // `ScriptWithExt` value in the trie doesn't indicate the Script value,
            // the Script value is copied/inserted into the first position of the
            // `extensions` array. So we must remove it to return the actual scx array val.
            let scx_slice = ext_subarray
                .and_then(|zslice| zslice.as_ule_slice().get(1..))
                .unwrap_or_default();
            ZeroSlice::from_ule_slice(scx_slice)
        } else if sc_with_ext.is_common() || sc_with_ext.is_inherited() {
            let ext_idx = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            let scx_val = self.extensions.get(ext_idx as usize);
            scx_val.unwrap_or_default()
        } else {
            // Note: `Script` and `ScriptWithExt` are both represented as the same
            // u16 value when the `ScriptWithExt` has no higher-order bits set.
            let script_ule_slice = core::slice::from_ref(sc_with_ext_ule);
            ZeroSlice::from_ule_slice(script_ule_slice)
        }
    }

    pub fn get_script_extensions_val(&self, code_point: u32) -> ScriptExtensionsSet {
        let sc_with_ext_ule = self.trie.get32_ule(code_point);

        ScriptExtensionsSet {
            values: match sc_with_ext_ule {
                Some(ule_ref) => self.get_scx_val_using_trie_val(ule_ref),
                None => ZeroSlice::from_ule_slice(&[]),
            },
        }
    }

    pub fn has_script(&self, code_point: u32, script: Script) -> bool {
        let sc_with_ext_ule = if let Some(scwe_ule) = self.trie.get32_ule(code_point) {
            scwe_ule
        } else {
            return false;
        };
        let sc_with_ext = <ScriptWithExt as AsULE>::from_unaligned(*sc_with_ext_ule);

        if !sc_with_ext.has_extensions() {
            let script_val = sc_with_ext.0;
            script == Script(script_val)
        } else {
            let scx_val = self.get_scx_val_using_trie_val(sc_with_ext_ule);
            let script_find = scx_val.iter().find(|&sc| sc == script);
            script_find.is_some()
        }
    }

    pub fn get_script_extensions_ranges(
        &self,
        script: Script,
    ) -> impl Iterator<Item = RangeInclusive<u32>> + '_ {
        self.trie
            .iter_ranges()
            .filter(move |cpm_range| {
                let sc_with_ext = ScriptWithExt(cpm_range.value.0);
                if sc_with_ext.has_extensions() {
                    self.get_scx_val_using_trie_val(&sc_with_ext.to_unaligned())
                        .iter()
                        .any(|sc| sc == script)
                } else {
                    script == sc_with_ext.into()
                }
            })
            .map(|cpm_range| RangeInclusive::new(*cpm_range.range.start(), *cpm_range.range.end()))
    }

    pub fn get_script_extensions_set(&self, script: Script) -> CodePointInversionList {
        CodePointInversionList::from_iter(self.get_script_extensions_ranges(script))
    }
}

macro_rules! expand {
    (
        ($(($bin_marker:ident, $bin_s:literal),)+),
        ($(($enum_marker:ident, $enum_s:literal, $value_ty:ident),)+)
    ) => {

            $(
                #[doc = core::concat!("Data marker for the '", $bin_s, "' Unicode property")]
                pub struct $bin_marker;

                impl DataMarker for $bin_marker {
                    type Yokeable = PropertyCodePointSetV1<'static>;
                }
                impl KeyedDataMarker for $bin_marker {
                    const KEY: DataKey = data_key!(concat!("props/", $bin_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $bin_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $bin_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$bin_marker }
                    }
                }
            )+

            $(
                #[doc = core::concat!("Data marker for the '", $enum_s, "' Unicode property")]
                pub struct $enum_marker;

                impl DataMarker for $enum_marker {
                    type Yokeable = PropertyCodePointMapV1<'static, crate::$value_ty>;
                }

                impl KeyedDataMarker for $enum_marker {
                    const KEY: DataKey = data_key!(concat!("props/", $enum_s, "@1"));
                }

                #[cfg(feature = "datagen")]
                impl Default for $enum_marker {
                    fn default() -> Self {
                        Self
                    }
                }

                #[cfg(feature = "datagen")]
                impl databake::Bake for $enum_marker {
                    fn bake(&self, env: &databake::CrateEnv) -> databake::TokenStream {
                        env.insert("icu_properties");
                        databake::quote!{ ::icu_properties::provider::$enum_marker }
                    }
                }
            )+
    };
}

expand!(
    (
        // Binary properties
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
        // Enum properties
        (
            CanonicalCombiningClassV1Marker,
            "ccc",
            CanonicalCombiningClass
        ),
        (GeneralCategoryV1Marker, "gc", GeneralCategory),
        (BidiClassV1Marker, "bc", BidiClass),
        (ScriptV1Marker, "sc", Script),
        (EastAsianWidthV1Marker, "ea", EastAsianWidth),
        (LineBreakV1Marker, "lb", LineBreak),
        (GraphemeClusterBreakV1Marker, "GCB", GraphemeClusterBreak),
        (WordBreakV1Marker, "WB", WordBreak),
        (SentenceBreakV1Marker, "SB", SentenceBreak),
    )
);
