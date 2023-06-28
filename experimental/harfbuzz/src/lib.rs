// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations

// TODO(#3275) Make this no_std again
//#![cfg_attr(not(any(test, feature = "std")), no_std)]

#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

//! Using ICU4X as the Unicode Database back end for HarfBuzz.
//!
//! # Examples
//!
//! ```
//! use harfbuzz::{Buffer, Direction, sys};
//! use icu_harfbuzz::new_hb_unicode_funcs_unstable;
//!
//! let mut b = Buffer::with("مساء الخير");
//!
//! let unicode_funcs = new_hb_unicode_funcs_unstable(&icu_testdata::unstable()).unwrap();
//!
//! // NOTE: This currently requires `unsafe` code. For progress toward a safe abstraction, see:
//! // <https://github.com/servo/rust-harfbuzz/pull/197>
//! unsafe {
//!     harfbuzz::sys::hb_buffer_set_unicode_funcs(b.as_ptr(), unicode_funcs.as_ptr());
//! }
//!
//! b.guess_segment_properties();
//! assert_eq!(b.get_direction(), Direction::RTL);
//! assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
//! ```

extern crate alloc;
mod error;

use crate::error::HarfBuzzError;
use alloc::boxed::Box;
use harfbuzz_sys::*;
use icu_normalizer::properties::CanonicalCombiningClassMap;
use icu_normalizer::properties::CanonicalComposition;
use icu_normalizer::properties::CanonicalDecomposition;
use icu_normalizer::properties::Decomposed;
use icu_normalizer::provider::CanonicalCompositionsV1Marker;
use icu_normalizer::provider::CanonicalDecompositionDataV1Marker;
use icu_normalizer::provider::CanonicalDecompositionTablesV1Marker;
use icu_normalizer::provider::NonRecursiveDecompositionSupplementV1Marker;
use icu_properties::bidi_data::BidiAuxiliaryProperties;
use icu_properties::maps::CodePointMapData;
use icu_properties::names::PropertyEnumToValueNameLinearTiny4Mapper;
use icu_properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker;
use icu_properties::provider::{
    GeneralCategoryV1Marker, ScriptV1Marker, ScriptValueToShortNameV1Marker,
};
use icu_properties::{GeneralCategory, Script};
use icu_provider::prelude::*;
use std::os::raw::{c_char, c_void};
use tinystr::{tinystr, TinyStr4};

