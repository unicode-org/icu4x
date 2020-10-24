// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
// A sample application which takes a comma separated list of locales,
// makes them canonical and serializes the list back into a comma separated list.

use std::env;

use icu_locid::{Locale};

const DEFAULT_INPUT: &str = "sr-cyrL-rS, es-mx, und-arab-u-ca-Buddhist";

fn canonicalize_locales(input: &str) -> String {
    // 1. Parse the input string into a list of locales.
    let locales: Vec<Locale> = input
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    // 2. Canonicalize and serialize the output.
    let canonical_locales: Vec<String> = locales
        .into_iter()
        .filter_map(|locale| icu_locid::Locale::canonicalize(locale.to_string()).ok())
        .collect();

    canonical_locales.join(", ")
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = if let Some(input) = args.get(1) {
        input.as_str()
    } else {
        DEFAULT_INPUT
    };
    let _output = canonicalize_locales(&input);

    #[cfg(debug_assertions)]
    println!("\nInput: {}\nOutput: {}", input, _output);
}

#[cfg(test)]
mod tests {
    use super::*;

    const DEFAULT_OUTPUT: &str = "sr-Cyrl-RS, es-MX, und-Arab-u-ca-buddhist";

    #[test]
    fn ensure_default_output() {
        assert_eq!(canonicalize_locales(DEFAULT_INPUT), DEFAULT_OUTPUT);
    }
}
