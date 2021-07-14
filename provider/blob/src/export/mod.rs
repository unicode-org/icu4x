// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Utilities for dumping data to an ICU4X filesystem tree.
//!
//! The `export` feature enables you to pull all data from some other data provider and persist it
//! in a blob of data to be read by [`StaticDataProvider`](crate::StaticDataProvider) at runtime.
//!
//! For a command-line user interface, see the `icu_datagen` crate.
//!
//! # Examples
//!
//! ```
//! use icu_provider::prelude::*;
//! use icu_provider::export::DataExporter;
//! use icu_provider::hello_world::{key, HelloWorldProvider};
//! use icu_provider_blob::StaticDataProvider;
//! use icu_provider_blob::export::BlobExporter;
//! use icu_locid_macros::langid;
//! use std::io::Read;
//!
//! let mut buffer: Vec<u8> = Vec::new();
//!
//! // Set up the exporter and export a key
//! {
//!     let mut exporter = BlobExporter::new_with_sink(Box::new(&mut buffer));
//!     let source_provider = HelloWorldProvider::new_with_placeholder_data();
//!     let result = icu_provider::export::export_from_iterable(
//!         &key::HELLO_WORLD_V1,
//!         &source_provider,
//!         &mut exporter)
//!     .expect("Should successfully export");
//!     exporter.close().expect("Should successfully dump to buffer");
//! }
//!
//! // Assert that the exported data equals the pre-computed hello_world.bincode
//! let mut expected_buffer: Vec<u8> = Vec::new();
//! std::fs::File::open(concat!(
//!     env!("CARGO_MANIFEST_DIR"),
//!     "/tests/data/hello_world.bincode"
//! ))
//! .expect("File should exist")
//! .read_to_end(&mut expected_buffer)
//! .expect("Reading pre-computed bincode buffer");
//!
//! assert_eq!(buffer, expected_buffer);
//! ```

mod blob_exporter;

pub use blob_exporter::BlobExporter;