/// The total number of General Category values is fixed per
/// https://www.unicode.org/policies/stability_policy.html :
/// "The enumeration of General_Category property values is fixed. No new values will be added."
///
/// Despite General Category logically being a fixed enumeration, the Unicode
/// Standard does not assign numeric identifiers to the possible General Category
/// values. Both ICU4X and HarfBuzz do, but in different order. This table provides
/// the permutation to go from the ICU4X number assignment in a Rust `enum` to
/// the HarfBuzz number assignment in a C `enum`.
///
/// The C `enum` used by HarfBuzz is wider than `u8` but only uses implied
/// values, so the values stay in the `u8` range. Let's use that to pack this.
/// Note: ICU4X does not declare the enum items in their numeric order!
static ICU4X_GENERAL_CATEGORY_TO_HARFBUZZ: [u8; 30] = [
    HB_UNICODE_GENERAL_CATEGORY_UNASSIGNED as u8, // Unassigned = 0,
    HB_UNICODE_GENERAL_CATEGORY_UPPERCASE_LETTER as u8, // UppercaseLetter = 1,
    HB_UNICODE_GENERAL_CATEGORY_LOWERCASE_LETTER as u8, // LowercaseLetter = 2,
    HB_UNICODE_GENERAL_CATEGORY_TITLECASE_LETTER as u8, // TitlecaseLetter = 3,
    HB_UNICODE_GENERAL_CATEGORY_MODIFIER_LETTER as u8, // ModifierLetter = 4,
    HB_UNICODE_GENERAL_CATEGORY_OTHER_LETTER as u8, // OtherLetter = 5,
    HB_UNICODE_GENERAL_CATEGORY_NON_SPACING_MARK as u8, // NonspacingMark = 6,
    HB_UNICODE_GENERAL_CATEGORY_ENCLOSING_MARK as u8, // EnclosingMark = 7
    HB_UNICODE_GENERAL_CATEGORY_SPACING_MARK as u8, // SpacingMark = 8,
    HB_UNICODE_GENERAL_CATEGORY_DECIMAL_NUMBER as u8, // DecimalNumber = 9,
    HB_UNICODE_GENERAL_CATEGORY_LETTER_NUMBER as u8, // LetterNumber = 10,
    HB_UNICODE_GENERAL_CATEGORY_OTHER_NUMBER as u8, // OtherNumber = 11,
    HB_UNICODE_GENERAL_CATEGORY_SPACE_SEPARATOR as u8, // SpaceSeparator = 12,
    HB_UNICODE_GENERAL_CATEGORY_LINE_SEPARATOR as u8, // LineSeparator = 13,
    HB_UNICODE_GENERAL_CATEGORY_PARAGRAPH_SEPARATOR as u8, // ParagraphSeparator = 14,
    HB_UNICODE_GENERAL_CATEGORY_CONTROL as u8,    // Control = 15,
    HB_UNICODE_GENERAL_CATEGORY_FORMAT as u8,     // Format = 16,
    HB_UNICODE_GENERAL_CATEGORY_PRIVATE_USE as u8, // PrivateUse = 17,
    HB_UNICODE_GENERAL_CATEGORY_SURROGATE as u8,  // Surrogate = 18,
    HB_UNICODE_GENERAL_CATEGORY_DASH_PUNCTUATION as u8, // DashPunctuation = 19,
    HB_UNICODE_GENERAL_CATEGORY_OPEN_PUNCTUATION as u8, // OpenPunctuation = 20,
    HB_UNICODE_GENERAL_CATEGORY_CLOSE_PUNCTUATION as u8, // ClosePunctuation = 21,
    HB_UNICODE_GENERAL_CATEGORY_CONNECT_PUNCTUATION as u8, // ConnectorPunctuation = 22,
    HB_UNICODE_GENERAL_CATEGORY_OTHER_PUNCTUATION as u8, // OtherPunctuation = 23,
    HB_UNICODE_GENERAL_CATEGORY_MATH_SYMBOL as u8, // MathSymbol = 24,
    HB_UNICODE_GENERAL_CATEGORY_CURRENCY_SYMBOL as u8, // CurrencySymbol = 25,
    HB_UNICODE_GENERAL_CATEGORY_MODIFIER_SYMBOL as u8, // ModifierSymbol = 26,
    HB_UNICODE_GENERAL_CATEGORY_OTHER_SYMBOL as u8, // OtherSymbol = 27,
    HB_UNICODE_GENERAL_CATEGORY_INITIAL_PUNCTUATION as u8, // InitialPunctuation = 28,
    HB_UNICODE_GENERAL_CATEGORY_FINAL_PUNCTUATION as u8, // FinalPunctuation = 29,
];

