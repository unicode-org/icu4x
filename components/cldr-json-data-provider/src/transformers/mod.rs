mod plurals;

pub use plurals::CldrPluralsDataProvider;

use crate::CldrPaths;
use icu_data_provider::data_provider::DataProvider;
use icu_data_provider::prelude::*;
use std::convert::TryFrom;
use std::sync::RwLock;

trait DataKeySupport {
    fn supports_key(data_key: &DataKey) -> Result<(), data_provider::Error>;
}

struct Foo<T> {
    data_provider: RwLock<Option<T>>,
}

impl<'a, 'b, 'd, T> Foo<T>
where
    T: DataProvider<'a, 'd> + DataKeySupport + TryFrom<&'b CldrPaths>,
    <T as TryFrom<&'b CldrPaths>>::Error: 'static + std::error::Error,
{
    pub fn try_load(
        &'a self,
        req: &data_provider::Request,
        cldr_paths: &'b CldrPaths,
    ) -> Result<Option<data_provider::Response<'d>>, data_provider::Error> {
        if T::supports_key(&req.data_key).is_err() {
            return Ok(None);
        }
        if self.data_provider.read().unwrap().is_none() {
            self.data_provider.write().unwrap().replace(
                T::try_from(cldr_paths)
                    .map_err(|e| data_provider::Error::ResourceError(Box::new(e)))?,
            );
        };
        return self.data_provider.read().unwrap().as_ref().unwrap().load(req).map(|r| Some(r));
    }
}

pub struct CldrDataProvider<'d> {
    pub cldr_paths: CldrPaths,
    plurals: Option<CldrPluralsDataProvider<'d>>,
    plurals1: Foo<CldrPluralsDataProvider<'d>>,
}

impl<'a, 'd> DataProvider<'a, 'd> for CldrDataProvider<'d> {
    fn load(
        &'a self,
        req: &data_provider::Request,
    ) -> Result<data_provider::Response<'d>, data_provider::Error> {
        // if let Some(provider) = &self.plurals {
        //     let provider_trait_object: &dyn DataProvider = provider;
        //     if let Some(response) = provider_trait_object.load_graceful(req)? {
        //         return Ok(response);
        //     }
        // }
        if let Some(response) = self.plurals1.try_load(req, &self.cldr_paths)? {
            return Ok(response);
        }
        Err(data_provider::Error::UnsupportedDataKey(req.data_key))
    }
}
