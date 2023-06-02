// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::provider::*;
use crate::{LocaleExpander, LocaleTransformError};
use icu_locid::subtags::Script;
use icu_locid::{subtags_script, Locale};
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
    /// The direction of the script is unknown or undefined.
    Unknown = 2,
}

/// The `LocaleDirectionality` provides methods to determine the direction of a locale based
/// on [`CLDR`] data.
///
/// # Examples
///
/// ```
/// use icu_locid::Locale;
/// use icu_locid_transform::LocaleDirectionality;
///
/// let ld = LocaleDirectionality::try_new_unstable(&icu_testdata::unstable())
///     .expect("create failed");
///
/// let locale: Locale = "en".parse().unwrap();
/// assert_eq!(ld.get(locale), icu_locid_transform::Direction::LeftToRight);
/// ```
///
/// [`CLDR`]: http://cldr.unicode.org/
#[derive(Debug)]
pub struct LocaleDirectionality {
    rtl: DataPayload<DirectionalityV1Marker>,
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
        P: DataProvider<DirectionalityV1Marker>
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

    // TODO: Do we need to allow user-provided LocaleExpanders? Can a LocaleExpander ever not
    // have the right amount of data? Make this public if so
    fn try_new_with_expander_unstable<P>(
        provider: &P,
        expander: LocaleExpander,
    ) -> Result<LocaleDirectionality, LocaleTransformError>
    where
        P: DataProvider<DirectionalityV1Marker> + ?Sized,
    {
        let rtl: DataPayload<DirectionalityV1Marker> =
            provider.load(Default::default())?.take_payload()?;

        Ok(LocaleDirectionality { rtl, expander })
    }

    /// Returns the script direction of the given locale.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::Locale;
    /// use icu_locid_transform::LocaleDirectionality;
    ///
    /// let ld = LocaleDirectionality::try_new_unstable(&icu_testdata::unstable())
    ///     .expect("create failed");
    ///
    /// let locale: Locale = "en-US".parse().unwrap();
    /// assert_eq!(ld.get(locale), icu_locid_transform::Direction::LeftToRight);
    ///
    /// let locale: Locale = "ar".parse().unwrap();
    /// assert_eq!(ld.get(locale), icu_locid_transform::Direction::RightToLeft);
    ///
    /// let locale: Locale = "fr-Brai-FR".parse().unwrap();
    /// assert_eq!(ld.get(locale), icu_locid_transform::Direction::Unknown);
    /// ```
    pub fn get(&self, locale: impl Into<Locale>) -> Direction {
        let mut locale = locale.into();

        // 1. Maximize the locale to utilize the likely subtags data
        let _transform_result = self.expander.maximize(&mut locale);
        // 2. Get script subtag of the locale
        let script = script_of_locale(&locale);

        // 3. Get the directionality of the script
        let directionality = self
            .rtl
            .get()
            .rtl
            .get(&script.into_tinystr().to_unvalidated());
        match directionality {
            Some(&directionality) => {
                <Direction as zerovec::ule::AsULE>::from_unaligned(directionality)
            }
            // TODO: Do we make `LocaleDirectionality::get` fallible despite the DataPayload
            // being defined to contain all scripts?
            None => Direction::Unknown,
        }
    }

    /// Returns true if the given locale is right-to-left.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_right_to_left(&self, locale: impl Into<Locale>) -> bool {
        self.get(locale) == Direction::RightToLeft
    }

    /// Returns true if the given locale is left-to-right.
    ///
    /// See [`LocaleDirectionality::get`] for more information.
    pub fn is_left_to_right(&self, locale: impl Into<Locale>) -> bool {
        self.get(locale) == Direction::LeftToRight
    }
}

fn script_of_locale(locale: &Locale) -> Script {
    match locale.id.script {
        Some(script) => script,
        None => subtags_script!("Zzzz"),
    }
}
