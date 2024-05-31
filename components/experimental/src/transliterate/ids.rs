// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use icu_locale_core::Locale;
use icu_provider::prelude::*;
use writeable::Writeable;

pub fn bcp47_to_data_key_attributes(locale: &Locale) -> DataKeyAttributes {
    #[allow(clippy::unwrap_used)] // any BCP-47 locale is a valid aux key
    DataKeyAttributes::try_from_iter(
        locale
            .write_to_string()
            .split('-')
            .map(str::parse)
            .filter_map(Result::ok),
    )
    .unwrap()
}

pub fn unparsed_bcp47_to_data_key_attributes(dep: &str) -> Result<DataKeyAttributes, DataError> {
    DataKeyAttributes::try_from_iter(
        dep.split('-')
            .map(str::parse)
            // TODO: Bubble a potential subtag parse error out. Currently
            // we ignore it inside of the `filter_map`.
            .filter_map(Result::ok),
    )
}
