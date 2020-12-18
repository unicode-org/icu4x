// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locale_canonicalizer::locale_canonicalizer::LocaleCanonicalizer;
use icu_locid::Locale;
use icu_locid_macros::langid;
use icu_provider_cldr::transform::LikelySubtagsProvider;
use std::convert::TryFrom;

#[test]
fn test_maximize() {
    let json_str =
        std::fs::read_to_string("../provider_cldr/tests/testdata/likelySubtags.json").unwrap();
    let provider = LikelySubtagsProvider::try_from(json_str.as_str()).unwrap();

    let mut lc = LocaleCanonicalizer::new(&provider);

    let test_cases: &[(&Locale, &str)] = &[
        (&langid!("en-US").into(), "en-Latn-US"),
        (&langid!("en-GB").into(), "en-Latn-GB"),
        (&langid!("es-AR").into(), "es-Latn-AR"),
        (&langid!("it").into(), "it-Latn-IT"),
        (&langid!("zh-Hans-CN").into(), "zh-Hans-CN"),
        (&langid!("de-AT").into(), "de-Latn-AT"),
        (&langid!("pl").into(), "pl-Latn-PL"),
        (&langid!("fr-FR").into(), "fr-Latn-FR"),
        (&langid!("de-AT").into(), "de-Latn-AT"),
        (&langid!("sr-Cyrl-SR").into(), "sr-Cyrl-SR"),
        (&langid!("nb-NO").into(), "nb-Latn-NO"),
        (&langid!("fr-FR").into(), "fr-Latn-FR"),
        (&langid!("mk").into(), "mk-Cyrl-MK"),
        (&langid!("uk").into(), "uk-Cyrl-UA"),
        (&langid!("und-PL").into(), "pl-Latn-PL"),
        (&langid!("und-Latn-AM").into(), "ku-Latn-AM"),
        (&langid!("ug-Cyrl").into(), "ug-Cyrl-KZ"),
        (&langid!("sr-ME").into(), "sr-Latn-ME"),
        (&langid!("mn-Mong").into(), "mn-Mong-CN"),
        (&langid!("lif-Limb").into(), "lif-Limb-IN"),
        (&langid!("gan").into(), "gan-Hans-CN"),
        (&langid!("zh-Hant").into(), "zh-Hant-TW"),
        (&langid!("yue-Hans").into(), "yue-Hans-CN"),
        (&langid!("unr").into(), "unr-Beng-IN"),
        (&langid!("unr-Deva").into(), "unr-Deva-NP"),
        (&langid!("und-Thai-CN").into(), "lcp-Thai-CN"),
        (&langid!("ug-Cyrl").into(), "ug-Cyrl-KZ"),
        (&langid!("en-Latn-DE").into(), "en-Latn-DE"),
        (&langid!("pl-FR").into(), "pl-Latn-FR"),
        (&langid!("de-CH").into(), "de-Latn-CH"),
        (&langid!("tuq").into(), "tuq-Latn-ZZ"),
        (&langid!("sr-ME").into(), "sr-Latn-ME"),
        (&langid!("ng").into(), "ng-Latn-NA"),
        (&langid!("klx").into(), "klx-Latn-ZZ"),
        (&langid!("kk-Arab").into(), "kk-Arab-CN"),
        (&langid!("en-Cyrl").into(), "en-Cyrl-US"),
        (&langid!("und-Cyrl-UK").into(), "ru-Cyrl-UK"),
        (&langid!("und-Arab").into(), "ar-Arab-EG"),
        (&langid!("und-Arab-FO").into(), "fo-Arab-FO"),
        (&langid!("zh-TW").into(), "zh-Hant-TW"),
        (&langid!("und").into(), "en-Latn-US"),
        (&langid!("zh-SG").into(), "zh-Hans-SG"),
        (&langid!("und-TW").into(), "zh-Hant-TW"),
    ];

    for case in test_cases {
        assert_eq!(lc.maximize(&case.0).unwrap().to_string(), case.1);
    }

    assert!(lc.maximize(&langid!("zz").into()).is_err());
}

#[test]
fn test_minimize() {
    use icu_locid_macros::langid;

    let json_str =
        std::fs::read_to_string("../provider_cldr/tests/testdata/likelySubtags.json").unwrap();
    let provider = LikelySubtagsProvider::try_from(json_str.as_str()).unwrap();

    let mut lc = LocaleCanonicalizer::new(&provider);
    assert_eq!(
        lc.minimize(&langid!("zh-Hant").into()).unwrap().to_string(),
        "zh-TW"
    );
    assert_eq!(
        lc.minimize(&langid!("en-Latn-US").into())
            .unwrap()
            .to_string(),
        "en"
    );
}
