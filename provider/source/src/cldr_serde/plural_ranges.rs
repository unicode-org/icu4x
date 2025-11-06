// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Serde structs representing CLDR JSON pluralRanges.json files.
//!
//! Sample file:
//! <https://github.com/unicode-org/cldr-json/blob/main/cldr-json/cldr-core/supplemental/pluralRanges.json>

use icu::locale::LanguageIdentifier;
use serde::{de::Visitor, Deserialize};
use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug)]
pub(crate) struct PluralRange {
    pub(crate) start: String,
    pub(crate) end: String,
}

impl<'de> Deserialize<'de> for PluralRange {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct PluralRangeVisitor;

        impl Visitor<'_> for PluralRangeVisitor {
            type Value = PluralRange;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(formatter,
                    "a plural range rule of the form pluralRange-start-<plural category>-end-<plural category>",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let v = v.strip_prefix("pluralRange-start-").ok_or_else(|| {
                    E::custom("expected prefix `pluralRange-start-` before start category")
                })?;
                let (start, v) = v.split_once('-').ok_or_else(|| {
                    E::custom("missing token `-` between start and end categories")
                })?;
                let end = v
                    .strip_prefix("end-")
                    .ok_or_else(|| E::custom("expected prefix `end-` before end category"))?;

                Ok(PluralRange {
                    start: start.into(),
                    end: end.into(),
                })
            }
        }

        deserializer.deserialize_string(PluralRangeVisitor)
    }
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct LocalePluralRanges(pub(crate) HashMap<PluralRange, String>);

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct PluralRanges(pub(crate) HashMap<LanguageIdentifier, LocalePluralRanges>);

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Supplemental {
    pub(crate) plurals: PluralRanges,
}

#[derive(PartialEq, Debug, Deserialize)]
pub(crate) struct Resource {
    pub(crate) supplemental: Supplemental,
}
