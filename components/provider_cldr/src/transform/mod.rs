// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
mod dates;
mod plurals;

pub use dates::DatesProvider;
pub use plurals::PluralsProvider;

use crate::support::LazyCldrProvider;
use crate::CldrPaths;
use icu_provider::prelude::*;

/// Returns a list of all ResourceKeys that this provider can produce.
pub fn get_all_resc_keys() -> Vec<ResourceKey> {
    let mut result: Vec<ResourceKey> = vec![];
    result.extend(&plurals::ALL_KEYS);
    result.extend(&dates::ALL_KEYS);
    result
}

#[derive(Debug)]
pub struct CldrJsonDataProvider<'a, 'd> {
    pub cldr_paths: &'a dyn CldrPaths,
    plurals: LazyCldrProvider<PluralsProvider<'d>>,
    dates: LazyCldrProvider<DatesProvider<'d>>,
}

impl<'a, 'd> CldrJsonDataProvider<'a, 'd> {
    pub fn new(cldr_paths: &'a dyn CldrPaths) -> Self {
        CldrJsonDataProvider {
            cldr_paths,
            plurals: Default::default(),
            dates: Default::default(),
        }
    }
}

impl<'a, 'd> ErasedDataProvider<'d> for CldrJsonDataProvider<'a, 'd> {
    fn load_to_receiver(
        &self,
        req: &DataRequest,
        receiver: &mut dyn DataReceiver<'d, 'static>,
    ) -> Result<DataResponse, DataError> {
        if let Some(result) = self.plurals.try_load(req, receiver, self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.dates.try_load(req, receiver, self.cldr_paths)? {
            return Ok(result);
        }
        Err(DataError::UnsupportedResourceKey(req.resource_path.key))
    }
}

impl<'a, 'd> IterableDataProvider<'d> for CldrJsonDataProvider<'a, 'd> {
    fn supported_options_for_key(
        &self,
        resc_key: &ResourceKey,
    ) -> Result<Box<dyn Iterator<Item = ResourceOptions>>, DataError> {
        if let Some(resp) = self
            .plurals
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(resp);
        }
        if let Some(resp) = self
            .dates
            .try_supported_options(resc_key, self.cldr_paths)?
        {
            return Ok(resp);
        }
        Err(DataError::UnsupportedResourceKey(*resc_key))
    }
}

impl<'a, 'd> KeyedDataProvider for CldrJsonDataProvider<'a, 'd> {
    fn supports_key(resc_key: &ResourceKey) -> Result<(), DataError> {
        PluralsProvider::supports_key(resc_key)
            .or_else(|err| DatesProvider::or_else_supports_key(err, resc_key))
    }
}
