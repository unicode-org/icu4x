// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Utilities for dumping data to an ICU4X filesystem tree.
//!
//! The `export` feature enables you to pull all data from some other data provider and persist it
//! on the filesystem to be read by an FsDataProvider at runtime.
//!
//! For a command-line user interface, see the `icu_datagen` crate.
//!
//! # Examples
//!
//! ```
//! use icu_locid_macros::langid;
//! use icu_provider::export::DataExporter;
//! use icu_provider::prelude::*;
//! use icu_provider::hello_world::*;
//! use icu_provider_fs::FsDataProvider;
//! use icu_provider_fs::export::fs_exporter;
//! use icu_provider_fs::export::serializers;
//! use std::borrow::Cow;
//! use std::path::PathBuf;
//!
//! let demo_path = std::env::temp_dir().join("icu4x_json_demo");
//!
//! // Set up the exporter
//! let mut options = serializers::json::Options::default();
//! let serializer = Box::new(serializers::json::Serializer::new(options));
//! let mut options = fs_exporter::ExporterOptions::default();
//! options.root = demo_path.clone();
//! let mut exporter = fs_exporter::FilesystemExporter::try_new(serializer, options)
//!     .expect("Should successfully initialize data output directory");
//!
//! // Export something
//! let payload = DataPayload::<HelloWorldV1Marker>::from_owned(
//!     HelloWorldV1 { message: Cow::Borrowed("Hi") }
//! );
//! let result = exporter.put_payload(
//!     HelloWorldV1Marker::KEY,
//!     Default::default(),
//!     payload.clone().into_serializable())
//! .expect("Should successfully export");
//!
//! // Create a filesystem provider reading from the demo directory
//! let provider = FsDataProvider::try_new(demo_path.clone())
//!     .expect("Should successfully read from filesystem");
//!
//! // Read the key from the filesystem and ensure it is as expected
//! let req = DataRequest {
//!     options: Default::default(),
//!     metadata: Default::default(),
//! };
//! let response: DataPayload<HelloWorldV1Marker> =
//!     provider.load_resource(&req).unwrap().take_payload().unwrap();
//!
//! assert_eq!(
//!     response.get(),
//!     payload.get(),
//! );
//!
//! // Clean up from demo
//! std::fs::remove_dir_all(&demo_path).expect("Should clean up test directory");
//! ```

mod aliasing;
pub mod fs_exporter;
pub mod serializers;
pub use fs_exporter::FilesystemExporter;
