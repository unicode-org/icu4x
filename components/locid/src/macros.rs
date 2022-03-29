// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

/// A macro allowing for compile-time construction of valid [`Language`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::{language, subtags::Language};
///
/// const DE: Language = language!("DE");
///
/// let de: Language = "DE".parse().unwrap();
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
                #[allow(clippy::panic)] // const context
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
/// use icu::locid::{script, subtags::Script};
///
/// const ARAB: Script = script!("aRAB");
///
/// let arab: Script = "aRaB".parse().unwrap();
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
                #[allow(clippy::panic)] // const context
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
/// use icu::locid::{region, subtags::Region};
///
/// const CN: Region = region!("cn");
///
/// let cn: Region = "cn".parse().unwrap();
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
                #[allow(clippy::panic)] // const context
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
/// use icu::locid::{variant, subtags::Variant};
///
/// const POSIX: Variant = variant!("Posix");
///
/// let posix: Variant = "Posix".parse().unwrap();
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
                #[allow(clippy::panic)] // const context
                _ => panic!(concat!("Invalid variant code: ", $variant)),
            };
        R
    }};
}

/// A macro allowing for compile-time construction of valid [`LanguageIdentifier`]s.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::{LanguageIdentifier, langid};
///
/// const DE_AT: LanguageIdentifier = langid!("de_at");
///
/// let de_at: LanguageIdentifier = "de_at".parse().unwrap();
///
/// assert_eq!(DE_AT, "de-AT");
/// assert_eq!(DE_AT, de_at);
/// ```
///
/// *Note*: As of Rust 1.47, the macro cannot produce language identifier
/// with variants due to const limitations (see [`Heap Allocations in Constants`]).
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
                #[allow(clippy::panic)] // const context
                _ => panic!(concat!("Invalid language code: ", $langid, " . Note that variant tags are not \
                                        supported by the langid! macro, use runtime parsing instead.")),
            };
        R
    }};
}

/// A macro allowing for compile-time construction of valid [`Locale`]s.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::{Locale, locale};
///
/// const DE_AT: Locale = locale!("de_at");
///
/// let de_at: Locale = "de_at".parse().unwrap();
///
/// assert_eq!(DE_AT, "de-AT");
/// assert_eq!(DE_AT, de_at);
/// ```
///
/// *Note*: As of Rust 1.47, the macro cannot produce locales with variants or
/// Unicode extensions due to const limitations (see [`Heap Allocations in Constants`]).
///
/// [`Locale`]: crate::Locale
/// [`Heap Allocations in Constants`]: https://github.com/rust-lang/const-eval/issues/20
#[macro_export]
macro_rules! locale {
    ($locale:literal) => {{
        const R: $crate::Locale =
            match $crate::LanguageIdentifier::from_bytes_without_variants($locale.as_bytes()) {
                Ok((language, script, region)) => $crate::Locale {
                    id: $crate::LanguageIdentifier {
                        language,
                        script,
                        region,
                        variants: $crate::subtags::Variants::new(),
                    },
                    extensions: $crate::extensions::Extensions::new(),
                },
                #[allow(clippy::panic)] // const context
                _ => panic!(concat!("Invalid language code: ", $locale, " . Note that variant tags and \
                                        Unicode extensions are not supported by the locale! macro, use \
                                        runtime parsing instead.")),
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
    const LOCALE: crate::Locale = locale!("de-Arab-AT");

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

    #[test]
    fn locale() {
        let locale = locale!("de_Arab_aT");
        
        assert_eq!(locale.to_string(), "de-Arab-AT");
        assert_eq!(LOCALE.to_string(), "de-Arab-AT");
        assert_eq!(locale, LOCALE);
    }
}
