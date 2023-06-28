// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data exporter for [`FsDataProvider`](crate::FsDataProvider).
//!
//! This module can be used as a target for the `icu_datagen` crate.
//!
//! # Examples
//!
//! ```
//! use icu_datagen::prelude::*;
//! use icu_provider_fs::export::*;
//!
//! let demo_path = std::env::temp_dir().join("icu4x_json_demo");
//! # let _ = std::fs::remove_dir_all(&demo_path);
//!
//! // Set up the exporter
//! let mut options = ExporterOptions::default();
//! options.root = demo_path.clone();
//! let serializer = Box::new(serializers::json::Serializer::default());
//! let mut exporter = FilesystemExporter::try_new(serializer, options)
//!     .expect("Should successfully initialize data output directory");
//!
//! // Export something
//! DatagenProvider::default()
//!     .export(
//!         [icu_provider::hello_world::HelloWorldV1Marker::KEY].into_iter().collect(),
//!         exporter
//!     ).unwrap();
//! #
//! # let _ = std::fs::remove_dir_all(&demo_path);
//! ```
//!
//! The resulting files can now be used like this:
//!
//! ```
//! use icu_locid::langid;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//! use icu_provider_fs::FsDataProvider;
//!
//! # let demo_path = std::path::Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/tests/data/json"));
//! // Create a filesystem provider reading from the demo directory
//! let provider = FsDataProvider::try_new(&demo_path)
//!     .expect("Should successfully read from filesystem");
//!
//! // Read the key from the filesystem
//! let response: DataPayload<HelloWorldV1Marker> = provider
//!     .as_deserializing()
//!     .load(DataRequest {
//!         locale: &langid!("en").into(),
//!         metadata: Default::default(),
//!     })
//!     .unwrap()
//!     .take_payload()
//!     .unwrap();
//!
//! assert_eq!(response.get().message, "Hello World");
//! ```

#![allow(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]

mod fs_exporter;
pub mod serializers;

pub use fs_exporter::ExporterOptions;
pub use fs_exporter::FilesystemExporter;
pub use fs_exporter::OverwriteOption;
