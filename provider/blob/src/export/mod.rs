// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data exporter for [`BlobDataProvider`](crate::BlobDataProvider).
//!
//! This module can be used as a target for the `icu_datagen` crate.
//!
//! # Examples
//! 
//! use icu_datagen::prelude::*;
//! use icu_provider::hello_world::*;
//! use icu_provider_blob::export::*;
//! use icu_provider_blob::BlobDataProvider;
//!
//! let mut buffer: Vec<u8> = Vec::new();
//! 
//! // Set up the exporter
//! let mut exporter = BlobExporter::new_with_sink(Box::new(&mut buffer));
//!
//! // Export something
//! DatagenProvider::default()
//!     .export(
//!         [HelloWorldV1Marker::KEY].into_iter().collect(),
//!         exporter
//!     ).unwrap();
//!
//! // Create a filesystem provider reading from the demo directory
//! let provider =
//!     BlobDataProvider::try_new_from_blob(buffer.into_boxed_slice())
//!         .expect("Should successfully read from buffer")
//!         .as_deserializing();
//!
//! // Read the key from the blob
//! let response: DataPayload<HelloWorldV1Marker> = provider
//!     .load(Default::default())
//!     .unwrap()
//!     .take_payload()
//!     .unwrap();
//! ```

mod blob_exporter;

pub use blob_exporter::BlobExporter;
