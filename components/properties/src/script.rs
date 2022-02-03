// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data and APIs for supporting both Script and Script_Extensions property
//! values in an efficient structure.

use crate::error::PropertiesError;
use crate::props::Script;

use core::iter::FromIterator;
use core::ops::RangeInclusive;
use icu_codepointtrie::{CodePointTrie, TrieValue};
use icu_provider::yoke::{self, *};
use icu_uniset::UnicodeSet;
use zerovec::{ule::AsULE, VarZeroVec, ZeroSlice};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// The number of bits at the low-end of a `ScriptWithExt` value used for
/// storing the `Script` value (or `extensions` index).
const SCRIPT_VAL_LENGTH: u16 = 10;

/// The bit mask necessary to retrieve the `Script` value (or `extensions` index)
/// from a `ScriptWithExt` value.
const SCRIPT_X_SCRIPT_VAL: u16 = (1 << SCRIPT_VAL_LENGTH) - 1;

/// An internal-use only pseudo-property that represents the values stored in
/// the trie of the special data structure [`ScriptExtensions`].
///
/// Note: The will assume a 12-bit layout. The 2 higher order bits in positions
/// 11..10 will indicate how to deduce the Script value and Script_Extensions,
/// and the lower 10 bits 9..0 indicate either the Script value or the index
/// into the `extensions` structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[repr(transparent)]
pub struct ScriptWithExt(pub u16);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
impl ScriptWithExt {
    pub const Unknown: ScriptWithExt = ScriptWithExt(0);
}

impl ScriptWithExt {
    /// Returns whether the [`ScriptWithExt`] value has Script_Extensions and
    /// also indicates a Script value of [`Script::Common`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_properties::script::ScriptWithExt;
    ///
    /// assert!(ScriptWithExt(0x04FF).is_common());
    /// assert!(ScriptWithExt(0x0400).is_common());
    ///
    /// assert!(!ScriptWithExt(0x08FF).is_common());
    /// assert!(!ScriptWithExt(0x0800).is_common());
    ///
    /// assert!(!ScriptWithExt(0x0CFF).is_common());
    /// assert!(!ScriptWithExt(0x0C00).is_common());
    ///
    /// assert!(!ScriptWithExt(0xFF).is_common());
    /// assert!(!ScriptWithExt(0x0).is_common());
    /// ```
    pub fn is_common(&self) -> bool {
        self.0 >> SCRIPT_VAL_LENGTH == 1
    }

    /// Returns whether the [`ScriptWithExt`] value has Script_Extensions and
    /// also indicates a Script value of [`Script::Inherited`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_properties::script::ScriptWithExt;
    ///
    /// assert!(!ScriptWithExt(0x04FF).is_inherited());
    /// assert!(!ScriptWithExt(0x0400).is_inherited());
    ///
    /// assert!(ScriptWithExt(0x08FF).is_inherited());
    /// assert!(ScriptWithExt(0x0800).is_inherited());
    ///
    /// assert!(!ScriptWithExt(0x0CFF).is_inherited());
    /// assert!(!ScriptWithExt(0x0C00).is_inherited());
    ///
    /// assert!(!ScriptWithExt(0xFF).is_inherited());
    /// assert!(!ScriptWithExt(0x0).is_inherited());
    /// ```
    pub fn is_inherited(&self) -> bool {
        self.0 >> SCRIPT_VAL_LENGTH == 2
    }

    /// Returns whether the [`ScriptWithExt`] value has Script_Extensions and
    /// also indicates that the Script value is neither [`Script::Common`] nor
    /// [`Script::Inherited`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_properties::script::ScriptWithExt;
    ///
    /// assert!(!ScriptWithExt(0x04FF).is_other());
    /// assert!(!ScriptWithExt(0x0400).is_other());
    ///
    /// assert!(!ScriptWithExt(0x08FF).is_other());
    /// assert!(!ScriptWithExt(0x0800).is_other());
    ///
    /// assert!(ScriptWithExt(0x0CFF).is_other());
    /// assert!(ScriptWithExt(0x0C00).is_other());
    ///
    /// assert!(!ScriptWithExt(0xFF).is_other());
    /// assert!(!ScriptWithExt(0x0).is_other());
    /// ```
    pub fn is_other(&self) -> bool {
        self.0 >> SCRIPT_VAL_LENGTH == 3
    }

