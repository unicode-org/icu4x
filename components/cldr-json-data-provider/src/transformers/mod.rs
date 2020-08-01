mod plurals;

pub use plurals::CldrPluralsDataProvider;

use icu_data_provider::prelude::*;

pub struct CldrDataProvider<'d> {
    pub plurals: Option<CldrPluralsDataProvider<'d>>,
}

impl<'a, 'd> DataProvider<'a, 'd> for CldrDataProvider<'d> {
    fn load(
        &'a self,
        req: &data_provider::Request,
    ) -> Result<data_provider::Response<'d>, data_provider::Error> {
        // TODO: Fix up error handling in this function
        let mut common_err: Option<data_provider::Error> = None;
        if let Some(provider) = &self.plurals {
            match provider.load(req) {
                Ok(response) => return Ok(response),
                Err(err) => common_err.replace(err),
            };
        }
        if let Some(err) = common_err {
            return Err(err);
        }
        Err(data_provider::Error::UnsupportedDataKey(req.data_key))
    }
}
