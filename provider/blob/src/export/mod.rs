// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data generation for [`BlobDataProvider`](crate::BlobDataProvider) data. See the `icu_datagen` crate.
//!
//! # Examples
//!
//! ```
//! use icu_provider::datagen::DataExporter;
//! use icu_provider::dynutil::*;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//! use icu_provider_blob::export::BlobExporter;
//! use icu_provider_blob::BlobDataProvider;
//! use std::borrow::Cow;
//! use std::io::Read;
//! use std::rc::Rc;
//!
//! let mut buffer: Vec<u8> = Vec::new();
//!
//! let payload = DataPayload::<HelloWorldV1Marker>::from_owned(HelloWorldV1 {
//!     message: Cow::Borrowed("Hi"),
//! });
//!
//! // Export something
//! {
//!     let mut exporter = BlobExporter::new_with_sink(Box::new(&mut buffer));
//!     exporter
//!         .put_payload(
//!             HelloWorldV1Marker::KEY,
//!             &Default::default(),
//!             &UpcastDataPayload::upcast(payload.clone()),
//!         )
//!         .expect("Should successfully export");
//!     exporter
//!         .close()
//!         .expect("Should successfully dump to buffer");
//! }
//!
//! // Create a blob provider reading from the buffer
//! let provider =
//!     BlobDataProvider::try_new_from_blob(buffer.into_boxed_slice())
//!         .expect("Should successfully read from buffer");
//!
//! // Read the key from the filesystem and ensure it is as expected
//! let req = DataRequest {
//!     locale: Default::default(),
//!     metadata: Default::default(),
//! };
//! let response: DataPayload<HelloWorldV1Marker> = provider
//!     .as_deserializing()
//!     .load(req)
//!     .unwrap()
//!     .take_payload()
//!     .unwrap();
//!
//! assert_eq!(response.get(), payload.get(),);
//! ```

mod blob_exporter;

pub use blob_exporter::BlobExporter;
