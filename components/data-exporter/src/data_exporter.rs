// use crate::better_vfs::BetterVFS;
use erased_serde::Serialize;
use icu_data_provider::*;
use std::io;
use vfs::VFS;
use vfs::VPath;
use std::path::PathBuf;
use std::borrow::Borrow;
use std::io::Write;

pub struct DataExporter<'a, 'd, V: VFS> {
    // pub filesystem: &'a mut dyn BetterVFS,
    pub filesystem: &'a mut V,
    pub file_extension: &'a str,
    pub data_provider: &'a dyn Combined<'a, 'd>,
    pub serialize_fn: fn(writer: &mut dyn Write, obj: &dyn Serialize) -> Result<(), Error>,
}

#[derive(Debug)]
pub enum Error {
    ResponseError(ResponseError),
    PayloadError(PayloadError),
    SerializerError(erased_serde::Error),
    IoError(std::io::Error),
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

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::IoError(err)
    }
}

impl<'a, 'd, V: VFS> DataExporter<'a, 'd, V> {
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
            let path_buf = self.path_for(data_key, &data_entry);
            let path_fs = self.filesystem.path(path_buf.to_str().unwrap());
            let mut vfile = path_fs.create()?;
            (self.serialize_fn)(&mut vfile, payload)?;
        }
        Ok(())
    }

    fn path_for(&mut self, data_key: &DataKey, data_entry: &DataEntry) -> PathBuf {
        let mut path = PathBuf::new();
        self.filesystem.path(path.to_str().unwrap()).mkdir().unwrap();
        path.push(data_key.category.to_string());
        self.filesystem.path(path.to_str().unwrap()).mkdir().unwrap();
        path.push(format!("{}@{}", data_key.sub_category.as_str(), data_key.version));
        self.filesystem.path(path.to_str().unwrap()).mkdir().unwrap();
        if let Some(variant) = &data_entry.variant {
            let variant_str: &str = variant.borrow();
            path.push(variant_str);
            self.filesystem.path(path.to_str().unwrap()).mkdir().unwrap();
        }
        path.push(format!("{}", data_entry.langid));
        path.set_extension(self.file_extension);
        path
    }
}
