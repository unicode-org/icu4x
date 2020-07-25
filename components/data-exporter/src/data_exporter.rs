// use crate::better_vfs::BetterVFS;
use erased_serde::Serialize;
use icu_data_provider::*;
use std::io;

pub struct DataExporter<'a, 'd> {
    // pub filesystem: &'a mut dyn BetterVFS,
    pub data_provider: &'a dyn Combined<'a, 'd>,
    pub serialize_fn: fn(writer: io::Stdout, obj: &dyn Serialize) -> Result<(), Error>,
}

#[derive(Debug)]
pub enum Error {
    ResponseError(ResponseError),
    PayloadError(PayloadError),
    SerializerError(erased_serde::Error),
}

impl From<ResponseError> for Error {
    fn from(err: ResponseError) -> Error {
        Error::ResponseError(err)
    }
}

impl From<PayloadError> for Error {
    fn from(err: PayloadError) -> Error {
        Error::PayloadError(err)
    }
}

impl From<erased_serde::Error> for Error {
    fn from(err: erased_serde::Error) -> Error {
        Error::SerializerError(err)
    }
}

impl<'a, 'd> DataExporter<'a, 'd> {
    pub fn write_data_key<T: 'static + Serialize>(
        &mut self,
        data_key: &DataKey,
    ) -> Result<(), Error> {
        for data_entry in self.data_provider.iter_for_key(data_key)? {
            let response = self.data_provider.load(&Request {
                data_key: *data_key,
                data_entry: data_entry.clone(),
            })?;
            let payload = response.borrow_payload::<T>()?;
            (self.serialize_fn)(io::stdout(), payload)?;
        }
        Ok(())
    }
}
