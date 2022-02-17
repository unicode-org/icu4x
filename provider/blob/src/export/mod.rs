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
//! use icu_provider::hello_world::*;
//! use icu_provider_blob::BlobDataProvider;
//! use icu_provider_blob::export::BlobExporter;
//! use std::borrow::Cow;
//! use std::io::Read;
//! use std::rc::Rc;
//!
//! let mut buffer: Vec<u8> = Vec::new();
//!
//! // Set up the exporter and export a key
//! let mut exporter = BlobExporter::new_with_sink(Box::new(&mut buffer));
//!    
//! // Export something
//! let payload = DataPayload::<HelloWorldV1Marker>::from_owned(
//!     HelloWorldV1 { message: Cow::Borrowed("Hi") }
//! );
//! let result = exporter.put_payload(
//!    HelloWorldV1Marker::KEY,
//!    Default::default(),
//!    payload.clone().into_serializable())
//! .expect("Should successfully export");
//! exporter.close().expect("Should successfully dump to buffer");
//!
//! drop(exporter);
//!
//! // Create a blob provider reading from the buffer
//! let provider = BlobDataProvider::new_from_rc_blob(buffer.into())
//!     .expect("Should successfully read from buffer");
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
//! ```

mod blob_exporter;

pub use blob_exporter::BlobExporter;
