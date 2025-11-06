// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This is an example demonstrating integration with the [`harfbuzz`] crate.
//!
//! For more information, see the [index](..).

use harfbuzz::{sys, Buffer, Direction, UnicodeFuncs, UnicodeFuncsBuilder};
use icu_normalizer::properties::*;
use icu_properties::{
    props::{BidiMirroringGlyph, GeneralCategory},
    script::HarfbuzzScriptData,
    CodePointMapData,
};

pub fn main() {
    let funcs = compiled_funcs();

    let mut b = Buffer::with("Hello World");
    b.set_unicode_funcs(&funcs);
    b.guess_segment_properties();

    assert_eq!(b.get_direction(), Direction::LTR);
    assert_eq!(b.get_script(), sys::HB_SCRIPT_LATIN);

    let mut b = Buffer::with("مساء الخير");
    b.set_unicode_funcs(&funcs);
    b.guess_segment_properties();

    assert_eq!(b.get_direction(), Direction::RTL);
    assert_eq!(b.get_script(), sys::HB_SCRIPT_ARABIC);
}

pub fn serde_funcs<P: icu_provider::buf::BufferProvider>(
    provider: &P,
) -> Result<UnicodeFuncs, icu_provider::DataError> {
    let provider = icu_provider::buf::AsDeserializingBufferProvider::as_deserializing(provider);
    let mut builder = UnicodeFuncsBuilder::new_with_empty_parent().unwrap();
    builder.set_general_category_func(Box::new(
        CodePointMapData::<GeneralCategory>::try_new_unstable(&provider)?,
    ));
    builder.set_combining_class_func(Box::new(CanonicalCombiningClassMap::try_new_unstable(
        &provider,
    )?));
    builder.set_mirroring_func(Box::new(
        CodePointMapData::<BidiMirroringGlyph>::try_new_unstable(&provider)?,
    ));
    builder.set_script_func(Box::new(HarfbuzzScriptData::try_new_unstable(&provider)?));
    builder.set_compose_func(Box::new(CanonicalComposition::try_new_unstable(&provider)?));
    builder.set_decompose_func(Box::new(CanonicalDecomposition::try_new_unstable(
        &provider,
    )?));
    Ok(builder.build())
}

pub fn compiled_funcs() -> UnicodeFuncs {
    /// We avoid allocations by boxing a zero-sized type and redirecting to compiled data.
    struct CompiledHarfbuzzData;

    let mut builder = UnicodeFuncsBuilder::new_with_empty_parent().unwrap();
    //  Note: `CompiledHarfbuzzData` is zero-sized, so this doesn't allocate memory.
    builder.set_general_category_func(Box::new(CompiledHarfbuzzData));
    builder.set_combining_class_func(Box::new(CompiledHarfbuzzData));
    builder.set_mirroring_func(Box::new(CompiledHarfbuzzData));
    builder.set_script_func(Box::new(CompiledHarfbuzzData));
    builder.set_compose_func(Box::new(CompiledHarfbuzzData));
    builder.set_decompose_func(Box::new(CompiledHarfbuzzData));

    use harfbuzz_traits::{
        CombiningClassFunc, ComposeFunc, DecomposeFunc, GeneralCategoryFunc, MirroringFunc,
        ScriptFunc,
    };

    impl GeneralCategoryFunc for CompiledHarfbuzzData {
        #[inline]
        fn general_category(&self, ch: char) -> harfbuzz_traits::GeneralCategory {
            GeneralCategoryFunc::general_category(&CodePointMapData::<GeneralCategory>::new(), ch)
        }
    }

    impl CombiningClassFunc for CompiledHarfbuzzData {
        #[inline]
        fn combining_class(&self, ch: char) -> u8 {
            CombiningClassFunc::combining_class(&CanonicalCombiningClassMap::new(), ch)
        }
    }

    impl MirroringFunc for CompiledHarfbuzzData {
        #[inline]
        fn mirroring(&self, ch: char) -> char {
            MirroringFunc::mirroring(&CodePointMapData::<BidiMirroringGlyph>::new(), ch)
        }
    }

    impl ScriptFunc for CompiledHarfbuzzData {
        #[inline]
        fn script(&self, ch: char) -> [u8; 4] {
            ScriptFunc::script(&HarfbuzzScriptData::new(), ch)
        }
    }

    impl ComposeFunc for CompiledHarfbuzzData {
        #[inline]
        fn compose(&self, a: char, b: char) -> Option<char> {
            ComposeFunc::compose(&CanonicalComposition::new(), a, b)
        }
    }

    impl DecomposeFunc for CompiledHarfbuzzData {
        #[inline]
        fn decompose(&self, ab: char) -> Option<(char, char)> {
            DecomposeFunc::decompose(&CanonicalDecomposition::new(), ab)
        }
    }
    builder.build()
}
