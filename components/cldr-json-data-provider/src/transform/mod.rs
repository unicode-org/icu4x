mod dates;
mod plurals;

pub use dates::DatesProvider;
pub use plurals::PluralsProvider;

use crate::support::LazyCldrProvider;
use crate::CldrPaths;
use icu_data_provider::iter::DataEntryCollection;
use icu_data_provider::prelude::*;

pub struct CldrJsonDataProvider<'a, 'd> {
    pub cldr_paths: &'a CldrPaths,
    plurals: LazyCldrProvider<PluralsProvider<'d>>,
    dates: LazyCldrProvider<DatesProvider<'d>>,
}

impl<'a, 'd> CldrJsonDataProvider<'a, 'd> {
    pub fn new(cldr_paths: &'a CldrPaths) -> Self {
        CldrJsonDataProvider {
            cldr_paths,
            plurals: Default::default(),
            dates: Default::default(),
        }
    }
}

impl<'a, 'd> DataProvider<'d> for CldrJsonDataProvider<'a, 'd> {
    fn load(&self, req: &DataRequest) -> Result<DataResponse<'d>, DataError> {
        if let Some(result) = self.plurals.try_load(req, &self.cldr_paths)? {
            return Ok(result);
        }
        if let Some(result) = self.dates.try_load(req, &self.cldr_paths)? {
            return Ok(result);
        }
        Err(DataError::UnsupportedDataKey(req.data_key))
    }
}

impl<'a, 'd> DataEntryCollection for CldrJsonDataProvider<'a, 'd> {
    fn iter_for_key(
        &self,
        data_key: &DataKey,
    ) -> Result<Box<dyn Iterator<Item = DataEntry>>, DataError> {
        if let Some(resp) = self.plurals.try_iter(data_key, &self.cldr_paths)? {
            return Ok(resp);
        }
        Err(DataError::UnsupportedDataKey(*data_key))
    }
}
