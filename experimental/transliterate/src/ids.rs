// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use alloc::string::ToString;
use icu_locid::Locale;
use icu_provider::prelude::*;

/// Conversion from an unknown legacy ID to an internal ID is handled by this function.
///
/// Known legacy IDs, i.e., ones that have associated BCP47 IDs in their metadata, are simply
/// that BCP47 ID. For unknown legacy IDs, the output is given by this function.
pub fn legacy_id_to_bcp_47(source: &str, target: &str, variant: Option<&str>) -> Locale {
    // TODO(#3891): Decide representation for unknown BCP47 IDs
    #[allow(clippy::indexing_slicing)] // safe indices and ASCII-only text
    #[allow(clippy::unwrap_used)] // correct BCP47 syntax
    if let Some(variant) = variant {
        alloc::format!(
            "und-x-{}-{}-{}",
            &source[..Ord::min(8, source.len())],
            &target[..Ord::min(8, target.len())],
            &variant[..Ord::min(8, variant.len())]
        )
    } else {
        alloc::format!(
            "und-x-{}-{}",
            &source[..Ord::min(8, source.len())],
            &target[..Ord::min(8, target.len())]
        )
    }
    // normalizing to ASCII lowercase to avoid duplicate dependencies like Any-null and Any-Null
    .to_ascii_lowercase()
    .parse()
    .unwrap()
}

pub fn bcp47_to_data_locale(locale: &Locale) -> DataLocale {
    let mut data_locale = DataLocale::default();
    #[allow(clippy::unwrap_used)] // any BCP-47 locale is a valid aux key
    data_locale.set_aux(locale.to_string().parse().unwrap());
    data_locale
}

pub fn unparsed_bcp47_to_data_locale(dep: &str) -> Result<DataLocale, DataError> {
    let mut data_locale = DataLocale::default();
    data_locale.set_aux(dep.parse()?);
    Ok(data_locale)
}
