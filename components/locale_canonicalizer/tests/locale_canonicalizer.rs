// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locale_canonicalizer::LocaleCanonicalizer;
use icu_locid::Locale;
use icu_locid_macros::langid;
use icu_provider_cldr::transform::LikelySubtagsProvider;
use std::convert::TryFrom;

#[test]
fn test_maximize() {
    let json_str =
        std::fs::read_to_string("../provider_cldr/tests/testdata/likelySubtags.json").unwrap();
    let provider = LikelySubtagsProvider::try_from(json_str.as_str()).unwrap();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();

    let test_cases: &[(&str, &str)] = &[
        ("en-US", "en-Latn-US"),
        ("en-GB", "en-Latn-GB"),
        ("es-AR", "es-Latn-AR"),
        ("it", "it-Latn-IT"),
        ("zh-Hans-CN", "zh-Hans-CN"),
        ("de-AT", "de-Latn-AT"),
        ("pl", "pl-Latn-PL"),
        ("fr-FR", "fr-Latn-FR"),
        ("de-AT", "de-Latn-AT"),
        ("sr-Cyrl-SR", "sr-Cyrl-SR"),
        ("nb-NO", "nb-Latn-NO"),
        ("fr-FR", "fr-Latn-FR"),
        ("mk", "mk-Cyrl-MK"),
        ("uk", "uk-Cyrl-UA"),
        ("und-PL", "pl-Latn-PL"),
        ("und-Latn-AM", "ku-Latn-AM"),
        ("ug-Cyrl", "ug-Cyrl-KZ"),
        ("sr-ME", "sr-Latn-ME"),
        ("mn-Mong", "mn-Mong-CN"),
        ("lif-Limb", "lif-Limb-IN"),
        ("gan", "gan-Hans-CN"),
        ("zh-Hant", "zh-Hant-TW"),
        ("yue-Hans", "yue-Hans-CN"),
        ("unr", "unr-Beng-IN"),
        ("unr-Deva", "unr-Deva-NP"),
        ("und-Thai-CN", "lcp-Thai-CN"),
        ("ug-Cyrl", "ug-Cyrl-KZ"),
        ("en-Latn-DE", "en-Latn-DE"),
        ("pl-FR", "pl-Latn-FR"),
        ("de-CH", "de-Latn-CH"),
        ("tuq", "tuq-Latn-ZZ"),
        ("sr-ME", "sr-Latn-ME"),
        ("ng", "ng-Latn-NA"),
        ("klx", "klx-Latn-ZZ"),
        ("kk-Arab", "kk-Arab-CN"),
        ("en-Cyrl", "en-Cyrl-US"),
        ("und-Cyrl-UK", "ru-Cyrl-UK"),
        ("und-Arab", "ar-Arab-EG"),
        ("und-Arab-FO", "fo-Arab-FO"),
        ("zh-TW", "zh-Hant-TW"),
        ("und", "en-Latn-US"),
        ("zh-SG", "zh-Hans-SG"),
        ("und-TW", "zh-Hant-TW"),
    ];

    for case in test_cases {
        let locale: Locale = case.0.parse().unwrap();
        assert_eq!(lc.maximize(&locale).unwrap().to_string(), case.1);
    }

    assert!(lc.maximize(&langid!("zz").into()).is_err());
}

#[test]
fn test_minimize() {
    use icu_locid_macros::langid;

    let json_str =
        std::fs::read_to_string("../provider_cldr/tests/testdata/likelySubtags.json").unwrap();
    let provider = LikelySubtagsProvider::try_from(json_str.as_str()).unwrap();
    let lc = LocaleCanonicalizer::new(&provider).unwrap();
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
