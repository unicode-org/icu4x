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
/// use icu::locid::{subtags::Variant, variant};
///
/// const POSIX: Variant = variant!("Posix");
///
/// let posix: Variant = "Posix".parse().unwrap();
///
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
/// use icu::locid::{langid, LanguageIdentifier};
///
/// const DE_AT: LanguageIdentifier = langid!("de_at");
///
/// let de_at: LanguageIdentifier = "de_at".parse().unwrap();
///
/// assert_eq!(DE_AT, de_at);
/// ```
///
/// *Note*: The macro cannot produce language identifiers with variants due to const
/// limitations (see [`Heap Allocations in Constants`]):
///
/// ```compile_fail
/// icu::locid::langid("de_at-foobar");
/// ```
///
/// Use runtime parsing instead:
/// ```
/// "de_at-foobar"
///     .parse::<icu::locid::LanguageIdentifier>()
///     .unwrap();
/// ```
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
/// use icu::locid::{locale, Locale};
///
/// const DE_AT: Locale = locale!("de_at");
///
/// let de_at: Locale = "de_at".parse().unwrap();
///
/// assert_eq!(DE_AT, de_at);
/// ```
///
/// *Note*: The macro cannot produce locales with variants or Unicode extensions due to
/// const limitations (see [`Heap Allocations in Constants`]):
///
/// ```compile_fail
/// icu::locid::locale!("en-US-u-ca-ja");
/// ```
/// Use runtime parsing instead:
/// ```
/// "en-US-u-ca-ja".parse::<icu::locid::Locale>().unwrap();
/// ```
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

/// A macro allowing for compile-time construction of valid Unicode [`Key`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::unicode::{Key, Value};
/// use icu::locid::unicode_ext_key;
/// use icu::locid::Locale;
/// use writeable::Writeable;
///
/// const CALENDAR_KEY: Key = unicode_ext_key!("ca");
///
/// let loc: Locale = "de-u-ca-buddhist".parse().unwrap();
///
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&CALENDAR_KEY),
///     Some(&Value::from_bytes(b"buddhist").unwrap())
/// );
/// ```
///
/// [`Key`]: crate::extensions::unicode::Key
#[macro_export]
macro_rules! unicode_ext_key {
    ($key:literal) => {{
        const R: $crate::extensions::unicode::Key =
            match $crate::extensions::unicode::Key::from_bytes($key.as_bytes()) {
                Ok(r) => r,
                #[allow(clippy::panic)] // const context
                _ => panic!(concat!("Invalid Unicode extension key: ", $key)),
            };
        R
    }};
}

/// A macro allowing for compile-time construction of valid Unicode [`Value`] subtag.
///
/// The macro only supports single-subtag values.
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::unicode::{Key, Value};
/// use icu::locid::Locale;
/// use icu::locid::{unicode_ext_key, unicode_ext_value};
/// use writeable::Writeable;
///
/// const CALENDAR_KEY: Key = unicode_ext_key!("ca");
/// const CALENDAR_VALUE: Value = unicode_ext_value!("buddhist");
///
/// let loc: Locale = "de-u-ca-buddhist".parse().unwrap();
///
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&CALENDAR_KEY),
///     Some(&CALENDAR_VALUE)
/// );
/// ```
///
/// [`Value`]: crate::extensions::unicode::Value
#[macro_export]
macro_rules! unicode_ext_value {
    ($value:literal) => {{
        // What we want:
        // const R: $crate::extensions::unicode::Value =
        //     match $crate::extensions::unicode::Value::try_from_single_subtag($value.as_bytes()) {
        //         Ok(r) => r,
        //         #[allow(clippy::panic)] // const context
        //         _ => panic!(concat!("Invalid Unicode extension value: ", $value)),
        //     };
        // Workaround until https://github.com/rust-lang/rust/issues/73255 lands:
        const R: $crate::extensions::unicode::Value =
            $crate::extensions::unicode::Value::from_tinystr(
                match $crate::extensions::unicode::Value::subtag_from_bytes($value.as_bytes()) {
                    Ok(r) => r,
                    _ => panic!(concat!("Invalid Unicode extension value: ", $value)),
                },
            );
        R
    }};
}

/// A macro allowing for compile-time construction of valid Transform [`Key`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::extensions::transform::{Key, Value};
/// use icu::locid::transform_ext_key;
/// use icu::locid::Locale;
/// use writeable::Writeable;
///
/// const HYBRID_KEY: Key = transform_ext_key!("h0");
///
/// let loc: Locale = "hi-t-en-h0-hybrid".parse().unwrap();
///
/// assert_eq!(
///     loc.extensions.transform.fields.get(&HYBRID_KEY),
///     Some(&Value::from_bytes(b"hybrid").unwrap())
/// );
/// ```
///
/// [`Key`]: crate::extensions::transform::Key
#[macro_export]
macro_rules! transform_ext_key {
    ($key:literal) => {{
        const R: $crate::extensions::transform::Key =
            match $crate::extensions::transform::Key::from_bytes($key.as_bytes()) {
                Ok(r) => r,
                #[allow(clippy::panic)] // const context
                _ => panic!(concat!("Invalid Transform extension key: ", $key)),
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
    const UNICODE_EXT_KEY: crate::extensions::unicode::Key = unicode_ext_key!("ms");
    const TRANSFORM_EXT_KEY: crate::extensions::transform::Key = transform_ext_key!("h0");

    #[test]
    fn language() {
        let lang = language!("Pl");
        assert_eq!(lang, LANG_PL);
        assert_eq!(lang.as_str(), "pl");
    }

    #[test]
    fn script() {
        let script = script!("latn");

        assert_eq!(script, SCRIPT_LATN);
        assert_eq!(script.as_str(), "Latn");
    }

    #[test]
    fn region() {
        let region = region!("us");

        assert_eq!(region, REGION_US);
        assert_eq!(region.as_str(), "US");
    }

    #[test]
    fn variant() {
        let variant = variant!("macOS");

        assert_eq!(variant, VARIANT_MACOS);
        assert_eq!(variant.as_str(), "macos");
    }

    #[test]
    fn langid() {
        let langid = langid!("de_Arab_aT");

        assert_eq!(langid, LANGID);
        assert_eq!(langid.to_string(), "de-Arab-AT");
    }

    #[test]
    fn locale() {
        let locale = locale!("de_Arab_aT");

        assert_eq!(locale, LOCALE);
        assert_eq!(locale.to_string(), "de-Arab-AT");
    }

    #[test]
    fn unicode_ext_key() {
        let unicode_ext_key = unicode_ext_key!("MS");

        assert_eq!(unicode_ext_key, UNICODE_EXT_KEY);
        assert_eq!(unicode_ext_key.to_string(), "ms");
    }

    #[test]
    fn transform_ext_key() {
        let transform_ext_key = transform_ext_key!("H0");

        assert_eq!(transform_ext_key, TRANSFORM_EXT_KEY);
        assert_eq!(transform_ext_key.to_string(), "h0");
    }
}
