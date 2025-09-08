// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use std::collections::HashSet;

use crate::cldr_serde;
use crate::SourceDataProvider;
use icu_locale_core::locale;
use icu_provider::prelude::*;

#[cfg(feature = "experimental")]
mod compact;
#[cfg(feature = "experimental")]
mod compact_decimal_pattern;
pub(crate) mod decimal_pattern;
mod symbols;

mod digits;

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

    /// Get all numbering systems supported by a langid, potentially excluding the default one
    fn get_supported_numsys_for_langid(
        &self,
        locale: &DataLocale,
        exclude_default: bool,
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
            .filter(|nsname| !exclude_default || **nsname != numbers.default_numbering_system)
            .filter_map(|nsname| Some(DataMarkerAttributes::try_from_str(nsname).ok()?.to_owned()))
            .collect())
    }

    /// Produce DataIdentifier's for all locale-numbering system pairs in the form <locale>/<numsys>
    /// This also includes a bare <locale>
    fn iter_ids_for_numbers_with_locales(
        &self,
    ) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .flat_map(|locale| {
                let last = locale;
                self.get_supported_numsys_for_langid(&locale, true)
                    .expect("All languages from list_locales should be present")
                    .into_iter()
                    .map(move |nsname| {
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::try_from_str(&nsname).unwrap(),
                            &locale,
                        )
                        .into_owned()
                    })
                    .chain([DataIdentifierCow::from_locale(last)])
            })
            .collect())
    }

    /// Produce DataIdentifier's for all *used* numbering systems in the form und/<numsys>
    fn iter_ids_for_used_numbers(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        Ok(self
            .cldr()?
            .numbers()
            .list_locales()?
            .flat_map(|locale| {
                self.get_supported_numsys_for_langid(&locale, false)
                    .expect("All languages from list_locales should be present")
                    .into_iter()
                    .map(move |nsname| {
                        DataIdentifierBorrowed::for_marker_attributes_and_locale(
                            DataMarkerAttributes::try_from_str(&nsname).unwrap(),
                            &locale!("und").into(),
                        )
                        .into_owned()
                    })
            })
            .collect())
    }

    /// Produce DataIdentifier's for all digit-based numbering systems in the form und/<numsys>
    #[allow(unused)] // TODO(#5824): Support user-specified numbering systems
    fn iter_all_number_ids(&self) -> Result<HashSet<DataIdentifierCow<'static>>, DataError> {
        use cldr_serde::numbering_systems::NumberingSystemType;
        let resource: &cldr_serde::numbering_systems::Resource = self
            .cldr()?
            .core()
            .read_and_parse("supplemental/numberingSystems.json")?;

        Ok(resource
            .supplemental
            .numbering_systems
            .iter()
            .filter(|(_nsname, data)| data.nstype == NumberingSystemType::Numeric)
            .map(|(nsname, _data)| {
                DataIdentifierBorrowed::for_marker_attributes_and_locale(
                    DataMarkerAttributes::try_from_str(nsname).unwrap(),
                    &locale!("und").into(),
                )
                .into_owned()
            })
            .collect())
    }
}