unsafe extern "C" fn icu4x_hb_unicode_combining_class(
    _ufuncs: *mut hb_unicode_funcs_t,
    unicode: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_unicode_combining_class_t {
    (*(user_data as *mut CanonicalCombiningClassMap))
        .get32(unicode)
        .0 as hb_unicode_combining_class_t
}

unsafe extern "C" fn icu4x_hb_unicode_combining_class_destroy(user_data: *mut c_void) {
    let _ = Box::from_raw(user_data as *mut CanonicalCombiningClassMap);
}

#[allow(clippy::indexing_slicing)]
unsafe extern "C" fn icu4x_hb_unicode_general_category(
    _ufuncs: *mut hb_unicode_funcs_t,
    unicode: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_unicode_general_category_t {
    // Indexing is OK, because `GeneralCategory` data is validated upon
    // deserialization so that there can be no out-of-range `GeneralCategory`
    // values (which would be UB to materialize). `GeneralCategory` is a
    // stable exhaustive enum, and the length of `ICU4X_GENERAL_CATEGORY_TO_HARFBUZZ`
    // matches the number of enum items, so the index will always be in range here.
    ICU4X_GENERAL_CATEGORY_TO_HARFBUZZ[(*(user_data as *mut CodePointMapData<GeneralCategory>))
        .as_borrowed()
        .get32(unicode) as usize] as hb_unicode_general_category_t
}

unsafe extern "C" fn icu4x_hb_unicode_general_category_destroy(user_data: *mut c_void) {
    let _ = Box::from_raw(user_data as *mut CodePointMapData<GeneralCategory>);
}

/// Returns the Bidi_Mirroring_Glyph, but adjusting the return value
/// to fix HarfBuzz expected behavior for code points whose property value
/// for Bidi_Mirroring_Glyph is the undefined value.
///
/// From HarfBuzz docs on `hb_unicode_mirroring_func_t`:
/// <note>Note: If a code point does not have a specified
/// Bi-Directional Mirroring Glyph defined, the method should
/// return the original code point.</note>
unsafe extern "C" fn icu4x_hb_unicode_mirroring(
    _ufuncs: *mut hb_unicode_funcs_t,
    unicode: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_codepoint_t {
    (*(user_data as *mut BidiAuxiliaryProperties))
        .as_borrowed()
        .get32_mirroring_props(unicode)
        .mirroring_glyph
        .map(u32::from)
        .unwrap_or(unicode) as hb_codepoint_t
}

unsafe extern "C" fn icu4x_hb_unicode_mirroring_destroy(user_data: *mut c_void) {
    let _ = Box::from_raw(user_data as *mut BidiAuxiliaryProperties);
}

struct ScriptDataForHarfBuzz {
    script_map: CodePointMapData<Script>,
    enum_to_name_mapper: PropertyEnumToValueNameLinearTiny4Mapper<Script>,
}

unsafe extern "C" fn icu4x_hb_unicode_script(
    _ufuncs: *mut hb_unicode_funcs_t,
    unicode: hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_script_t {
    let script_data: &ScriptDataForHarfBuzz = &*(user_data as *mut ScriptDataForHarfBuzz);
    let script: Script = script_data.script_map.as_borrowed().get32(unicode);
    let enum_to_name_mapper = script_data.enum_to_name_mapper.as_borrowed();
    let name: TinyStr4 = enum_to_name_mapper
        .get(script)
        .unwrap_or(tinystr!(4, "Zzzz"));
    hb_script_from_string(
        name.as_ptr() as *const c_char,
        name.len().try_into().unwrap_or(0),
    )
}

unsafe extern "C" fn icu4x_hb_unicode_script_destroy(user_data: *mut c_void) {
    let _ = Box::from_raw(user_data as *mut ScriptDataForHarfBuzz);
}

unsafe extern "C" fn icu4x_hb_unicode_compose(
    _ufuncs: *mut hb_unicode_funcs_t,
    a: hb_codepoint_t,
    b: hb_codepoint_t,
    ab: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_bool_t {
    // It appears that HarfBuzz will pass valid scalar values
    // unless the application violated the contract of
    // `hb_buffer_add_codepoints` and passed in non-scalar values.
    // If we treated `hb_buffer_add_codepoints` as conceptually
    // `unsafe`, it would be appropriate not to do scalar value
    // validation here.
    let first = if let Some(first) = core::char::from_u32(a) {
        first
    } else {
        // GIGO case
        debug_assert!(false);
        return false as hb_bool_t;
    };
    let second = if let Some(second) = core::char::from_u32(b) {
        second
    } else {
        // GIGO case
        debug_assert!(false);
        return false as hb_bool_t;
    };
    if let Some(c) = (*(user_data as *mut CanonicalComposition)).compose(first, second) {
        core::ptr::write(ab, c as hb_codepoint_t);
        true as hb_bool_t
    } else {
        false as hb_bool_t
    }
}

unsafe extern "C" fn icu4x_hb_unicode_compose_destroy(user_data: *mut c_void) {
    let _ = Box::from_raw(user_data as *mut CanonicalComposition);
}

unsafe extern "C" fn icu4x_hb_unicode_decompose(
    _ufuncs: *mut hb_unicode_funcs_t,
    ab: hb_codepoint_t,
    a: *mut hb_codepoint_t,
    b: *mut hb_codepoint_t,
    user_data: *mut c_void,
) -> hb_bool_t {
    // It appears that HarfBuzz will pass valid scalar values
    // unless the application violated the contract of
    // `hb_buffer_add_codepoints` and passed in non-scalar values.
    // If we treated `hb_buffer_add_codepoints` as conceptually
    // `unsafe`, it would be appropriate not to do scalar value
    // validation here.
    let composed = if let Some(composed) = core::char::from_u32(ab) {
        composed
    } else {
        // GIGO case
        debug_assert!(false);
        return false as hb_bool_t;
    };
    match (*(user_data as *mut CanonicalDecomposition)).decompose(composed) {
        Decomposed::Default => false as hb_bool_t,
        Decomposed::Expansion(first, second) => {
            core::ptr::write(a, first as hb_codepoint_t);
            core::ptr::write(b, second as hb_codepoint_t);
            true as hb_bool_t
        }
        Decomposed::Singleton(single) => {
            core::ptr::write(a, single as hb_codepoint_t);
            core::ptr::write(b, 0);
            true as hb_bool_t
        }
    }
}

unsafe extern "C" fn icu4x_hb_unicode_decompose_destroy(user_data: *mut c_void) {
    let _ = Box::from_raw(user_data as *mut CanonicalDecomposition);
}

/// RAII holder for `*mut hb_unicode_funcs_t`.
#[derive(Debug)]
pub struct UnicodeFuncs {
    raw: *mut hb_unicode_funcs_t,
}

impl UnicodeFuncs {
    /// Takes ownership of a `*mut hb_unicode_funcs_t` without incrementing
    /// the refcount.
    ///
    /// # Safety
    ///
    /// After the call, the previous owner must not call
    /// `hb_unicode_funcs_destroy()`, since `UnicodeFuncs` will now
    /// take care of it.
    pub unsafe fn from_raw(raw: *mut hb_unicode_funcs_t) -> Self {
        Self { raw }
    }

    /// Transfers the ownership of the wrapped pointer to the caller.
    /// The caller is responsible for calling `hb_unicode_funcs_destroy()`;
    /// `UnicodeFuncs` will no longer take care of it.
    pub fn into_raw(funcs: Self) -> *mut hb_unicode_funcs_t {
        let ret = funcs.raw;
        core::mem::forget(funcs);
        ret
    }

    /// Borrows the wrapped raw pointer without transferring ownership
    /// and without affecting the refcount.
    pub fn as_ptr(&self) -> *mut hb_unicode_funcs_t {
        self.raw
    }
}

impl Drop for UnicodeFuncs {
    fn drop(&mut self) {
        unsafe {
            hb_unicode_funcs_destroy(self.raw);
        }
    }
}

/// Sets up a `hb_unicode_funcs_t` with ICU4X as the back end as the Unicode
/// Database operations that HarfBuzz needs. The `hb_unicode_funcs_t` held
/// by the returned `UnicodeFuncs` is marked immutable.
// TODO(#2838): Document the conditions under which multithreaded lookups via the
// `*mut hb_unicode_funcs_t` are OK. HarfBuzz itself takes care of atomic
// refcounting of the `hb_unicode_funcs_t`, but if `DataPayload` is built
// without the `sync` feature, do bad things still happen if the freeing
// thread doesn't match the allocation thread? At least the trait bounds
// are violated in principle.
pub fn new_hb_unicode_funcs_unstable<D>(data_provider: &D) -> Result<UnicodeFuncs, HarfBuzzError>
where
    D: DataProvider<BidiAuxiliaryPropertiesV1Marker>
        + DataProvider<CanonicalCompositionsV1Marker>
        + DataProvider<CanonicalDecompositionDataV1Marker>
        + DataProvider<CanonicalDecompositionTablesV1Marker>
        + DataProvider<NonRecursiveDecompositionSupplementV1Marker>
        + DataProvider<GeneralCategoryV1Marker>
        + DataProvider<ScriptValueToShortNameV1Marker>
        + DataProvider<ScriptV1Marker>
        + ?Sized,
{
    // Let's do all the provider operations up front so that if there's
    // a provider failure we can return early without having to clean up
    // an already-allocated `ufuncs`.
    let canonical_combining_class_map =
        Box::new(CanonicalCombiningClassMap::try_new_unstable(data_provider)?);
    let general_category_map =
        Box::new(icu_properties::maps::load_general_category(data_provider)?);
    let bidi_auxiliary_props_map = Box::new(
        icu_properties::bidi_data::load_bidi_auxiliary_properties_unstable(data_provider)?,
    );
    let script_map = icu_properties::maps::load_script(data_provider)?;
    let script_enum_to_short_name_lookup = Script::get_enum_to_short_name_mapper(data_provider)?;
    let script_data = Box::new(ScriptDataForHarfBuzz {
        script_map,
        enum_to_name_mapper: script_enum_to_short_name_lookup,
    });
    let canonical_composition = Box::new(CanonicalComposition::try_new_unstable(data_provider)?);
    let canonical_decomposition =
        Box::new(CanonicalDecomposition::try_new_unstable(data_provider)?);
    unsafe {
        let empty = hb_unicode_funcs_get_empty();
        // The HarfBuzz refcounting convention is that "create"
        // sets refcount to one, not zero.
        // https://harfbuzz.github.io/object-model-lifecycle.html
        let ufuncs = hb_unicode_funcs_create(empty);
        if ufuncs == empty {
            return Err(HarfBuzzError::Alloc);
        }
        // Below this point, the only return is upon success. Now we
        // can turn the `Box`es into raw pointers. Let's do that
        // as an intermediate step with explicit types to be convinced
        // that the types before casting to `*mut c_void` are the same
        // ones that we'll cast back to in the callbacks.
        let canonical_combining_class_map_ptr: *mut CanonicalCombiningClassMap =
            Box::into_raw(canonical_combining_class_map);
        let general_category_map_ptr: *mut CodePointMapData<GeneralCategory> =
            Box::into_raw(general_category_map);
        let bidi_auxiliary_props_map_ptr: *mut BidiAuxiliaryProperties =
            Box::into_raw(bidi_auxiliary_props_map);
        let script_map_ptr: *mut ScriptDataForHarfBuzz = Box::into_raw(script_data);
        let canonical_composition_ptr: *mut CanonicalComposition =
            Box::into_raw(canonical_composition);
        let canonical_decomposition_ptr: *mut CanonicalDecomposition =
            Box::into_raw(canonical_decomposition);

        hb_unicode_funcs_set_combining_class_func(
            ufuncs,
            Some(icu4x_hb_unicode_combining_class),
            canonical_combining_class_map_ptr as *mut c_void,
            Some(icu4x_hb_unicode_combining_class_destroy),
        );
        hb_unicode_funcs_set_general_category_func(
            ufuncs,
            Some(icu4x_hb_unicode_general_category),
            general_category_map_ptr as *mut c_void,
            Some(icu4x_hb_unicode_general_category_destroy),
        );
        hb_unicode_funcs_set_mirroring_func(
            ufuncs,
            Some(icu4x_hb_unicode_mirroring),
            bidi_auxiliary_props_map_ptr as *mut c_void,
            Some(icu4x_hb_unicode_mirroring_destroy),
        );
        hb_unicode_funcs_set_script_func(
            ufuncs,
            Some(icu4x_hb_unicode_script),
            script_map_ptr as *mut c_void,
            Some(icu4x_hb_unicode_script_destroy),
        );
        hb_unicode_funcs_set_compose_func(
            ufuncs,
            Some(icu4x_hb_unicode_compose),
            canonical_composition_ptr as *mut c_void,
            Some(icu4x_hb_unicode_compose_destroy),
        );
        hb_unicode_funcs_set_decompose_func(
            ufuncs,
            Some(icu4x_hb_unicode_decompose),
            canonical_decomposition_ptr as *mut c_void,
            Some(icu4x_hb_unicode_decompose_destroy),
        );

        // Compatibility decomposition and East Asian Width lookups
        // are deprecated, and there's no need to set up the callbacks
        // for those.

        hb_unicode_funcs_make_immutable(ufuncs);
        Ok(UnicodeFuncs::from_raw(ufuncs))
    }
}
