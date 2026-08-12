// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

pub(crate) mod displayname;
pub(crate) mod essentials;
pub(crate) mod extended;
pub(crate) mod fractions;
pub(crate) mod no_currency;
pub(crate) mod patterns;
pub(crate) mod symbols;

use std::borrow::Cow;
use std::collections::HashSet;

use icu_pattern::DoublePlaceholderPattern;
use icu_provider::prelude::*;
use zerovec::VarZeroVec;

use crate::DataIdentifierCached;
use crate::SourceDataProvider;
use crate::cldr_serde;

/// Helper to collect and deduplicate `DoublePlaceholderPattern`s into a `VarZeroVec`.
#[derive(Default)]
struct PatternSet {
    patterns: Vec<Box<DoublePlaceholderPattern>>,
}

impl PatternSet {
    fn new() -> Self {
        Self::default()
    }

    fn add(&mut self, opt_cow: Option<Cow<'_, DoublePlaceholderPattern>>) -> Option<u8> {
        opt_cow.map(|cow| {
            let pat: Box<DoublePlaceholderPattern> = cow.into_owned();
            if let Some(idx) = self.patterns.iter().position(|p| p == &pat) {
                idx as u8
            } else {
                let idx = self.patterns.len() as u8;
                self.patterns.push(pat);
                idx
            }
        })
    }

    fn into_var_zero_vec<'data>(self) -> VarZeroVec<'data, DoublePlaceholderPattern> {
        VarZeroVec::from(&self.patterns)
    }
}

/// Helper to iterate through locales and numbering system patterns in `numbers.json`.
fn iter_numsys_pattern_ids<F>(
    provider: &SourceDataProvider,
    predicate: F,
) -> Result<HashSet<DataIdentifierCached>, DataError>
where
    F: Fn(&cldr_serde::numbers::CurrencyFormattingPatterns) -> bool,
{
    let mut ids = HashSet::new();
    for locale in provider.cldr()?.numbers().list_locales()? {
        let numbers_resource: &cldr_serde::numbers::Resource = provider
            .cldr()?
            .numbers()
            .read_and_parse(&locale, "numbers.json")?;
        let numbers = &numbers_resource.main.value.numbers;
        let default_numsys = &numbers.default_numbering_system;

        for (nsname, patterns) in &numbers.numsys_data.currency_patterns {
            if !predicate(patterns) {
                continue;
            }
            if nsname == default_numsys {
                ids.insert(DataIdentifierCached::from_locale(locale));
            } else {
                ids.insert(
                    DataIdentifierCached::from_writeable_attributes_and_locale(nsname, locale)?
                        .into_owned(),
                );
            }
        }
    }
    Ok(ids)
}
