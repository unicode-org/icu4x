// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use super::serializers::AbstractSerializer;
use crate::manifest::Manifest;
use icu_provider::datagen::*;
use icu_provider::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fs;
#[allow(deprecated)]
// We're using SipHash, which is deprecated, but we want a stable hasher
// (we're fine with it not being cryptographically secure since we're just using it to track diffs)
use std::hash::{Hasher, SipHasher};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

/// Choices of what to do if [`FilesystemExporter`] tries to write to a pre-existing directory.
#[non_exhaustive]
#[derive(Copy, Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum OverwriteOption {
    /// If the directory doesn't exist, create it.
    /// If it does exist, remove it safely (rmdir) and re-create it.
    CheckEmpty,
    /// If the directory doesn't exist, create it.
    /// If it does exist, remove it aggressively (rm -rf) and re-create it.
    RemoveAndReplace,
}

impl Default for OverwriteOption {
    fn default() -> Self {
        Self::CheckEmpty
    }
}

/// Options bag for initializing a [`FilesystemExporter`].
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExporterOptions {
    /// Directory in the filesystem to write output.
    pub root: PathBuf,
    /// Option for initializing the output directory.
    pub overwrite: OverwriteOption,
    #[deprecated(
        since = "1.3.0",
        note = "this feature was mainly intended for internal use and will be removed in a future version"
    )]
    /// Whether to create a fingerprint file with hashes
    pub fingerprint: bool,
}

impl Default for ExporterOptions {
    fn default() -> Self {
        #[allow(deprecated)] // obviously
        Self {
            root: PathBuf::from("icu4x_data"),
            overwrite: Default::default(),
            fingerprint: false,
        }
    }
}

impl From<PathBuf> for ExporterOptions {
    fn from(root: PathBuf) -> Self {
        ExporterOptions {
            root,
            ..Default::default()
        }
    }
}

/// A data exporter that writes data to a filesystem hierarchy.
/// See the module-level docs for an example.
#[derive(Debug)]
pub struct FilesystemExporter {
    root: PathBuf,
    manifest: Manifest,
    serializer: Box<dyn AbstractSerializer + Sync>,
    fingerprints: Option<Mutex<Vec<String>>>,
}

impl FilesystemExporter {
    /// Creates a new [`FilesystemExporter`] with a [serializer] and [options].
    ///
    /// See the module-level docs for an example.
    ///
    /// [serializer]: crate::export::serializers
    /// [options]: ExporterOptions
    pub fn try_new(
        serializer: Box<dyn AbstractSerializer + Sync>,
        options: ExporterOptions,
    ) -> Result<Self, DataError> {
        #[allow(deprecated)] // obviously
        let result = FilesystemExporter {
            root: options.root,
            manifest: Manifest::for_format(serializer.get_buffer_format())?,
            serializer,
            fingerprints: if options.fingerprint {
                Some(Mutex::new(vec![]))
            } else {
                None
            },
        };

        match options.overwrite {
            OverwriteOption::CheckEmpty if result.root.exists() => fs::remove_dir(&result.root),
            OverwriteOption::RemoveAndReplace if result.root.exists() => {
                fs::remove_dir_all(&result.root)
            }
            _ => Ok(()),
        }
        .and_then(|_| fs::create_dir_all(&result.root))
        .map_err(|e| DataError::from(e).with_path_context(&result.root))?;

        result.manifest.write(&result.root)?;
        Ok(result)
    }
}

impl DataExporter for FilesystemExporter {
    fn put_payload(
        &self,
        key: DataKey,
        locale: &DataLocale,
        obj: &DataPayload<ExportMarker>,
    ) -> Result<(), DataError> {
        let mut path_buf = self.root.clone().into_os_string();
        write!(
            &mut path_buf,
            "/{key}/{locale}.{}",
            self.manifest.file_extension
        )
        .expect("infallible");

        #[allow(clippy::unwrap_used)] // has parent by construction
        let parent_dir = Path::new(&path_buf).parent().unwrap();

        fs::create_dir_all(parent_dir)
            .map_err(|e| DataError::from(e).with_path_context(&parent_dir))?;

        let mut file = HashingFile {
            file: if self.serializer.is_text_format() {
                Box::new(crlify::BufWriterWithLineEndingFix::new(
                    fs::File::create(&path_buf)
                        .map_err(|e| DataError::from(e).with_path_context(&path_buf))?,
                ))
            } else {
                Box::new(std::io::BufWriter::new(
                    fs::File::create(&path_buf)
                        .map_err(|e| DataError::from(e).with_path_context(&path_buf))?,
                ))
            },
            hash_size: if self.fingerprints.is_some() {
                #[allow(deprecated)]
                Some((SipHasher::new(), 0))
            } else {
                None
            },
        };

        self.serializer
            .serialize(obj, &mut file)
            .map_err(|e| e.with_path_context(&path_buf))?;
        if let Some((hash, size)) = file.hash_size {
            self.fingerprints
                .as_ref()
                .expect("present iff file.1 is present")
                .lock()
                .expect("poison")
                .push(format!("{key}, {locale}, {size}B, {:x}", hash.finish()));
        }
        Ok(())
    }

    fn flush(&self, key: DataKey) -> Result<(), DataError> {
        let mut path_buf = self.root.clone().into_os_string();
        write!(&mut path_buf, "/{key}").expect("infallible");

        if !Path::new(&path_buf).exists() {
            fs::create_dir_all(&path_buf)
                .map_err(|e| DataError::from(e).with_path_context(&path_buf))?;
            write!(&mut path_buf, "/.empty").expect("infallible");
            fs::File::create(Path::new(&path_buf))?;
        }

        Ok(())
    }

    fn close(&mut self) -> Result<(), DataError> {
        if let Some(fingerprints) = self.fingerprints.as_mut() {
            let fingerprints = fingerprints.get_mut().expect("poison");
            fingerprints.sort();
            let path = self.root.join("fingerprints.csv");
            let mut file = crlify::BufWriterWithLineEndingFix::new(
                std::fs::File::create(&path)
                    .map_err(|e| DataError::from(e).with_path_context(&path))?,
            );
            for line in fingerprints {
                use std::io::Write;
                writeln!(file, "{line}")?;
            }
        }
        Ok(())
    }
}

struct HashingFile {
    file: Box<dyn std::io::Write>,
    #[allow(deprecated)]
    hash_size: Option<(SipHasher, usize)>,
}

impl std::io::Write for HashingFile {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if let Some((hash, size)) = self.hash_size.as_mut() {
            hash.write(buf);
            *size += buf.len();
        }
        self.file.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.file.flush()
    }
}
