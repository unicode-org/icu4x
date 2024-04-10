// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations

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
//! use harfbuzz::{Buffer, Direction, UnicodeFuncsBuilder, sys};
//!
//! let mut b = Buffer::with("مساء الخير");
//!
//! let unicode_funcs = icu_harfbuzz::UnicodeFuncs::new().unwrap();
//!
//! b.set_unicode_funcs(&unicode_funcs);
//!
//! b.guess_segment_properties();
//! assert_eq!(b.get_direction(), Direction::RTL);
//! assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
//! ```

extern crate alloc;
mod error;

use crate::error::HarfBuzzError;
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

use harfbuzz_traits::{
    CombiningClassFunc, ComposeFunc, DecomposeFunc, GeneralCategoryFunc, MirroringFunc, ScriptFunc,
};

/// Implementor of various Harbfuzz traits using Unicode data.
///
/// Can be passed to the `harfbuzz` crate's `Buffer::set_unicode_funcs()`.
#[derive(Debug)]
pub struct UnicodeFuncs {
    gc: CodePointMapData<GeneralCategory>,
    script: CodePointMapData<Script>,
    script_name: PropertyEnumToValueNameLinearTiny4Mapper<Script>,
    bidi: BidiAuxiliaryProperties,
    comp: CanonicalComposition,
    decomp: CanonicalDecomposition,
    ccc: CanonicalCombiningClassMap,
}

impl UnicodeFuncs {
    /// Sets up a `UnicodeFuncs` with ICU4X compiled data as the back end as the Unicode
    /// Database operations that HarfBuzz needs.
    ///
    /// ✨ *Enabled with the `compiled_data` Cargo feature.*
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    #[cfg(feature = "compiled_data")]
    pub fn new() -> Self {
        Self {
            gc: maps::general_category().static_to_owned(),
            script: maps::script().static_to_owned(),
            script_name: Script::enum_to_short_name_mapper().static_to_owned(),
            bidi: bidi_data::bidi_auxiliary_properties().static_to_owned(),
            ccc: CanonicalCombiningClassMap::new(),
            comp: CanonicalComposition::new(),
            decomp: CanonicalDecomposition::new(),
        }
    }
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn new_unstable<D>(provider: &D) -> Result<Self, HarfBuzzError>
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
        let ccc = CanonicalCombiningClassMap::try_new_unstable(provider)?;
        let gc = maps::load_general_category(provider)?;
        let bidi = bidi_data::load_bidi_auxiliary_properties_unstable(provider)?;
        let script = maps::load_script(provider)?;
        let script_name = Script::get_enum_to_short_name_mapper(provider)?;
        let comp = CanonicalComposition::try_new_unstable(provider)?;
        let decomp = CanonicalDecomposition::try_new_unstable(provider)?;

        Ok(Self {
            gc,
            script,
            script_name,
            bidi,
            comp,
            decomp,
            ccc,
        })
    }
}

