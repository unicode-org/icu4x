// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module provides convenience macros.
//!
//! # Examples
//!
//! ```rust
//! use icu::locid::{language, region, langid};
//!
//! let lid = langid!("EN_US");
//!
//! assert_eq!(lid.language, language!("en"));
//! assert_eq!(lid.region, Some(region!("US")));
//! ```

/// A macro allowing for compile-time construction of valid [`Language`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Language;
/// use icu::locid::language;
///
/// const DE: Language = language!("DE");
///
/// let de: Language = "DE".parse()
///     .expect("Failed to parse language subtag.");
///
/// assert_eq!(DE, "de");
/// assert_eq!(DE, de);
/// ```
///
/// [`Language`]: crate::subtags::Language
#[macro_export]
macro_rules! language {
    ($language:literal) => {{
        const R: $crate::subtags::Language =
            match $crate::subtags::Language::from_bytes($language.as_bytes()) {
                Ok(r) => r,
                _ => panic!(concat!("Invalid language code: ", $language)),
            };
        R
    }};
}

/// A macro allowing for compile-time construction of valid [`Script`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Script;
/// use icu::locid::script;
///
/// const ARAB: Script = script!("aRAB");
///
/// let arab: Script = "aRaB".parse()
///     .expect("Failed to parse script subtag.");
///
/// assert_eq!(ARAB, "Arab");
/// assert_eq!(ARAB, arab);
/// ```
///
/// [`Script`]: crate::subtags::Script
#[macro_export]
macro_rules! script {
    ($script:literal) => {{
        const R: $crate::subtags::Script =
            match $crate::subtags::Script::from_bytes($script.as_bytes()) {
                Ok(r) => r,
                _ => panic!(concat!("Invalid script code: ", $script)),
            };
        R
    }};
}

/// A macro allowing for compile-time construction of valid [`Region`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Region;
/// use icu::locid::region;
///
/// const CN: Region = region!("cn");
///
/// let cn: Region = "cn".parse()
///     .expect("Failed to parse region subtag.");
///
/// assert_eq!(CN, "CN");
/// assert_eq!(CN, cn);
/// ```
///
/// [`Region`]: crate::subtags::Region
#[macro_export]
macro_rules! region {
    ($region:literal) => {{
        const R: $crate::subtags::Region =
            match $crate::subtags::Region::from_bytes($region.as_bytes()) {
                Ok(r) => r,
                _ => panic!(concat!("Invalid region code: ", $region)),
            };
        R
    }};
}

/// A macro allowing for compile-time construction of valid [`Variant`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Variant;
/// use icu::locid::variant;
///
/// const POSIX: Variant = variant!("Posix");
///
/// let posix: Variant = "Posix".parse()
///     .expect("Failed to parse variant subtag.");
///
/// assert_eq!(POSIX, "posix");
/// assert_eq!(POSIX, posix);
/// ```
///
/// [`Variant`]: crate::subtags::Variant
#[macro_export]
macro_rules! variant {
    ($variant:literal) => {{
        const R: $crate::subtags::Variant =
            match $crate::subtags::Variant::from_bytes($variant.as_bytes()) {
                Ok(r) => r,
                _ => panic!(concat!("Invalid variant code: ", $variant)),
            };
        R
    }};
}

/// A macro allowing for compile-time construction of valid [`LanguageIdentifier`].
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::LanguageIdentifier;
/// use icu::locid::langid;
///
/// const DE_AT: LanguageIdentifier = langid!("de_at");
///
/// let de_at: LanguageIdentifier = "de_at".parse()
///     .expect("Failed to parse language identifier.");
///
/// assert_eq!(DE_AT, "de-AT");
/// assert_eq!(DE_AT, de_at);
/// ```
///
/// *Note*: As of Rust 1.47, the macro cannot produce language identifier
/// with variants in the const mode pending [`Heap Allocations in Constants`].
///
/// [`LanguageIdentifier`]: crate::LanguageIdentifier
/// [`Heap Allocations in Constants`]: https://github.com/rust-lang/const-eval/issues/20
#[macro_export]
macro_rules! langid {
    ($langid:literal) => {{
        const R: $crate::LanguageIdentifier =
            match $crate::LanguageIdentifier::from_bytes_without_variants($langid.as_bytes()) {
                Ok((language, script, region)) => $crate::LanguageIdentifier {
                    language,
                    script,
                    region,
                    variants: $crate::subtags::Variants::new(),
                },
                Err(_) => panic!(concat!("Invalid language code: ", $langid)),
            };
        R
    }};
}

#[cfg(test)]
mod test {
    const LANG_PL: crate::subtags::Language = language!("pL");
    const SCRIPT_LATN: crate::subtags::Script = script!("lAtN");
    const REGION_US: crate::subtags::Region = region!("us");
    const VARIANT_MACOS: crate::subtags::Variant = variant!("MACOS");
    const LANGID: crate::LanguageIdentifier = langid!("de-Arab-AT");

    #[test]
    fn language() {
        let lang = language!("Pl");

        assert_eq!(lang, "pl");
        assert_eq!(LANG_PL, "pl");
        assert_eq!(lang, LANG_PL);
    }

    #[test]
    fn script() {
        let script = script!("latn");

        assert_eq!(script, "Latn");
        assert_eq!(SCRIPT_LATN, "Latn");
        assert_eq!(script, SCRIPT_LATN);
    }

    #[test]
    fn region() {
        let region = region!("us");

        assert_eq!(region, "US");
        assert_eq!(REGION_US, "US");
        assert_eq!(region, REGION_US);
    }

    #[test]
    fn variant() {
        let variant = variant!("macOS");

        assert_eq!(variant, "macos");
        assert_eq!(VARIANT_MACOS, "macos");
        assert_eq!(variant, VARIANT_MACOS);
    }

    #[test]
    fn langid() {
        let langid = langid!("de_Arab_aT");

        assert_eq!(langid.to_string(), "de-Arab-AT");
        assert_eq!(LANGID.to_string(), "de-Arab-AT");
        assert_eq!(langid, LANGID);
    }
}
