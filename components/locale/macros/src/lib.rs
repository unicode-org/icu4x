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

#[proc_macro]
pub fn language(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Language = val.parse().expect("Malformed Language Subtag");

    let lang = language_output(parsed.into_raw());

    lang.parse().unwrap()
}

#[proc_macro]
pub fn script(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Script = val.parse().expect("Malformed Script Subtag");

    let script = script_output(parsed.into_raw());

    script.parse().unwrap()
}

#[proc_macro]
pub fn region(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Region = val.parse().expect("Malformed Region Subtag");

    let region = region_output(parsed.into_raw());

    region.parse().unwrap()
}

#[proc_macro]
pub fn variant(input: TokenStream) -> TokenStream {
    let val = get_value_from_token_stream(input);

    let parsed: icu_locale::subtags::Variant = val.parse().expect("Malformed Variant Subtag");

    let variant = variant_output(parsed.into_raw());

    variant.parse().unwrap()
}

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
    let variants = format!("icu_locale::subtags::Variants::from_vec_unchecked(vec![])");

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
