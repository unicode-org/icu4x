// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Data exporter that creates a file system structure for use with [`FsDataProvider`](crate::FsDataProvider).
//!
//! This module can be used as a target for the `icu_provider_export` crate.
//!
//! See our [datagen tutorial](https://github.com/unicode-org/icu4x/blob/main/tutorials/data_management.md) for more information about different data providers.
//!
//! # Examples
//!
//! ```
//! use icu_provider_export::prelude::*;
//! use icu_provider::hello_world::*;
//! use icu_provider_fs::export::*;
//!
//! let demo_path = std::env::temp_dir().join("icu4x_json_demo");
//! # let _ = std::fs::remove_dir_all(&demo_path);
//!
//! // Set up the exporter
//! let mut options = Options::default();
//! options.root = demo_path.clone();
//! let serializer = Box::new(serializers::Json::default());
//! let mut exporter = FilesystemExporter::try_new(serializer, options)
//!     .expect("Should successfully initialize data output directory");
//!
//! // Export something. Make sure to use the same fallback data at runtime!
//! ExportDriver::new([DataLocaleFamily::FULL], DeduplicationStrategy::Maximal.into(), LocaleFallbacker::new().static_to_owned())
//!     .export(&HelloWorldProvider, exporter)
//!     .unwrap();
//! #
//! # let _ = std::fs::remove_dir_all(&demo_path);
//! ```
//!
//! The resulting files can now be used like this:
//!
//! ```
//! use icu_locale_core::locale;
//! use icu_provider::hello_world::*;
//! use icu_provider::prelude::*;
//! use icu_provider_fs::FsDataProvider;
//!
//! # let demo_path = "tests/data/json".into();
//! // Create a filesystem provider reading from the demo directory
//! let provider = FsDataProvider::try_new(demo_path)
//!     .expect("Should successfully read from filesystem");
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

#![allow(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic
)]

mod fs_exporter;
pub mod serializers;

pub use fs_exporter::*;