impl GeneralCategoryFunc for UnicodeFuncs {
    fn general_category(&self, ch: char) -> harfbuzz_traits::GeneralCategory {
        match self.gc.as_borrowed().get(ch) {
            GeneralCategory::Unassigned => harfbuzz_traits::GeneralCategory::Unassigned,
            GeneralCategory::UppercaseLetter => harfbuzz_traits::GeneralCategory::UppercaseLetter,
            GeneralCategory::LowercaseLetter => harfbuzz_traits::GeneralCategory::LowercaseLetter,
            GeneralCategory::TitlecaseLetter => harfbuzz_traits::GeneralCategory::TitlecaseLetter,
            GeneralCategory::ModifierLetter => harfbuzz_traits::GeneralCategory::ModifierLetter,
            GeneralCategory::OtherLetter => harfbuzz_traits::GeneralCategory::OtherLetter,
            GeneralCategory::NonspacingMark => harfbuzz_traits::GeneralCategory::NonSpacingMark,
            GeneralCategory::SpacingMark => harfbuzz_traits::GeneralCategory::SpacingMark,
            GeneralCategory::EnclosingMark => harfbuzz_traits::GeneralCategory::EnclosingMark,
            GeneralCategory::DecimalNumber => harfbuzz_traits::GeneralCategory::DecimalNumber,
            GeneralCategory::LetterNumber => harfbuzz_traits::GeneralCategory::LetterNumber,
            GeneralCategory::OtherNumber => harfbuzz_traits::GeneralCategory::OtherNumber,
            GeneralCategory::SpaceSeparator => harfbuzz_traits::GeneralCategory::SpaceSeparator,
            GeneralCategory::LineSeparator => harfbuzz_traits::GeneralCategory::LineSeparator,
            GeneralCategory::ParagraphSeparator => {
                harfbuzz_traits::GeneralCategory::ParagraphSeparator
            }
            GeneralCategory::Control => harfbuzz_traits::GeneralCategory::Control,
            GeneralCategory::Format => harfbuzz_traits::GeneralCategory::Format,
            GeneralCategory::PrivateUse => harfbuzz_traits::GeneralCategory::PrivateUse,
            GeneralCategory::Surrogate => harfbuzz_traits::GeneralCategory::Surrogate,
            GeneralCategory::DashPunctuation => harfbuzz_traits::GeneralCategory::DashPunctuation,
            GeneralCategory::OpenPunctuation => harfbuzz_traits::GeneralCategory::OpenPunctuation,
            GeneralCategory::ClosePunctuation => harfbuzz_traits::GeneralCategory::ClosePunctuation,
            GeneralCategory::ConnectorPunctuation => {
                harfbuzz_traits::GeneralCategory::ConnectPunctuation
            }
            GeneralCategory::InitialPunctuation => {
                harfbuzz_traits::GeneralCategory::InitialPunctuation
            }
            GeneralCategory::FinalPunctuation => harfbuzz_traits::GeneralCategory::FinalPunctuation,
            GeneralCategory::OtherPunctuation => harfbuzz_traits::GeneralCategory::OtherPunctuation,
            GeneralCategory::MathSymbol => harfbuzz_traits::GeneralCategory::MathSymbol,
            GeneralCategory::CurrencySymbol => harfbuzz_traits::GeneralCategory::CurrencySymbol,
            GeneralCategory::ModifierSymbol => harfbuzz_traits::GeneralCategory::ModifierSymbol,
            GeneralCategory::OtherSymbol => harfbuzz_traits::GeneralCategory::OtherSymbol,
        }
    }
}

impl CombiningClassFunc for UnicodeFuncs {
    fn combining_class(&self, ch: char) -> u8 {
        self.ccc.get(ch).0
    }
}

impl MirroringFunc for UnicodeFuncs {
    fn mirroring(&self, ch: char) -> char {
        self.bidi
            .as_borrowed()
            .get32_mirroring_props(ch.into())
            .mirroring_glyph
            .unwrap_or(ch)
    }
}

impl ScriptFunc for UnicodeFuncs {
    fn script(&self, ch: char) -> [u8; 4] {
        let script = self.script.as_borrowed().get(ch);
        let name = self
            .script_name
            .as_borrowed()
            .get(script)
            .unwrap_or(tinystr!(4, "Zzzz"));
        *name.all_bytes()
    }
}

impl ComposeFunc for UnicodeFuncs {
    fn compose(&self, a: char, b: char) -> Option<char> {
        self.comp.compose(a, b)
    }
}

impl DecomposeFunc for UnicodeFuncs {
    fn decompose(&self, ab: char) -> Option<(char, char)> {
        match self.decomp.decompose(ab) {
            Decomposed::Default => None,
            Decomposed::Expansion(first, second) => Some((first, second)),
            Decomposed::Singleton(single) => Some((single, '\0')),
        }
    }
}
