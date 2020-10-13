// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use crate::error::Error;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::io::Write;
use std::path::PathBuf;

#[cfg(target_family = "unix")]
use std::os::unix::fs::symlink as symlink_file;

#[cfg(target_family = "windows")]
use std::os::windows::fs::symlink_file;

pub(crate) struct Options<'a> {
    pub root: PathBuf,
    pub symlink_file_extension: &'a str,
    pub data_file_prefix: &'a str,
    pub data_file_extension: &'a str,
}

pub(crate) struct AliasCollection<T> {
    pub root: PathBuf,
    symlink_file_extension: String,
    data_file_prefix: String,
    data_file_extension: String,
    map: HashMap<T, Vec<PathBuf>>,
    flushed: bool,
}

impl<T: fmt::Debug + Eq + Hash + Ord + AsRef<[u8]>> AliasCollection<T> {
    pub fn new(options: Options) -> Self {
        AliasCollection {
            root: options.root,
            symlink_file_extension: options.symlink_file_extension.to_owned(),
            data_file_prefix: options.data_file_prefix.to_owned(),
            data_file_extension: options.data_file_extension.to_owned(),
            map: HashMap::new(),
            flushed: false,
        }
    }

    pub fn put(&mut self, mut path_buf: PathBuf, data_item: T) {
        path_buf.set_extension(&self.symlink_file_extension);
        self.map
            .entry(data_item)
            .or_insert_with(Vec::new)
            .push(path_buf);
    }

    pub fn flush(&mut self) -> Result<(), Error> {
        self.flushed = true;
        // TODO: Make sure the directory is empty
        fs::create_dir_all(&self.root).map_err(|e| (e, &self.root))?;
        let mut unique_data_items: Vec<(&T, &Vec<PathBuf>)> = self.map.iter().collect();
        unique_data_items.sort(); // guarantee canonical order
        for (i, (data_item, link_paths)) in unique_data_items.iter().enumerate() {
            let mut data_filename = PathBuf::from(format!("{}{}", self.data_file_prefix, i));
            data_filename.set_extension(&self.data_file_extension);
            let mut data_path = self.root.clone();
            data_path.extend(&data_filename);
            let mut data_file = fs::File::create(&data_path).map_err(|e| (e, &data_path))?;
            data_file
                .write_all(data_item.as_ref())
                .map_err(|e| (e, &data_path))?;
            for link_path in link_paths.iter() {
                symlink_file(&data_filename, link_path).map_err(|e| (e, link_path))?;
            }
        }
        Ok(())
    }
}

impl<T> Drop for AliasCollection<T> {
    fn drop(&mut self) {
        if !self.flushed {
            panic!("Please call flush before dropping AliasCollection");
        }
    }
}
