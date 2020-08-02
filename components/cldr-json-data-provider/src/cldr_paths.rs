use crate::error::MissingSourceError;
use crate::Error;
use std::default::Default;
use std::path::PathBuf;

pub struct CldrPaths {
    pub cldr_core: Result<PathBuf, MissingSourceError>,
}

impl Default for CldrPaths {
    fn default() -> CldrPaths {
        CldrPaths {
            cldr_core: Err(MissingSourceError { src: "cldr-core" }),
        }
    }
}

impl CldrPaths {
    pub fn plurals_json(&self) -> Result<PathBuf, Error> {
        self.cldr_core
            .clone()
            .map(|p| p.join("supplemental").join("plurals.json"))
            .map_err(|e| (&e).into())
    }

    pub fn ordinals_json(&self) -> Result<PathBuf, Error> {
        self.cldr_core
            .clone()
            .map(|p| p.join("supplemental").join("ordinals.json"))
            .map_err(|e| (&e).into())
    }
}
