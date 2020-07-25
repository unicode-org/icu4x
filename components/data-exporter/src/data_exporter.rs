use erased_serde::Serialize;
use icu_data_provider::*;
use std::borrow::Borrow;
use std::path::Path;
use std::path::PathBuf;

use crate::Error;

pub trait FileWriter {
    fn write_to_path(
        &mut self,
        path_without_extension: &Path,
        obj: &dyn Serialize,
    ) -> Result<(), Error>;
}

pub struct DataExporter<'a, 'd> {
    pub data_provider: &'a dyn Combined<'a, 'd>,
    pub file_writer: &'a mut dyn FileWriter,
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
            let path_buf = self.path_for(data_key, &data_entry);
            self.file_writer.write_to_path(&path_buf, payload)?;
        }
        Ok(())
    }

    fn path_for(&mut self, data_key: &DataKey, data_entry: &DataEntry) -> PathBuf {
        let mut path = PathBuf::new();
        path.push(data_key.category.to_string());
        path.push(format!(
            "{}@{}",
            data_key.sub_category.as_str(),
            data_key.version
        ));
        if let Some(variant) = &data_entry.variant {
            let variant_str: &str = variant.borrow();
            path.push(variant_str);
        }
        path.push(format!("{}", data_entry.langid));
        path
    }
}
