// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use env_preferences::parse::windows::WindowsLocale;
use icu_locale_core::Locale;

fn expect_success(src: &str, expected: &str) {
    let windows_locale = WindowsLocale::try_from_str(src).expect(src);
    let locale = windows_locale.try_convert_lossy().expect(src);

    assert_eq!(
        locale,
        Locale::try_from_str(expected).unwrap(),
        "Case: {src}"
    );
}

#[test]
fn collation() {
    /// All MS-LCID collation entries with a known matching CLDR collation value
    const CASES: [(&str, &str); 12] = [
        ("de-DE_phoneb", "de-DE-u-co-phonebk"),
        ("es-ES_tradnl", "es-ES-u-co-trad"),
        ("ja-JP_radstr", "ja-JP-u-co-unihan"),
        ("zh-CN_phoneb", "zh-CN-u-co-phonebk"),
        ("zh-CN_stroke", "zh-CN-u-co-stroke"),
        ("zh-HK_radstr", "zh-HK-u-co-unihan"),
        ("zh-MO_radstr", "zh-MO-u-co-unihan"),
        ("zh-MO_stroke", "zh-MO-u-co-stroke"),
        ("zh-SG_phoneb", "zh-SG-u-co-phonebk"),
        ("zh-SG_stroke", "zh-SG-u-co-stroke"),
        ("zh-TW_pronun", "zh-TW-u-co-zhuyin"),
        ("zh-TW_radstr", "zh-TW-u-co-unihan"),
    ];

    for (src, expected) in CASES {
        expect_success(src, expected);
    }
}

#[test]
fn collation_strip_known_invalid() {
    // All MS-LCID collation entries with NO known matching CLDR collation value
    expect_success("hu-HU_tchncl", "hu-HU");
    expect_success("ka-GE_modern", "ka-GE");
}

#[test]
fn collation_strip_unknown() {
    expect_success("en-US_unknown", "en-US");
    expect_success("en-US_unknown_multiple_underscores", "en-US");
    expect_success("en-US_unknown-with-hyphens", "en-US");
}

#[test]
fn alias() {
    expect_success("zh-yue-HK", "yue-HK");
    expect_success("x-IV-mathan", "und");
}