    /// Returns whether the [`ScriptWithExt`] value has Script_Extensions.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_properties::script::ScriptWithExt;
    ///
    /// assert!(ScriptWithExt(0x04FF).has_extensions());
    /// assert!(ScriptWithExt(0x0400).has_extensions());
    ///
    /// assert!(ScriptWithExt(0x08FF).has_extensions());
    /// assert!(ScriptWithExt(0x0800).has_extensions());
    ///
    /// assert!(ScriptWithExt(0x0CFF).has_extensions());
    /// assert!(ScriptWithExt(0x0C00).has_extensions());
    ///
    /// assert!(!ScriptWithExt(0xFF).has_extensions());
    /// assert!(!ScriptWithExt(0x0).has_extensions());
    /// ```
    pub fn has_extensions(&self) -> bool {
        let high_order_bits = self.0 >> SCRIPT_VAL_LENGTH;
        high_order_bits > 0
    }
}

impl From<ScriptWithExt> for u32 {
    fn from(swe: ScriptWithExt) -> Self {
        swe.0 as u32
    }
}

impl From<ScriptWithExt> for Script {
    fn from(swe: ScriptWithExt) -> Self {
        Script(swe.0)
    }
}

/// A data structure that represents the data for both Script and
/// Script_Extensions properties in an efficient way. This structure matches
/// the data and data structures that are stored in the corresponding ICU data
/// file for these properties.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug, Eq, PartialEq, Yokeable, ZeroCopyFrom)]
pub struct ScriptExtensions<'data> {
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
    trie: CodePointTrie<'data, ScriptWithExt>,

    /// This companion structure stores Script_Extensions values, which are
    /// themselves arrays / vectors. This structure only stores the values for
    /// cases in which `scx(cp) != [ sc(cp) ]`. Each sub-vector is distinct. The
    /// sub-vector represents the Script_Extensions array value for a code point,
    /// and may also indicate Script value, as described for the `trie` field.
    #[cfg_attr(feature = "serde", serde(borrow))]
    extensions: VarZeroVec<'data, ZeroSlice<Script>>,
}

impl<'data> ScriptExtensions<'data> {
    pub fn try_new(
        trie: CodePointTrie<'data, ScriptWithExt>,
        extensions: VarZeroVec<'data, ZeroSlice<Script>>,
    ) -> Result<ScriptExtensions<'data>, PropertiesError> {
        // TODO: do validation here

