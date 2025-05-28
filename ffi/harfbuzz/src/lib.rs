// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

// https://github.com/unicode-org/icu4x/blob/main/documents/process/boilerplate.md#library-annotations

#![cfg_attr(not(any(test, doc)), no_std)]
#![cfg_attr(
    not(test),
    deny(
        clippy::indexing_slicing,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::panic,
        clippy::exhaustive_structs,
        clippy::exhaustive_enums,
        clippy::trivially_copy_pass_by_ref,
        missing_debug_implementations,
    )
)]
#![warn(missing_docs)]

//! Using ICU4X as the Unicode Database back end for HarfBuzz.
//!
//! # Examples
//!
//! See `examples/cargo/harfbuzz` in the ICU4X repo for an example.
//!
//! If you wish to load data dynamically, you can individually load [`GeneralCategoryData`], [`CombiningClassData`],
//! [`MirroringData`], [`ScriptData`], [`ComposeData`], [`DecomposeData`] and set them as the relevant funcs.
extern crate alloc;

use icu_normalizer::properties::CanonicalCombiningClassMap;
#[cfg(feature = "compiled_data")]
use icu_normalizer::properties::CanonicalCombiningClassMapBorrowed;
use icu_normalizer::properties::CanonicalComposition;
#[cfg(feature = "compiled_data")]
use icu_normalizer::properties::CanonicalCompositionBorrowed;
use icu_normalizer::properties::CanonicalDecomposition;
#[cfg(feature = "compiled_data")]
use icu_normalizer::properties::CanonicalDecompositionBorrowed;
use icu_normalizer::properties::Decomposed;
use icu_normalizer::provider::{
    NormalizerNfcV1, NormalizerNfdDataV1, NormalizerNfdSupplementV1, NormalizerNfdTablesV1,
};
use icu_properties::props::{BidiMirroringGlyph, GeneralCategory, Script};
use icu_properties::provider::{
    PropertyEnumBidiMirroringGlyphV1, PropertyEnumGeneralCategoryV1, PropertyEnumScriptV1,
    PropertyNameShortScriptV1,
};
use icu_properties::{CodePointMapData, PropertyNamesShort};
use icu_provider::prelude::*;

use harfbuzz_traits::{
    CombiningClassFunc, ComposeFunc, DecomposeFunc, GeneralCategoryFunc, MirroringFunc, ScriptFunc,
};

/// A single copyable AllUnicodeFuncs type that implements all of the `harfbuzz_trait` traits with compiled data.
///
/// Can be passed to the `harfbuzz` crate's `UnicodeFuncsBuilder`.
#[cfg(feature = "compiled_data")]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[allow(clippy::exhaustive_structs)] // unit struct
pub struct AllUnicodeFuncs;

#[cfg(feature = "compiled_data")]
impl AllUnicodeFuncs {
    /// Construct a new boxed [`AllUnicodeFuncs`]. As [`AllUnicodeFuncs`] is zero-sized,
    /// this does not allocate memory. Useful for use with harfbuzz's `UnicodeFuncs::set_*_func()`
    pub fn boxed() -> alloc::boxed::Box<Self> {
        alloc::boxed::Box::new(Self)
    }
}

#[cfg(feature = "compiled_data")]
impl GeneralCategoryFunc for AllUnicodeFuncs {
    #[inline]
    fn general_category(&self, ch: char) -> harfbuzz_traits::GeneralCategory {
        convert_gc(CodePointMapData::<GeneralCategory>::new().get(ch))
    }
}

#[cfg(feature = "compiled_data")]
impl CombiningClassFunc for AllUnicodeFuncs {
    #[inline]
    fn combining_class(&self, ch: char) -> u8 {
        CanonicalCombiningClassMapBorrowed::new().get_u8(ch)
    }
}

#[cfg(feature = "compiled_data")]
impl MirroringFunc for AllUnicodeFuncs {
    #[inline]
    fn mirroring(&self, ch: char) -> char {
        CodePointMapData::<BidiMirroringGlyph>::new()
            .get(ch)
            .mirroring_glyph
            .unwrap_or(ch)
    }
}

