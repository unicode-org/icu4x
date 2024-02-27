// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/docs/process/boilerplate.md#library-annotations

#![cfg_attr(not(any(test, feature = "std")), no_std)]
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
//!
//! let mut b = Buffer::with("مساء الخير");
//!
//! let unicode_funcs = icu_harfbuzz::UnicodeFuncs::new().unwrap();
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
use core::ffi::{c_char, c_void};
use harfbuzz_sys::*;
use icu_normalizer::properties::CanonicalCombiningClassMap;
use icu_normalizer::properties::CanonicalComposition;
use icu_normalizer::properties::CanonicalDecomposition;
use icu_normalizer::properties::Decomposed;
use icu_normalizer::provider::{
    CanonicalCompositionsV1Marker, CanonicalDecompositionDataV1Marker,
    CanonicalDecompositionTablesV1Marker, NonRecursiveDecompositionSupplementV1Marker,
};
use icu_properties::bidi_data;
use icu_properties::bidi_data::BidiAuxiliaryProperties;
use icu_properties::maps;
use icu_properties::maps::CodePointMapData;
use icu_properties::names::PropertyEnumToValueNameLinearTiny4Mapper;
use icu_properties::provider::bidi_data::BidiAuxiliaryPropertiesV1Marker;
use icu_properties::provider::{
    GeneralCategoryV1Marker, ScriptV1Marker, ScriptValueToShortNameV1Marker,
};
use icu_properties::{GeneralCategory, Script};
use icu_provider::prelude::*;
use tinystr::tinystr;

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

