// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains provider implementations backed by built-in segmentation data.

use crate::DataHasher;
use crate::IterableDataProviderCached;
use crate::SourceDataProvider;
use crate::source::Cache;
use icu::collections::codepointinvlist::CodePointInversionList;
use icu::locale::extensions::unicode::key;
use icu::segmenter::options::WordType;
use icu::segmenter::provider::*;
use icu_provider::prelude::*;
use std::collections::HashSet;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

mod dictionary;
mod lstm;
mod unihan;

#[test]
#[ignore]
#[cfg(all(feature = "unstable", feature = "networking"))]
#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
fn download() {
    use std::fs::File;
    use std::io::Write;

    let data_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/segmenter/pri555");

    for file in std::fs::read_dir(&data_root).unwrap() {
        let file = file.unwrap();
        if !file.file_type().unwrap().is_file() {
            continue;
        }
        crlify::BufWriterWithLineEndingFix::new(File::create(file.path()).unwrap())
            .write_all(
                &crate::source::AbstractFs::new_from_url(format!(
                    "https://unicode.org/review/pri555/{}",
                    SourceDataProvider::TESTED_UNICODE_TAG
                ))
                .read_to_buf(file.file_name().to_str().unwrap())
                .unwrap(),
            )
            .unwrap();
    }
}

type TailoredSegmenter = (
    SegmenterStateMachine<'static>,
    BTreeMap<DataIdentifierCow<'static>, SegmenterStateMachineOverride<'static>>,
    u64,
);

