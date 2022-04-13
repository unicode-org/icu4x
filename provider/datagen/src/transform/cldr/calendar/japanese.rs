// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::error::DatagenError;
use crate::transform::cldr::reader::open_reader;
use crate::transform::cldr::serde;
use crate::SourceData;
use icu_calendar::provider::*;
use icu_locid::langid;
use icu_provider::datagen::IterableResourceProvider;
use icu_provider::prelude::*;
use std::collections::BTreeMap;
use std::env;
use std::str::FromStr;
use tinystr::TinyStr16;
use zerovec::ule::AsULE;
use zerovec::ZeroVec;

const JAPANESE_FILE: &str = include_str!("./snapshot-japanese@1.json");

/// A data provider reading from CLDR JSON Japanese calendar files.
#[derive(Debug)]
pub struct JapaneseErasProvider {
    source: SourceData,
}

impl From<&SourceData> for JapaneseErasProvider {
    fn from(source: &SourceData) -> Self {
        Self {
            source: source.clone(),
        }
    }
}

impl ResourceProvider<JapaneseErasV1Marker> for JapaneseErasProvider {
    fn load_resource(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<JapaneseErasV1Marker>, DataError> {
        if !req.options.is_empty() {
            return Err(DataErrorKind::ExtraneousResourceOptions.into_error());
        }

        // The era codes depend on the Latin romanizations of the eras, found
        // in the `en` locale. We load this data to construct era codes but
        // actual user code only needs to load the data for the locales it cares about.
        let era_names_path = self
            .source
            .get_cldr_paths()?
            .cldr_dates("japanese")
            .join("main")
            .join("en")
            .join("ca-japanese.json");
        let era_dates_path = self
            .source
            .get_cldr_paths()?
            .cldr_core()
            .join("supplemental")
            .join("calendarData.json");

        let era_names: serde::ca::Resource = serde_json::from_reader(open_reader(&era_names_path)?)
            .map_err(|e| DatagenError::from((e, era_names_path)))?;
        let era_dates: serde::japanese::Resource =
            serde_json::from_reader(open_reader(&era_dates_path)?)
                .map_err(|e| DatagenError::from((e, era_dates_path)))?;

        let era_name_map = &era_names
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
        let era_dates_map = &era_dates.supplemental.calendar_data.japanese.eras;

        let mut ret = JapaneseErasV1 {
            dates_to_eras: ZeroVec::new(),
            dates_to_historical_eras: ZeroVec::new(),
        };

        for (era_id, era_name) in era_name_map.iter() {
            // These don't exist but may in the future
            if era_id.contains("variant") {
                continue;
            }
            let date = &era_dates_map
                .get(era_id)
                .ok_or_else(|| {
                    DatagenError::Custom(
                        format!(
                            "calendarData.json contains no data for japanese era index {}",
                            era_id
                        ),
                        None,
                    )
                })?
                .start;

            let start_date = EraStartDate::from_str(date).map_err(|_| {
                DatagenError::Custom(
                    format!(
                        "calendarData.json contains unparseable data for japanese era index {}",
                        era_id
                    ),
                    None,
                )
            })?;

            let code = era_to_code(era_name, start_date.year)
                .map_err(|e| DatagenError::Custom(e, None))?;
            if start_date.year >= 1868 {
                ret.dates_to_eras
                    .to_mut()
                    .push((start_date, code).to_unaligned());
            } else {
                ret.dates_to_historical_eras
                    .to_mut()
                    .push((start_date, code).to_unaligned());
            }
        }

        ret.dates_to_eras.to_mut().sort_unstable();
        ret.dates_to_historical_eras.to_mut().sort_unstable();

        // Integrity check
        //
        // Era code generation relies on the English era data which could in theory change; we have an integrity check
        // to catch such cases. It is relatively rare for a new era to be added, and in those cases the integrity check can
        // be disabled when generating new data.
        if env::var("ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK").is_err() {
            #[allow(clippy::expect_used)] // TODO(#1668) Clippy exceptions need docs or fixing.
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
        .ok_or_else(|| format!("Era name {} doesn't contain any text", original))?;
    let name = name
        .replace(&['ō', 'Ō'], "o")
        .replace(&['ū', 'Ū'], "u")
        .replace(&['-', '\'', '’'], "")
        .to_lowercase();
    if !name.is_ascii() {
        return Err(format!(
            "Era name {} (parsed from {}) contains non-ascii characters",
            name, original
        ));
    }

    let code = if year >= 1868 {
        name
    } else {
        format!("{}-{}", name, year)
    };

    // In case of future or past eras that do not fit, we may need to introduce a truncation scheme
    let code = code.parse().map_err(|e| {
        format!(
            "Era code {} (parsed from {}) does not fit into a TinyStr16: {}",
            code, original, e
        )
    })?;
    Ok(code)
}

icu_provider::impl_dyn_provider!(
    JapaneseErasProvider,
    [JapaneseErasV1Marker,],
    SERDE_SE,
    ITERABLE_SERDE_SE,
    DATA_CONVERTER
);

impl IterableResourceProvider<JapaneseErasV1Marker> for JapaneseErasProvider {
    fn supported_options(&self) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(core::iter::once(ResourceOptions::default())))
    }
}

pub fn get_era_code_map() -> BTreeMap<String, TinyStr16> {
    let snapshot: JapaneseErasV1 = serde_json::from_str(JAPANESE_FILE)
        .expect("Failed to parse the precached snapshot-japanese@1.json. This is a bug.");
    snapshot
        .dates_to_historical_eras
        .iter()
        .chain(snapshot.dates_to_eras.iter())
        .enumerate()
        .map(|(i, (_, value))| (i.to_string(), value))
        .collect()
}
