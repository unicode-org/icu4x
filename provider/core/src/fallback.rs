// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Options to define fallback behaviour.
//!
//! These options are consumed by the `LocaleFallbacker` in the `icu_locales` crate
//! (or the `icu::locales` module), but are defined here because they are used by `DataKey`.

/// Hint for which subtag to prioritize during fallback.
///
/// For example, `"en-US"` might fall back to either `"en"` or `"und-US"` depending
/// on this enum.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[non_exhaustive]
pub enum LocaleFallbackPriority {
    /// Prioritize the language. This is the default behavior.
    ///
    /// For example, `"en-US"` should go to `"en"` and then `"und"`.
    Language,
    /// Prioritize the region.
    ///
    /// For example, `"en-US"` should go to `"und-US"` and then `"und"`.
    Region,
    /// Collation-specific fallback rules. Similar to language priority.
    ///
    /// For example, `"zh-Hant"` goes to `"zh"` before `"und"`.
    Collation,
}

impl LocaleFallbackPriority {
    /// Const-friendly version of [`Default::default`].
    pub const fn const_default() -> Self {
        Self::Language
    }
}

impl Default for LocaleFallbackPriority {
    fn default() -> Self {
        Self::const_default()
    }
}

/// What additional data is required to load when performing fallback.
#[derive(Debug, PartialEq, Eq, Copy, Clone, PartialOrd, Ord)]
#[non_exhaustive]
pub enum LocaleFallbackSupplement {
    /// Collation supplement
    Collation,
}

/// Configuration settings for a particular fallback operation.
#[derive(Debug, Clone, PartialEq, Eq, Copy)]
#[non_exhaustive]
pub struct LocaleFallbackConfig {
    /// Strategy for choosing which subtags to drop during locale fallback.
    ///
    /// # Examples
    ///
    /// Retain the language and script subtags until the final step:
    ///
    /// ```
    /// use icu::locale::langid;
    /// use icu::locale::fallback::LocaleFallbackConfig;
    /// use icu::locale::fallback::LocaleFallbackPriority;
    /// use icu::locale::LocaleFallbacker;
    ///
    /// // Set up the fallback iterator.
    /// let fallbacker = LocaleFallbacker::new();
    /// let mut config = LocaleFallbackConfig::default();
    /// config.priority = LocaleFallbackPriority::Language;
    /// let mut fallback_iterator = fallbacker
    ///     .for_config(config)
    ///     .fallback_for(langid!("ca-ES-valencia").into());
    ///
    /// // Run the algorithm and check the results.
    /// assert_eq!(fallback_iterator.get(), &langid!("ca-ES-valencia"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("ca-ES"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("ca-valencia"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("ca"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("und"));
    /// ```
    ///
    /// Retain the region subtag until the final step:
    ///
    /// ```
    /// use icu::locale::langid;
    /// use icu::locale::fallback::LocaleFallbackConfig;
    /// use icu::locale::fallback::LocaleFallbackPriority;
    /// use icu::locale::LocaleFallbacker;
    ///
    /// // Set up the fallback iterator.
    /// let fallbacker = LocaleFallbacker::new();
    /// let mut config = LocaleFallbackConfig::default();
    /// config.priority = LocaleFallbackPriority::Region;
    /// let mut fallback_iterator = fallbacker
    ///     .for_config(config)
    ///     .fallback_for(langid!("ca-ES-valencia").into());
    ///
    /// // Run the algorithm and check the results.
    /// assert_eq!(fallback_iterator.get(), &langid!("ca-ES-valencia"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("ca-ES"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("und-ES-valencia"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("und-ES"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("und"));
    /// ```
    pub priority: LocaleFallbackPriority,
    /// Fallback supplement data key to customize fallback rules.
    ///
    /// For example, most data keys for collation add additional parent locales, such as
    /// "yue" to "zh-Hant", and data used for the `"-u-co"` extension keyword fallback.
    ///
    /// Currently the only supported fallback supplement is `LocaleFallbackSupplement::Collation`, but more may be
    /// added in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu::locale::langid;
    /// use icu::locale::fallback::LocaleFallbackConfig;
    /// use icu::locale::fallback::LocaleFallbackPriority;
    /// use icu::locale::fallback::LocaleFallbackSupplement;
    /// use icu::locale::LocaleFallbacker;
    ///
    /// // Set up the fallback iterator.
    /// let fallbacker = LocaleFallbacker::new();
    /// let mut config = LocaleFallbackConfig::default();
    /// config.priority = LocaleFallbackPriority::Collation;
    /// config.fallback_supplement = Some(LocaleFallbackSupplement::Collation);
    /// let mut fallback_iterator = fallbacker
    ///     .for_config(config)
    ///     .fallback_for(langid!("yue-HK").into());
    ///
    /// // Run the algorithm and check the results.
    /// assert_eq!(fallback_iterator.get(), &langid!("yue-HK"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("yue"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("zh-Hant"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("zh"));
    /// fallback_iterator.step();
    /// assert_eq!(fallback_iterator.get(), &langid!("und"));
    /// ```
    pub fallback_supplement: Option<LocaleFallbackSupplement>,
}

impl LocaleFallbackConfig {
    /// Const version of [`Default::default`].
    pub const fn const_default() -> Self {
        Self {
            priority: LocaleFallbackPriority::const_default(),
            fallback_supplement: None,
        }
    }
}

impl Default for LocaleFallbackConfig {
    fn default() -> Self {
        Self::const_default()
    }
}