#[derive(Debug, Default)]
pub(crate) struct NeoSegmenters {
    line: Cache<TailoredSegmenter>,
    word: Cache<TailoredSegmenter>,
    sentence: Cache<TailoredSegmenter>,
    grapheme_cluster: Cache<TailoredSegmenter>,
}

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
impl SourceDataProvider {
    fn line_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.rscd()?
            .segmenter_cache
            .line
            .get_or_init(|| {
                self.build_segmenter("LineBreak", |s| if s == "Mandatory" { 1 } else { 0 })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn word_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.rscd()?
            .segmenter_cache
            .word
            .get_or_init(|| {
                self.build_segmenter("WordBreak", |s| match s {
                    "Letter" => WordType::Letter,
                    "Number" => WordType::Number,
                    _ => WordType::None,
                } as u8)
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn sentence_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.rscd()?
            .segmenter_cache
            .sentence
            .get_or_init(|| {
                self.build_segmenter(
                    "SentenceBreak",
                    |s| {
                        if s == "Nonterminated" { 1 } else { 0 }
                    },
                )
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn grapheme_cluster_segmenter(&self) -> Result<&TailoredSegmenter, DataError> {
        self.rscd()?
            .segmenter_cache
            .grapheme_cluster
            .get_or_init(|| {
                self.build_segmenter("GraphemeClusterBreak", |s| match s {
                    "" => 0,
                    s => unreachable!("{s}"),
                })
            })
            .as_ref()
            .map_err(|&e| e)
    }

    fn build_segmenter(
        &self,
        prefix: &str,
        status_lookup: fn(&str) -> u8,
    ) -> Result<TailoredSegmenter, DataError> {
        let rscd = self.rscd()?;

        let mut magic_symbols = BTreeMap::new();
        let mut complex_symbols = BTreeMap::new();
        let symbols = rscd.read_to_string(&format!("ucd/auxiliary/{prefix}Symbols.txt"))?;
        let symbols = symbols
            .lines()
            .map(|l| l.split('#').next().unwrap().trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut iter = line.split(';').map(str::trim);
                let symbol = iter.next().unwrap();
                let unicode_set = iter.next().unwrap();

                if let Some(non_complex_equivalent) = iter.next()
                    && !non_complex_equivalent.is_empty()
                {
                    complex_symbols.insert(symbol, non_complex_equivalent);
                }

                let set = icu::properties::unicodeset_parse::parse_unstable(unicode_set, self)
                    .map_err(|e| {
                        DataError::custom("unicodeset parse")
                            .with_display_context(&e.fmt_with_source(unicode_set))
                    })?
                    .0;
                for string in set.strings().iter() {
                    assert_eq!(magic_symbols.insert(String::from(string), symbol), None);
                }
                let set = set.code_points().clone();
                Ok((symbol.to_owned(), set))
            })
            .collect::<Result<BTreeMap<_, _>, DataError>>()?;
        let eot_symbol = magic_symbols.remove("eot").unwrap_or("eot").to_string();
        let magic_symbols = magic_symbols;
        let complex_symbols = complex_symbols;

        let states = rscd.read_to_string(&format!("ucd/auxiliary/{prefix}States.txt"))?;
        let states = states
            .lines()
            .map(|l| l.split('#').next().unwrap().trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut iter = line.split(';');
                let state = iter.next().unwrap().trim();
                let accepting = iter.next().unwrap().trim();
                let lookahead = iter.next().unwrap().trim();
                let status = iter.next().unwrap().trim();
                (
                    state,
                    (accepting, Some(lookahead).filter(|s| !s.is_empty()), status),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let transitions = rscd.read_to_string(&format!("ucd/auxiliary/{prefix}Transitions.txt"))?;
        let transitions = transitions
            .lines()
            .map(|l| l.split('#').next().unwrap().trim())
            .filter(|l| !l.is_empty())
            .map(|line| {
                let mut iter = line.split(';');
                let state = iter.next().unwrap().trim();
                let symbol = iter.next().unwrap().trim();
                let next_state = iter.next().unwrap().trim();
                ((state, symbol), next_state)
            })
            .collect::<BTreeMap<_, _>>();

        let lookaheads = states
            .iter()
            .flat_map(|(_, &(_, lookahead, _))| lookahead)
            .collect::<BTreeSet<_>>();

        let complex_languages = match prefix {
            "LineBreak" => [
                (ComplexScript::Myanmar, "[:sc=Myanmar:]&[:lb=SA:]"),
                (ComplexScript::Khmer, "[:sc=Khmer:]&[:lb=SA:]"),
                (ComplexScript::Lao, "[:sc=Lao:]&[:lb=SA:]"),
                (ComplexScript::Thai, "[:sc=Thai:]&[:lb=SA:]"),
            ]
            .as_slice(),
            "WordBreak" => [
                (ComplexScript::Myanmar, "[:sc=Myanmar:]&[:lb=SA:]"),
                (
                    ComplexScript::ChineseOrJapanese,
                    "[[[:sc=Han:] [:sc=Hiragana:] [:wb=Katakana:] 가-힣] - [:lb=SA:]]",
                ),
                (ComplexScript::Khmer, "[:sc=Khmer:]&[:lb=SA:]"),
                (ComplexScript::Lao, "[:sc=Lao:]&[:lb=SA:]"),
                (ComplexScript::Thai, "[:sc=Thai:]&[:lb=SA:]"),
            ]
            .as_slice(),
            _ => &[],
        }
        .iter()
        .map(|&(l, set)| {
            (
                l,
                icu::properties::unicodeset_parse::parse_unstable(set, self)
                    .unwrap()
                    .0
                    .code_points()
                    .clone(),
            )
        })
        .collect::<Vec<_>>();

        let mut tailorings = BTreeMap::new();

        let cldr = self.cldr()?.segments();

        for locale in cldr.list_locales()? {
            if !cldr.file_exists(&locale, "tailorings.json")? {
                continue;
            }
            let Some(ts) = cldr
                .read_and_parse::<crate::cldr_serde::segmentation::Resource>(
                    &locale,
                    "tailorings.json",
                )?
                .segments
                .segmentations
                .0
                .get(prefix)
            else {
                continue;
            };

            for (keywords, lines) in ts.iter().map(|(k, v)| (&k.extensions.unicode.keywords, v)) {
                let mut overrides = BTreeMap::<String, BTreeSet<char>>::new();

                for line in lines {
                    let mut iter = line.split(';');
                    let unicode_set = iter.next().unwrap().trim();
                    let target = iter.next().unwrap().trim();

                    let set = icu::properties::unicodeset_parse::parse_unstable(unicode_set, self)
                        .map_err(|e| {
                            DataError::custom("unicodeset parse")
                                .with_display_context(&e.fmt_with_source(unicode_set))
                        })?
                        .0;

                    let target = icu::properties::unicodeset_parse::parse_unstable(target, self)
                        .map_err(|e| {
                            DataError::custom("unicodeset parse")
                                .with_display_context(&e.fmt_with_source(unicode_set))
                        })?
                        .0;

                    let target_symbol = if target.has_strings() {
                        magic_symbols[target.strings().iter().next().unwrap()]
                    } else {
                        let target = target.code_points().iter_chars().next().unwrap();
                        symbols
                            .iter()
                            .find(|(_, set)| set.contains(target))
                            .unwrap()
                            .0
                            .as_str()
                    };

                    for c in set.code_points().iter_chars() {
                        overrides
                            .entry(target_symbol.to_owned())
                            .or_default()
                            .insert(c);
                    }
                }

                let id = if prefix == "LineBreak" {
                    let x;
                    DataIdentifierCow::from_marker_attributes_owned(
                        DataMarkerAttributes::try_from_string(format!(
                            "{}{}{}",
                            if locale.is_unknown() {
                                ""
                            } else {
                                x = locale.to_string();
                                &x
                            },
                            if locale.is_unknown() || keywords.is_empty() {
                                ""
                            } else {
                                "-"
                            },
                            keywords
                                .get(&key!("lb"))
                                .or_else(|| keywords.get(&key!("lw")))
                                .map(|v| v.to_string())
                                .unwrap_or_default()
                        ))
                        .unwrap(),
                    )
                } else {
                    DataIdentifierCow::from_owned(
                        DataMarkerAttributes::try_from_string(
                            keywords
                                .get(&key!("lb"))
                                .or_else(|| keywords.get(&key!("lw")))
                                .map(|v| v.to_string())
                                .unwrap_or_default(),
                        )
                        .unwrap(),
                        locale,
                    )
                };

                tailorings.insert(
                    id,
                    overrides
                        .into_iter()
                        .map(|(k, v)| {
                            let mut builder = CodePointInversionListBuilder::new();
                            v.into_iter().for_each(|c| builder.add_char(c));
                            (k, builder.build())
                        })
                        .collect::<BTreeMap<_, CodePointInversionList>>(),
                );
            }
        }

        // We now mutate the state machine.

        let mut symbols = symbols;
        let mut transitions = transitions;
        let mut pseudo_symbol_map = BTreeMap::<String, (String, ComplexScript)>::new();

        // Create pseudo symbols for complex scripts, allowing the state machine to use the correct
        // dictionary without further lookup.
        for (&symbol, &non_complex_symbol) in &complex_symbols {
            let set = symbols.get(symbol).unwrap().clone();

            let mut set_builder = CodePointInversionListBuilder::new();
            set_builder.add_set(&set);

            for &(language, ref language_set) in &complex_languages {
                if language_set
                    .iter_ranges()
                    .all(|mut range| range.all(|c| !set.contains32(c)))
                {
                    // no overlap
                    continue;
                }

                set_builder.remove_set(language_set);

                let mut intersection = CodePointInversionListBuilder::new();
                intersection.add_set(language_set);
                for r in set.iter_ranges_complemented() {
                    intersection.remove_range32(r);
                }

                let intersection_symbol = format!("{symbol}_{language:?}");

                pseudo_symbol_map.insert(
                    intersection_symbol.clone(),
                    (non_complex_symbol.into(), language),
                );
                symbols.insert(intersection_symbol, intersection.build());
            }

            if symbol != non_complex_symbol {
                let symbol_transitions = transitions
                    .iter()
                    .filter(|&(&(_, s), _)| s == symbol)
                    .map(|(&(before, _), &after)| (before, after))
                    .collect::<BTreeSet<_>>();
                let non_complex_symbol_transitions = transitions
                    .iter()
                    .filter(|&(&(_, s), _)| s == non_complex_symbol)
                    .map(|(&(before, _), &after)| (before, after))
                    .collect::<BTreeSet<_>>();

                if symbol_transitions == non_complex_symbol_transitions {
                    let non_complex_set = symbols.get_mut(non_complex_symbol).unwrap();
                    let mut non_complex_set_builder = CodePointInversionListBuilder::new();
                    non_complex_set_builder.add_set(non_complex_set);
                    non_complex_set_builder.add_set(&set_builder.build());
                    *non_complex_set = non_complex_set_builder.build();

                    symbols.remove(symbol);
                    transitions.retain(|&(_, s), _| s != symbol);
                } else {
                    log::warn!(
                        "{symbol}/{non_complex_symbol}: {:?} != {:?}",
                        symbol_transitions
                            .difference(&non_complex_symbol_transitions)
                            .collect::<Vec<_>>(),
                        non_complex_symbol_transitions
                            .difference(&symbol_transitions)
                            .collect::<Vec<_>>()
                    );
                }
            }
        }

        // Create pseudo symbols for all tailorings sets.
        for (tailoring, overrides) in tailorings.clone() {
            for (rule, set) in overrides {
                for (symbol, set2) in symbols.clone().into_iter().collect::<Vec<_>>() {
                    if set.iter_chars().any(|c| set2.contains(c)) {
                        // Overlapping sets. We need to create a new pseudo-symbol.
                        let pseudo_symbol = format!("{symbol}_{tailoring}_{rule}");
                        // Add the intersection as a new symbol.
                        symbols.insert(pseudo_symbol.clone(), {
                            let mut builder = CodePointInversionListBuilder::new();
                            builder.add_set(&set);
                            for r in set2.iter_ranges_complemented() {
                                builder.remove_range32(r);
                            }
                            builder.build()
                        });
                        pseudo_symbol_map.insert(pseudo_symbol, {
                            let mut s = &*symbol;
                            // Non-pseudo symbols have Language::Other
                            let mut l = ComplexScript::None;
                            while let Some(&(ref x, y)) = pseudo_symbol_map.get(s) {
                                s = x.as_str();
                                l = y;
                            }
                            (s.to_string(), l)
                        });
                        // Remove the intersection from the root symbol.
                        symbols.insert(symbol, {
                            let mut builder = CodePointInversionListBuilder::new();
                            builder.add_set(&set2);
                            builder.remove_set(&set);
                            builder.build()
                        });
                    }
                }
            }
        }

        let mut unused_pseudo_symbols = pseudo_symbol_map.keys().cloned().collect::<BTreeSet<_>>();
        let tailorings = tailorings
            .into_iter()
            .map(|(tailoring, overrides)| {
                let mut tailored_pseudo_symbol_map = BTreeMap::new();

                for (target_symbol, set) in overrides {
                    // TODO?
                    let target_language = ComplexScript::None;
                    // The set might cover multiple pseudo symbols
                    for c in set.iter_chars() {
                        let pseudo_symbol =
                            symbols.iter().find(|(_, set)| set.contains(c)).unwrap().0;
                        unused_pseudo_symbols.remove(pseudo_symbol);
                        tailored_pseudo_symbol_map.insert(
                            pseudo_symbol.to_owned(),
                            (target_symbol.clone(), target_language),
                        );
                    }
                }

                (tailoring, tailored_pseudo_symbol_map)
            })
            .collect::<BTreeMap<_, _>>();

        // Remove unused pseudo symbols. It's hard to not generate unused pseudo symbols, because when we split
        // a previously created pseudo symbol, we don't know which half the other tailoring actually needs.
        for unused in unused_pseudo_symbols {
            if pseudo_symbol_map.get(&unused).unwrap().1 != ComplexScript::None {
                continue;
            }
            let resolved = pseudo_symbol_map.remove(&unused).unwrap().0;
            let set = symbols.remove(&unused).unwrap();
            let resolved_set = symbols.get_mut(&resolved).unwrap();

            let mut builder = CodePointInversionListBuilder::new();
            builder.add_set(resolved_set);
            builder.add_set(&set);
            *resolved_set = builder.build();
        }

        // Remove unused symbols
        symbols.retain(|n, set| {
            if pseudo_symbol_map.contains_key(n) {
                // Symbol is a pseudo symbol
                return true;
            }

            if !set.is_empty() {
                // Symbol used in root
                return true;
            }

            if pseudo_symbol_map
                .values()
                .any(|(root_symbol, _)| root_symbol == n)
                || tailorings.values().any(|tailored_pseudo_symbol_map| {
                    tailored_pseudo_symbol_map
                        .values()
                        .any(|(target_symbol, _)| target_symbol == n)
                })
            {
                // Symbol is a pseudo symbol target
                return true;
            }

            transitions.retain(|&(_, m), _| m != n);

            false
        });

        let symbols = symbols;
        let pseudo_symbol_map = pseudo_symbol_map;

        // Done. The rest of this function encodes the state machine.

        let hash = {
            use core::hash::{Hash, Hasher};

            let mut hash = DataHasher::new();
            symbols.hash(&mut hash);
            pseudo_symbol_map.hash(&mut hash);
            states.hash(&mut hash);
            transitions.hash(&mut hash);
            hash.finish()
        };

        let symbol_lookup = symbols
            .keys()
            .filter(|&s| s != &eot_symbol && !pseudo_symbol_map.contains_key(s))
            .enumerate()
            .map(|(i, symbol)| (symbol.as_str(), Symbol::try_from(i + 1).unwrap()))
            .chain([(eot_symbol.as_str(), SegmenterStateMachine::EOT_SYMBOL)])
            .collect::<BTreeMap<_, _>>();

        let pseudo_symbol_shift = symbol_lookup.values().copied().max().unwrap() + 1;
        let pseudo_symbol_lookup = pseudo_symbol_map
            .keys()
            .enumerate()
            .map(|(i, k)| {
                (
                    k.as_str(),
                    Symbol::try_from(i + usize::from(pseudo_symbol_shift)).unwrap(),
                )
            })
            .collect::<BTreeMap<_, _>>();

        // Reserve two states for START and TRASH
        assert!(states.len() < usize::from(State::MAX) - 2);
        let state_lookup = core::iter::once("START")
            .chain(states.keys().filter(|&&s| s != "START").copied())
            .enumerate()
            .map(|(i, state)| (state, State::try_from(i).unwrap()))
            .collect::<BTreeMap<_, _>>();
        assert!(lookaheads.len() < 0b11111);
        let lookahead_lookup = lookaheads
            .iter()
            .enumerate()
            .map(|(i, lookahead)| (*lookahead, Lookahead::try_from(i).unwrap()))
            .collect::<BTreeMap<_, _>>();

        use icu::collections::codepointinvlist::CodePointInversionListBuilder;
        use icu::collections::codepointtrie::TrieType;
        use icu_codepointtrie_builder::CodePointTrieBuilder;

        let mut builder = CodePointTrieBuilder::new(0, 0, TrieType::Fast);
        let mut missing_codepoints = CodePointInversionListBuilder::new();
        missing_codepoints.add_set(&CodePointInversionList::all());
        for (symbol, set) in &symbols {
            for range in set.iter_ranges() {
                missing_codepoints.remove_range32(range.clone());
                builder.set_range_value(
                    range.clone(),
                    symbol_lookup
                        .get(symbol.as_str())
                        .or_else(|| pseudo_symbol_lookup.get(symbol.as_str()))
                        .copied()
                        .unwrap(),
                );
            }
        }
        let missing_codepoints = missing_codepoints.build();
        assert!(missing_codepoints.is_empty(), "{missing_codepoints:?}");
        let symbols = builder.build();

        let states = states
            .iter()
            .map(|(&state, &(accepting, lookahead, status))| {
                let status = status_lookup(status);
                // This bound comes from Acceptance::to_unaligned
                assert!(status < 0b111);

                let acceptance = match accepting {
                    "Yes" => Acceptance::Accept(status),
                    "No" => Acceptance::Continue,
                    l => Acceptance::Conditional(lookahead_lookup[l], status),
                };

                (
                    state_lookup[state],
                    (acceptance, lookahead.as_ref().map(|l| lookahead_lookup[l])),
                )
            })
            .collect::<BTreeMap<_, _>>()
            .into_values()
            .collect();

        let transitions = transitions
            .iter()
            .map(|(&(state, symbol), &next_state)| {
                (
                    usize::from(state_lookup[state])
                        + state_lookup.len() * usize::from(symbol_lookup[symbol]),
                    *state_lookup.get(next_state).expect(next_state),
                )
            })
            .collect::<BTreeMap<_, _>>();

        let transitions = (0..=*transitions.last_key_value().unwrap().0)
            .map(|i| {
                transitions
                    .get(&i)
                    .copied()
                    .unwrap_or(SegmenterStateMachine::TRASH_STATE)
            })
            .collect();

        let build_pseudo_map = |map: &BTreeMap<String, (String, ComplexScript)>| {
            map.iter()
                .map(|(pseudo_symbol, &(ref symbol, complex_script))| {
                    (
                        pseudo_symbol_lookup[pseudo_symbol.as_str()],
                        (symbol_lookup[symbol.as_str()], complex_script),
                    )
                })
                .collect::<BTreeMap<_, _>>()
                .into_values()
                .collect()
        };

        let tailorings = tailorings
            .into_iter()
            .map(|(tailoring, tailored_pseudo_symbol_map)| {
                (
                    tailoring,
                    SegmenterStateMachineOverride {
                        pseudo_symbol_map: build_pseudo_map(
                            &pseudo_symbol_map
                                .clone()
                                .into_iter()
                                .chain(tailored_pseudo_symbol_map)
                                .collect(),
                        ),
                    },
                )
            })
            .collect();

        Ok((
            SegmenterStateMachine {
                transitions,
                symbols,
                states,
                num_lookaheads: lookahead_lookup.len(),
                pseudo_symbol_shift,
                pseudo_symbol_map: build_pseudo_map(&pseudo_symbol_map),
            },
            tailorings,
            hash,
        ))
    }
}

impl DataProvider<SegmenterBreakLineV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBreakLineV2>, DataError> {
        self.check_req::<SegmenterBreakLineV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakLineV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(self.line_segmenter()?.2),
            payload: DataPayload::from_owned(self.line_segmenter()?.0.clone()),
        })
    }
}

impl DataProvider<SegmenterBreakWordV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBreakWordV2>, DataError> {
        self.check_req::<SegmenterBreakWordV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakWordV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(self.word_segmenter()?.2),
            payload: DataPayload::from_owned(self.word_segmenter()?.0.clone()),
        })
    }
}

impl DataProvider<SegmenterBreakSentenceV2> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<SegmenterBreakSentenceV2>, DataError> {
        self.check_req::<SegmenterBreakSentenceV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakSentenceV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(self.sentence_segmenter()?.2),
            payload: DataPayload::from_owned(self.sentence_segmenter()?.0.clone()),
        })
    }
}