        Ok(ScriptExtensions { trie, extensions })
    }

    /// Returns the `Script` property value for this code point.
    pub fn get_script_val(&self, code_point: u32) -> Script {
        let sc_with_ext = self.trie.get(code_point);

        if sc_with_ext.is_other() {
            let ext_idx = sc_with_ext.0 & SCRIPT_X_SCRIPT_VAL;
            let scx_val = self.extensions.get(ext_idx as usize);
            let scx_first_sc = scx_val.and_then(|scx| scx.get(0));

            let default_sc_val = <Script as TrieValue>::DATA_GET_ERROR_VALUE;

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
    /// (normally Common or Inherited) is not included in the [`ZeroSlice`].
    ///
    /// If c does not have Script_Extensions, then the one Script code is put
    /// into the [`ZeroSlice`] and also returned.
    ///
    /// If c is not a valid code point, then return an empty [`ZeroSlice`].
    pub fn get_script_extensions_val(&self, code_point: u32) -> &ZeroSlice<Script> {
        let sc_with_ext_ule = self.trie.get_ule(code_point);

        match sc_with_ext_ule {
            Some(ule_ref) => self.get_scx_val_using_trie_val(ule_ref),
            None => ZeroSlice::from_ule_slice(&[]),
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
    /// use icu_provider::prelude::*;
    /// use icu_properties::provider::key;
    /// use icu::properties::provider::ScriptExtensionsPropertyV1Marker;
    /// use icu_properties::Script;
    /// use icu::properties::script::ScriptExtensions;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let payload: DataPayload<ScriptExtensionsPropertyV1Marker> = provider
    ///     .load_payload(&DataRequest {
    ///         resource_path: ResourcePath {
    ///             key: key::SCRIPT_EXTENSIONS_V1,
    ///             options: ResourceOptions::default(),
    ///         },
    ///     })
    ///     .expect("The data should be valid")
    ///     .take_payload()
    ///     .expect("Loading was successful");
    ///
    /// let scx: &ScriptExtensions = &payload.get().data;
    ///
    /// // U+0650 ARABIC KASRA
    /// assert!(!scx.has_script(0x0650, Script::Inherited)); // main Script value
    /// assert!(scx.has_script(0x0650, Script::Arabic));
    /// assert!(scx.has_script(0x0650, Script::Syriac));
    /// assert!(!scx.has_script(0x0650, Script::Thaana));
    ///
    /// // U+0660 ARABIC-INDIC DIGIT ZERO
    /// assert!(!scx.has_script(0x0660, Script::Common)); // main Script value
    /// assert!(scx.has_script(0x0660, Script::Arabic));
    /// assert!(!scx.has_script(0x0660, Script::Syriac));
    /// assert!(scx.has_script(0x0660, Script::Thaana));
    ///
    /// // U+FDF2 ARABIC LIGATURE ALLAH ISOLATED FORM
    /// assert!(!scx.has_script(0xFDF2, Script::Common));
    /// assert!(scx.has_script(0xFDF2, Script::Arabic)); // main Script value
    /// assert!(!scx.has_script(0xFDF2, Script::Syriac));
    /// assert!(scx.has_script(0xFDF2, Script::Thaana));
    /// ```
    pub fn has_script(&self, code_point: u32, script: Script) -> bool {
        let sc_with_ext_ule = if let Some(scwe_ule) = self.trie.get_ule(code_point) {
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
    /// use core::ops::RangeInclusive;
    /// use icu_provider::prelude::*;
    /// use icu_properties::provider::key;
    /// use icu::properties::provider::ScriptExtensionsPropertyV1Marker;
    /// use icu_properties::Script;
    /// use icu::properties::script::ScriptExtensions;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let payload: DataPayload<ScriptExtensionsPropertyV1Marker> = provider
    ///     .load_payload(&DataRequest {
    ///         resource_path: ResourcePath {
    ///             key: key::SCRIPT_EXTENSIONS_V1,
    ///             options: ResourceOptions::default(),
    ///         },
    ///     })
    ///     .expect("The data should be valid")
    ///     .take_payload()
    ///     .expect("Loading was successful");
    ///
    /// let scx: &ScriptExtensions = &payload.get().data;
    /// let syriac_script_extensions_ranges = scx.get_script_extensions_ranges(Script::Syriac);
    ///
    /// let exp_ranges = vec![
    ///     0x060C..=0x060C,   // ARABIC COMMA
    ///     0x061B..=0x061B,   // ARABIC SEMICOLON
    ///     0x061C..=0x061C,   // ARABIC LETTER MARK
    ///     0x061F..=0x061F,   // ARABIC QUESTION MARK
    ///     0x0640..=0x0640,   // ARABIC TATWEEL
    ///     0x064B..=0x0655,   // ARABIC FATHATAN..ARABIC HAMZA BELOW
    ///     0x0670..=0x0670,   // ARABIC LETTER SUPERSCRIPT ALEF
    ///     0x0700..=0x070D,   // Syriac block begins at U+0700
    ///     0x070F..=0x074A,   // Syriac block
    ///     0x074D..=0x074F,   // Syriac block ends at U+074F
    ///     0x0860..=0x086A,   // Syriac Supplement block is U+0860..=U+086F
    ///     0x1DF8..=0x1DF8,   // U+1DF8 COMBINING DOT ABOVE LEFT
    ///     0x1DFA..=0x1DFA,   // U+1DFA COMBINING DOT BELOW LEFT
    /// ];
    /// let mut exp_ranges_iter = exp_ranges.iter();
    ///
    /// for act_range in syriac_script_extensions_ranges {
    ///     let exp_range = exp_ranges_iter.next()
    ///         .expect("There are too many ranges returned by get_script_extensions_ranges()");
    ///     assert_eq!(act_range.start(), exp_range.start());
    ///     assert_eq!(act_range.end(), exp_range.end());
    /// }
    /// assert!(exp_ranges_iter.next().is_none(), "There are too few ranges returned by get_script_extensions_ranges()");
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
                    self.get_scx_val_using_trie_val(&sc_with_ext.as_unaligned())
                        .iter()
                        .any(|sc| sc == script)
                } else {
                    script == sc_with_ext.into()
                }
            })
            .map(|cpm_range| RangeInclusive::new(*cpm_range.range.start(), *cpm_range.range.end()))
    }

    /// Returns a [`UnicodeSet`] for the given [`Script`] which represents all
    /// code points for which `has_script` will return true.
    ///
    /// # Examples
    ///
    /// ```
    /// use core::ops::RangeInclusive;
    /// use icu_provider::prelude::*;
    /// use icu_properties::provider::key;
    /// use icu::properties::provider::ScriptExtensionsPropertyV1Marker;
    /// use icu_properties::Script;
    /// use icu::properties::script::ScriptExtensions;
    ///
    /// let provider = icu_testdata::get_provider();
    ///
    /// let payload: DataPayload<ScriptExtensionsPropertyV1Marker> = provider
    ///     .load_payload(&DataRequest {
    ///         resource_path: ResourcePath {
    ///             key: key::SCRIPT_EXTENSIONS_V1,
    ///             options: ResourceOptions::default(),
    ///         },
    ///     })
    ///     .expect("The data should be valid")
    ///     .take_payload()
    ///     .expect("Loading was successful");
    ///
    /// let scx: &ScriptExtensions = &payload.get().data;
    /// let syriac = scx.get_script_extensions_set(Script::Syriac);
    ///
    /// assert!(!syriac.contains_u32(0x061E)); // ARABIC TRIPLE DOT PUNCTUATION MARK
    /// assert!(syriac.contains_u32(0x061F)); // ARABIC QUESTION MARK
    /// assert!(!syriac.contains_u32(0x0620)); // ARABIC LETTER KASHMIRI YEH
    ///
    /// assert!(syriac.contains_u32(0x0700)); // SYRIAC END OF PARAGRAPH
    /// assert!(syriac.contains_u32(0x074A)); // SYRIAC BARREKH
    /// assert!(!syriac.contains_u32(0x074B)); // unassigned
    /// assert!(syriac.contains_u32(0x074F)); // SYRIAC LETTER SOGDIAN FE
    /// assert!(!syriac.contains_u32(0x0750)); // ARABIC LETTER BEH WITH THREE DOTS HORIZONTALLY BELOW
    ///
    /// assert!(syriac.contains_u32(0x1DF8)); // COMBINING DOT ABOVE LEFT
    /// assert!(!syriac.contains_u32(0x1DF9)); // COMBINING WIDE INVERTED BRIDGE BELOW
    /// assert!(syriac.contains_u32(0x1DFA)); // COMBINING DOT BELOW LEFT
    /// assert!(!syriac.contains_u32(0x1DFB)); // COMBINING DELETION MARK
    /// ```
    pub fn get_script_extensions_set(&self, script: Script) -> UnicodeSet {
        UnicodeSet::from_iter(self.get_script_extensions_ranges(script))
    }
}
