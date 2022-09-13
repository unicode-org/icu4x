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
    /// A special data structure for `Script` and `Script_Extensions`.
    #[cfg_attr(feature = "serde", serde(borrow))]
    pub data: ScriptWithExtensions<'data>,
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

/// A data structure that represents the data for both Script and
/// Script_Extensions properties in an efficient way. This structure matches
/// the data and data structures that are stored in the corresponding ICU data
/// file for these properties.
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_properties::provider),
)]
#[derive(Clone, Debug, Eq, PartialEq, yoke::Yokeable, zerofrom::ZeroFrom)]
pub struct ScriptWithExtensions<'data> {
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

impl<'data> ScriptWithExtensions<'data> {
    // This method is intended to be used by constructors of deserialized data
    // in a data provider.
    #[doc(hidden)]
    pub fn new(
        trie: CodePointTrie<'data, ScriptWithExt>,
        extensions: VarZeroVec<'data, ZeroSlice<Script>>,
    ) -> ScriptWithExtensions<'data> {
        ScriptWithExtensions { trie, extensions }
    }

    /// Returns the `Script` property value for this code point.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let payload = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let swe = &data_struct.data;
    ///
    /// // U+0640 ARABIC TATWEEL
    /// assert_eq!(swe.get_script_val(0x0640), Script::Common); // main Script value
    /// assert_ne!(swe.get_script_val(0x0640), Script::Arabic);
    /// assert_ne!(swe.get_script_val(0x0640), Script::Syriac);
    /// assert_ne!(swe.get_script_val(0x0640), Script::Thaana);
    ///
    /// // U+0650 ARABIC KASRA
    /// assert_eq!(swe.get_script_val(0x0650), Script::Inherited); // main Script value
    /// assert_ne!(swe.get_script_val(0x0650), Script::Arabic);
    /// assert_ne!(swe.get_script_val(0x0650), Script::Syriac);
    /// assert_ne!(swe.get_script_val(0x0650), Script::Thaana);
    ///
    /// // U+0660 ARABIC-INDIC DIGIT ZERO
    /// assert_ne!(swe.get_script_val(0x0660), Script::Common);
    /// assert_eq!(swe.get_script_val(0x0660), Script::Arabic); // main Script value
    /// assert_ne!(swe.get_script_val(0x0660), Script::Syriac);
    /// assert_ne!(swe.get_script_val(0x0660), Script::Thaana);
    ///
    /// // U+FDF2 ARABIC LIGATURE ALLAH ISOLATED FORM
    /// assert_ne!(swe.get_script_val(0xFDF2), Script::Common);
    /// assert_eq!(swe.get_script_val(0xFDF2), Script::Arabic); // main Script value
    /// assert_ne!(swe.get_script_val(0xFDF2), Script::Syriac);
    /// assert_ne!(swe.get_script_val(0xFDF2), Script::Thaana);
    /// ```
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

    /// Return the `Script_Extensions` property value for this code point.
    ///
    /// If `code_point` has Script_Extensions, then return the Script codes in
    /// the Script_Extensions. In this case, the Script property value
    /// (normally Common or Inherited) is not included in the [`ScriptExtensionsSet`].
    ///
    /// If c does not have Script_Extensions, then the one Script code is put
    /// into the [`ScriptExtensionsSet`] and also returned.
    ///
    /// If c is not a valid code point, then return an empty [`ScriptExtensionsSet`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let payload = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let swe = &data_struct.data;
    ///
    /// assert_eq!(
    ///     swe.get_script_extensions_val('𐓐' as u32) // U+104D0 OSAGE CAPITAL LETTER KHA
    ///         .iter()
    ///         .collect::<Vec<Script>>(),
    ///     vec![Script::Osage]
    /// );
    /// assert_eq!(
    ///     swe.get_script_extensions_val('🥳' as u32) // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
    ///         .iter()
    ///         .collect::<Vec<Script>>(),
    ///     vec![Script::Common]
    /// );
    /// assert_eq!(
    ///     swe.get_script_extensions_val(0x200D) // ZERO WIDTH JOINER
    ///         .iter()
    ///         .collect::<Vec<Script>>(),
    ///     vec![Script::Inherited]
    /// );
    /// assert_eq!(
    ///     swe.get_script_extensions_val('௫' as u32) // U+0BEB TAMIL DIGIT FIVE
    ///         .iter()
    ///         .collect::<Vec<Script>>(),
    ///     vec![Script::Tamil, Script::Grantha]
    /// );
    /// ```
    pub fn get_script_extensions_val(&self, code_point: u32) -> ScriptExtensionsSet {
        let sc_with_ext_ule = self.trie.get32_ule(code_point);

        ScriptExtensionsSet {
            values: match sc_with_ext_ule {
                Some(ule_ref) => self.get_scx_val_using_trie_val(ule_ref),
                None => ZeroSlice::from_ule_slice(&[]),
            },
        }
    }
    /// Returns whether `script` is contained in the Script_Extensions
    /// property value if the code_point has Script_Extensions, otherwise
    /// if the code point does not have Script_Extensions then returns
    /// whether the Script property value matches.
    ///
    /// Some characters are commonly used in multiple scripts. For more information,
    /// see UAX #24: <http://www.unicode.org/reports/tr24/>.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let provider = icu_testdata::unstable();
    /// let payload =
    ///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let swe = &data_struct.data;
    ///
    /// // U+0650 ARABIC KASRA
    /// assert!(!swe.has_script(0x0650, Script::Inherited)); // main Script value
    /// assert!(swe.has_script(0x0650, Script::Arabic));
    /// assert!(swe.has_script(0x0650, Script::Syriac));
    /// assert!(!swe.has_script(0x0650, Script::Thaana));
    ///
    /// // U+0660 ARABIC-INDIC DIGIT ZERO
    /// assert!(!swe.has_script(0x0660, Script::Common)); // main Script value
    /// assert!(swe.has_script(0x0660, Script::Arabic));
    /// assert!(!swe.has_script(0x0660, Script::Syriac));
    /// assert!(swe.has_script(0x0660, Script::Thaana));
    ///
    /// // U+FDF2 ARABIC LIGATURE ALLAH ISOLATED FORM
    /// assert!(!swe.has_script(0xFDF2, Script::Common));
    /// assert!(swe.has_script(0xFDF2, Script::Arabic)); // main Script value
    /// assert!(!swe.has_script(0xFDF2, Script::Syriac));
    /// assert!(swe.has_script(0xFDF2, Script::Thaana));
    /// ```
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

    /// Returns all of the matching `CodePointMapRange`s for the given [`Script`]
    /// in which `has_script` will return true for all of the contained code points.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let payload = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let swe = &data_struct.data;
    ///
    /// let syriac_script_extensions_ranges = swe.get_script_extensions_ranges(Script::Syriac);
    ///
    /// let exp_ranges = vec![
    ///     0x060C..=0x060C, // ARABIC COMMA
    ///     0x061B..=0x061B, // ARABIC SEMICOLON
    ///     0x061C..=0x061C, // ARABIC LETTER MARK
    ///     0x061F..=0x061F, // ARABIC QUESTION MARK
    ///     0x0640..=0x0640, // ARABIC TATWEEL
    ///     0x064B..=0x0655, // ARABIC FATHATAN..ARABIC HAMZA BELOW
    ///     0x0670..=0x0670, // ARABIC LETTER SUPERSCRIPT ALEF
    ///     0x0700..=0x070D, // Syriac block begins at U+0700
    ///     0x070F..=0x074A, // Syriac block
    ///     0x074D..=0x074F, // Syriac block ends at U+074F
    ///     0x0860..=0x086A, // Syriac Supplement block is U+0860..=U+086F
    ///     0x1DF8..=0x1DF8, // U+1DF8 COMBINING DOT ABOVE LEFT
    ///     0x1DFA..=0x1DFA, // U+1DFA COMBINING DOT BELOW LEFT
    /// ];
    /// let mut exp_ranges_iter = exp_ranges.iter();
    ///
    /// for act_range in syriac_script_extensions_ranges {
    ///     let exp_range = exp_ranges_iter
    ///         .next()
    ///         .expect("There are too many ranges returned by get_script_extensions_ranges()");
    ///     assert_eq!(act_range.start(), exp_range.start());
    ///     assert_eq!(act_range.end(), exp_range.end());
    /// }
    /// assert!(
    ///     exp_ranges_iter.next().is_none(),
    ///     "There are too few ranges returned by get_script_extensions_ranges()"
    /// );
    /// ```
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

    /// Returns a [`CodePointInversionList`] for the given [`Script`] which represents all
    /// code points for which `has_script` will return true.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let payload = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let swe = &data_struct.data;
    ///
    /// let syriac = swe.get_script_extensions_set(Script::Syriac);
    ///
    /// assert!(!syriac.contains32(0x061E)); // ARABIC TRIPLE DOT PUNCTUATION MARK
    /// assert!(syriac.contains32(0x061F)); // ARABIC QUESTION MARK
    /// assert!(!syriac.contains32(0x0620)); // ARABIC LETTER KASHMIRI YEH
    ///
    /// assert!(syriac.contains32(0x0700)); // SYRIAC END OF PARAGRAPH
    /// assert!(syriac.contains32(0x074A)); // SYRIAC BARREKH
    /// assert!(!syriac.contains32(0x074B)); // unassigned
    /// assert!(syriac.contains32(0x074F)); // SYRIAC LETTER SOGDIAN FE
    /// assert!(!syriac.contains32(0x0750)); // ARABIC LETTER BEH WITH THREE DOTS HORIZONTALLY BELOW
    ///
    /// assert!(syriac.contains32(0x1DF8)); // COMBINING DOT ABOVE LEFT
    /// assert!(!syriac.contains32(0x1DF9)); // COMBINING WIDE INVERTED BRIDGE BELOW
    /// assert!(syriac.contains32(0x1DFA)); // COMBINING DOT BELOW LEFT
    /// assert!(!syriac.contains32(0x1DFB)); // COMBINING DELETION MARK
    /// ```
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
