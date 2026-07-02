// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#![cfg_attr(
    not(any(feature = "use_wasm", feature = "use_icu4c")),
    allow(unused_imports, dead_code)
)]

use crate::SourceDataProvider;
use crate::properties::ucd_helpers;
use icu::casemap::provider::data::{CaseMapData, CaseType, DotType};
use icu::casemap::provider::exceptions::{CaseMapExceptions, Exception};
use icu::casemap::provider::{CaseMap, CaseMapUnfold, CaseMapUnfoldV1, CaseMapV1};
use icu::properties::{CodePointMapData, CodePointSetData, props};
use icu_provider::prelude::*;
use std::collections::{BTreeMap, BTreeSet, HashSet, VecDeque};
use std::convert::TryFrom;

#[cfg(not(any(feature = "use_wasm", feature = "use_icu4c")))]
impl DataProvider<CaseMapV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CaseMapV1>, DataError> {
        self.check_req::<CaseMapV1>(req)?;
        Err(DataError::custom(
            "The `use_wasm` or `use_icu4c` feature must be enabled to use CaseMapV1",
        ))
    }
}

#[cfg(any(feature = "use_wasm", feature = "use_icu4c"))]
impl DataProvider<CaseMapV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CaseMapV1>, DataError> {
        self.check_req::<CaseMapV1>(req)?;

        let mut simple = BTreeMap::new();
        for line in self.parse_ucd_lines("ucd/UnicodeData.txt")? {
            let Some(line) = line.skip_missing_rule() else {
                continue;
            };
            let mut fields = line.fields();

            // UnicodeData.txt has 14 fields
            // 0: Code_Point
            // ...
            // 12: Simple_Uppercase_Mapping
            // 13: Simple_Lowercase_Mapping
            // 14: Simple_Titlecase_Mapping
            let Ok(cp) = char::try_from(ucd_helpers::parse_cp(fields.next().unwrap())) else {
                continue;
            };

            for _ in 1..=11 {
                fields.next();
            }
            let upper = fields
                .next()
                .filter(|s| !s.is_empty())
                .map(ucd_helpers::parse_cp);
            let lower = fields
                .next()
                .filter(|s| !s.is_empty())
                .map(ucd_helpers::parse_cp);
            let title = fields
                .next()
                .filter(|s| !s.is_empty())
                .map(ucd_helpers::parse_cp)
                .or(upper);
            assert_eq!(fields.next(), None);

            simple.insert(
                cp,
                (
                    upper.map(|cp| char::from_u32(cp).unwrap()),
                    lower.map(|cp| char::from_u32(cp).unwrap()),
                    title.map(|cp| char::from_u32(cp).unwrap()),
                ),
            );
        }

        let mut special = BTreeMap::<char, (String, String, String, bool)>::new();
        for line in self.parse_ucd_lines("ucd/SpecialCasing.txt")? {
            let Some(line) = line.skip_missing_rule() else {
                continue;
            };
            let mut fields = line.fields();

            let cp = char::from_u32(ucd_helpers::parse_cp(fields.next().unwrap())).unwrap();
            let lower = ucd_helpers::parse_cps(fields.next().unwrap());
            let title = ucd_helpers::parse_cps(fields.next().unwrap());
            let upper = ucd_helpers::parse_cps(fields.next().unwrap());
            let condition = fields.next().filter(|s| !s.is_empty());

            // There can be multiple entries for the same code point, so we need to merge them together.
            let entry = special.entry(cp).or_default();
            if condition.is_none() {
                entry.0 = lower;
                entry.1 = upper;
                entry.2 = title;
            } else {
                // Ignore the actual mappings here. We hardcode them in runtime code.
                entry.3 = true;
            }
        }
        // TODO: We currently set this in our data, but don't handle this at runtime,
        // see https://unicode-org.atlassian.net/browse/ICU-13416
        special.entry('և').or_default().3 = true;

        let mut case_folds = BTreeMap::<char, (Option<char>, Option<String>, bool)>::new();
        for line in self.parse_ucd_lines("ucd/CaseFolding.txt")? {
            let Some(line) = line.skip_missing_rule() else {
                continue;
            };
            let mut fields = line.fields();
            let cp = char::from_u32(ucd_helpers::parse_cp(fields.next().unwrap())).unwrap();
            let status = fields.next().unwrap();
            let full_mapping = ucd_helpers::parse_cps(fields.next().unwrap());
            let simple_mapping = full_mapping.chars().next().unwrap();

            // There can be multiple entries for the same code point, so we need to merge them together.
            let entry = case_folds.entry(cp).or_default();
            match status {
                "C" => {
                    entry.0 = Some(simple_mapping);
                    entry.1 = Some(full_mapping);
                }
                "F" => {
                    entry.1 = Some(full_mapping);
                }
                "S" => {
                    entry.0 = Some(simple_mapping);
                }
                "T" => {
                    // Ignore the actual mappings here. We hardcode them in runtime code.
                    entry.2 = true;
                }
                _ => unreachable!("Invalid status in CaseFolding.txt: {}", status),
            }
        }

