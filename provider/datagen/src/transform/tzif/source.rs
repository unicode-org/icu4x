// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::source::AbstractFs;
use icu_provider::DataError;
use std::collections::HashMap;
use std::fmt::Debug;
use std::path::Path;
use tzif::data::tzif::TzifData;

#[derive(Debug)]
pub(crate) struct TzifPaths(AbstractFs);

impl TzifPaths {
    const TZIF_HEADER_LEN: usize = 4;

    pub(crate) fn new<T: AsRef<Path>>(root: T) -> Result<Self, DataError> {
        AbstractFs::new(root).map(Self)
    }

    pub(crate) fn read_and_parse(&self) -> Result<HashMap<String, TzifData>, DataError> {
        self.0
            .list("", true)?
            // Filter only the paths that lead to files. We don't want to try to parse
            // TZif data from any other types of entries. If the metadata itself failed
            // to be gathered, we want to keep this entry to propagate the error.
            .filter(|path| self.0.is_file(path).unwrap_or(true))
            // Filter only files whose length in bytes is at least large enough to fit
            // the TZif file header. If the metadata itself failed to be gathered,
            // we want to keep this entry to propagate the error.
            .filter(|path| {
                self.0
                    .size(path)
                    .map(|size| size > Self::TZIF_HEADER_LEN as u64)
                    .unwrap_or(true)
            })
            // Read the first four bytes of each file to check if the sequence
            // matches the TZif file header. If the read itself failed, we want to
            // keep this entry to propagate the error.
            .filter(|path| {
                self.0
                    .read_to_buf_exact(Self::TZIF_HEADER_LEN, &path.to_string())
                    .map(|buf| tzif::is_tzif(&buf))
                    .unwrap_or(true)
            })
            .map(|path| -> Result<_, DataError> {
                let buf = self.0.read_to_buf(&path.to_string())?;
                let data = tzif::parse_tzif(&buf).map_err(|e| {
                    DataError::custom("TZif parse")
                        .with_display_context(&e)
                        .with_path_context(&path)
                })?;
                Ok((path, data))
            })
            .collect()
    }
}