impl DataProvider<SegmenterBreakGraphemeClusterV2> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterBreakGraphemeClusterV2>, DataError> {
        self.check_req::<SegmenterBreakGraphemeClusterV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakGraphemeClusterV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: DataResponseMetadata::default()
                .with_checksum(self.grapheme_cluster_segmenter()?.2),
            payload: DataPayload::from_owned(self.grapheme_cluster_segmenter()?.0.clone()),
        })
    }
}

impl IterableDataProviderCached<SegmenterBreakLineV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

impl IterableDataProviderCached<SegmenterBreakSentenceV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

impl IterableDataProviderCached<SegmenterBreakWordV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

impl IterableDataProviderCached<SegmenterBreakGraphemeClusterV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok([Default::default()].into_iter().collect())
    }
}

impl DataProvider<SegmenterBreakLineOverrideV2> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterBreakLineOverrideV2>, DataError> {
        self.check_req::<SegmenterBreakLineOverrideV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakLineOverrideV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(self.line_segmenter()?.2),
            payload: DataPayload::from_owned(
                self.line_segmenter()?
                    .1
                    .get(&req.id.as_cow())
                    .ok_or_else(|| {
                        DataErrorKind::IdentifierNotFound
                            .with_req(SegmenterBreakLineOverrideV2::INFO, req)
                    })?
                    .clone(),
            ),
        })
    }
}

