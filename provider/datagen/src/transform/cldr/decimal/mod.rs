// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_locid::extensions::unicode::key;
use icu_locid::extensions::unicode::Value;
use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;
use tinystr::TinyAsciiStr;

mod compact;
mod compact_decimal_pattern;
mod decimal_pattern;
mod symbols;

impl crate::DatagenProvider {
    /// Returns the digits for the given numbering system name.
    fn get_digits_for_numbering_system(
        &self,
        nsname: TinyAsciiStr<8>,
    ) -> Result<[char; 10], DataError> {
        let resource: &cldr_serde::numbering_systems::Resource = self
            .source
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

        match resource.supplemental.numbering_systems.get(&nsname) {
            Some(ns) => ns.digits.as_deref().and_then(digits_str_to_chars),
            None => None,
        }
        .ok_or_else(|| {
            DataError::custom("Could not process numbering system").with_display_context(&nsname)
        })
    }

    fn get_supported_numsys_for_langid_without_default(
        &self,
        langid: &LanguageIdentifier,
    ) -> Result<Vec<TinyAsciiStr<8>>, DataError> {
        let resource: &cldr_serde::numbers::Resource = self
            .source
            .cldr()?
            .numbers()
            .read_and_parse(langid, "numbers.json")?;

        let numbers = &resource
            .main
            .0
            .get(langid)
            .expect("CLDR file contains the expected language")
            .numbers;

        Ok(numbers
            .numsys_data
            .symbols
            .keys()
            .filter(|nsname| **nsname != numbers.default_numbering_system)
            .copied()
            .collect())
    }

    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(self.filter_data_locales(
            self.source
                .cldr()?
                .numbers()
                .list_langs()?
                .flat_map(|langid| {
                    let last = DataLocale::from(&langid);
                    self.get_supported_numsys_for_langid_without_default(&langid)
                        .expect("All languages from list_langs should be present")
                        .into_iter()
                        .map(move |nsname| {
                            let mut data_locale = DataLocale::from(&langid);
                            data_locale.set_unicode_ext(
                                key!("nu"),
                                Value::try_from_single_subtag(nsname.as_bytes())
                                    .expect("CLDR should have valid numbering system names"),
                            );
                            data_locale
                        })
                        .chain([last])
                })
                .collect(),
        ))
    }
}
