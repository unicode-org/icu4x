// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data and APIs for supporting both Script and Script_Extensions property
//! values in an efficient structure.

use crate::error::PropertiesError;
use crate::props::Script;
use crate::props::ScriptULE;
use crate::provider::*;

use core::ops::RangeInclusive;
use icu_collections::codepointinvlist::CodePointInversionList;
use icu_provider::prelude::*;
use zerovec::{ule::AsULE, ZeroSlice};

/// The number of bits at the low-end of a `ScriptWithExt` value used for
/// storing the `Script` value (or `extensions` index).
const SCRIPT_VAL_LENGTH: u16 = 10;

/// The bit mask necessary to retrieve the `Script` value (or `extensions` index)
/// from a `ScriptWithExt` value.
pub(crate) const SCRIPT_X_SCRIPT_VAL: u16 = (1 << SCRIPT_VAL_LENGTH) - 1;

/// An internal-use only pseudo-property that represents the values stored in
/// the trie of the special data structure [`ScriptWithExtensionsPropertyV1`].
///
/// Note: The will assume a 12-bit layout. The 2 higher order bits in positions
/// 11..10 will indicate how to deduce the Script value and Script_Extensions,
/// and the lower 10 bits 9..0 indicate either the Script value or the index
/// into the `extensions` structure.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "datagen", derive(databake::Bake))]
#[cfg_attr(feature = "datagen", databake(path = icu_properties::script))]
#[repr(transparent)]
#[doc(hidden)]
// `ScriptWithExt` not intended as public-facing but for `ScriptWithExtensionsPropertyV1` constructor
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ScriptWithExt(pub u16);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
#[doc(hidden)] // `ScriptWithExt` not intended as public-facing but for `ScriptWithExtensionsPropertyV1` constructor
impl ScriptWithExt {
    pub const Unknown: ScriptWithExt = ScriptWithExt(0);
}

impl AsULE for ScriptWithExt {
    type ULE = ScriptULE;

    #[inline]
    fn to_unaligned(self) -> Self::ULE {
        Script(self.0).to_unaligned()
    }

    #[inline]
    fn from_unaligned(unaligned: Self::ULE) -> Self {
        ScriptWithExt(Script::from_unaligned(unaligned).0)
    }
}

#[doc(hidden)] // `ScriptWithExt` not intended as public-facing but for `ScriptWithExtensionsPropertyV1` constructor
impl ScriptWithExt {
    /// Returns whether the [`ScriptWithExt`] value has Script_Extensions and
    /// also indicates a Script value of [`Script::Common`].
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::script::ScriptWithExt;
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
    /// use icu::properties::script::ScriptWithExt;
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
    /// use icu::properties::script::ScriptWithExt;
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
    /// use icu::properties::script::ScriptWithExt;
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

/// A struct that wraps a [`Script`] array, such as in the return value for
/// [`get_script_extensions_val`](ScriptWithExtensionsPropertyV1::get_script_extensions_val).
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct ScriptExtensionsSet<'a> {
    pub(crate) values: &'a ZeroSlice<Script>,
}

impl ScriptExtensionsSet<'_> {
    /// Returns whether this set contains the given script.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{script, Script};
    /// let data =
    ///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let swe = data.as_borrowed();
    ///
    /// assert!(swe
    ///     .get_script_extensions_val(0x11303) // GRANTHA SIGN VISARGA
    ///     .contains(&Script::Grantha));
    /// ```
    pub fn contains(&self, x: &Script) -> bool {
        ZeroSlice::binary_search(&*self.values, x).is_ok()
    }

    /// Gets an iterator over the elements.
    ///
    /// # Example
    ///
    /// ```
    /// use icu::properties::{script, Script};
    /// let data =
    ///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let swe = data.as_borrowed();
    ///
    /// assert_eq!(
    ///     swe.get_script_extensions_val('௫' as u32) // U+0BEB TAMIL DIGIT FIVE
    ///         .iter()
    ///         .collect::<Vec<Script>>(),
    ///     vec![Script::Tamil, Script::Grantha]
    /// );
    /// ```
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = Script> + '_ {
        ZeroSlice::iter(&*self.values)
    }
}
/// A wrapper around script extensions data. Can be obtained via [`load_script_with_extensions_unstable()`] and
/// related getters.
///
/// Most useful methods are on [`ScriptWithExtensionsBorrowed`] obtained by calling [`ScriptWithExtensions::as_borrowed()`]
pub struct ScriptWithExtensions {
    data: DataPayload<ScriptWithExtensionsPropertyV1Marker>,
}

/// A borrowed wrapper around script extension data, returned by
/// [`ScriptWithExtensions::as_borrowed()`]. More efficient to query.
#[derive(Clone, Copy)]
pub struct ScriptWithExtensionsBorrowed<'a> {
    data: &'a ScriptWithExtensionsPropertyV1<'a>,
}

impl ScriptWithExtensions {
    /// Construct a borrowed version of this type that can be queried.
    ///
    /// This avoids a potential small underlying cost per API call (ex: `contains()`) by consolidating it
    /// up front.
    #[inline]
    pub fn as_borrowed(&self) -> ScriptWithExtensionsBorrowed<'_> {
        ScriptWithExtensionsBorrowed {
            data: self.data.get(),
        }
    }

    /// Construct a new one from loaded data
    ///
    /// Typically it is preferable to use getters like [`load_script_with_extensions_unstable()`] instead
    pub fn from_data(data: DataPayload<ScriptWithExtensionsPropertyV1Marker>) -> Self {
        Self { data }
    }
}

