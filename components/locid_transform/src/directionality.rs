// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use crate::{LocaleExpander, LocaleTransformError};
use icu_locid::Locale;
use icu_provider::{DataPayload, DataProvider};

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
/// let locale = locale!("en");
/// assert_eq!(ld.get(locale), Some(Direction::LeftToRight));
/// ```
///
/// [`CLDR`]: http://cldr.unicode.org/
#[derive(Debug)]
pub struct LocaleDirectionality {
    rtl: DataPayload<ScriptDirectionV1Marker>,
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

    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: skip,
        error: LocaleTransformError
    );

    /// Creates a [`LocaleDirectionality`] with a custom [`LocaleExpander`] object.
    pub fn try_new_with_expander_unstable<P>(
        provider: &P,
        expander: LocaleExpander,
    ) -> Result<LocaleDirectionality, LocaleTransformError>
    where
        P: DataProvider<ScriptDirectionV1Marker> + ?Sized,
    {
        let rtl: DataPayload<ScriptDirectionV1Marker> =
            provider.load(Default::default())?.take_payload()?;

        Ok(LocaleDirectionality { rtl, expander })
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
    /// let locale = locale!("en-US");
    /// assert_eq!(ld.get(locale), Some(Direction::LeftToRight));
    ///
    /// let locale = locale!("ar");
    /// assert_eq!(ld.get(locale), Some(Direction::RightToLeft));
    ///
    /// let locale = locale!("fr-Brai-FR");
    /// assert_eq!(ld.get(locale), None);
    /// ```
    pub fn get(&self, locale: impl Into<Locale>) -> Option<Direction> {
        let mut locale = locale.into();

        let script = locale.id.script.or_else(|| {
            self.expander.maximize(&mut locale);
            locale.id.script
        });
        let script = match script {
            Some(script) => script,
            // If the locale has no script, we cannot determine the directionality.
            None => return None,
        };

        let direction = self
            .rtl
            .get()
            .rtl
            .get_copied(&script.into_tinystr().to_unvalidated());

        direction
    }

    /// Returns true if the given locale is right-to-left.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_right_to_left(&self, locale: impl Into<Locale>) -> bool {
        self.get(locale) == Some(Direction::RightToLeft)
    }

    /// Returns true if the given locale is left-to-right.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_left_to_right(&self, locale: impl Into<Locale>) -> bool {
        self.get(locale) == Some(Direction::LeftToRight)
    }
}
