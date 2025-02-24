// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale::subtags::{language, region, Language, Region};

/// Find a BCP-47 language/region from a list of POSIX locale aliases.
///
/// Some of these conversions are approximate and not exact (e.g. "C"->"und").
/// This is based on GNU libc's `intl/locale.alias` file, with some manual processing
/// to remove duplicates. The original file is available at:
/// https://sourceware.org/git/?p=glibc.git;a=blob;f=intl/locale.alias;hb=HEAD
pub fn get_bcp47_subtags_from_posix_alias(alias: &str) -> Option<(Language, Option<Region>)> {
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
