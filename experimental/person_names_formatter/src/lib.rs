// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

#[macro_use]
extern crate alloc;

use alloc::collections::BTreeMap;

use icu_locid::Locale;

use crate::api::{NameField, PersonName, PersonNamesFormatter, PreferredOrder};
use crate::provider::PersonNamesFormattingDefinitionV1;

pub mod api;
mod formatter;
pub mod provider;

///
/// Returns a new person name to format.
///
pub fn new_person_name(
    person_data: BTreeMap<NameField, String>,
    locale: Option<Locale>,
    preferred_order: Option<PreferredOrder>,
) -> Result<Box<dyn PersonName>, String> {
    formatter::new_person_name(person_data, locale, preferred_order)
}

///
/// Returns a new person names formatter.
///
pub fn new_formatter<'a>(
    config: PersonNamesFormattingDefinitionV1<'a>,
) -> Result<Box<dyn PersonNamesFormatter + 'a>, String> {
    formatter::new_formatter(config)
}
