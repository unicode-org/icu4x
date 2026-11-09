// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashMap;

use serde_json::{Map, Value};
use tinystr::TinyAsciiStr;

type Lang = TinyAsciiStr<3>;

/// test-corpora doesn't have non-BMP text.
const MAX_SCALAR: char = '\u{FFFE}';

const SCORECARD_SIZE: usize = 1 + MAX_SCALAR as usize;

fn init_needs_counting(needs_counting: &mut [bool]) {
    let comp = icu_normalizer::properties::CanonicalCompositionBorrowed::new();
    let decomp = icu_normalizer::properties::CanonicalDecompositionBorrowed::new();
    for c in '\u{0}'..=MAX_SCALAR {
        if ('\u{AC00}'..='\u{D7AF}').contains(&c) {
            continue;
        }
        let icu_normalizer::properties::Decomposed::Expansion(a, b) = decomp.decompose(c) else {
            continue;
        };
        let Some(composed) = comp.compose(a, b) else {
            continue;
        };
        assert_eq!(composed, c);
        needs_counting[c as usize] = true;
    }
}

fn map_get<'a>(v: &'a Value, key: &str) -> &'a Value {
    let map = to_map(v);
    let Some(val) = map.get(key) else {
        eprintln!("Expected to find a value for: {}", key);
        std::process::exit(-6);
    };
    val
}

fn to_map(v: &Value) -> &Map<String, Value> {
    let Value::Object(map) = v else {
        eprintln!("Expected a JSON object");
        std::process::exit(-5);
    };
    map
}

fn to_str_f64(v: &Value) -> f64 {
    let Value::String(s) = v else {
        eprintln!("Expected a JSON string");
        std::process::exit(-7);
    };
    let Ok(f) = s.parse() else {
        eprintln!("String did not parse as f64");
        std::process::exit(-7);
    };
    f
}

fn read_lang_stats(lang_pop: &mut HashMap<Lang, f64>, lang_stat_path: std::ffi::OsString) {
    let Ok(s) = std::fs::read_to_string(&lang_stat_path) else {
        let path_utf8 = lang_stat_path.to_string_lossy();
        eprintln!("IO or UTF-8 failure: {}", path_utf8);
        std::process::exit(-3);
    };
    let Ok(stats) = serde_json::from_str::<Value>(&s) else {
        let path_utf8 = lang_stat_path.to_string_lossy();
        eprintln!("JSON failure: {}", path_utf8);
        std::process::exit(-4);
    };
    let territory_info = to_map(map_get(map_get(&stats, "supplemental"), "territoryInfo"));
    // Combine regional variats for now. (Dari, Canadian French, and European Portuguese go into the main count.)
    for (territory, obj) in territory_info {
        if territory == "ZZ" {
            continue;
        }
        let pop = to_str_f64(map_get(obj, "_population"));
        // eprintln!("Territory: {}, pop: {}", territory, pop);
        let literacy = to_str_f64(map_get(obj, "_literacyPercent")) / 100.0f64;
        // eprintln!("Territory: {}, lit: {}", territory, literacy);
        let literate_pop = pop * literacy;
        let langs = to_map(map_get(obj, "languagePopulation"));
        for (lang_string, lang_info_val) in langs {
            let lang = if lang_string == "pa_Arab" {
                // In-value-space made-up alias
                Lang::try_from_str("xx").unwrap()
            } else if lang_string == "ms_Arab" {
                // In-value-space made-up alias
                Lang::try_from_str("xy").unwrap()
            } else {
                // Ignore non-default scripts for now.
                if lang_string.contains('_') {
                    continue;
                }
                let Ok(lang) = Lang::try_from_str(lang_string) else {
                    eprintln!("Unsupported language tag: {}", lang_string);
                    std::process::exit(-8);
                };
                lang
            };
            let lang_info = to_map(lang_info_val);
            // We may have a language-level override for literacy rate.
            let mut lit_pop = if let Some(literacy_val) = lang_info.get("_literacyPercent") {
                (to_str_f64(literacy_val) / 100.0f64) * pop
            } else {
                literate_pop
            };
            lit_pop *= to_str_f64(map_get(lang_info_val, "_populationPercent")) / 100.0f64;
            // We may have a writing percent
            if let Some(writing_val) = lang_info.get("_writingPercent") {
                lit_pop *= to_str_f64(writing_val) / 100.0f64;
            };
            let tally = lang_pop.entry(lang).or_default();
            *tally += lit_pop;
        }
    }
}

