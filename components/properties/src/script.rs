// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data and APIs for supporting both Script and Script_Extensions property
//! values in an efficient structure.

use crate::error::PropertiesError;
use crate::props::Script;
use crate::props::ScriptULE;
use crate::provider::*;

use icu_provider::prelude::*;
use zerovec::{ule::AsULE, ZeroSlice};

/// The number of bits at the low-end of a `ScriptWithExt` value used for
/// storing the `Script` value (or `extensions` index).
const SCRIPT_VAL_LENGTH: u16 = 10;

/// The bit mask necessary to retrieve the `Script` value (or `extensions` index)
/// from a `ScriptWithExt` value.
pub(crate) const SCRIPT_X_SCRIPT_VAL: u16 = (1 << SCRIPT_VAL_LENGTH) - 1;

/// An internal-use only pseudo-property that represents the values stored in
/// the trie of the special data structure [`ScriptWithExtensionsV1`].
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
#[doc(hidden)] // `ScriptWithExt` not intended as public-facing but for `ScriptWithExtensionsV1` constructor
#[allow(clippy::exhaustive_structs)] // this type is stable
pub struct ScriptWithExt(pub u16);

#[allow(missing_docs)] // These constants don't need individual documentation.
#[allow(non_upper_case_globals)]
#[doc(hidden)] // `ScriptWithExt` not intended as public-facing but for `ScriptWithExtensionsV1` constructor
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

#[doc(hidden)] // `ScriptWithExt` not intended as public-facing but for `ScriptWithExtensionsV1` constructor
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
/// [`get_script_extensions_val`](ScriptWithExtensionsV1::get_script_extensions_val).
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
    /// let payload =
    ///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let swe = &data_struct.data;
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
    /// let payload =
    ///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
    ///         .expect("The data should be valid");
    /// let data_struct = payload.get();
    /// let swe = &data_struct.data;
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

/// Returns a [`ScriptWithExtensionsV1`] struct that represents the data for the Script
/// and Script_Extensions properties.
///
/// # Examples
///
/// ```
/// use icu::properties::{script, Script};
///
/// let payload =
///     script::load_script_with_extensions_unstable(&icu_testdata::unstable())
///         .expect("The data should be valid");
/// let data_struct = payload.get();
/// let swe = &data_struct.data;
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
) -> Result<DataPayload<ScriptWithExtensionsPropertyV1Marker>, PropertiesError> {
    Ok(provider
        .load(Default::default())
        .and_then(DataResponse::take_payload)?)
}

icu_provider::gen_any_buffer_constructors!(
    locale: skip,
    options: skip,
    result: Result<DataPayload<ScriptWithExtensionsPropertyV1Marker>, PropertiesError>,
    functions: [
        load_script_with_extensions_unstable,
        load_script_with_extensions_with_any_provider,
        load_script_with_extensions_with_buffer_provider
    ]
);
