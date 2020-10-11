// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::Error;
use crate::manifest::Manifest;
use crate::manifest::MANIFEST_FILE;
use icu_data_provider::prelude::*;
use icu_data_provider::structs;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

/// A data provider that reads ICU4X data from a filesystem directory.
#[derive(Debug, PartialEq)]
pub struct FsDataProvider {
    res_root: PathBuf,
    manifest: Manifest,
}

impl FsDataProvider {
    /// Create a new FsDataProvider given a filesystem directory.
    pub fn try_new<T: Into<PathBuf>>(root: T) -> Result<Self, Error> {
        let root_path_buf: PathBuf = root.into();
        let manifest_path = root_path_buf.join(MANIFEST_FILE);
        let manifest_str = fs::read_to_string(&manifest_path).map_err(|e| (e, &manifest_path))?;
        let manifest: Manifest =
            serde_json::from_str(&manifest_str).map_err(|e| (e, &manifest_path))?;
        Ok(Self {
            res_root: root_path_buf,
            manifest,
        })
    }
}

impl DataProvider<'_> for FsDataProvider {
    fn load(&self, req: &DataRequest) -> Result<DataResponse<'static>, DataError> {
        type Error = DataError;
        let mut path_buf = self.res_root.clone();
        path_buf.extend(req.data_key.get_components().iter());
        if !path_buf.exists() {
            path_buf.pop();
            if !path_buf.exists() {
                return Err(Error::UnsupportedCategory(req.data_key.category));
            } else {
                return Err(Error::UnsupportedDataKey(req.data_key));
            }
        }
        // TODO: Implement proper locale fallback
        path_buf.extend(req.data_entry.get_components().iter());
        path_buf.set_extension(self.manifest.syntax.get_file_extension());
        if !path_buf.exists() {
            return Err(Error::UnavailableEntry(req.clone()));
        }
        let file = match File::open(&path_buf) {
            Ok(file) => file,
            Err(err) => return Err(Error::ResourceError(Box::new(err))),
        };
        let reader = BufReader::new(file);
        // TODO: Eliminate this dispatch.
        // https://github.com/unicode-org/icu4x/issues/196
        if req.data_key.category == DataCategory::Plurals {
            // TODO: Pick deserializer based on manifest
            let obj: structs::plurals::PluralRuleStringsV1 = match serde_json::from_reader(reader) {
                Ok(obj) => obj,
                Err(err) => return Err(Error::ResourceError(Box::new(err))),
            };
            let response = DataResponseBuilder {
                // TODO: Return the actual locale when fallbacks are implemented.
                data_langid: req.data_entry.langid.clone(),
            }
            .with_owned_payload(obj);
            Ok(response)
        } else if req.data_key.category == DataCategory::Dates {
            // TODO: Pick deserializer based on manifest
            let obj: structs::dates::gregory::DatesV1 = match serde_json::from_reader(reader) {
                Ok(obj) => obj,
                Err(err) => return Err(Error::ResourceError(Box::new(err))),
            };
            let response = DataResponseBuilder {
                // TODO: Return the actual locale when fallbacks are implemented.
                data_langid: req.data_entry.langid.clone(),
            }
            .with_owned_payload(obj);
            Ok(response)
        } else {
            panic!("Don't know how to parse this data key, but it is on the filesystem");
        }
    }
}
