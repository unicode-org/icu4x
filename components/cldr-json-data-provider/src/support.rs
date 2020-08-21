use crate::CldrPaths;
use icu_data_provider::iter::DataEntryCollection;
use icu_data_provider::prelude::*;
use std::convert::TryFrom;
use std::sync::RwLock;

pub(crate) trait DataKeySupport {
    fn supports_key(data_key: &DataKey) -> Result<(), DataError>;
}

pub(crate) struct LazyCldrProvider<T> {
    src: RwLock<Option<T>>,
}

fn map_poison<E>(_err: E) -> DataError {
    // Can't return the PoisonError directly because it has lifetime parameters.
    DataError::new_resc_error(crate::error::Error::PoisonError)
}

/// A lazy-initialized CLDR JSON data provider.
impl<'b, 'd, T> LazyCldrProvider<T>
where
    T: DataProvider<'d> + DataKeySupport + DataEntryCollection + TryFrom<&'b CldrPaths>,
    <T as TryFrom<&'b CldrPaths>>::Error: 'static + std::error::Error,
{
    pub fn new() -> Self {
        Self {
            src: RwLock::new(None),
        }
    }

    /// Call T::load, initializing T if necessary.
    pub fn try_load(
        &self,
        req: &DataRequest,
        cldr_paths: &'b CldrPaths,
    ) -> Result<Option<DataResponse<'d>>, DataError> {
        if T::supports_key(&req.data_key).is_err() {
            return Ok(None);
        }
        if let Some(data_provider) = self.src.read().map_err(map_poison)?.as_ref() {
            return data_provider.load(req).map(Some);
        }
        let mut src = self.src.write().map_err(map_poison)?;
        if src.is_none() {
            src.replace(T::try_from(cldr_paths).map_err(DataError::new_resc_error)?);
        }
        // The RwLock is guaranteed to be populated at this point.
        if let Some(data_provider) = src.as_ref() {
            return data_provider.load(req).map(Some);
        }
        unreachable!();
    }

    /// Call T::iter_for_key, initializing T if necessary.
    pub fn try_iter(
        &self,
        data_key: &DataKey,
        cldr_paths: &'b CldrPaths,
    ) -> Result<Option<Box<dyn Iterator<Item = DataEntry>>>, DataError> {
        if T::supports_key(data_key).is_err() {
            return Ok(None);
        }
        if let Some(data_provider) = self.src.read().map_err(map_poison)?.as_ref() {
            return data_provider.iter_for_key(data_key).map(Some);
        }
        let mut src = self.src.write().map_err(map_poison)?;
        if src.is_none() {
            src.replace(T::try_from(cldr_paths).map_err(DataError::new_resc_error)?);
        }
        // The RwLock is guaranteed to be populated at this point.
        if let Some(data_provider) = src.as_ref() {
            return data_provider.iter_for_key(data_key).map(Some);
        }
        unreachable!();
    }
}
