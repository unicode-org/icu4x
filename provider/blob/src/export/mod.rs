// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data exporter that creates a binary blob for use with [`BlobDataProvider`](crate::BlobDataProvider).
//!
//! This module can be used as a target for the `icu_datagen` crate.
//!
//! See our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md) for more information about different data providers.
//!
//! # Examples
//!
//! ```
//! use icu_datagen::prelude::*;
//! use icu_provider_blob::export::*;
//!
//! let mut blob: Vec<u8> = Vec::new();
//!
//! // Set up the exporter
//! let mut exporter = BlobExporter::new_v2_with_sink(Box::new(&mut blob));
//!
//! // Export something
//! DatagenDriver::new()
//!     .with_markers([icu_provider::hello_world::HelloWorldV1Marker::INFO])
//!     .with_locales_and_fallback([LocaleFamily::FULL], Default::default())
//!     .export(&icu_provider::hello_world::HelloWorldProvider, exporter)
//!     .unwrap();
//!
//! // communicate the blob to the client application (network, disk, etc.)
//! # assert_eq!(blob, include_bytes!("../../tests/data/v2.postcard"));
//! ```
//!
//! The resulting blob can now be used like this:
//!
//! ```
//! use icu_locale_core::locale;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//! use icu_provider_blob::BlobDataProvider;
//!
//! // obtain the data blob
//! # let blob = include_bytes!("../../tests/data/v2.postcard").to_vec();
//!
//! // Create a provider reading from the blob
//! let provider = BlobDataProvider::try_new_from_blob(blob.into_boxed_slice())
//!     .expect("Should successfully read from blob");
//!
//! // Use the provider as a `BufferProvider`
//! let formatter = HelloWorldFormatter::try_new_with_buffer_provider(
//!     &provider,
//!     &locale!("en").into(),
//! )
//! .unwrap();
//!
//! assert_eq!(formatter.format_to_string(), "Hello World");
//! ```

mod blob_exporter;

mod iter;

pub use blob_exporter::BlobExporter;
