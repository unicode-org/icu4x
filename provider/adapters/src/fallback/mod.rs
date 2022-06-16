// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Tools for locale fallback, enabling arbitrary input locales to be mapped into the nearest
//! locale with data.
//!
//! # Examples
//!
//! Run the locale fallback algorithm:
//!
//! ```
//! use icu_provider_adapters::fallback::LocaleFallbacker;
//!
//! // Set up a LocaleFallbacker with data.
//! let provider = icu_testdata::get_provider();
//! let fallbacker = LocaleFallbacker::try_new(&provider).expect("data");
//!
//! // Create a LocaleFallbackerWithConfig with a configuration for a specific key.
//! // By default, uses language priority with no additional extension keywords.
//! let key_fallbacker = fallbacker.for_config(Default::default());
//!
//! // Set up the fallback iterator.
//! let loc = icu_locid::locale!("hi-Latn-IN");
//! let mut fallback_iterator = key_fallbacker.fallback_for(loc.into());
//!
//! // Run the algorithm and check the results.
//! assert_eq!(fallback_iterator.get().to_string(), "hi-Latn-IN");
//! fallback_iterator.step();
//! assert_eq!(fallback_iterator.get().to_string(), "hi-Latn");
//! fallback_iterator.step();
//! assert_eq!(fallback_iterator.get().to_string(), "en-IN");
//! fallback_iterator.step();
//! assert_eq!(fallback_iterator.get().to_string(), "en-001");
//! fallback_iterator.step();
//! assert_eq!(fallback_iterator.get().to_string(), "en");
//! fallback_iterator.step();
//! assert_eq!(fallback_iterator.get().to_string(), "und");
//! ```

use icu_locid::extensions::unicode::{Key, Value};
use icu_locid::subtags::Variants;
use icu_provider::prelude::*;

mod algorithms;

pub mod provider;

use provider::*;

/// Strategy for choosing which subtags to drop during locale fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum LocaleFallbackStrategy {
    /// Retain the language and script subtags until the final step.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider_adapters::fallback::LocaleFallbacker;
    /// use icu_provider_adapters::fallback::LocaleFallbackConfig;
    /// use icu_provider_adapters::fallback::LocaleFallbackStrategy;
    ///
    /// // Set up the fallback iterator.
    /// let loc = icu_locid::Locale::from_bytes(b"ca-ES-valencia").unwrap();
    /// let provider = icu_testdata::get_provider();
    /// let fallbacker = LocaleFallbacker::try_new(&provider).expect("data");
    /// let mut config = LocaleFallbackConfig::default();
    /// config.strategy = LocaleFallbackStrategy::LanguagePriority;
    /// let key_fallbacker = fallbacker.for_config(config);
    /// let mut fallback_iterator = key_fallbacker.fallback_for(loc.into());
    ///
    /// // Run the algorithm and check the results.
    /// assert_eq!(fallback_iterator.get().to_string(), "ca-ES-valencia");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "ca-ES");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "ca");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "und");
    /// ```
    LanguagePriority,

    /// Retain the region subtag until the final step.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider_adapters::fallback::LocaleFallbacker;
    /// use icu_provider_adapters::fallback::LocaleFallbackConfig;
    /// use icu_provider_adapters::fallback::LocaleFallbackStrategy;
    ///
    /// // Set up the fallback iterator.
    /// let loc = icu_locid::Locale::from_bytes(b"ca-ES-valencia").unwrap();
    /// let provider = icu_testdata::get_provider();
    /// let fallbacker = LocaleFallbacker::try_new(&provider).expect("data");
    /// let mut config = LocaleFallbackConfig::default();
    /// config.strategy = LocaleFallbackStrategy::RegionPriority;
    /// let key_fallbacker = fallbacker.for_config(config);
    /// let mut fallback_iterator = key_fallbacker.fallback_for(loc.into());
    ///
    /// // Run the algorithm and check the results.
    /// assert_eq!(fallback_iterator.get().to_string(), "ca-ES-valencia");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "ca-ES");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "und-ES-valencia");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "und-ES");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "und");
    /// ```
    RegionPriority,
}

impl Default for LocaleFallbackStrategy {
    fn default() -> Self {
        Self::LanguagePriority
    }
}

