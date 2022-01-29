// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
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
    data: LiteMap<&'static str, LiteMap<LanguageIdentifier, cldr_serde::ca::Dates>>,
}

impl TryFrom<&dyn CldrPaths> for CommonDateProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        let mut data = LiteMap::new();

        // Raise an error if Gregorian paths are not available
        let _ = cldr_paths.cldr_dates("gregory")?;

        for (cldr_cal, bcp_cal, path) in cldr_paths.cldr_dates_all() {
            let mut cal_data = LiteMap::new();
            let path = path.join("main");
            let locale_dirs = get_langid_subdirectories(&path)?;

            let cal_file = format!("ca-{}.json", cldr_cal);
            for dir in locale_dirs {
                let path = dir.join(&cal_file);

                let resource: cldr_serde::ca::Resource =
                    serde_json::from_reader(open_reader(&path)?).map_err(|e| (e, path))?;
                for (k, mut v) in resource.main.0.into_tuple_vec().drain(..) {
                    let v = v.dates.calendars.remove(cldr_cal).ok_or_else(|| {
                        Error::Custom(
                            format!("{} does not have {} field", cal_file, cldr_cal),
                            None,
                        )
                    })?;
                    cal_data.insert(k, v);
                }
            }
            data.insert(bcp_cal, cal_data);
        }

        Ok(Self { data })
    }
}

impl CommonDateProvider {
    pub fn dates_for<'a, M: ResourceMarker>(
        &'a self,
        req: &DataRequest,
    ) -> Result<&'a cldr_serde::ca::Dates, DataError> {
        let langid = req.try_langid(M::KEY)?;
        let variant = req
            .options
            .variant
            .as_ref()
            .ok_or_else(|| DataErrorKind::NeedsVariant.with_req(M::KEY, req))?;
        let map = self
            .data
            .get(&**variant)
            .ok_or_else(|| DataErrorKind::MissingVariant.with_req(M::KEY, req))?;
        match map.get(langid) {
            Some(date) => Ok(date),
            None => Err(DataErrorKind::MissingLocale.with_req(M::KEY, req)),
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