fn main() {
    let norm = icu_normalizer::ComposingNormalizerBorrowed::new_nfc();

    let mut args = std::env::args_os();
    // Ignore executable name
    let _ = args.next();
    let Some(lang_stat_path) = args.next() else {
        eprintln!(
            "Must have three arguments: language statistic file, corpus directory, and output file. Got none."
        );
        std::process::exit(-1);
    };
    let Some(corpus_path) = args.next() else {
        eprintln!(
            "Must have three arguments: language statistic file, corpus directory, and output file. Got one."
        );
        std::process::exit(-1);
    };
    let Some(output_path) = args.next() else {
        eprintln!(
            "Must have three arguments: language statistic file, corpus directory, and output file. Got two."
        );
        std::process::exit(-1);
    };
    if !args.next().is_none() {
        eprintln!(
            "Must have three arguments: language statistic file, corpus directory, and output file. Got more."
        );
        std::process::exit(-1);
    }
    let Ok(lang_stat_metadata) = std::fs::metadata(&lang_stat_path) else {
        let path_utf8 = lang_stat_path.to_string_lossy();
        eprintln!("Cannot read metadata for: {}", path_utf8);
        std::process::exit(-2);
    };
    let Ok(corpus_metadata) = std::fs::metadata(&corpus_path) else {
        let path_utf8 = corpus_path.to_string_lossy();
        eprintln!("Cannot read metadata for: {}", path_utf8);
        std::process::exit(-2);
    };
    if !lang_stat_metadata.is_file() {
        let path_utf8 = lang_stat_path.to_string_lossy();
        eprintln!("Not a file but expected file: {}", path_utf8);
        std::process::exit(-3);
    }
    if !corpus_metadata.is_dir() {
        let path_utf8 = corpus_path.to_string_lossy();
        eprintln!("Not a directory but expected directory: {}", path_utf8);
        std::process::exit(-3);
    }

    let mut lang_pop: HashMap<Lang, f64> = HashMap::new();

    read_lang_stats(&mut lang_pop, lang_stat_path);

    // We count only canonical compositions and ignore other characters so that other
    // (much more common) characters don't interfere with scaling later on.
    let mut needs_counting = vec![false; SCORECARD_SIZE];
    init_needs_counting(&mut needs_counting);
    let mut scores: HashMap<Lang, Box<[u64]>> = HashMap::new();

    let Ok(dirs) = std::fs::read_dir(&corpus_path) else {
        let path_utf8 = corpus_path.to_string_lossy();
        eprintln!("Cannot read directory: {}", path_utf8);
        std::process::exit(-9);
    };

    for e in dirs {
        let Ok(entry) = e else {
            let path_utf8 = corpus_path.to_string_lossy();
            eprintln!("Cannot read directory entry from: {}", path_utf8);
            std::process::exit(-9);
        };
        let Ok(ft) = entry.file_type() else {
            let path_utf8 = corpus_path.to_string_lossy();
            eprintln!(
                "Cannot read metadata for directory entry under: {}",
                path_utf8
            );
            std::process::exit(-9);
        };
        if !ft.is_dir() {
            continue;
        }
        let dir_name = entry.file_name();
        let lang = if dir_name == "ndc-ZW" {
            Lang::try_from_str("ndc").unwrap()
        } else if dir_name == "mni-Mtei" {
            Lang::try_from_str("mni").unwrap()
        } else if dir_name == "sat-Latn" {
            // Should probably have some kind of script scaling,
            // but at least this notes non-zero counts for the Romanized version.
            Lang::try_from_str("sat").unwrap()
        } else if dir_name == "ber-Latn" {
            // Script scaling irrelevant due to missing language data!
            Lang::try_from_str("ber").unwrap()
        } else if dir_name == "pa-Arab" {
            // In-value-space made-up alias.
            Lang::try_from_str("xx").unwrap()
        } else if dir_name == "ms-Arab" {
            // In-value-space made-up alias.
            Lang::try_from_str("xy").unwrap()
        } else {
            let dir_name_bytes = dir_name.as_encoded_bytes();
            if dir_name_bytes.contains(&b'-') {
                // Assuming (withouth checking) that Dari, Canadian French, and European Portuguese
                // don't substantially differ from the Persian, French French, and Brazilian Portuguese
                // in character frequencies.
                // zh-Hant is normalization-invariant.
                // bm-Nkoo is normalization-invariant.
                // chr-Latn is ASCII
                // iu-Latn is ASCII
                continue;
            }
            let Ok(lang) = Lang::try_from_utf8(dir_name_bytes) else {
                let name = dir_name.to_string_lossy();
                eprintln!("Directory name is not a supported language tag: {}", name);
                std::process::exit(-10);
            };
            lang
        };

        let scorecard = scores
            .entry(lang)
            .or_insert_with(|| vec![0u64; SCORECARD_SIZE].into_boxed_slice());

        let lang_dir_path = entry.path();
        let Ok(files) = std::fs::read_dir(&lang_dir_path) else {
            let path_utf8 = lang_dir_path.to_string_lossy();
            eprintln!("Cannot read directory: {}", path_utf8);
            std::process::exit(-9);
        };
        for f in files {
            let Ok(file_entry) = f else {
                let path_utf8 = lang_dir_path.to_string_lossy();
                eprintln!("Cannot read directory entry from: {}", path_utf8);
                std::process::exit(-9);
            };
            let Ok(ft) = file_entry.file_type() else {
                let path_utf8 = lang_dir_path.to_string_lossy();
                eprintln!(
                    "Cannot read metadata for directory entry under: {}",
                    path_utf8
                );
                std::process::exit(-9);
            };
            if !ft.is_file() {
                continue;
            }
            let file_path = file_entry.path();
            let Ok(s) = std::fs::read_to_string(&file_path) else {
                let path_utf8 = file_path.to_string_lossy();
                eprintln!("IO or UTF-8 failure: {}", path_utf8);
                std::process::exit(-3);
            };
            // println!("Processing {}", file_path.to_string_lossy());
            let nfc = norm.normalize(&s);
            for c in nfc.chars() {
                if c > MAX_SCALAR {
                    // Not counted; the tables don't cover these.
                    continue;
                }
                let i = c as usize;
                if needs_counting[i] {
                    scorecard[i] += 1;
                }
            }
        }
    }

    let mut combined_scores = vec![0f64; SCORECARD_SIZE];

    for (lang, scorecard) in scores {
        // Give a small default value to missing populations to have non-zero
        // results.
        let pop_scale = lang_pop.get(&lang).unwrap_or(&1.0f64);
        // eprintln!("Lang: {} Pop scale: {}", lang, pop_scale);
        for (scaled_score, score) in combined_scores.iter_mut().zip(scorecard.iter()) {
            *scaled_score += (*score as f64) * pop_scale;
        }
    }

    let mut max = 0.0f64;
    for &f in &combined_scores {
        if f > max {
            max = f;
        }
    }

    if max == 0.0 {
        let path_utf8 = corpus_path.to_string_lossy();
        eprintln!("No characters to count found under: {}", path_utf8);
        std::process::exit(-11);
    }

    // Let's scale the scores to JavaScript-safe integer range
    // for easier subsequent use. Minus one here to avoid rounding
    // problem later.
    let integer_scale_factor = 9007199254740990.0f64 / max;
    if integer_scale_factor < 1.0 {
        eprintln!(
            "So much text input that score scaling factor is below 1.0. Some Berber Latin counts might round to zero."
        );
    }

    let mut out_map = Map::new();

    let mut buf = [0u8; 4];
    for (i, (needed, score)) in needs_counting
        .iter()
        .copied()
        .zip(combined_scores.iter().copied())
        .enumerate()
    {
        if !needed {
            continue;
        }
        // Floating-point rounding can take the product for the highest
        // score above the numerator of `integer_scale_factor`, so clamp.
        let rescaled = (score * integer_scale_factor).min(9007199254740990.0f64);
        assert!(rescaled < 9007199254740991.0f64);
        let c = char::from_u32(i as u32).expect("`needed` should be `false` for surrogates");
        let key = c.encode_utf8(&mut buf);
        let v = out_map.insert(key.to_string(), Value::Number((rescaled as u64).into()));
        assert!(v.is_none());
    }

    if std::fs::write(&output_path, serde_json::to_vec_pretty(&out_map).unwrap()).is_err() {
        let path_utf8 = output_path.to_string_lossy();
        eprintln!("Cannot write out put to: {}", path_utf8);
        std::process::exit(-9);
    }
}
