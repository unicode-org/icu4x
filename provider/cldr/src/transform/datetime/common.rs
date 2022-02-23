// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::cldr_serde;
use crate::error::Error;
use crate::reader::{get_langid_subdirectories, get_langid_subdirectory, open_reader};
use crate::CldrPaths;

use icu_provider::prelude::*;
use std::convert::TryFrom;
use std::path::PathBuf;

/// Common code for a data provider reading from CLDR JSON dates files.
#[derive(Debug)]
pub struct CommonDateProvider {
    paths: Vec<(&'static str, &'static str, PathBuf)>,
}

impl TryFrom<&dyn CldrPaths> for CommonDateProvider {
    type Error = Error;
    fn try_from(cldr_paths: &dyn CldrPaths) -> Result<Self, Self::Error> {
        Ok(Self {
            paths: cldr_paths.cldr_dates_all(),
        })
    }
}

impl CommonDateProvider {
    pub fn dates_for<M: ResourceMarker>(
        &self,
        req: &DataRequest,
    ) -> Result<cldr_serde::ca::Dates, DataError> {
        let langid = req
            .get_langid()
            .ok_or_else(|| DataErrorKind::NeedsLocale.with_req(M::KEY, req))?;
        let variant = req
            .options
            .variant
            .as_ref()
            .ok_or_else(|| DataErrorKind::NeedsVariant.with_req(M::KEY, req))?;

        let (cldr_cal, _, path) = self
            .paths
            .iter()
            .find(|(_, bcp_cal, _)| bcp_cal == &&**variant)
            .ok_or_else(|| DataErrorKind::MissingVariant.with_req(M::KEY, req))?;

        let locale_dir = get_langid_subdirectory(&path.join("main"), langid)?
            .ok_or_else(|| DataErrorKind::MissingLocale.with_req(M::KEY, req))?;

        let cal_file = format!("ca-{}.json", cldr_cal);
        let path = locale_dir.join(&cal_file);

        let mut resource: cldr_serde::ca::Resource =
            serde_json::from_reader(open_reader(&path)?).map_err(|e| Error::Json(e, Some(path)))?;

        Ok(resource
            .main
            .0
            .remove(langid)
            .expect("CLDR file contains the expected language")
            .dates
            .calendars
            .remove(*cldr_cal)
            .ok_or_else(|| {
                Error::Custom(
                    format!("{} does not have {} field", cal_file, cldr_cal),
                    None,
                )
            })?)
    }
}

impl CommonDateProvider {
    pub fn supported_options(
        &self,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions> + '_>, DataError> {
        // Raise an error if Gregorian paths are not available
        self.paths
            .iter()
            .find(|(_, cal, _)| cal == &"gregory")
            .ok_or(Error::MissingSource(crate::error::MissingSourceError {
                src: "cldr-dates/gregory",
            }))?;

        let mut r = Vec::new();
        for (_, cal, path) in &self.paths {
            let cal = Some((*cal).into());
            r.extend(
                get_langid_subdirectories(&path.join("main"))?
                    .map(Into::<ResourceOptions>::into)
                    .map(move |mut r| {
                        r.variant = cal.clone();
                        r
                    }),
            );
        }
        Ok(Box::new(r.into_iter()))
    }
}
