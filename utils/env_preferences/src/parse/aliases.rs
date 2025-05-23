// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Platform-specific conversion from locale strings to BCP-47 identifiers.

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
            "radstr" => value!("unihan"),
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