        let adjacency_list = build_adjacency_list(&simple, &special, &case_folds);
        let case_sensitive_set = case_sensitive_set(&simple, &special, &case_folds);

        let case_ignorable = CodePointSetData::try_new_unstable::<props::CaseIgnorable>(self)?;
        let case_ignorable = case_ignorable.as_borrowed();

        let soft_dotted = CodePointSetData::try_new_unstable::<props::SoftDotted>(self)?;
        let soft_dotted = soft_dotted.as_borrowed();

        let ccc = CodePointMapData::<props::CanonicalCombiningClass>::try_new_unstable(self)?;
        let ccc = ccc.as_borrowed();

        let is_lowercase = CodePointSetData::try_new_unstable::<props::Lowercase>(self)?;
        let is_lowercase = is_lowercase.as_borrowed();

        let is_uppercase = CodePointSetData::try_new_unstable::<props::Uppercase>(self)?;
        let is_uppercase = is_uppercase.as_borrowed();

        let gc = CodePointMapData::<props::GeneralCategory>::try_new_unstable(self)?;
        let gc = gc.as_borrowed();

        let mut builder = icu_codepointtrie_builder::CodePointTrieBuilder::new(
            CaseMapData::UNCASED_INSENSITIVE_NO_DOT,
            CaseMapData::UNCASED_INSENSITIVE_NO_DOT,
            self.trie_type().into(),
        );
        let mut exceptions = Vec::<Exception>::new();

        for (&c, &(simple_upper, simple_lower, simple_title)) in &simple {
            let (special_lower, special_upper, special_title, has_conditional_special) = special
                .get(&c)
                .map(|(lower, title, upper, has_conditional_special)| {
                    (
                        Some(lower.as_str()),
                        Some(title.as_str()),
                        Some(upper.as_str()),
                        *has_conditional_special,
                    )
                })
                .unwrap_or((None, None, None, false));

            let (simple_fold, special_fold, has_conditional_fold) = case_folds
                .get(&c)
                .map(|(simple, full, turkic)| (*simple, full.as_deref(), *turkic))
                .unwrap_or((None, None, false));

            // BFS to find all characters that are reachable from c via case mappings and case folds.
            let mut full_closure = BTreeSet::from_iter([c]);
            let mut queue = VecDeque::from_iter([c]);
            while let Some(u) = queue.pop_front() {
                if let Some(neighbors) = adjacency_list.get(&u) {
                    for &v in neighbors {
                        if full_closure.insert(v) {
                            queue.push_back(v);
                        }
                    }
                }
            }
            full_closure.remove(&c);

            let dot_type = if soft_dotted.contains(c) {
                DotType::SoftDotted
            } else {
                match ccc.get(c) {
                    props::CanonicalCombiningClass::Above => DotType::Above,
                    props::CanonicalCombiningClass::NotReordered => DotType::NoDot,
                    _ => DotType::OtherAccent,
                }
            };

            let ignoreable = case_ignorable.contains(c);
            let sensitive = case_sensitive_set.contains(&c);

            let case_type = if is_lowercase.contains(c) {
                Some(CaseType::Lower)
            } else if is_uppercase.contains(c) {
                Some(CaseType::Upper)
            } else if gc.get(c) == props::GeneralCategory::TitlecaseLetter {
                Some(CaseType::Title)
            } else {
                None
            };

            builder.set_value(
                c as u32,
                CaseMapData::new(
                    &mut exceptions,
                    c,
                    ignoreable,
                    sensitive,
                    dot_type,
                    case_type,
                    simple_upper,
                    simple_lower,
                    simple_title,
                    special_lower,
                    special_upper,
                    special_title,
                    has_conditional_special,
                    simple_fold,
                    special_fold,
                    has_conditional_fold,
                    full_closure,
                ),
            );
        }

        let case_mapping = CaseMap {
            trie: builder.build(),
            exceptions: CaseMapExceptions {
                exceptions: exceptions.as_slice().into(),
            },
        };

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(case_mapping),
        })
    }
}

impl crate::IterableDataProviderCached<CaseMapV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

