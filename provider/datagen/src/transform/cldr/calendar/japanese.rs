// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::transform::cldr::cldr_serde;
use icu_calendar::provider::*;
use icu_locid::langid;
use icu_provider::datagen::IterableDataProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::env;
use std::str::FromStr;
use tinystr::tinystr;
use tinystr::TinyStr16;

const JAPANESE_FILE: &str = include_str!("./snapshot-japanese@1.json");

impl crate::DatagenProvider {
    fn load_japanese_eras(
        &self,
        japanext: bool,
    ) -> Result<DataResponse<JapaneseErasV1Marker>, DataError> {
        // The era codes depend on the Latin romanizations of the eras, found
        // in the `en` locale. We load this data to construct era codes but
        // actual user code only needs to load the data for the locales it cares about.
        let era_name_map = &self
            .source
            .cldr()?
            .dates("japanese")
            .read_and_parse::<cldr_serde::ca::Resource>(&langid!("en"), "ca-japanese.json")?
            .main
            .0
            .get(&langid!("en"))
            .ok_or(DataError::custom(
                "ca-japanese.json does not have en locale",
            ))?
            .dates
            .calendars
            .get("japanese")
            .ok_or(DataError::custom(
                "ca-japanese.json does not contain 'japanese' calendar",
            ))?
            .eras
            .abbr;

        let era_dates_map = &self
            .source
            .cldr()?
            .core()
            .read_and_parse::<cldr_serde::japanese::Resource>("supplemental/calendarData.json")?
            .supplemental
            .calendar_data
            .japanese
            .eras;

        let mut dates_to_eras = BTreeMap::new();

        for (era_id, date) in era_dates_map.iter() {
            // These don't exist but may in the future
            if era_id.contains("variant") {
                continue;
            }

            if era_id == "-1" || era_id == "-2" {
                // These eras are handled in code
                continue;
            }

            let start_date =
                EraStartDate::from_str(date.start.as_ref().unwrap()).map_err(|_| {
                    DataError::custom(
                        "calendarData.json contains unparseable data for a japanese era",
                    )
                    .with_display_context(&format!("era index {era_id}"))
                })?;

            if start_date.year >= 1868 || japanext {
                let code = era_to_code(era_name_map.get(era_id).unwrap(), start_date.year)
                    .map_err(|e| DataError::custom("Era codes").with_display_context(&e))?;

                dates_to_eras.insert(start_date, code);
            }
        }

        let ret = JapaneseErasV1 {
            dates_to_eras: dates_to_eras.into_iter().collect(),
        };

        // Integrity check
        //
        // Era code generation relies on the English era data which could in theory change; we have an integrity check
        // to catch such cases. It is relatively rare for a new era to be added, and in those cases the integrity check can
        // be disabled when generating new data.
        if japanext && env::var("ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK").is_err() {
            let snapshot: JapaneseErasV1 = serde_json::from_str(JAPANESE_FILE)
                .expect("Failed to parse the precached snapshot-japanese@1.json. This is a bug.");

            if snapshot != ret {
                return Err(DataError::custom(
                    "Era data has changed! This can be for two reasons: Either the CLDR locale data for Japanese eras has \
                    changed in an incompatible way, or there is a new Japanese era. Please comment out the integrity \
                    check in icu_datagen's japanese.rs and inspect the update to japanese@1.json (resource key `calendar/japanese`) \
                    in the generated JSON by rerunning the datagen tool (`cargo make testdata` in the ICU4X repo). \
                    Rerun with ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK=1 to regenerate testdata properly, and check which situation \
                    it is. If a new era has been introduced, copy over the new testdata to snapshot-japanese@1.json \
                    in icu_datagen. If not, it's likely that japanese.rs in icu_datagen will need \
                    to be updated to handle the data changes. Once done, be sure to regenerate datetime/symbols@1 as well if not \
                    doing so already"
                ));
            }
        }

        Ok(DataResponse {
            metadata: DataResponseMetadata::default(),
            payload: Some(DataPayload::from_owned(ret)),
        })
    }
}

impl DataProvider<JapaneseErasV1Marker> for crate::DatagenProvider {
    fn load(&self, req: DataRequest) -> Result<DataResponse<JapaneseErasV1Marker>, DataError> {
        self.check_req::<JapaneseErasV1Marker>(req)?;
        self.load_japanese_eras(false)
    }
}

impl DataProvider<JapaneseExtendedErasV1Marker> for crate::DatagenProvider {
    fn load(
        &self,
        req: DataRequest,
    ) -> Result<DataResponse<JapaneseExtendedErasV1Marker>, DataError> {
        self.check_req::<JapaneseExtendedErasV1Marker>(req)?;
        let DataResponse { metadata, payload } = self.load_japanese_eras(true)?;
        Ok(DataResponse {
            metadata,
            payload: payload.map(|p| p.cast()),
        })
    }
}

/// See https://docs.google.com/document/d/1vMVhMHgCYRyx2gmwEfKRyXWDg_lrQadd8iMVU9uPK1o/edit?usp=chrome_omnibox&ouid=111665445991279316689
/// for the era identifier spec
fn era_to_code(original: &str, year: i32) -> Result<TinyStr16, String> {
    // Some examples of CLDR era names:
    // "Shōryaku (1077–1081)", "Nin’an (1166–1169)", "Tenpyō-kampō (749–749)"
    //
    // We want to produce a unique readable era code that
    // contains ascii lowercase letters, followed
    // by a hyphen and then a year name (except for post-Meiji era codes)
    //
    // We also want it to fit in a TinyAsciiStr<16>. What we will do is:
    //
    // - only look at the actual name
    // - normalize by removing hyphens and apostrophes, as well as converting ō/ū
    //   to ascii o/u
    // - truncating to fit 16 characters

    let name = original
        .split(' ')
        .next()
        .expect("split iterator is non-empty");
    let name = name
        .replace(['ō', 'Ō'], "o")
        .replace(['ū', 'Ū'], "u")
        .replace(['-', '\'', '’'], "")
        .to_lowercase();
    if !name.is_ascii() {
        return Err(format!(
            "Era name {name} (parsed from {original}) contains non-ascii characters"
        ));
    }

    let code = if year >= 1868 {
        name
    } else {
        format!("{name}-{year}")
    };

    // In case of future or past eras that do not fit, we may need to introduce a truncation scheme
    let code = code.parse().map_err(|e| {
        format!("Era code {code} (parsed from {original}) does not fit into a TinyStr16: {e}")
    })?;
    Ok(code)
}

impl IterableDataProvider<JapaneseErasV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

impl IterableDataProvider<JapaneseExtendedErasV1Marker> for crate::DatagenProvider {
    fn supported_locales(&self) -> Result<Vec<DataLocale>, DataError> {
        Ok(vec![Default::default()])
    }
}

pub fn get_era_code_map() -> BTreeMap<String, TinyStr16> {
    let snapshot: JapaneseErasV1 = serde_json::from_str(JAPANESE_FILE)
        .expect("Failed to parse the precached snapshot-japanese@1.json. This is a bug.");
    let mut map: BTreeMap<_, _> = snapshot
        .dates_to_eras
        .iter()
        .enumerate()
        .map(|(i, (_, value))| (i.to_string(), value))
        .collect();
    // Splice in details about gregorian eras for pre-meiji dates
    map.insert("-2".to_string(), tinystr!(16, "bce"));
    map.insert("-1".to_string(), tinystr!(16, "ce"));
    map
}