impl<'a> ScriptWithExtensionsBorrowed<'a> {
    /// Returns the `Script` property value for this code point.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let data = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let swe = data.as_borrowed();
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
        self.data.get_script_val(code_point)
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
    /// let data = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let swe = data.as_borrowed();
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
    pub fn get_script_extensions_val(&self, code_point: u32) -> ScriptExtensionsSet<'a> {
        self.data.get_script_extensions_val(code_point)
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
    /// let data =
    ///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let swe = data.as_borrowed();
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
        self.data.has_script(code_point, script)
    }

    /// Returns all of the matching `CodePointMapRange`s for the given [`Script`]
    /// in which `has_script` will return true for all of the contained code points.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let data = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let swe = data.as_borrowed();
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
        self.data.get_script_extensions_ranges(script)
    }

    /// Returns a [`CodePointInversionList`] for the given [`Script`] which represents all
    /// code points for which `has_script` will return true.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::properties::{script, Script};
    ///
    /// let data = script::load_script_with_extensions_unstable(&icu_testdata::unstable()).expect("The data should be valid");
    /// let swe = data.as_borrowed();
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
        self.data.get_script_extensions_set(script)
    }
}

/// Returns a [`ScriptWithExtensionsPropertyV1`] struct that represents the data for the Script
/// and Script_Extensions properties.
///
/// # Examples
///
/// ```
/// use icu::properties::{script, Script};
///
/// let data =
///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
///         .expect("The data should be valid");
/// let swe = data.as_borrowed();
///
/// // get the `Script` property value
/// assert_eq!(swe.get_script_val(0x0640), Script::Common); // U+0640 ARABIC TATWEEL
/// assert_eq!(swe.get_script_val(0x0650), Script::Inherited); // U+0650 ARABIC KASRA
/// assert_eq!(swe.get_script_val(0x0660), Script::Arabic); // // U+0660 ARABIC-INDIC DIGIT ZERO
/// assert_eq!(swe.get_script_val(0xFDF2), Script::Arabic); // U+FDF2 ARABIC LIGATURE ALLAH ISOLATED FORM
///
/// // get the `Script_Extensions` property value
/// assert_eq!(
///     swe.get_script_extensions_val(0x0640) // U+0640 ARABIC TATWEEL
///         .iter().collect::<Vec<Script>>(),
///     vec![Script::Arabic, Script::Syriac, Script::Mandaic, Script::Manichaean,
///          Script::PsalterPahlavi, Script::Adlam, Script::HanifiRohingya, Script::Sogdian,
///          Script::OldUyghur]
/// );
/// assert_eq!(
///     swe.get_script_extensions_val('🥳' as u32) // U+1F973 FACE WITH PARTY HORN AND PARTY HAT
///         .iter().collect::<Vec<Script>>(),
///     vec![Script::Common]
/// );
/// assert_eq!(
///     swe.get_script_extensions_val(0x200D) // ZERO WIDTH JOINER
///         .iter().collect::<Vec<Script>>(),
///     vec![Script::Inherited]
/// );
/// assert_eq!(
///     swe.get_script_extensions_val('௫' as u32) // U+0BEB TAMIL DIGIT FIVE
///         .iter().collect::<Vec<Script>>(),
///     vec![Script::Tamil, Script::Grantha]
/// );
///
/// // check containment of a `Script` value in the `Script_Extensions` value
/// // U+0650 ARABIC KASRA
/// assert!(!swe.has_script(0x0650, Script::Inherited)); // main Script value
/// assert!(swe.has_script(0x0650, Script::Arabic));
/// assert!(swe.has_script(0x0650, Script::Syriac));
/// assert!(!swe.has_script(0x0650, Script::Thaana));
///
/// // get a `CodePointInversionList` for when `Script` value is contained in `Script_Extensions` value
/// let syriac = swe.get_script_extensions_set(Script::Syriac);
/// assert!(syriac.contains32(0x0650)); // ARABIC KASRA
/// assert!(!syriac.contains32(0x0660)); // ARABIC-INDIC DIGIT ZERO
/// assert!(!syriac.contains32(0xFDF2)); // ARABIC LIGATURE ALLAH ISOLATED FORM
/// assert!(syriac.contains32(0x0700)); // SYRIAC END OF PARAGRAPH
/// assert!(syriac.contains32(0x074A)); // SYRIAC BARREKH
/// ```
pub fn load_script_with_extensions_unstable(
    provider: &(impl DataProvider<ScriptWithExtensionsPropertyV1Marker> + ?Sized),
) -> Result<ScriptWithExtensions, PropertiesError> {
    Ok(ScriptWithExtensions::from_data(
        provider
            .load(Default::default())
            .and_then(DataResponse::take_payload)?,
    ))
}

icu_provider::gen_any_buffer_constructors!(
    locale: skip,
    options: skip,
    result: Result<ScriptWithExtensions, PropertiesError>,
    functions: [
        load_script_with_extensions_unstable,
        load_script_with_extensions_with_any_provider,
        load_script_with_extensions_with_buffer_provider
    ]
);
