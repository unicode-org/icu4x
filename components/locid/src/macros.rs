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
/// use icu::locid::{subtags_language as language, subtags::Language};
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
macro_rules! subtags_language {
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
/// use icu::locid::{subtags_script as script, subtags::Script};
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
macro_rules! subtags_script {
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
/// use icu::locid::{subtags_region as region, subtags::Region};
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
macro_rules! subtags_region {
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
/// use icu::locid::{subtags::Variant, subtags_variant as variant};
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
macro_rules! subtags_variant {
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
/// *Note*: The macro cannot produce language identifiers with more than one variants due to const
/// limitations (see [`Heap Allocations in Constants`]):
///
/// ```compile_fail
/// icu::locid::langid!("und-variant1-variant2");
/// ```
///
/// Use runtime parsing instead:
/// ```
/// "und-variant1-variant2"
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
            match $crate::LanguageIdentifier::from_bytes_with_single_variant($langid.as_bytes()) {
                Ok((language, script, region, variant)) => $crate::LanguageIdentifier {
                    language,
                    script,
                    region,
                    variants: match variant {
                        Some(v) => $crate::subtags::Variants::from_variant(v),
                        None => $crate::subtags::Variants::new(),
                    }
                },
                #[allow(clippy::panic)] // const context
                _ => panic!(concat!("Invalid language code: ", $langid, " . Note langid! macro can only support up to a single variant tag. Use runtime parsing instead.")),
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
/// *Note*: The macro cannot produce locales with more than one variant or extensions due to const
/// limitations (see [`Heap Allocations in Constants`]):
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
            match $crate::LanguageIdentifier::from_bytes_with_single_variant($locale.as_bytes()) {
                Ok((language, script, region, variant)) => $crate::Locale {
                    id: $crate::LanguageIdentifier {
                        language,
                        script,
                        region,
                        variants: match variant {
                            Some(v) => $crate::subtags::Variants::from_variant(v),
                            None => $crate::subtags::Variants::new(),
                        },
                    },
                    extensions: $crate::extensions::Extensions::new(),
                },
                #[allow(clippy::panic)] // const context
                _ => panic!(concat!(
                    "Invalid language code: ",
                    $locale,
                    " . Note the locale! macro only supports up to one variant tag,\
                                        Unicode extensions are not supported, use \
                                        runtime parsing instead."
                )),
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
/// use icu::locid::{extensions_unicode_key as key, extensions_unicode_value as value};
/// use icu::locid::Locale;
///
/// const CALENDAR_KEY: Key = key!("ca");
/// const CALENDAR_VALUE: Value = value!("buddhist");
///
/// let loc: Locale = "de-u-ca-buddhist".parse().unwrap();
///
/// assert_eq!(
///     loc.extensions.unicode.keywords.get(&CALENDAR_KEY),
///     Some(&CALENDAR_VALUE)
/// );
/// ```
///
/// [`Key`]: crate::extensions::unicode::Key
#[macro_export]
macro_rules! extensions_unicode_key {
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
/// use icu::locid::{extensions_unicode_key as key, extensions_unicode_value as value};
/// use icu::locid::Locale;
///
/// const CALENDAR_KEY: Key = key!("ca");
/// const CALENDAR_VALUE: Value = value!("buddhist");
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
macro_rules! extensions_unicode_value {
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
/// use icu::locid::extensions_transform_key as key;
/// use icu::locid::Locale;
///
/// const HYBRID_KEY: Key = key!("h0");
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
macro_rules! extensions_transform_key {
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
    use crate::LanguageIdentifier;
    use crate::Locale;

    #[test]
    fn test_langid_macro_can_parse_langid_with_single_variant() {
        const DE_AT_FOOBAR: LanguageIdentifier = langid!("de_at-foobar");
        let de_at_foobar: LanguageIdentifier = "de_at-foobar".parse().unwrap();
        assert_eq!(DE_AT_FOOBAR, de_at_foobar);
    }

    #[test]
    fn test_locale_macro_can_parse_locale_with_single_variant() {
        const DE_AT_FOOBAR: Locale = locale!("de_at-foobar");
        let de_at_foobar: Locale = "de_at-foobar".parse().unwrap();
        assert_eq!(DE_AT_FOOBAR, de_at_foobar);
    }
}
