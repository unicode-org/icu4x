// Simple app
use icu_locale::{subtags, LanguageIdentifier};

const LANGIDS: &[&str] = &[
    "de",
    "en-US",
    "zh-Hant",
    "sr-Cyrl",
    "fr-CA",
    "es-CL",
    "pl",
    "en-Latn-US",
    "ca-valencia",
    "und-Arab",
];

fn main() {
    // 1. Parse input strings into LanguageIdentifiers
    let langids = LANGIDS
        .iter()
        .map(|s| s.parse())
        .collect::<Result<Vec<LanguageIdentifier>, _>>()
        .expect("Failed to parse Language Identifier");

    {
        // 2. Filter list of identifiers that match a given Language Identifier.
        let reference: LanguageIdentifier = "en-US"
            .parse()
            .expect("Failed to parse Language Identifier");

        let _matching = langids.iter().filter(|langid| *langid == &reference);
    }

    {
        // 3. Filter list of identifiers that match a given language subtag.
        let reference: subtags::Language = "en".parse().expect("Failed to parse Language");

        let _matching = langids.iter().filter(|langid| langid.language == reference);
    }

    // 4. Serialize Language Identifiers to a vector of strings.
    let _canonicalized: Vec<_> = langids.iter().map(|langid| langid.to_string()).collect();
}
