// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// A sample application which takes a comma separated list of language identifiers,
// filters out identifiers with language subtags different than `en` and serializes
// the list back into a comma separated list in canonical syntax.
//
// Note: This is an example of the API use, and is not a good base for language matching.
// For language matching, please consider algorithms such as Locale Matcher.
use std::env;

use icu_locale::{subtags, LanguageIdentifier};

const DEFAULT_INPUT: &str =
    "de, en-us, zh-hant, sr-cyrl, fr-ca, es-cl, pl, en-latn-us, ca-valencia, und-arab";

fn filter_input(input: &str) -> String {
    // 1. Parse the input string into a list of language identifiers.
    let langids: Vec<LanguageIdentifier> = input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // 2. Filter for LanguageIdentifiers with Language subtag `en`.
    let en_lang: subtags::Language = "en".parse().expect("Failed to parse language subtag.");

    let en_langids = langids
        .into_iter()
        .filter(|langid| langid.language == en_lang);

    // 3. Serialize the output.
    let en_strs: Vec<String> = en_langids.map(|langid| langid.to_string()).collect();

    en_strs.join(", ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if let Some(input) = args.get(1) {
        input.as_str()
    } else {
        DEFAULT_INPUT
    };
    let _output = filter_input(&input);

    #[cfg(debug_assertions)]
    println!("\nInput: {}\nOutput: {}", input, _output);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_OUTPUT: &str = "en-US, en-Latn-US";

    #[test]
    fn ensure_default_output() {
        assert_eq!(filter_input(DEFAULT_INPUT), DEFAULT_OUTPUT);
    }
}