/// RAII holder for `*mut hb_unicode_funcs_t`.
// TODO(#2838): Document the conditions under which multithreaded lookups via the
// `*mut hb_unicode_funcs_t` are OK. HarfBuzz itself takes care of atomic
// refcounting of the `hb_unicode_funcs_t`, but if `DataPayload` is built
// without the `sync` feature, do bad things still happen if the freeing
// thread doesn't match the allocation thread? At least the trait bounds
// are violated in principle.
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

    fn empty() -> Result<Self, HarfBuzzError> {
        let empty = unsafe { hb_unicode_funcs_get_empty() };
        // The HarfBuzz refcounting convention is that "create"
        // sets refcount to one, not zero.
        // https://harfbuzz.github.io/object-model-lifecycle.html
        let raw = unsafe { hb_unicode_funcs_create(empty) };
        if raw == empty {
            Err(HarfBuzzError::Alloc)
        } else {
            Ok(Self { raw })
        }
    }

    /// Sets up a `hb_unicode_funcs_t` with ICU4X compiled data as the back end as the Unicode
    /// Database operations that HarfBuzz needs. The `hb_unicode_funcs_t` held
    /// by the returned `UnicodeFuncs` is marked immutable.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new() -> Result<Self, HarfBuzzError> {
        let ufuncs = Self::empty()?;

        unsafe {
            hb_unicode_funcs_set_combining_class_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        _: *mut c_void,
                    ) -> hb_unicode_combining_class_t {
                        CanonicalCombiningClassMap::new().get32(unicode).0
                            as hb_unicode_combining_class_t
                    }
                    cb
                }),
                core::ptr::null_mut(),
                None,
            );
        }

        unsafe {
            hb_unicode_funcs_set_general_category_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        _: *mut c_void,
                    ) -> hb_unicode_general_category_t {
                        // Indexing is OK, because `GeneralCategory` data is validated upon
                        // deserialization so that there can be no out-of-range `GeneralCategory`
                        // values (which would be UB to materialize). `GeneralCategory` is a
                        // stable exhaustive enum, and the length of `ICU4X_GENERAL_CATEGORY_TO_HARFBUZZ`
                        // matches the number of enum items, so the index will always be in range here.
                        #![allow(clippy::indexing_slicing)]
                        ICU4X_GENERAL_CATEGORY_TO_HARFBUZZ
                            [maps::general_category().get32(unicode) as usize]
                            as hb_unicode_general_category_t
                    }
                    cb
                }),
                core::ptr::null_mut(),
                None,
            );
        }

        // Returns the Bidi_Mirroring_Glyph, but adjusting the return value
        // to fix HarfBuzz expected behavior for code points whose property value
        // for Bidi_Mirroring_Glyph is the undefined value.
        //
        // From HarfBuzz docs on `hb_unicode_mirroring_func_t`:
        // <note>Note: If a code point does not have a specified
        // Bi-Directional Mirroring Glyph defined, the method should
        // return the original code point.</note>
        unsafe {
            hb_unicode_funcs_set_mirroring_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        _: *mut c_void,
                    ) -> hb_codepoint_t {
                        bidi_data::bidi_auxiliary_properties()
                            .get32_mirroring_props(unicode)
                            .mirroring_glyph
                            .map(u32::from)
                            .unwrap_or(unicode) as hb_codepoint_t
                    }
                    cb
                }),
                core::ptr::null_mut(),
                None,
            );
        }

        unsafe {
            hb_unicode_funcs_set_script_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        _: *mut c_void,
                    ) -> hb_script_t {
                        let script = maps::script().get32(unicode);
                        let name = Script::enum_to_short_name_mapper()
                            .get(script)
                            .unwrap_or(tinystr!(4, "Zzzz"));

                        unsafe {
                            hb_script_from_string(
                                name.as_ptr() as *const c_char,
                                name.len().try_into().unwrap_or(0),
                            )
                        }
                    }
                    cb
                }),
                core::ptr::null_mut(),
                None,
            );
        }

        unsafe {
            hb_unicode_funcs_set_compose_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        a: hb_codepoint_t,
                        b: hb_codepoint_t,
                        ab: *mut hb_codepoint_t,
                        _: *mut c_void,
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
                        if let Some(c) = CanonicalComposition::new().compose(first, second) {
                            unsafe {
                                core::ptr::write(ab, c as hb_codepoint_t);
                            }
                            true as hb_bool_t
                        } else {
                            false as hb_bool_t
                        }
                    }
                    cb
                }),
                core::ptr::null_mut(),
                None,
            );
        }

        unsafe {
            hb_unicode_funcs_set_decompose_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        ab: hb_codepoint_t,
                        a: *mut hb_codepoint_t,
                        b: *mut hb_codepoint_t,
                        _: *mut c_void,
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
                        match CanonicalDecomposition::new().decompose(composed) {
                            Decomposed::Default => false as hb_bool_t,
                            Decomposed::Expansion(first, second) => {
                                unsafe {
                                    core::ptr::write(a, first as hb_codepoint_t);
                                }
                                unsafe {
                                    core::ptr::write(b, second as hb_codepoint_t);
                                }
                                true as hb_bool_t
                            }
                            Decomposed::Singleton(single) => {
                                unsafe {
                                    core::ptr::write(a, single as hb_codepoint_t);
                                }
                                unsafe {
                                    core::ptr::write(b, 0);
                                }
                                true as hb_bool_t
                            }
                        }
                    }
                    cb
                }),
                core::ptr::null_mut(),
                None,
            );
        }

        // Compatibility decomposition and East Asian Width lookups
        // are deprecated, and there's no need to set up the callbacks
        // for those.

        unsafe {
            hb_unicode_funcs_make_immutable(ufuncs.raw);
        }
        Ok(ufuncs)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, new_hb_unicode_funcs)]
    pub fn new_unstable<D>(provider: &D) -> Result<UnicodeFuncs, HarfBuzzError>
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
        let ufuncs = Self::empty()?;

        let canonical_combining_class_map = CanonicalCombiningClassMap::try_new_unstable(provider)?;
        let general_category_map = maps::load_general_category(provider)?;
        let bidi_auxiliary_props_map =
            bidi_data::load_bidi_auxiliary_properties_unstable(provider)?;
        let script_map = maps::load_script(provider)?;
        let script_enum_to_short_name_lookup = Script::get_enum_to_short_name_mapper(provider)?;
        let canonical_composition = CanonicalComposition::try_new_unstable(provider)?;
        let canonical_decomposition = CanonicalDecomposition::try_new_unstable(provider)?;

        unsafe {
            hb_unicode_funcs_set_combining_class_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        user_data: *mut c_void,
                    ) -> hb_unicode_combining_class_t {
                        unsafe { &*(user_data as *mut CanonicalCombiningClassMap) }
                            .get32(unicode)
                            .0 as hb_unicode_combining_class_t
                    }
                    cb
                }),
                Box::into_raw(Box::new(canonical_combining_class_map)) as *mut c_void,
                Some({
                    extern "C" fn cb(user_data: *mut c_void) {
                        drop(unsafe {
                            Box::from_raw(user_data as *mut CanonicalCombiningClassMap)
                        });
                    }
                    cb
                }),
            );
        }

        unsafe {
            hb_unicode_funcs_set_general_category_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        user_data: *mut c_void,
                    ) -> hb_unicode_general_category_t {
                        // Indexing is OK, because `GeneralCategory` data is validated upon
                        // deserialization so that there can be no out-of-range `GeneralCategory`
                        // values (which would be UB to materialize). `GeneralCategory` is a
                        // stable exhaustive enum, and the length of `ICU4X_GENERAL_CATEGORY_TO_HARFBUZZ`
                        // matches the number of enum items, so the index will always be in range here.
                        #![allow(clippy::indexing_slicing)]
                        ICU4X_GENERAL_CATEGORY_TO_HARFBUZZ[unsafe {
                            &*(user_data as *mut CodePointMapData<GeneralCategory>)
                        }
                        .as_borrowed()
                        .get32(unicode)
                            as usize] as hb_unicode_general_category_t
                    }
                    cb
                }),
                Box::into_raw(Box::new(general_category_map)) as *mut c_void,
                Some({
                    extern "C" fn cb(user_data: *mut c_void) {
                        drop(unsafe {
                            Box::from_raw(user_data as *mut CodePointMapData<GeneralCategory>)
                        });
                    }
                    cb
                }),
            );
        }

        // Returns the Bidi_Mirroring_Glyph, but adjusting the return value
        // to fix HarfBuzz expected behavior for code points whose property value
        // for Bidi_Mirroring_Glyph is the undefined value.
        //
        // From HarfBuzz docs on `hb_unicode_mirroring_func_t`:
        // <note>Note: If a code point does not have a specified
        // Bi-Directional Mirroring Glyph defined, the method should
        // return the original code point.</note>
        unsafe {
            hb_unicode_funcs_set_mirroring_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        user_data: *mut c_void,
                    ) -> hb_codepoint_t {
                        unsafe { &*(user_data as *mut BidiAuxiliaryProperties) }
                            .as_borrowed()
                            .get32_mirroring_props(unicode)
                            .mirroring_glyph
                            .map(u32::from)
                            .unwrap_or(unicode) as hb_codepoint_t
                    }
                    cb
                }),
                Box::into_raw(Box::new(bidi_auxiliary_props_map)) as *mut c_void,
                Some({
                    extern "C" fn cb(user_data: *mut c_void) {
                        drop(unsafe { Box::from_raw(user_data as *mut BidiAuxiliaryProperties) });
                    }
                    cb
                }),
            );
        }

        struct ScriptDataForHarfBuzz {
            script_map: CodePointMapData<Script>,
            enum_to_name_mapper: PropertyEnumToValueNameLinearTiny4Mapper<Script>,
        }

        unsafe {
            hb_unicode_funcs_set_script_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
                        unicode: hb_codepoint_t,
                        user_data: *mut c_void,
                    ) -> hb_script_t {
                        let script = unsafe { &*(user_data as *mut ScriptDataForHarfBuzz) }
                            .script_map
                            .as_borrowed()
                            .get32(unicode);
                        let name = unsafe { &*(user_data as *mut ScriptDataForHarfBuzz) }
                            .enum_to_name_mapper
                            .as_borrowed()
                            .get(script)
                            .unwrap_or(tinystr!(4, "Zzzz"));

                        unsafe {
                            hb_script_from_string(
                                name.as_ptr() as *const c_char,
                                name.len().try_into().unwrap_or(0),
                            )
                        }
                    }
                    cb
                }),
                Box::into_raw(Box::new(ScriptDataForHarfBuzz {
                    script_map,
                    enum_to_name_mapper: script_enum_to_short_name_lookup,
                })) as *mut c_void,
                Some({
                    extern "C" fn cb(user_data: *mut c_void) {
                        drop(unsafe { Box::from_raw(user_data as *mut ScriptDataForHarfBuzz) });
                    }
                    cb
                }),
            );
        }

        unsafe {
            hb_unicode_funcs_set_compose_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
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
                        if let Some(c) = unsafe { &*(user_data as *mut CanonicalComposition) }
                            .compose(first, second)
                        {
                            unsafe {
                                core::ptr::write(ab, c as hb_codepoint_t);
                            }
                            true as hb_bool_t
                        } else {
                            false as hb_bool_t
                        }
                    }
                    cb
                }),
                Box::into_raw(Box::new(canonical_composition)) as *mut c_void,
                Some({
                    extern "C" fn cb(user_data: *mut c_void) {
                        drop(unsafe { Box::from_raw(user_data as *mut CanonicalComposition) });
                    }
                    cb
                }),
            );
        }

        unsafe {
            hb_unicode_funcs_set_decompose_func(
                ufuncs.raw,
                Some({
                    extern "C" fn cb(
                        _: *mut hb_unicode_funcs_t,
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
                        match unsafe { &*(user_data as *mut CanonicalDecomposition) }
                            .decompose(composed)
                        {
                            Decomposed::Default => false as hb_bool_t,
                            Decomposed::Expansion(first, second) => {
                                unsafe { core::ptr::write(a, first as hb_codepoint_t) };
                                unsafe { core::ptr::write(b, second as hb_codepoint_t) };
                                true as hb_bool_t
                            }
                            Decomposed::Singleton(single) => {
                                unsafe { core::ptr::write(a, single as hb_codepoint_t) };
                                unsafe { core::ptr::write(b, 0) };
                                true as hb_bool_t
                            }
                        }
                    }
                    cb
                }),
                Box::into_raw(Box::new(canonical_decomposition)) as *mut c_void,
                Some({
                    extern "C" fn cb(user_data: *mut c_void) {
                        drop(unsafe { Box::from_raw(user_data as *mut CanonicalDecomposition) });
                    }
                    cb
                }),
            );
        }

        // Compatibility decomposition and East Asian Width lookups
        // are deprecated, and there's no need to set up the callbacks
        // for those.

        unsafe {
            hb_unicode_funcs_make_immutable(ufuncs.raw);
        }

        Ok(ufuncs)
    }
}

impl Drop for UnicodeFuncs {
    fn drop(&mut self) {
        unsafe {
            hb_unicode_funcs_destroy(self.raw);
        }
    }
}