impl DataProvider<CaseMapUnfoldV1> for SourceDataProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<CaseMapUnfoldV1>, DataError> {
        self.check_req::<CaseMapUnfoldV1>(req)?;

        let mut unfold_map: BTreeMap<String, BTreeSet<char>> = BTreeMap::new();

        for line in self.parse_ucd_lines("ucd/CaseFolding.txt")? {
            let Some(line) = line.skip_missing_rule() else {
                continue;
            };
            let mut fields = line.fields();
            let cp = char::from_u32(ucd_helpers::parse_cp(fields.next().unwrap())).unwrap();
            let status = fields.next().unwrap();
            let full_mapping = ucd_helpers::parse_cps(fields.next().unwrap());
            if matches!(status, "C" | "F") && full_mapping.chars().count() > 1 {
                unfold_map.entry(full_mapping).or_default().insert(cp);
            }
        }

        let map = unfold_map
            .iter()
            .map(|(full_mapping, cps)| {
                (
                    potential_utf::PotentialUtf8::from_str(full_mapping),
                    cps.iter().copied().collect::<String>(),
                )
            })
            .collect();

        Ok(DataResponse {
            metadata: Default::default(),
            payload: DataPayload::from_owned(CaseMapUnfold { map }),
        })
    }
}

impl crate::IterableDataProviderCached<CaseMapUnfoldV1> for SourceDataProvider {
    fn iter_ids_cached(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(HashSet::from_iter([Default::default()]))
    }
}

/// Generates the case sensitive set, which is all characters that have a case mapping or case fold,
/// or are a target of a case mapping or case fold.
#[allow(clippy::type_complexity)]
fn case_sensitive_set(
    basic_casing: &BTreeMap<char, (Option<char>, Option<char>, Option<char>)>,
    special_casing: &BTreeMap<char, (String, String, String, bool)>,
    case_folds: &BTreeMap<char, (Option<char>, Option<String>, bool)>,
) -> BTreeSet<char> {
    let mut case_sensitive_set = BTreeSet::new();

    for (&c, &(upper, lower, title)) in basic_casing {
        case_sensitive_set
            .extend((lower.is_some() || upper.is_some() || title.is_some()).then_some(c));
        case_sensitive_set.extend(lower);
        case_sensitive_set.extend(upper);
        case_sensitive_set.extend(title);
    }

    for (&c, &(simple_fold, ref special_fold, _)) in case_folds {
        case_sensitive_set.insert(c);
        case_sensitive_set.extend(simple_fold);
        case_sensitive_set.extend(special_fold.as_deref().unwrap_or_default().chars());
    }

    for (&c, (special_lower, special_upper, special_title, _)) in special_casing {
        case_sensitive_set.insert(c);
        case_sensitive_set.extend(special_lower.chars());
        case_sensitive_set.extend(special_upper.chars());
        case_sensitive_set.extend(special_title.chars());
    }

    case_sensitive_set
}

/// Constructs an adjacency list of all characters that are connected via case mappings or case folds.
#[allow(clippy::type_complexity)]
fn build_adjacency_list(
    basic_casing: &BTreeMap<char, (Option<char>, Option<char>, Option<char>)>,
    special_casing: &BTreeMap<char, (String, String, String, bool)>,
    case_folds: &BTreeMap<char, (Option<char>, Option<String>, bool)>,
) -> BTreeMap<char, BTreeSet<char>> {
    let mut adj_list: BTreeMap<char, BTreeSet<char>> = BTreeMap::new();
    let mut add_edge = |u: char, v: Option<char>| {
        if let Some(v) = v
            && u != v
        {
            adj_list.entry(u).or_default().insert(v);
            adj_list.entry(v).or_default().insert(u);
        }
    };

    for (&c, &(upper, lower, title)) in basic_casing {
        add_edge(c, lower);
        add_edge(c, upper);
        add_edge(c, title);
    }
    for (&c, &(simple_fold, ref special_fold, _)) in case_folds {
        add_edge(c, simple_fold);
        if let Some(special_fold) = special_fold
            && special_fold.chars().count() == 1
        {
            add_edge(c, special_fold.chars().next());
        }
    }

    for (&c, (special_lower, special_upper, special_title, _)) in special_casing {
        if special_lower.chars().count() == 1 {
            add_edge(c, special_lower.chars().next());
        }
        if special_upper.chars().count() == 1 {
            add_edge(c, special_upper.chars().next());
        }
        if special_title.chars().count() == 1 {
            add_edge(c, special_title.chars().next());
        }
    }

    adj_list
}
