// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_collections::codepointinvlist::CodePointInversionList;
use icu_experimental::transliterate::provider::Baked;
use icu_experimental::transliterate::{Transliterator, TransliteratorBuilder};
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

    assert_eq!(
        autocomplete_transliterator(false).transliterate("Täst 😒 Ω".into()),
        "tast 😒 ω"
    );

    assert_eq!(
        autocomplete_transliterator(true).transliterate("Täst 😒 Ω".into()),
        "taest 😒 ω"
    );
}

// ::Lower;
//
// ::NFD;
// {o \u0308} → oe; # is_de only
// {a \u0308} → ae; # is_de only
// {u \u0308} → ue; # is_de only
// ::[:Nonspacing Mark:] Remove;
// ::NFC;
//
// ::Latin-ASCII;
fn autocomplete_transliterator(is_de: bool) -> Transliterator {
    let mut t = TransliteratorBuilder::default()
        .lower(CodePointInversionList::all())
        .nfd(CodePointInversionList::all());
    if is_de {
        t = t
            .replace(["o\u{0308}"].into_iter().collect(), "oe".into())
            .replace(["a\u{0308}"].into_iter().collect(), "ae".into())
            .replace(["u\u{0308}"].into_iter().collect(), "ue".into());
    }
    t.remove(
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
    .unwrap()
}
