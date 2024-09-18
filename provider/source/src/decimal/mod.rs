// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu_provider::prelude::*;

#[cfg(feature = "experimental")]
mod compact;
#[cfg(feature = "experimental")]
mod compact_decimal_pattern;
pub(crate) mod decimal_pattern;
mod symbols;

impl SourceDataProvider {
    /// Returns the digits for the given numbering system name.
    fn get_digits_for_numbering_system(&self, nsname: &str) -> Result<[char; 10], DataError> {
        let resource: &cldr_serde::numbering_systems::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/numberingSystems.json")?;

        fn digits_str_to_chars(digits_str: &str) -> Option<[char; 10]> {
            let mut chars = digits_str.chars();
            Some([
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
                chars.next()?,
            ])
        }

        match resource.supplemental.numbering_systems.get(nsname) {
            Some(ns) => ns.digits.as_deref().and_then(digits_str_to_chars),
            None => None,
        }
        .ok_or_else(|| {
            DataError::custom("Could not process numbering system").with_display_context(nsname)
        })
    }

    fn get_supported_numsys_for_langid_without_default(
        &self,
        locale: &DataLocale,
    ) -> Result<Vec<Box<DataMarkerAttributes>>, DataError> {
        let resource: &cldr_serde::numbers::Resource = self
            .cldr()?
            .numbers()
            .read_and_parse(locale, "numbers.json")?;

        let numbers = &resource.main.value.numbers;

        Ok(numbers
            .numsys_data
            .symbols
            .keys()
            .filter(|nsname| **nsname != numbers.default_numbering_system)
            .filter_map(|nsname| Some(DataMarkerAttributes::try_from_str(nsname).ok()?.to_owned()))
            .collect())
    }

    fn iter_ids_for_numbers(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .flat_map(|locale| {
                let data_locale = locale.clone();
                let last = data_locale.clone();
                self.get_supported_numsys_for_langid_without_default(&locale)
                    .expect("All languages from list_locales should be present")
                    .into_iter()
                    .map(move |nsname| {
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::try_from_str(&nsname).unwrap(),
                            &data_locale,
                        )
                        .into_owned()
                    })
                    .chain([DataIdentifierCow::from_locale(last)])
            })
            .collect())
    }
}
