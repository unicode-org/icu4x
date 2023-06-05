// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use crate::{LocaleExpander, LocaleTransformError};
use icu_locid::subtags::Language;
use icu_locid::Locale;
use icu_provider::prelude::*;

/// Represents the direction of a script.
///
/// [`LocaleDirectionality`] can be used to get this information.
#[zerovec::make_ule(DirectionULE)]
#[zerovec::derive(Debug)]
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[cfg_attr(
    feature = "datagen",
    derive(serde::Serialize, databake::Bake),
    databake(path = icu_locid_transform::provider),
)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[non_exhaustive]
pub enum Direction {
    /// The script is left-to-right.
    LeftToRight = 0,
    /// The script is right-to-left.
    RightToLeft = 1,
}

/// The `LocaleDirectionality` provides methods to determine the direction of a locale based
/// on [`CLDR`] data.
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_locid_transform::{Direction, LocaleDirectionality};
///
/// let ld = LocaleDirectionality::try_new_unstable(&icu_testdata::unstable())
///     .expect("create failed");
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
    /// A constructor which takes a [`DataProvider`] and creates a [`LocaleDirectionality`].
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
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

    // Note: This is a custom impl because the bounds on `try_new_unstable` don't suffice
    #[doc = icu_provider::gen_any_buffer_docs!(ANY, icu_provider, Self::try_new_unstable)]
    pub fn try_new_with_any_provider(
        provider: &(impl AnyProvider + ?Sized),
    ) -> Result<LocaleDirectionality, LocaleTransformError> {
        let expander = LocaleExpander::try_new_with_any_provider(provider)?;
        Self::try_new_with_expander_unstable(&provider.as_downcasting(), expander)
    }

    // Note: This is a custom impl because the bounds on `try_new_unstable` don't suffice
    #[doc = icu_provider::gen_any_buffer_docs!(BUFFER, icu_provider, Self::try_new_unstable)]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider(
        provider: &(impl BufferProvider + ?Sized),
    ) -> Result<LocaleDirectionality, LocaleTransformError> {
        let expander = LocaleExpander::try_new_with_buffer_provider(provider)?;
        Self::try_new_with_expander_unstable(&provider.as_deserializing(), expander)
    }

    /// Creates a [`LocaleDirectionality`] with a custom [`LocaleExpander`] object.
    ///
    /// For example, use this constructor if you wish to support all languages.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
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
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_locid_transform::{Direction, LocaleDirectionality};
    ///
    /// let ld = LocaleDirectionality::try_new_unstable(&icu_testdata::unstable())
    ///     .expect("create failed");
    ///
    /// assert_eq!(ld.get(&locale!("en-US")), Some(Direction::LeftToRight));
    ///
    /// assert_eq!(ld.get(&locale!("ar")), Some(Direction::RightToLeft));
    ///
    /// assert_eq!(ld.get(&locale!("fr-Brai-FR")), None);
    /// ```
    pub fn get(&self, locale: &Locale) -> Option<Direction> {
        let script = locale.id.script.or_else(|| {
            let expander = self.expander.as_borrowed();
            match (locale.id.language, locale.id.region) {
                (Language::UND, Some(region)) => expander.get_r(region).map(|(_, s)| s),
                (Language::UND, None) => None,
                (lang, Some(region)) => expander.get_lr(lang, region),
                (lang, None) => expander.get_l(lang).map(|(s, _)| s),
            }
        })?;

        self.script_direction
            .get()
            .rtl
            .get_copied(&script.into_tinystr().to_unvalidated())
    }

    /// Returns true if the given locale is right-to-left.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_right_to_left(&self, locale: &Locale) -> bool {
        self.get(locale) == Some(Direction::RightToLeft)
    }

    /// Returns true if the given locale is left-to-right.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_left_to_right(&self, locale: &Locale) -> bool {
        self.get(locale) == Some(Direction::LeftToRight)
    }
}