/// Configuration settings for a particular fallback operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
#[non_exhaustive]
pub struct LocaleFallbackConfig {
    /// Strategy for choosing which subtags to drop during locale fallback.
    ///
    /// See [`LocaleFallbackStrategy`] for examples.
    pub strategy: LocaleFallbackStrategy,
    /// An extension keyword to retain during locale fallback.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_provider_adapters::fallback::LocaleFallbacker;
    /// use icu_provider_adapters::fallback::LocaleFallbackConfig;
    /// use icu_provider_adapters::fallback::LocaleFallbackStrategy;
    ///
    /// // Set up the fallback iterator.
    /// let loc = icu_locid::Locale::from_bytes(b"ar-EG-u-ns-latn").unwrap();
    /// let provider = icu_testdata::get_provider();
    /// let fallbacker = LocaleFallbacker::try_new(&provider).expect("data");
    /// let mut config = LocaleFallbackConfig::default();
    /// config.extension_kw = Some(icu_locid::unicode_ext_key!("ns"));
    /// let key_fallbacker = fallbacker.for_config(config);
    /// let mut fallback_iterator = key_fallbacker.fallback_for(loc.into());
    ///
    /// // Run the algorithm and check the results.
    /// assert_eq!(fallback_iterator.get().to_string(), "ar-EG-u-ns-latn");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "ar-EG");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "ar");
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get().to_string(), "und");
    /// ```
    pub extension_kw: Option<Key>,
}

/// Entry type for locale fallbacking.
///
/// See the module-level documentation for an example.
pub struct LocaleFallbacker {
    likely_subtags: DataPayload<LocaleFallbackLikelySubtagsV1Marker>,
    parents: DataPayload<LocaleFallbackParentsV1Marker>,
}

/// Intermediate type for spawning locale fallback iterators based on a specific configuration.
///
/// See the module-level documentation for an example.
pub struct LocaleFallbackerWithConfig<'a> {
    likely_subtags: &'a LocaleFallbackLikelySubtagsV1<'a>,
    parents: &'a LocaleFallbackParentsV1<'a>,
    key_metadata: LocaleFallbackConfig,
}

/// Iteration type for locale fallback operations.
///
/// Because the `Iterator` trait does not allow items to borrow from the iterator, this class does
/// not implement that trait. Instead, use `.step()` and `.get()`.
pub struct LocaleFallbackIterator<'a, 'b> {
    likely_subtags: &'a LocaleFallbackLikelySubtagsV1<'a>,
    parents: &'a LocaleFallbackParentsV1<'a>,
    key_metadata: &'b LocaleFallbackConfig,
    current: ResourceOptions,
    backup_extension: Option<Value>,
    backup_subdivision: Option<Value>,
    backup_variants: Option<Variants>,
}

impl LocaleFallbacker {
    /// Creates a [`LocaleFallbacker`] with fallback data (likely subtags and parent locales).
    pub fn try_new<P>(provider: &P) -> Result<Self, DataError>
    where
        P: ResourceProvider<LocaleFallbackLikelySubtagsV1Marker>
            + ResourceProvider<LocaleFallbackParentsV1Marker>
            + ?Sized,
    {
        let likely_subtags = provider
            .load_resource(&Default::default())?
            .take_payload()?;
        let parents = provider
            .load_resource(&Default::default())?
            .take_payload()?;
        Ok(LocaleFallbacker {
            likely_subtags,
            parents,
        })
    }

    /// Creates a [`LocaleFallbacker`] without fallback data. Using this constructor may result in
    /// surprising behavior, especially in multi-script languages.
    pub fn new_without_data() -> Self {
        LocaleFallbacker {
            likely_subtags: DataPayload::from_owned(Default::default()),
            parents: DataPayload::from_owned(Default::default()),
        }
    }

    /// Creates the intermediate [`LocaleFallbackerWithConfig`] with configuration options.
    pub fn for_config(&self, key_metadata: LocaleFallbackConfig) -> LocaleFallbackerWithConfig {
        LocaleFallbackerWithConfig {
            likely_subtags: self.likely_subtags.get(),
            parents: self.parents.get(),
            key_metadata,
        }
    }
}

impl<'a> LocaleFallbackerWithConfig<'a> {
    /// Creates an iterator based on a [`ResourceOptions`] (which can be created from [`Locale`]).
    ///
    /// When first initialized, the locale is normalized according to the fallback algorithm.
    ///
    /// [`Locale`]: icu_locid::Locale
    pub fn fallback_for<'b>(&'b self, mut ro: ResourceOptions) -> LocaleFallbackIterator<'a, 'b> {
        self.normalize(&mut ro);
        LocaleFallbackIterator {
            likely_subtags: self.likely_subtags,
            parents: self.parents,
            key_metadata: &self.key_metadata,
            current: ro,
            backup_extension: None,
            backup_subdivision: None,
            backup_variants: None,
        }
    }
}

impl<'a, 'b> LocaleFallbackIterator<'a, 'b> {
    /// Gets the current [`ResourceOptions`] under fallback.
    pub fn get(&self) -> &ResourceOptions {
        &self.current
    }
}
