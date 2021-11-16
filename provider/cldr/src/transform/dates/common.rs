// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::cldr_json;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, open_reader};
use crate::CldrPaths;

use icu_locid::LanguageIdentifier;

use icu_provider::prelude::*;
use litemap::LiteMap;
use std::convert::TryFrom;

/// Common code for a data provider reading from CLDR JSON dates files.
#[derive(PartialEq, Debug)]
pub struct CommonDateProvider {
    // map of calendars to their locale-data mappings
    data: LiteMap<&'static str, LiteMap<LanguageIdentifier, cldr_json::LangDates>>,
}

impl TryFrom<&dyn CldrPaths> for CommonDateProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = LiteMap::new();

        // Raise an error if Gregorian paths are not available
        let _ = cldr_paths.cldr_dates("gregorian")?;

        for (calendar, path) in cldr_paths.cldr_dates_all() {
            let mut cal_data = LiteMap::new();
            let path = path.join("main");
            let locale_dirs = get_langid_subdirectories(&path)?;

            let cal_file = format!("ca-{}.json", calendar);
            for dir in locale_dirs {
                let path = dir.join(&cal_file);

                let mut resource: cldr_json::Resource =
                    serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
                for (k, v) in resource.main.0.drain(..) {
                    cal_data.insert(k, v);
                }
            }
            data.insert(calendar, cal_data);
        }

        Ok(Self { data })
    }
}

impl CommonDateProvider {
    pub fn dates_for<'a>(&'a self, req: &DataRequest) -> Result<&'a cldr_json::Dates, DataError> {
        let langid = req.try_langid()?;
        let variant = req
            .resource_path
            .options
            .variant
            .as_ref()
            .ok_or_else(|| DataError::MissingResourceOptions(req.clone()))?;
        let map = self
            .data
            .get(&**variant)
            .ok_or_else(|| DataError::MissingResourceOptions(req.clone()))?;
        match map.get(langid) {
            Some(date) => Ok(&date.dates),
            None => Err(DataError::MissingResourceOptions(req.clone())),
        }
    }
}

impl CommonDateProvider {
    #[allow(clippy::needless_collect)] // https://github.com/rust-lang/rust-clippy/issues/7526
    pub fn supported_options_for_key(
        &self,
        _resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        let list: Vec<ResourceOptions> = self
            .data
            .iter()
            .flat_map(|(cal, map)| {
                map.iter().map(move |(l, _)| ResourceOptions {
                    variant: Some((*cal).into()),
                    // TODO: Avoid the clone
                    langid: Some(l.clone()),
                })
            })
            .collect();
        Ok(Box::new(list.into_iter()))
    }
}
