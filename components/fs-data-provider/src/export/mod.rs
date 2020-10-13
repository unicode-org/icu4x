// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
//! The `export` feature enables you to pull all data from some other data provider and persist it
//! on the filesystem to be read by an FsDataProvider at runtime.
//!
//! Also see the binary "icu4x-cldr-export".
//!
//! # Examples
//!
//! ```
//! use icu_data_provider::prelude::*;
//! use icu_data_provider::InvariantDataProvider;
//! use icu_data_provider::structs::plurals::PluralRuleStringsV1;
//! use icu_data_provider::iter::IterableDataProvider;
//! use icu_fs_data_provider::FsDataProvider;
//! use icu_fs_data_provider::export::fs_exporter;
//! use icu_fs_data_provider::export::serializers;
//! use std::path::PathBuf;
//!
//! let DEMO_PATH = std::env::temp_dir().join("icu4x_json_demo");
//! let DATA_KEY = icu_data_key!(plurals: cardinal@1);
//!
//! // Set up the exporter
//! let mut options = serializers::JsonSerializerOptions::default();
//! let json_serializer = Box::new(serializers::JsonSerializer::new(&options));
//! let mut options = fs_exporter::ExporterOptions::default();
//! options.root = DEMO_PATH.clone();
//! let mut exporter = fs_exporter::FilesystemExporter::try_new(json_serializer, options)
//!     .expect("Should successfully initialize data output directory");
//!
//! // Export a key
//! let inv_provider = InvariantDataProvider;
//! let result = inv_provider.export_key(&DATA_KEY, &mut exporter);
//! // Ensure flush() is called, even when the result is an error
//! exporter.flush().expect("Should successfully flush");
//! result.expect("Should successfully export");
//!
//! // Create a filesystem provider reading from the demo directory
//! let fs_provider = FsDataProvider::try_new(DEMO_PATH.clone())
//!     .expect("Should successfully read from filesystem");
//!
//! // Read the key from the filesystem and ensure it is as expected
//! let req = DataRequest {
//!     data_key: DATA_KEY,
//!     data_entry: DataEntry {
//!         variant: None,
//!         langid: Default::default(),
//!     }
//! };
//! let inv_response = inv_provider.load(&req).unwrap();
//! let fs_response = fs_provider.load(&req)
//!     .expect("Should successfully read from filesystem");
//!
//! assert_eq!(
//!     inv_response.borrow_payload::<PluralRuleStringsV1>().unwrap(),
//!     fs_response.borrow_payload::<PluralRuleStringsV1>().unwrap(),
//! );
//!
//! // Clean up from demo
//! std::fs::remove_dir_all(&DEMO_PATH).expect("Should clean up test directory");
//! ```

mod aliasing;
pub mod fs_exporter;
pub mod serializers;
pub use fs_exporter::FilesystemExporter;
