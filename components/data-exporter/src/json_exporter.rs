use crate::Error;
use crate::FileWriter;

use std::fs;
use std::path::Path;
use std::path::PathBuf;

pub struct JsonFileWriter {
    pub root: PathBuf,
}

impl FileWriter for JsonFileWriter {
    fn write_to_path(
        &mut self,
        path_without_extension: &Path,
        obj: &dyn erased_serde::Serialize,
    ) -> Result<(), Error> {
        let mut path_buf: std::path::PathBuf = self.root.clone();
        path_buf.extend(path_without_extension);
        path_buf.set_extension("json");

        if let Some(parent_dir) = path_buf.parent() {
            fs::create_dir_all(&parent_dir)?;
        }

        let file = fs::File::create(&path_buf)?;
        let mut json = serde_json::Serializer::new(file);
        obj.erased_serialize(&mut erased_serde::Serializer::erase(&mut json))?;

        Ok(())
    }
}