#[cfg(feature = "compiled_data")]
impl ScriptFunc for AllUnicodeFuncs {
    #[inline]
    fn script(&self, ch: char) -> [u8; 4] {
        let script = CodePointMapData::<Script>::new().get(ch);
        PropertyNamesShort::<Script>::new()
            .get_locale_script(script)
            .unwrap_or(icu_locale_core::subtags::script!("Zzzz"))
            .into_raw()
    }
}

#[cfg(feature = "compiled_data")]
impl ComposeFunc for AllUnicodeFuncs {
    #[inline]
    fn compose(&self, a: char, b: char) -> Option<char> {
        CanonicalCompositionBorrowed::new().compose(a, b)
    }
}
#[cfg(feature = "compiled_data")]
impl DecomposeFunc for AllUnicodeFuncs {
    #[inline]
    fn decompose(&self, ab: char) -> Option<(char, char)> {
        match CanonicalDecompositionBorrowed::new().decompose(ab) {
            Decomposed::Default => None,
            Decomposed::Expansion(first, second) => Some((first, second)),
            Decomposed::Singleton(single) => Some((single, '\0')),
        }
    }
}

/// Implementer of [`GeneralCategoryFunc`] using dynamically loaded Unicode data.
///
/// Can be passed to the `harfbuzz` crate's `AllUnicodeFuncsBuilder`.
#[derive(Debug)]
pub struct GeneralCategoryData {
    gc: CodePointMapData<GeneralCategory>,
}

impl GeneralCategoryData {
    /// Construct a new [`GeneralCategoryData`] from a data provider.
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<PropertyEnumGeneralCategoryV1> + ?Sized,
    {
        let gc = CodePointMapData::<GeneralCategory>::try_new_unstable(provider)?;

        Ok(Self { gc })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::try_new_unstable)]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Self::try_new_unstable(&provider.as_deserializing())
    }
}

fn convert_gc(gc: GeneralCategory) -> harfbuzz_traits::GeneralCategory {
    match gc {
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
        GeneralCategory::ParagraphSeparator => harfbuzz_traits::GeneralCategory::ParagraphSeparator,
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
        GeneralCategory::InitialPunctuation => harfbuzz_traits::GeneralCategory::InitialPunctuation,
        GeneralCategory::FinalPunctuation => harfbuzz_traits::GeneralCategory::FinalPunctuation,
        GeneralCategory::OtherPunctuation => harfbuzz_traits::GeneralCategory::OtherPunctuation,
        GeneralCategory::MathSymbol => harfbuzz_traits::GeneralCategory::MathSymbol,
        GeneralCategory::CurrencySymbol => harfbuzz_traits::GeneralCategory::CurrencySymbol,
        GeneralCategory::ModifierSymbol => harfbuzz_traits::GeneralCategory::ModifierSymbol,
        GeneralCategory::OtherSymbol => harfbuzz_traits::GeneralCategory::OtherSymbol,
    }
}

impl GeneralCategoryFunc for GeneralCategoryData {
    #[inline]
    fn general_category(&self, ch: char) -> harfbuzz_traits::GeneralCategory {
        convert_gc(self.gc.as_borrowed().get(ch))
    }
}

/// Implementer of [`CombiningClassFunc`] using dynamically loaded Unicode data.
///
/// Can be passed to the `harfbuzz` crate's `AllUnicodeFuncsBuilder`.
#[derive(Debug)]
pub struct CombiningClassData {
    ccc: CanonicalCombiningClassMap,
}

impl CombiningClassData {
    /// Construct a new [`CombiningClassData`] from a data provider.
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<NormalizerNfdDataV1> + ?Sized,
    {
        let ccc = CanonicalCombiningClassMap::try_new_unstable(provider)?;

        Ok(Self { ccc })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::try_new_unstable)]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Self::try_new_unstable(&provider.as_deserializing())
    }
}

impl CombiningClassFunc for CombiningClassData {
    #[inline]
    fn combining_class(&self, ch: char) -> u8 {
        self.ccc.as_borrowed().get_u8(ch)
    }
}

/// Implementer of [`CombiningClassFunc`] using dynamically loaded Unicode data.
///
/// Can be passed to the `harfbuzz` crate's `AllUnicodeFuncsBuilder`.
#[derive(Debug)]
pub struct MirroringData {
    bidi: CodePointMapData<BidiMirroringGlyph>,
}

