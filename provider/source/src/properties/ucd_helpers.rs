// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! This module contains helpers for parsing files from the UCD

use crate::SourceDataProvider;
use icu_provider::prelude::*;

pub(crate) fn parse_range(range_str: &str) -> std::ops::RangeInclusive<u32> {
    let (a, b) = range_str.split_once("..").unwrap_or((range_str, range_str));
    let a = parse_cp(a);
    let b = parse_cp(b);
    a..=b
}

pub(crate) fn parse_cp(cp: &str) -> u32 {
    u32::from_str_radix(cp, 16).unwrap()
}

pub(crate) fn parse_cps(cps: &str) -> String {
    cps.split_whitespace()
        .map(parse_cp)
        .map(|cp| char::from_u32(cp).unwrap())
        .collect()
}

#[derive(Copy, Clone, Debug)]
pub(crate) struct UcdFields<'a>(&'a str);

#[derive(Copy, Clone, Debug)]
pub(crate) enum UcdLine<'a> {
    Fields(UcdFields<'a>),
    Missing(UcdFields<'a>),
}

impl<'a> UcdFields<'a> {
    pub(crate) fn fields(self) -> impl Iterator<Item = &'a str> + 'a {
        self.0.split(';').map(str::trim)
    }
}

impl<'a> UcdLine<'a> {
    pub(crate) fn skip_missing_rule(self) -> Option<UcdFields<'a>> {
        if let Self::Fields(c) = self {
            Some(c)
        } else {
            None
        }
    }
}

impl SourceDataProvider {
    /// Helper to parse UCD files line-by-line, providing an iterator over the fields of each line.
    ///
    /// It reads the file, strips comments (lines starting with `#` or anything after `#`),
    /// skips empty lines, and returns an iterator over each line.
    ///
    /// It handles `# @missing` rules by returning them as `UcdLine::Missing`
    pub(crate) fn parse_ucd_lines<'a>(
        &'a self,
        file: &str,
    ) -> Result<impl Iterator<Item = UcdLine<'a>>, DataError> {
        Ok(self
            .unicode()?
            .read_to_string(file)?
            .lines()
            .filter_map(|line| {
                if let Some(l) = line.strip_prefix("# @missing: ") {
                    Some(UcdLine::Missing(UcdFields(l)))
                } else {
                    let line = line.split('#').next().unwrap().trim();
                    if line.is_empty() {
                        return None;
                    }
                    Some(UcdLine::Fields(UcdFields(line)))
                }
            }))
    }
}
