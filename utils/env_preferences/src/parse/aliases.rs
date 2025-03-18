// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Platform-specific conversion from locale strings to BCP-47 identifiers.

/// Find a BCP-47 language/region from a list of POSIX locale aliases.
///
/// Some of these conversions are approximate and not exact (e.g. "C"->"und").
/// This is based on GNU libc's `intl/locale.alias` file, with some manual processing
/// to remove duplicates. The original file is available at:
/// <https://sourceware.org/git/?p=glibc.git;a=blob;f=intl/locale.alias;hb=HEAD>
#[cfg(any(doc, feature = "parse_posix", target_os = "linux"))]
pub fn find_posix_alias(
    alias: &str,
) -> Option<(
    icu_locale::subtags::Language,
    Option<icu_locale::subtags::Region>,
)> {
    use icu_locale::subtags::{language, region, Language};

    match alias {
        "C" | "POSIX" => Some((Language::UND, None)),
        "bokmal" => Some((language!("nb"), Some(region!("NO")))),
        "catalan" => Some((language!("ca"), Some(region!("ES")))),
        "croatian" => Some((language!("hr"), Some(region!("HR")))),
        "czech" => Some((language!("cs"), Some(region!("CZ")))),
        "danish" => Some((language!("da"), Some(region!("DK")))),
        "dansk" => Some((language!("da"), Some(region!("DK")))),
        "deutsch" => Some((language!("de"), Some(region!("DE")))),
        "dutch" => Some((language!("nl"), Some(region!("NL")))),
        "eesti" => Some((language!("et"), Some(region!("EE")))),
        "estonian" => Some((language!("et"), Some(region!("EE")))),
        "finnish" => Some((language!("fi"), Some(region!("FI")))),
        "french" => Some((language!("fr"), Some(region!("FR")))),
        "galego" => Some((language!("gl"), Some(region!("ES")))),
        "galician" => Some((language!("gl"), Some(region!("ES")))),
        "german" => Some((language!("de"), Some(region!("DE")))),
        "greek" => Some((language!("el"), Some(region!("GR")))),
        "hebrew" => Some((language!("he"), Some(region!("IL")))),
        "hrvatski" => Some((language!("hr"), Some(region!("HR")))),
        "hungarian" => Some((language!("hu"), Some(region!("HU")))),
        "icelandic" => Some((language!("is"), Some(region!("IS")))),
        "italian" => Some((language!("it"), Some(region!("IT")))),
        "japanese" => Some((language!("ja"), Some(region!("JP")))),
        "korean" => Some((language!("ko"), Some(region!("KR")))),
        "lithuanian" => Some((language!("lt"), Some(region!("LT")))),
        "norwegian" => Some((language!("nb"), Some(region!("NO")))),
        "nynorsk" => Some((language!("nn"), Some(region!("NO")))),
        "polish" => Some((language!("pl"), Some(region!("PL")))),
        "portuguese" => Some((language!("pt"), Some(region!("PT")))),
        "romanian" => Some((language!("ro"), Some(region!("RO")))),
        "russian" => Some((language!("ru"), Some(region!("RU")))),
        "slovak" => Some((language!("sk"), Some(region!("SK")))),
        "slovene" => Some((language!("sl"), Some(region!("SI")))),
        "slovenian" => Some((language!("sl"), Some(region!("SI")))),
        "spanish" => Some((language!("es"), Some(region!("ES")))),
        "swedish" => Some((language!("sv"), Some(region!("SE")))),
        "thai" => Some((language!("th"), Some(region!("TH")))),
        "turkish" => Some((language!("tr"), Some(region!("TR")))),
        _ => None,
    }
}

/// Strip any Windows "Sort Order Identifier" and return a matching CLDR collation value.
///
/// The full table is available at:
/// <https://learn.microsoft.com/en-us/windows/win32/intl/sort-order-identifiers>
#[cfg(any(doc, feature = "parse_windows", target_os = "windows"))]
pub fn strip_windows_collation_suffix_lossy(
    lcid: &str,
) -> (&str, Option<icu_locale::extensions::unicode::Value>) {
    use icu_locale::extensions::unicode::value;

    // All known LCIDs containing an underscore are used for a collation suffix
    if let Some((prefix, suffix)) = lcid.split_once('_') {
        let collation_value = match suffix {
            "phoneb" => value!("phonebk"),
            "pronun" => value!("zhuyin"),
            "radstr" => value!("stroke"),
            "stroke" => value!("stroke"),
            "tradnl" => value!("trad"),
            // Strip the suffix on LCIDs with an underscore but no (known) matching CLDR data
            "tchncl" | "modern" | _ => return (prefix, None),
        };

        // Return the LCID with the stripped prefix, and the matching CLDR collation key
        (prefix, Some(collation_value))
    } else {
        // No underscore found, return the LCID as-is
        (lcid, None)
    }
}

/// Find a BCP-47 identifier from a list of known Windows aliases.
#[cfg(any(doc, feature = "parse_windows", target_os = "windows"))]
pub fn find_windows_language_alias_lossy(lcid: &str) -> Option<icu_locale::LanguageIdentifier> {
    use icu_locale::langid;

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
