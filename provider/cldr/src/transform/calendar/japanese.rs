// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::open_reader;
use crate::CldrPaths;
use icu_locid_macros::langid;
use icu_provider::iter::{IterableDataProviderCore, KeyedDataProvider};
use icu_provider::prelude::*;
use icu_provider::resource_key;
use icu_provider::yoke::{self, *};
use litemap::LiteMap;
use std::convert::TryFrom;
use std::env;
use std::str::FromStr;
use tinystr::TinyStr16;

/// A [`ResourceKey`] to [`JapaneseErasV1`]
// TODO (#1116) move this into icu_calendars
pub const JAPANESE_ERAS_V1: ResourceKey = resource_key!(Calendar, "japanese", 1);

const JAPANESE_FILE: &'static str = include_str!("./snapshot-japanese@1.json");
/// All keys that this module is able to produce.
pub const ALL_KEYS: [ResourceKey; 1] = [JAPANESE_ERAS_V1];

/// The date at which an era started
///
/// The order of fields in this struct is important!
// TODO (#1116) move this into icu_calendars
#[derive(
    Copy,
    Clone,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
    Yokeable,
    ZeroCopyFrom,
)]
pub struct EraStartDate {
    year: i16,
    month: u8,
    day: u8,
}

#[icu_provider::data_struct]
#[derive(Debug, PartialEq, Clone, Default, serde::Serialize, serde::Deserialize)]
#[yoke(cloning_zcf)]
// TODO (#1116) move this into icu_calendars (and make it zero-copy)
pub struct JapaneseErasV1 {
    pub dates_to_historical_eras: LiteMap<EraStartDate, TinyStr16>,
    pub dates_to_eras: LiteMap<EraStartDate, TinyStr16>,
}

/// Common code for a data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug, Default)]
pub struct JapaneseErasProvider {
    // Maps of era start dates to their names
    // We store historical_data separately for performance
    historical_data: LiteMap<EraStartDate, TinyStr16>,
    data: LiteMap<EraStartDate, TinyStr16>,
}

impl TryFrom<&dyn CldrPaths> for JapaneseErasProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut historical_data = LiteMap::new();
        let mut data = LiteMap::new();

        let era_names_path = cldr_paths
            .cldr_dates("japanese")?
            .join("main")
            .join("en")
            .join("ca-japanese.json");
        let era_dates_path = cldr_paths
            .cldr_core()?
            .join("supplemental")
            .join("calendarData.json");

        let era_names: cldr_serde::ca::Resource =
            serde_json::from_reader(open_reader(&era_names_path)?)
                .map_err(|e| (e, era_names_path))?;
        let era_dates: cldr_serde::japanese::Resource =
            serde_json::from_reader(open_reader(&era_dates_path)?)
                .map_err(|e| (e, era_dates_path))?;

        let era_name_map = &era_names
            .main
            .0
            .get(&langid!("en"))
            .ok_or_else(|| {
                Error::Custom(
                    String::from("ca-japanese.json does not have en locale"),
                    None,
                )
            })?
            .dates
            .calendars
            .get("japanese")
            .ok_or_else(|| {
                Error::Custom(
                    String::from("ca-japanese.json does not contain 'japanese' calendar"),
                    None,
                )
            })?
            .eras
            .abbr;
        let era_dates_map = &era_dates.supplemental.calendar_data.japanese.eras;

        for (era_id, era_name) in era_name_map.iter() {
            // These don't exist but may in the future
            if era_id.contains("variant") {
                continue;
            }
            let date = &era_dates_map
                .get(era_id)
                .ok_or_else(|| {
                    Error::Custom(
                        format!(
                            "calendarData.json contains no data for japanese era index {}",
                            era_id
                        ),
                        None,
                    )
                })?
                .start;

            let start_date = EraStartDate::from_str(date).map_err(|_| {
                Error::Custom(
                    format!(
                        "calendarData.json contains unparseable data for japanese era index {}",
                        era_id
                    ),
                    None,
                )
            })?;

            let code =
                era_to_code(era_name, start_date.year).map_err(|e| Error::Custom(e, None))?;
            if start_date.year >= 1868 {
                data.insert(start_date, code);
            } else {
                historical_data.insert(start_date, code);
            }
        }

        let ret = Self {
            data,
            historical_data,
        };

        // Integrity check
        if env::var("ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK").is_err() {
            let snapshot: JapaneseErasV1 = serde_json::from_str(JAPANESE_FILE)
                .expect("Failed to parse the precached snapshot-japanese@1.json. This is a bug.");

            if snapshot.dates_to_historical_eras != ret.historical_data
                || snapshot.dates_to_eras != ret.data
            {
                return Err(Error::Custom("Era data has changed! This can be for two reasons: Either the CLDR locale data for Japanese eras has \
                                          changed in an incompatible way, or there is a new Japanese era. Please comment out the integrity \
                                          check in icu_provider_cldr's japanese.rs and inspect the update to japanese@1.json (resource key `calendar/japanese`) \
                                          in the generated JSON by rerunning the datagen tool (`cargo make testdata` in the ICU4X repo). \
                                          Rerun with ICU4X_SKIP_JAPANESE_INTEGRITY_CHECK=1 to regenerate testdata properly, and check which situation \
                                          it is. If a new era has been introduced, copy over the new testdata to snapshot-japanese@1.json \
                                          in icu_provider_cldr. If not, it's likely that japanese.rs in icu_provider_cldr will need \
                                          to be updated to handle the data changes.".to_owned(), None));
            }
        }

        Ok(ret)
    }
}

