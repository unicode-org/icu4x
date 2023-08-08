// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use crate::{LocaleExpander, LocaleTransformError};
use icu_locid::subtags::Script;
use icu_locid::Locale;
use icu_provider::prelude::*;

/// Represents the direction of a script.
///
/// [`LocaleDirectionality`] can be used to get this information.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3722">#3722</a>
/// </div>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[non_exhaustive]
pub enum Direction {
    /// The script is left-to-right.
    LeftToRight,
    /// The script is right-to-left.
    RightToLeft,
}

/// The `LocaleDirectionality` provides methods to determine the direction of a locale based
/// on [`CLDR`] data.
///
/// <div class="stab unstable">
/// 🚧 This code is experimental; it may change at any time, in breaking or non-breaking ways,
/// including in SemVer minor releases. It can be enabled with the "experimental" Cargo feature
/// of the icu meta-crate. Use with caution.
/// <a href="https://github.com/unicode-org/icu4x/issues/3722">#3722</a>
/// </div>
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_locid_transform::{Direction, LocaleDirectionality};
///
/// let ld = LocaleDirectionality::new();
///
/// assert_eq!(ld.get(&locale!("en")), Some(Direction::LeftToRight));
/// ```
///
/// [`CLDR`]: http://cldr.unicode.org/
#[derive(Debug)]
pub struct LocaleDirectionality {
    script_direction: DataPayload<ScriptDirectionV1Marker>,
    expander: LocaleExpander,
}

impl LocaleDirectionality {
    /// A constructor which creates a [`LocaleDirectionality`] from compiled data.
    #[cfg(feature = "compiled_data")]
    pub const fn new() -> Self {
        Self::new_with_expander(LocaleExpander::new())
    }

    // Note: This is a custom impl because the bounds on `try_new_unstable` don't suffice
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(ANY, Self::new)]
    pub fn try_new_with_any_provider(
        provider: &(impl AnyProvider + ?Sized),
    ) -> Result<LocaleDirectionality, LocaleTransformError> {
        let expander = LocaleExpander::try_new_with_any_provider(provider)?;
        Self::try_new_with_expander_unstable(&provider.as_downcasting(), expander)
    }

    // Note: This is a custom impl because the bounds on `try_new_unstable` don't suffice
    #[doc = icu_provider::gen_any_buffer_unstable_docs!(BUFFER, Self::new)]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider(
        provider: &(impl BufferProvider + ?Sized),
    ) -> Result<LocaleDirectionality, LocaleTransformError> {
        let expander = LocaleExpander::try_new_with_buffer_provider(provider)?;
        Self::try_new_with_expander_unstable(&provider.as_deserializing(), expander)
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new)]
    pub fn try_new_unstable<P>(provider: &P) -> Result<LocaleDirectionality, LocaleTransformError>
    where
        P: DataProvider<ScriptDirectionV1Marker>
            + DataProvider<LikelySubtagsForLanguageV1Marker>
            + DataProvider<LikelySubtagsForScriptRegionV1Marker>
            + ?Sized,
    {
        let expander = LocaleExpander::try_new_unstable(provider)?;
        Self::try_new_with_expander_unstable(provider, expander)
    }

    /// Creates a [`LocaleDirectionality`] with a custom [`LocaleExpander`] and compiled data.
    ///
    /// For example, use this constructor if you wish to support all languages.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_locid_transform::{Direction, LocaleDirectionality, LocaleExpander};
    ///
    /// let ld_default = LocaleDirectionality::new();
    ///
    /// assert_eq!(ld_default.get(&locale!("jbn")), None);
    ///
    /// let expander = LocaleExpander::new_extended();
    /// let ld_extended = LocaleDirectionality::new_with_expander(expander);
    ///
    /// assert_eq!(ld_extended.get(&locale!("jbn")), Some(Direction::RightToLeft));
    /// ```
    #[cfg(feature = "compiled_data")]
    pub const fn new_with_expander(expander: LocaleExpander) -> Self {
        LocaleDirectionality {
            script_direction: DataPayload::from_static_ref(
                crate::provider::Baked::SINGLETON_LOCID_TRANSFORM_SCRIPT_DIR_V1,
            ),
            expander,
        }
    }

    #[doc = icu_provider::gen_any_buffer_unstable_docs!(UNSTABLE, Self::new_with_expander)]
    pub fn try_new_with_expander_unstable<P>(
        provider: &P,
        expander: LocaleExpander,
    ) -> Result<LocaleDirectionality, LocaleTransformError>
    where
        P: DataProvider<ScriptDirectionV1Marker> + ?Sized,
    {
        let script_direction = provider.load(Default::default())?.take_payload()?;

        Ok(LocaleDirectionality {
            script_direction,
            expander,
        })
    }

    /// Returns the script direction of the given locale.
    ///
    /// Note that the direction is a property of the script of a locale, not of the language. As such,
    /// when given a locale without an associated script tag (i.e., `locale!("en")` vs. `locale!("en-Latn")`),
    /// this method first tries to infer the script using the language and region before returning its direction.
    ///
    /// If you already have a script struct and want to get its direction, you should use
    /// `Locale::from(Some(my_script))` and call this method.
    ///
    /// # Examples
    ///
    /// Using an existing locale:
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_locid_transform::{Direction, LocaleDirectionality};
    ///
    /// let ld = LocaleDirectionality::new();
    ///
    /// assert_eq!(ld.get(&locale!("en-US")), Some(Direction::LeftToRight));
    ///
    /// assert_eq!(ld.get(&locale!("ar")), Some(Direction::RightToLeft));
    ///
    /// assert_eq!(ld.get(&locale!("foo")), None);
    /// ```
    ///
    /// Using a script directly:
    ///
    /// ```
    /// use icu_locid::subtags::script;
    /// use icu_locid::Locale;
    /// use icu_locid_transform::{Direction, LocaleDirectionality};
    ///
    /// let ld = LocaleDirectionality::new();
    ///
    /// assert_eq!(ld.get(&Locale::from(Some(script!("Latn")))), Some(Direction::LeftToRight));
    /// ```
    pub fn get(&self, locale: &Locale) -> Option<Direction> {
        let script = self.expander.get_likely_script(&locale.id)?;

        if self.script_in_ltr(script) {
            Some(Direction::LeftToRight)
        } else if self.script_in_rtl(script) {
            Some(Direction::RightToLeft)
        } else {
            None
        }
    }

    /// Returns true if the given locale is right-to-left.
    ///
    /// Note that if this method returns `false`, it does not mean that the locale is left-to-right.
    /// You should use `LocaleDirectionality::get` if you need to differentiate between these cases.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_right_to_left(&self, locale: &Locale) -> bool {
        self.expander
            .get_likely_script(&locale.id)
            .map(|s| self.script_in_rtl(s))
            .unwrap_or(false)
    }

    /// Returns true if the given locale is left-to-right.
    ///
    /// Note that if this method returns `false`, it does not mean that the locale is right-to-left.
    /// You should use `LocaleDirectionality::get` if you need to differentiate between these cases.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_left_to_right(&self, locale: &Locale) -> bool {
        self.expander
            .get_likely_script(&locale.id)
            .map(|s| self.script_in_ltr(s))
            .unwrap_or(false)
    }

    fn script_in_rtl(&self, script: Script) -> bool {
        self.script_direction
            .get()
            .rtl
            .binary_search(&script.into_tinystr().to_unvalidated())
            .is_ok()
    }

    fn script_in_ltr(&self, script: Script) -> bool {
        self.script_direction
            .get()
            .ltr
            .binary_search(&script.into_tinystr().to_unvalidated())
            .is_ok()
    }
}