impl IterableDataProviderCached<SegmenterBreakLineOverrideV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_marker(SegmenterBreakLineOverrideV2::INFO));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(self.line_segmenter()?.1.keys().cloned().collect())
    }
}

impl DataProvider<SegmenterBreakSentenceOverrideV2> for SourceDataProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<SegmenterBreakSentenceOverrideV2>, DataError> {
        self.check_req::<SegmenterBreakSentenceOverrideV2>(req)?;

        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_req(SegmenterBreakSentenceOverrideV2::INFO, req));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(DataResponse {
            metadata: DataResponseMetadata::default().with_checksum(self.sentence_segmenter()?.2),
            payload: DataPayload::from_owned(
                self.sentence_segmenter()?
                    .1
                    .get(&req.id.as_cow())
                    .ok_or_else(|| {
                        DataErrorKind::IdentifierNotFound
                            .with_req(SegmenterBreakSentenceOverrideV2::INFO, req)
                    })?
                    .clone(),
            ),
        })
    }
}

impl IterableDataProviderCached<SegmenterBreakSentenceOverrideV2> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        #[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
        return Err(DataError::custom(
            "icu_provider_source must be built with use_icu4c or use_wasm to build segmentation rules",
        )
        .with_marker(SegmenterBreakSentenceOverrideV2::INFO));

        #[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
        Ok(self.sentence_segmenter()?.1.keys().cloned().collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_grapheme_cluster_data() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<SegmenterBreakGraphemeClusterV1> = provider
            .load(Default::default())
            .expect("Loading should succeed!");
        assert_eq!(
            response.payload.get().complex_property,
            127,
            "Grapheme cluster data doesn't handle SA"
        );
    }

    #[test]
    fn load_line_data_v1() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<SegmenterBreakLineV1> = provider
            .load(Default::default())
            .expect("Loading should succeed!");
        let data = response.payload.get();
        // Note: The following match statement had been used in line.rs:
        //
        // match codepoint {
        //     0x20000..=0x2fffd => ID,
        //     0x30000..=0x3fffd => ID,
        //     0xe0001 => CM,
        //     0xe0020..=0xe007f => CM,
        //     0xe0100..=0xe01ef => CM,
        //     _ => XX,
        // }

        const CM: u8 = 14;
        const XX: u8 = 52;
        const ID: u8 = 25;

        assert_eq!(data.property_table.get32(0x20000), ID);
        assert_eq!(data.property_table.get32(0x3fffd), ID);
        assert_eq!(data.property_table.get32(0xd0000), XX);
        assert_eq!(data.property_table.get32(0xe0001), CM);
        assert_eq!(data.property_table.get32(0xe0020), CM);
    }

    #[test]
    #[cfg(feature = "unstable")]
    fn load_line_data() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<SegmenterBreakLineV3> = provider
            .load(Default::default())
            .expect("Loading should succeed!");
        let data = response.payload.get();
        // Note: The following match statement had been used in line.rs:
        //
        // match codepoint {
        //     0x20000..=0x2fffd => ID,
        //     0x30000..=0x3fffd => ID,
        //     0xe0001 => CM,
        //     0xe0020..=0xe007f => CM,
        //     0xe0100..=0xe01ef => CM,
        //     _ => XX,
        // }

        const CM: u8 = 18;
        const XX: u8 = 65;
        const ID: u8 = 36;

        assert_eq!(data.property_table.get32(0x20000), ID);
        assert_eq!(data.property_table.get32(0x3fffd), ID);
        assert_eq!(data.property_table.get32(0xd0000), XX);
        assert_eq!(data.property_table.get32(0xe0001), CM);
        assert_eq!(data.property_table.get32(0xe0020), CM);
    }

    #[test]
    #[should_panic]
    fn missing_locale_data() {
        let provider = SourceDataProvider::new_testing();
        let response: DataResponse<SegmenterBreakSentenceOverrideV1> = provider
            .load(Default::default())
            .expect("Loading should succeed!");
        response.payload.get();
    }
    // TODO: Add loading override table data. But no locales in testdata.
}
