extern crate proc_macro;

use proc_macro::TokenStream;

fn get_value_from_token_stream(input: TokenStream) -> String {
    let val = format!("{}", input);
    if !val.starts_with('"') || !val.ends_with('"') {
        panic!("Argument must be a string literal.");
    }
    let len = val.len();
    (&val[1..len - 1]).to_string()
}

fn language_output(input: Option<u64>) -> String {
    let val = if let Some(raw) = input {
        format!("Some({})", raw)
    } else {
        "None".to_string()
    };
    format!("unsafe {{ icu_locale::subtags::Language::from_raw_unchecked({}) }}", val)
}

fn script_output(input: u32) -> String {
    format!("unsafe {{ icu_locale::subtags::Script::from_raw_unchecked({}) }}", input)
}

fn region_output(input: u32) -> String {
    format!("unsafe {{ icu_locale::subtags::Region::from_raw_unchecked({}) }}", input)
}

fn variant_output(input: u64) -> String {
    format!("unsafe {{ icu_locale::subtags::Variant::from_raw_unchecked({}) }}", input)
}

fn variants_output(input: Option<Box<[icu_locale::subtags::Variant]>>) -> String {
    if let Some(variants) = input {
        let v: Vec<_> = variants
            .iter()
            .map(|v| {
                let variant: u64 = v.into_raw();
                format!("unsafe {{ icu_locale::subtags::Variant::from_raw_unchecked({}) }}", variant)
            })
            .collect();
        let variants = format!("Some(Box::new([{}]))", v.join(", "));
        format!("icu_locale::subtags::Variants::from_raw_unchecked({})", variants)
    } else {
        format!("icu_locale::subtags::Variants::from_raw_unchecked(None)")
    }
}

/// A macro allowing for compile-time construction of valid [`Language`] subtag.
///
/// The macro will perform canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Language;
/// use icu_locale_macros::language;
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
/// [`Language`]: ../icu_locale/subtags/struct.Language.html
#[proc_macro]
pub fn language(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Language = val.parse().expect("Malformed Language Subtag");

    let lang = language_output(parsed.into_raw());

    lang.parse().unwrap()
}

/// A macro allowing for compile-time construction of valid [`Script`] subtag.
///
/// The macro will perform canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Script;
/// use icu_locale_macros::script;
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
/// [`Script`]: ../icu_locale/subtags/struct.Script.html
#[proc_macro]
pub fn script(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Script = val.parse().expect("Malformed Script Subtag");

    let script = script_output(parsed.into_raw());

    script.parse().unwrap()
}

/// A macro allowing for compile-time construction of valid [`Region`] subtag.
///
/// The macro will perform canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Region;
/// use icu_locale_macros::region;
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
/// [`Region`]: ../icu_locale/subtags/struct.Region.html
#[proc_macro]
pub fn region(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Region = val.parse().expect("Malformed Region Subtag");

    let region = region_output(parsed.into_raw());

    region.parse().unwrap()
}

/// A macro allowing for compile-time construction of valid [`Variant`] subtag.
///
/// The macro will perform canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu_locale::subtags::Variant;
/// use icu_locale_macros::variant;
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
/// [`Variant`]: ../icu_locale/subtags/struct.Variant.html
#[proc_macro]
pub fn variant(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Variant = val.parse().expect("Malformed Variant Subtag");

    let variant = variant_output(parsed.into_raw());

    variant.parse().unwrap()
}


/// A macro allowing for compile-time construction of valid [`LanguageIdentifier`].
///
/// The macro will perform canonicalization of the tag.
///
/// # Examples
///
/// ```
/// use icu_locale::LanguageIdentifier;
/// use icu_locale_macros::langid;
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
/// *Note*: As of Rust 1.47, the the macro cannot produce language identifier
/// with variants in the const mode pending [`Heap Allocations in Constants`].
///
/// [`LanguageIdentifier`]: ../icu_locale/struct.LanguageIdentifier.html
/// [`Heap Allocations in Constants`]: https://github.com/rust-lang/const-eval/issues/20
#[proc_macro]
pub fn langid(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let langid: icu_locale::LanguageIdentifier = val.parse().expect("Malformed Language Identifier");

    let lang = language_output(langid.language.into_raw());

    let script = if let Some(script) = langid.script {
        format!("Some({})", script_output(script.into_raw()))
    } else {
        "None".to_string()
    };
    let region = if let Some(region) = langid.region {
        format!("Some({})", region_output(region.into_raw()))
    } else {
        "None".to_string()
    };

    let variants = variants_output(langid.variants.into_raw());

    let output = format!(r#"
icu_locale::LanguageIdentifier {{
    language: {},
    script: {},
    region: {},
    variants: {},
}}
"#, lang, script, region, variants);

    output.parse().unwrap()
}
