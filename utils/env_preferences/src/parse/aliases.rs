// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Platform-specific conversion from locale strings to BCP-47 identifiers.

/// Find a BCP-47 language/region from a list of POSIX 'locale name' aliases.
///
/// This is based on GNU libc's `intl/locale.alias` file, with some manual processing
/// to remove duplicates. The notable exception is that the default `C`/`POSIX` locales
/// map to `en-US`. The original file is available at:
/// <https://sourceware.org/git/?p=glibc.git;a=blob;f=intl/locale.alias;hb=HEAD>
pub fn find_posix_locale_name_alias(
    posix_locale: &str,
) -> Option<(
    icu_locale_core::subtags::Language,
    icu_locale_core::subtags::Region,
)> {
    use icu_locale_core::subtags::{language, region};

    match posix_locale {
        "C" | "POSIX" => Some((language!("en"), region!("US"))),
        "bokmal" => Some((language!("nb"), region!("NO"))),
        "catalan" => Some((language!("ca"), region!("ES"))),
        "croatian" => Some((language!("hr"), region!("HR"))),
        "czech" => Some((language!("cs"), region!("CZ"))),
        "danish" => Some((language!("da"), region!("DK"))),
        "dansk" => Some((language!("da"), region!("DK"))),
        "deutsch" => Some((language!("de"), region!("DE"))),
        "dutch" => Some((language!("nl"), region!("NL"))),
        "eesti" => Some((language!("et"), region!("EE"))),
        "estonian" => Some((language!("et"), region!("EE"))),
        "finnish" => Some((language!("fi"), region!("FI"))),
        "french" => Some((language!("fr"), region!("FR"))),
        "galego" => Some((language!("gl"), region!("ES"))),
        "galician" => Some((language!("gl"), region!("ES"))),
        "german" => Some((language!("de"), region!("DE"))),
        "greek" => Some((language!("el"), region!("GR"))),
        "hebrew" => Some((language!("he"), region!("IL"))),
        "hrvatski" => Some((language!("hr"), region!("HR"))),
        "hungarian" => Some((language!("hu"), region!("HU"))),
        "icelandic" => Some((language!("is"), region!("IS"))),
        "italian" => Some((language!("it"), region!("IT"))),
        "japanese" => Some((language!("ja"), region!("JP"))),
        "korean" => Some((language!("ko"), region!("KR"))),
        "lithuanian" => Some((language!("lt"), region!("LT"))),
        "norwegian" => Some((language!("nb"), region!("NO"))),
        "nynorsk" => Some((language!("nn"), region!("NO"))),
        "polish" => Some((language!("pl"), region!("PL"))),
        "portuguese" => Some((language!("pt"), region!("PT"))),
        "romanian" => Some((language!("ro"), region!("RO"))),
        "russian" => Some((language!("ru"), region!("RU"))),
        "slovak" => Some((language!("sk"), region!("SK"))),
        "slovene" => Some((language!("sl"), region!("SI"))),
        "slovenian" => Some((language!("sl"), region!("SI"))),
        "spanish" => Some((language!("es"), region!("ES"))),
        "swedish" => Some((language!("sv"), region!("SE"))),
        "thai" => Some((language!("th"), region!("TH"))),
        "turkish" => Some((language!("tr"), region!("TR"))),
        _ => None,
    }
}

/// Strip any Windows "Sort Order Identifier" and return a matching CLDR collation value.
///
/// The full table is available at:
/// <https://learn.microsoft.com/en-us/windows/win32/intl/sort-order-identifiers>
pub fn strip_windows_collation_suffix_lossy(
    lcid: &str,
) -> (&str, Option<icu_locale_core::extensions::unicode::Value>) {
    use icu_locale_core::extensions::unicode::value;

    // All known LCIDs containing an underscore are used for a collation suffix
    if let Some((prefix, suffix)) = lcid.split_once('_') {
        let collation_value = match suffix {
            "phoneb" => value!("phonebk"),
            "pronun" => value!("zhuyin"),
            "radstr" => value!("stroke"),
            "stroke" => value!("stroke"),
            "tradnl" => value!("trad"),
            // Strip the suffix on LCIDs with an underscore but no (known) matching CLDR data
            _ => return (prefix, None),
        };

        // Return the LCID with the stripped prefix, and the matching CLDR collation key
        (prefix, Some(collation_value))
    } else {
        // No underscore found, return the LCID as-is
        (lcid, None)
    }
}

/// Find a BCP-47 identifier from a list of known Windows aliases.
pub fn find_windows_language_alias_lossy(
    lcid: &str,
) -> Option<icu_locale_core::LanguageIdentifier> {
    use icu_locale_core::langid;

    match lcid {
        "zh-yue-HK" => Some(langid!("yue-HK")),
        // LCID with no (known) matching CLDR data: "math alphanumeric sorting"
        // This would be `x-IV_mathan`, but the collation suffix may already be stripped by
        // `strip_windows_collation_suffix_lossy`. For some reason, `LocaleEnumProcEx` also uses
        // `x-IV-mathan`, so that is included here too.
        // https://learn.microsoft.com/en-us/windows/win32/api/winnls/nc-winnls-locale_enumprocex
        "x-IV" | "x-IV_mathan" | "x-IV-mathan" => Some(langid!("und")),
        _ => None,
    }
}
