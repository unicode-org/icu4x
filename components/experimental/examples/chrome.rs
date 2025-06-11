// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointinvlist::CodePointInversionList;
use icu_experimental::transliterate::provider::Baked;
use icu_experimental::transliterate::TransliteratorBuilder;
use icu_properties::{props::BidiClass, CodePointMapData};

fn main() {
    assert_eq!(
        TransliteratorBuilder::from_rules(Baked::TRANSLITERATOR_RULES_V1_UND_UND_HIRA_T_UND_KANA)
            .load_nfc()
            .load_nfkc()
            .build()
            .unwrap()
            .transliterate("ウィキペディア".into()),
        "うぃきぺでぃあ"
    );

    assert_eq!(
        TransliteratorBuilder::from_rules(Baked::TRANSLITERATOR_RULES_V1_UND_UND_KANA_T_UND_HIRA)
            .load_nfc()
            .load_nfkc()
            .build()
            .unwrap()
            .transliterate("うぃきぺでぃあ".into()),
        "ウィキペディア"
    );

    // ::Lower;
    //
    // ::NFD;
    // ::[:Nonspacing Mark:] Remove;
    // ::NFC;
    //
    // ::Latin-ASCII;
    let t = TransliteratorBuilder::default()
        .lower(CodePointInversionList::all())
        .nfd(CodePointInversionList::all())
        .remove({
            use icu_properties::{props::BidiClass, CodePointMapData};
            CodePointMapData::<BidiClass>::new()
                .get_set_for_value(BidiClass::NonspacingMark)
                .to_code_point_inversion_list()
                .into_owned()
        })
        .nfc(CodePointInversionList::all())
        .call(
            Baked::TRANSLITERATOR_RULES_V1_UND_UND_T_UND_LATN_D0_ASCII,
            CodePointInversionList::all(),
        )
        .build()
        .unwrap();

    assert_eq!(t.transliterate("Täst 😒 Ω".into()), "tast 😒 ω");

    // [ö {o \u0308} Ö {O \u0308}] → oe;
    // [ä {a \u0308} Ä {A \u0308}] → ae;
    // [ü {u \u0308} Ü {U \u0308}] → ue;
    //
    // :: Autocomplete;
    let t = TransliteratorBuilder::default()
        .replace(&["ö", "o\u{0308}", "Ö", "O\u{0308}"], "oe")
        .replace(&["ä", "a\u{0308}", "Ä", "A\u{0308}"], "ae")
        .replace(&["ü", "u\u{0308}", "Ö", "U\u{0308}"], "ue")
        .lower(CodePointInversionList::all())
        .nfd(CodePointInversionList::all())
        .remove(
            CodePointMapData::<BidiClass>::new()
                .get_set_for_value(BidiClass::NonspacingMark)
                .to_code_point_inversion_list()
                .into_owned(),
        )
        .nfc(CodePointInversionList::all())
        .call(
            Baked::TRANSLITERATOR_RULES_V1_UND_UND_T_UND_LATN_D0_ASCII,
            CodePointInversionList::all(),
        )
        .build()
        .unwrap();

    assert_eq!(t.transliterate("Täst 😒 Ω".into()), "taest 😒 ω");
}
