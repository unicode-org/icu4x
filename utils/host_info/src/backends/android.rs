// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{
    backends::{
        shared::posix::{raw_locale_categories, LocaleCategory},
        HostInfoBackend, RawHostInfoBackend,
    },
    error::HostInfoError,
};

pub struct AndroidHostInfoBackend;

impl HostInfoBackend for AndroidHostInfoBackend {}

impl RawHostInfoBackend for AndroidHostInfoBackend {
    fn raw_requested_locales() -> Result<Vec<String>, HostInfoError> {
        let mut categories = raw_locale_categories()?;
        let mut locales = Vec::with_capacity(categories.len());

        // Add LC_ALL if it exists
        if let Some(primary_locale) = categories.remove(&LocaleCategory::All) {
            locales.push(primary_locale);
        }

        // Add any remaining locales that were explicitly set
        for s in categories.into_values() {
            if !locales.contains(&s) {
                locales.push(s);
            }
        }
        Ok(locales)
    }
}