/// See https://docs.google.com/document/d/1vMVhMHgCYRyx2gmwEfKRyXWDg_lrQadd8iMVU9uPK1o/edit?usp=chrome_omnibox&ouid=111665445991279316689
/// for the era identifier spec
fn era_to_code(original: &str, year: i16) -> Result<TinyStr16, String> {
    // Some examples of CLDR era names:
    // "Shōryaku (1077–1081)", "Nin’an (1166–1169)", "Tenpyō-kampō (749–749)"
    //
    // We want to produce a unique readable era code that
    // contains ascii lowercase letters, followed
    // by a hyphen and then a year name (except for post-Meiji era codes)
    //
    // We also want it to fit in a tinystr16. What we will do is:
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
        .replace(&['ō', 'Ō'][..], "o")
        .replace(&['ū', 'Ū'][..], "u")
        .to_lowercase()
        .replace(&['-', '\'', '’'][..], "");
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

impl FromStr for EraStartDate {
    type Err = ();
    fn from_str(mut s: &str) -> Result<Self, ()> {
        let mut sign = 1;
        if s.starts_with('-') {
            s = &s[1..];
            sign = -1;
        }

        let mut split = s.split('-');
        let mut year: i16 = split.next().ok_or(())?.parse().map_err(|_| ())?;
        year *= sign;
        let month: u8 = split.next().ok_or(())?.parse().map_err(|_| ())?;
        let day: u8 = split.next().ok_or(())?.parse().map_err(|_| ())?;

        Ok(EraStartDate { year, month, day })
    }
}

impl KeyedDataProvider for JapaneseErasProvider {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        JAPANESE_ERAS_V1.match_key(*resc_key)
    }
}

impl DataProvider<JapaneseErasV1Marker> for JapaneseErasProvider {
    fn load_payload(
        &self,
        req: &DataRequest,
    ) -> Result<DataResponse<JapaneseErasV1Marker>, DataError> {
        if req.resource_path.options.langid.is_some() || req.resource_path.options.variant.is_some()
        {
            return Err(DataError::ExtraneousVariantOrId(req.clone()));
        }
        Ok(DataResponse {
            metadata: DataResponseMetadata { data_langid: None },
            payload: Some(DataPayload::from_owned(JapaneseErasV1 {
                dates_to_eras: self.data.clone(),
                dates_to_historical_eras: self.historical_data.clone(),
            })),
        })
    }
}

icu_provider::impl_dyn_provider!(JapaneseErasProvider, {
    _ => JapaneseErasV1Marker,
}, SERDE_SE);

impl IterableDataProviderCore for JapaneseErasProvider {
    fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        Ok(Box::new(
            Some(ResourceOptions {
                variant: None,
                langid: None,
            })
            .into_iter(),
        ))
    }
}
