// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! [`icu_locid_macros`](crate) is one of the ICU4X components.
//!
//! This API provides convenience macros for [`icu_locid`].
//!
//! # Examples
//!
//! ```rust
//! use icu_locid_macros::{language, region, langid};
//!
//! let lid = langid!("EN_US");
//!
//! assert_eq!(lid.language, language!("en"));
//! assert_eq!(lid.region, Some(region!("US")));
//! ```

mod token_stream;

extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro_crate::{crate_name, FoundCrate};
use token_stream::IntoTokenStream;

fn get_crate_name() -> String {
    match crate_name("icu") {
        Ok(FoundCrate::Itself) => "icu::locid".to_string(),
        Ok(FoundCrate::Name(name)) => format!("{}::locid", name),
        Err(_) => match crate_name("icu_locid") {
            Ok(FoundCrate::Name(name)) => name,
            _ => "icu_locid".to_string(),
        },
    }
}

fn get_value_from_token_stream(input: TokenStream) -> String {
    let val = format!("{}", input);
    if !val.starts_with('"') || !val.ends_with('"') {
        panic!("Argument must be a string literal.");
    }
    let len = val.len();
    (&val[1..len - 1]).to_string()
}

/// A macro allowing for compile-time construction of valid [`Language`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Language;
/// use icu::locid::macros::language;
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
/// [`Language`]: icu_locid::subtags::Language
#[proc_macro]
pub fn language(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locid::subtags::Language = val.parse().expect("Malformed Language Subtag");

    parsed
        .into_token_stream()
        .expect("Failed to parse token stream.")
}

/// A macro allowing for compile-time construction of valid [`Script`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Script;
/// use icu::locid::macros::script;
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
/// [`Script`]: icu_locid::subtags::Script
#[proc_macro]
pub fn script(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locid::subtags::Script = val.parse().expect("Malformed Script Subtag");

    parsed
        .into_token_stream()
        .expect("Failed to parse token stream.")
}

/// A macro allowing for compile-time construction of valid [`Region`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Region;
/// use icu::locid::macros::region;
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
/// [`Region`]: icu_locid::subtags::Region
#[proc_macro]
pub fn region(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locid::subtags::Region = val.parse().expect("Malformed Region Subtag");

    parsed
        .into_token_stream()
        .expect("Failed to parse token stream.")
}

/// A macro allowing for compile-time construction of valid [`Variant`] subtag.
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::subtags::Variant;
/// use icu::locid::macros::variant;
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
/// [`Variant`]: icu_locid::subtags::Variant
#[proc_macro]
pub fn variant(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locid::subtags::Variant = val.parse().expect("Malformed Variant Subtag");

    parsed
        .into_token_stream()
        .expect("Failed to parse token stream.")
}

/// A macro allowing for compile-time construction of valid [`LanguageIdentifier`].
///
/// The macro will perform syntax canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu::locid::LanguageIdentifier;
/// use icu::locid::macros::langid;
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
/// [`LanguageIdentifier`]: icu_locid::LanguageIdentifier
/// [`Heap Allocations in Constants`]: https://github.com/rust-lang/const-eval/issues/20
#[proc_macro]
pub fn langid(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let langid: icu_locid::LanguageIdentifier = val.parse().expect("Malformed Language Identifier");

    let lang = langid.language.into_token_stream_string();

    let script = langid.script.into_token_stream_string();
    let region = langid.region.into_token_stream_string();

    let variants = langid.variants.into_token_stream_string();

    let output = format!(
        r#"
{}::LanguageIdentifier {{
    language: {},
    script: {},
    region: {},
    variants: {},
}}
"#,
        get_crate_name(),
        lang,
        script,
        region,
        variants
    );

    output.parse().expect("Output should parse.")
}
