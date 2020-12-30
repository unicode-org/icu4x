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
//! use icu_provider::prelude::*;
//! use icu_provider::InvariantDataProvider;
//! use icu_provider::structs;
//! use icu_provider::iter::DataExporter;
//! use icu_provider_fs::FsDataProvider;
//! use icu_provider_fs::export::fs_exporter;
//! use icu_provider_fs::export::serializers;
//! use std::borrow::Cow;
//! use std::path::PathBuf;
//!
//! let demo_path = std::env::temp_dir().join("icu4x_json_demo");
//! const DATA_KEY: icu_provider::ResourceKey = structs::plurals::key::CARDINAL_V1;
//!
//! // Set up the exporter
//! let mut options = serializers::json::Options::default();
//! let serializer = Box::new(serializers::json::Serializer::new(options));
//! let mut options = fs_exporter::ExporterOptions::default();
//! options.root = demo_path.clone();
//! let mut exporter = fs_exporter::FilesystemExporter::try_new(serializer, options)
//!     .expect("Should successfully initialize data output directory");
//!
//! // Export a key
//! let inv_provider = InvariantDataProvider;
//! let result = exporter.put_key_from_provider(&DATA_KEY, &inv_provider);
//! // Ensure flush() is called, even when the result is an error
//! exporter.flush().expect("Should successfully flush");
//! result.expect("Should successfully export");
//!
//! // Create a filesystem provider reading from the demo directory
//! let fs_provider = FsDataProvider::try_new(demo_path.clone())
//!     .expect("Should successfully read from filesystem");
//!
//! // Read the key from the filesystem and ensure it is as expected
//! let req = DataRequest {
//!     resource_path: ResourcePath {
//!         key: DATA_KEY,
//!         options: ResourceOptions::default(),
//!     }
//! };
//! let inv_response: DataResponse<structs::plurals::PluralRuleStringsV1> =
//!     inv_provider.load_payload(&req).unwrap();
//! let fs_response: DataResponse<structs::plurals::PluralRuleStringsV1> =
//!     fs_provider.load_payload(&req).unwrap();
//!
//! assert_eq!(
//!     inv_response.payload,
//!     fs_response.payload,
//! );
//!
//! // Clean up from demo
//! std::fs::remove_dir_all(&demo_path).expect("Should clean up test directory");
//! ```

mod aliasing;
pub mod fs_exporter;
pub mod serializers;
pub use fs_exporter::FilesystemExporter;
