// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use crate::{provider::*, LocaleTransformError};

use core::mem;
use icu_locid::subtags::{Language, Region, Script};
use icu_locid::LanguageIdentifier;
use icu_provider::prelude::*;

use crate::TransformResult;

/// LocaleExpander supports the `minimize` and `maximize` likely subtags
/// algorithms as described in [`UTS #35: Unicode LDML 3. Likely Subtags`].
///
/// The maximize method potentially updates a passed in locale in place
/// depending up the results of running the 'Add Likely Subtags' algorithm
/// from [`UTS #35: Unicode LDML 3. Likely Subtags`].
///
/// This minimize method returns a new Locale that is the result of running the
/// 'Remove Likely Subtags' algorithm from [`UTS #35: Unicode LDML 3. Likely Subtags`].
///
/// # Examples
///
/// ```
/// use icu_locid::locale;
/// use icu_locid_transform::{LocaleExpander, TransformResult};
///
/// let lc = LocaleExpander::try_new_unstable(&icu_testdata::unstable())
///     .expect("create failed");
///
/// let mut locale = locale!("zh-CN");
/// assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
/// assert_eq!(locale, locale!("zh-Hans-CN"));
///
/// let mut locale = locale!("zh-Hant-TW");
/// assert_eq!(lc.maximize(&mut locale), TransformResult::Unmodified);
/// assert_eq!(locale, locale!("zh-Hant-TW"));
/// ```
///
/// ```
/// use icu_locid::locale;
/// use icu_locid_transform::{LocaleExpander, TransformResult};
///
/// let lc = LocaleExpander::try_new_unstable(&icu_testdata::unstable())
///     .expect("create failed");
///
/// let mut locale = locale!("zh-Hans-CN");
/// assert_eq!(lc.minimize(&mut locale), TransformResult::Modified);
/// assert_eq!(locale, locale!("zh"));
///
/// let mut locale = locale!("zh");
/// assert_eq!(lc.minimize(&mut locale), TransformResult::Unmodified);
/// assert_eq!(locale, locale!("zh"));
/// ```
///
/// [`CLDR`]: http://cldr.unicode.org/
/// [`UTS #35: Unicode LDML 3. Likely Subtags`]: https://www.unicode.org/reports/tr35/#Likely_Subtags.
pub struct LocaleExpander {
    likely_subtags: DataPayload<LikelySubtagsV1Marker>,
}

#[inline]
fn update_langid(
    language: Language,
    script: Option<Script>,
    region: Option<Region>,
    langid: &mut LanguageIdentifier,
) -> TransformResult {
    let mut modified = false;

    if langid.language.is_empty() && !language.is_empty() {
        langid.language = language;
        modified = true;
    }

    if langid.script.is_none() && script.is_some() {
        langid.script = script;
        modified = true;
    }

    if langid.region.is_none() && region.is_some() {
        langid.region = region;
        modified = true;
    }

    if modified {
        TransformResult::Modified
    } else {
        TransformResult::Unmodified
    }
}

impl LocaleExpander {
    /// A constructor which takes a [`DataProvider`] and creates a [`LocaleExpander`].
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_unstable<P>(provider: &P) -> Result<LocaleExpander, LocaleTransformError>
    where
        P: DataProvider<LikelySubtagsV1Marker> + ?Sized,
    {
        let likely_subtags: DataPayload<LikelySubtagsV1Marker> =
            provider.load(Default::default())?.take_payload()?;

        Ok(LocaleExpander { likely_subtags })
    }

    icu_provider::gen_any_buffer_constructors!(
        locale: skip,
        options: skip,
        error: LocaleTransformError
    );