impl MirroringData {
    /// Construct a new [`MirroringData`] from a data provider.
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<PropertyEnumBidiMirroringGlyphV1> + ?Sized,
    {
        let bidi = CodePointMapData::try_new_unstable(provider)?;

        Ok(Self { bidi })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::try_new_unstable)]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Self::try_new_unstable(&provider.as_deserializing())
    }
}

impl MirroringFunc for MirroringData {
    #[inline]
    fn mirroring(&self, ch: char) -> char {
        self.bidi
            .as_borrowed()
            .get(ch)
            .mirroring_glyph
            .unwrap_or(ch)
    }
}

/// Implementer of [`CombiningClassFunc`] using dynamically loaded Unicode data.
///
/// Can be passed to the `harfbuzz` crate's `AllUnicodeFuncsBuilder`.
#[derive(Debug)]
pub struct ScriptData {
    script: CodePointMapData<Script>,
    script_names: PropertyNamesShort<Script>,
}

impl ScriptData {
    /// Construct a new [`ScriptData`] from a data provider.
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<PropertyEnumScriptV1> + DataProvider<PropertyNameShortScriptV1> + ?Sized,
    {
        let script_set = CodePointMapData::<Script>::try_new_unstable(provider)?;
        let script_names = PropertyNamesShort::try_new_unstable(provider)?;
        Ok(Self {
            script: script_set,
            script_names,
        })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::try_new_unstable)]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Self::try_new_unstable(&provider.as_deserializing())
    }
}

impl ScriptFunc for ScriptData {
    #[inline]
    fn script(&self, ch: char) -> [u8; 4] {
        let script = self.script.as_borrowed().get(ch);
        self.script_names
            .as_borrowed()
            .get_locale_script(script)
            .unwrap_or(icu_locale_core::subtags::script!("Zzzz"))
            .into_raw()
    }
}

/// Implementer of [`CombiningClassFunc`] using dynamically loaded Unicode data.
///
/// Can be passed to the `harfbuzz` crate's `AllUnicodeFuncsBuilder`.
#[derive(Debug)]
pub struct ComposeData {
    comp: CanonicalComposition,
}

impl ComposeData {
    /// Construct a new [`ComposeData`] from a data provider.
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<NormalizerNfcV1> + ?Sized,
    {
        let comp = CanonicalComposition::try_new_unstable(provider)?;

        Ok(Self { comp })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::try_new_unstable)]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Self::try_new_unstable(&provider.as_deserializing())
    }
}

impl ComposeFunc for ComposeData {
    #[inline]
    fn compose(&self, a: char, b: char) -> Option<char> {
        self.comp.as_borrowed().compose(a, b)
    }
}

/// Implementer of [`CombiningClassFunc`] using dynamically loaded Unicode data.
///
/// Can be passed to the `harfbuzz` crate's `AllUnicodeFuncsBuilder`.
#[derive(Debug)]
pub struct DecomposeData {
    decomp: CanonicalDecomposition,
}

impl DecomposeData {
    /// Construct a new [`DecomposeData`] from a data provider.
    pub fn try_new_unstable<D>(provider: &D) -> Result<Self, DataError>
    where
        D: DataProvider<NormalizerNfdDataV1>
            + DataProvider<NormalizerNfdSupplementV1>
            + DataProvider<NormalizerNfdTablesV1>
            + ?Sized,
    {
        let decomp = CanonicalDecomposition::try_new_unstable(provider)?;

        Ok(Self { decomp })
    }

    #[cfg(feature = "serde")]
    #[doc = icu_provider::gen_buffer_unstable_docs!(BUFFER,Self::try_new_unstable)]
    pub fn try_new_with_buffer_provider(
        provider: &(impl icu_provider::buf::BufferProvider + ?Sized),
    ) -> Result<Self, DataError> {
        Self::try_new_unstable(&provider.as_deserializing())
    }
}

impl DecomposeFunc for DecomposeData {
    #[inline]
    fn decompose(&self, ab: char) -> Option<(char, char)> {
        match self.decomp.as_borrowed().decompose(ab) {
            Decomposed::Default => None,
            Decomposed::Expansion(first, second) => Some((first, second)),
            Decomposed::Singleton(single) => Some((single, '\0')),
        }
    }
}