    /// The maximize method potentially updates a passed in locale in place
    /// depending up the results of running the 'Add Likely Subtags' algorithm
    /// from <https://www.unicode.org/reports/tr35/#Likely_Subtags>.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns [`TransformResult::Modified`]. Otherwise, the method
    /// returns [`TransformResult::Unmodified`] and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_locid_transform::{LocaleExpander, TransformResult};
    ///
    /// let lc = LocaleExpander::try_new_unstable(&icu_testdata::unstable())
    ///     .expect("create failed");
    ///
    /// let mut locale = locale!("zh-CN");
    /// assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
    /// assert_eq!(locale, locale!("zh-Hans-CN"));
    ///
    /// let mut locale = locale!("zh-Hant-TW");
    /// assert_eq!(lc.maximize(&mut locale), TransformResult::Unmodified);
    /// assert_eq!(locale, locale!("zh-Hant-TW"));
    /// ```
    pub fn maximize<T: AsMut<LanguageIdentifier>>(&self, mut langid: T) -> TransformResult {
        let langid = langid.as_mut();
        let data = self.likely_subtags.get();

        if !langid.language.is_empty() && langid.script.is_some() && langid.region.is_some() {
            return TransformResult::Unmodified;
        }

        if !langid.language.is_empty() {
            if let Some(region) = langid.region {
                if let Some(script) = data
                    .language_region
                    .get(&(langid.language.into(), region.into()))
                    .copied()
                {
                    return update_langid(Language::UND, Some(script), None, langid);
                }
            }
            if let Some(script) = langid.script {
                if let Some(region) = data
                    .language_script
                    .get(&(langid.language.into(), script.into()))
                    .copied()
                {
                    return update_langid(Language::UND, None, Some(region), langid);
                }
            }
            if let Some((script, region)) = data
                .language
                .get(&langid.language.into())
                .map(|u| zerovec::ule::AsULE::from_unaligned(*u))
            {
                return update_langid(Language::UND, Some(script), Some(region), langid);
            }
        }
        if let Some(script) = langid.script {
            if let Some(region) = langid.region {
                if let Some(language) = data
                    .script_region
                    .get(&(script.into(), region.into()))
                    .copied()
                {
                    return update_langid(language, None, None, langid);
                }
            }
            if let Some((language, region)) = data
                .script
                .get(&script.into())
                .map(|u| zerovec::ule::AsULE::from_unaligned(*u))
            {
                return update_langid(language, None, Some(region), langid);
            }
        }
        if let Some(region) = langid.region {
            if let Some((language, script)) = data
                .region
                .get(&region.into())
                .map(|u| zerovec::ule::AsULE::from_unaligned(*u))
            {
                return update_langid(language, Some(script), None, langid);
            }
        }

        update_langid(data.und.0, Some(data.und.1), Some(data.und.2), langid)
    }

    /// This returns a new Locale that is the result of running the
    /// 'Remove Likely Subtags' algorithm from
    /// <https://www.unicode.org/reports/tr35/#Likely_Subtags>.
    ///
    /// If the result of running the algorithm would result in a new locale, the
    /// locale argument is updated in place to match the result, and the method
    /// returns [`TransformResult::Modified`]. Otherwise, the method
    /// returns [`TransformResult::Unmodified`] and the locale argument is
    /// unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use icu_locid::locale;
    /// use icu_locid_transform::{LocaleExpander, TransformResult};
    ///
    /// let lc = LocaleExpander::try_new_unstable(&icu_testdata::unstable())
    ///     .expect("creation failed");
    ///
    /// let mut locale = locale!("zh-Hans-CN");
    /// assert_eq!(lc.minimize(&mut locale), TransformResult::Modified);
    /// assert_eq!(locale, locale!("zh"));
    ///
    /// let mut locale = locale!("zh");
    /// assert_eq!(lc.minimize(&mut locale), TransformResult::Unmodified);
    /// assert_eq!(locale, locale!("zh"));
    /// ```
    pub fn minimize<T: AsMut<LanguageIdentifier>>(&self, mut langid: T) -> TransformResult {
        let langid = langid.as_mut();

        let mut max = langid.clone();
        self.maximize(&mut max);
        let variants = mem::take(&mut max.variants);
        max.variants.clear();
        let mut trial = max.clone();

        trial.script = None;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if langid.language != max.language || langid.script.is_some() || langid.region.is_some()
            {
                if langid.language != max.language {
                    langid.language = max.language
                }
                if langid.script.is_some() {
                    langid.script = None;
                }
                if langid.region.is_some() {
                    langid.region = None;
                }
                langid.variants = variants;
                return TransformResult::Modified;
            } else {
                return TransformResult::Unmodified;
            }
        }

        trial.script = None;
        trial.region = max.region;
        self.maximize(&mut trial);
        if trial == max {
            if langid.language != max.language
                || langid.script.is_some()
                || langid.region != max.region
            {
                if langid.language != max.language {
                    langid.language = max.language
                }
                if langid.script.is_some() {
                    langid.script = None;
                }
                if langid.region != max.region {
                    langid.region = max.region;
                }
                langid.variants = variants;
                return TransformResult::Modified;
            } else {
                return TransformResult::Unmodified;
            }
        }

        trial.script = max.script;
        trial.region = None;
        self.maximize(&mut trial);
        if trial == max {
            if langid.language != max.language
                || langid.script != max.script
                || langid.region.is_some()
            {
                if langid.language != max.language {
                    langid.language = max.language
                }
                if langid.script != max.script {
                    langid.script = max.script;
                }
                if langid.region.is_some() {
                    langid.region = None;
                }
                langid.variants = variants;
                return TransformResult::Modified;
            } else {
                return TransformResult::Unmodified;
            }
        }

        if langid.language != max.language
            || langid.script != max.script
            || langid.region != max.region
        {
            if langid.language != max.language {
                langid.language = max.language
            }
            if langid.script != max.script {
                langid.script = max.script;
            }
            if langid.region != max.region {
                langid.region = max.region;
            }
            TransformResult::Modified
        } else {
            TransformResult::Unmodified
        }
    }
}
