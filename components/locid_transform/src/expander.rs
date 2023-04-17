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
/// Add likely subtags:
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
/// Remove likely subtags:
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
/// Normally, only CLDR locales with Basic or higher coverage are included. To include more
/// locales for maximization, use [`try_new_extended`](Self::try_new_extended_unstable):
///
/// ```
/// use icu_locid::locale;
/// use icu_locid_transform::{LocaleExpander, TransformResult};
///
/// let lc =
///     LocaleExpander::try_new_extended_unstable(&icu_testdata::unstable())
///         .expect("create failed");
///
/// let mut locale = locale!("atj");
/// assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
/// assert_eq!(locale, locale!("atj-Latn-CA"));
/// ```
///
/// [`CLDR`]: http://cldr.unicode.org/
/// [`UTS #35: Unicode LDML 3. Likely Subtags`]: https://www.unicode.org/reports/tr35/#Likely_Subtags.
#[derive(Debug)]
pub struct LocaleExpander {
    likely_subtags_l: DataPayload<LikelySubtagsForLanguageV1Marker>,
    likely_subtags_sr: DataPayload<LikelySubtagsForScriptRegionV1Marker>,
    likely_subtags_ext: Option<DataPayload<LikelySubtagsExtendedV1Marker>>,
}

struct LocaleExpanderBorrowed<'a> {
    likely_subtags_l: &'a LikelySubtagsForLanguageV1<'a>,
    likely_subtags_sr: &'a LikelySubtagsForScriptRegionV1<'a>,
    likely_subtags_ext: Option<&'a LikelySubtagsExtendedV1<'a>>,
}

impl LocaleExpanderBorrowed<'_> {
    fn get_l(&self, l: Language) -> Option<(Script, Region)> {
        let key = &l.into();
        self.likely_subtags_l.language.get_copied(key).or_else(|| {
            self.likely_subtags_ext
                .and_then(|ext| ext.language.get_copied(key))
        })
    }

    fn get_ls(&self, l: Language, s: Script) -> Option<Region> {
        let key = &(l.into(), s.into());
        self.likely_subtags_l
            .language_script
            .get_copied(key)
            .or_else(|| {
                self.likely_subtags_ext
                    .and_then(|ext| ext.language_script.get_copied(key))
            })
    }

    fn get_lr(&self, l: Language, r: Region) -> Option<Script> {
        let key = &(l.into(), r.into());
        self.likely_subtags_l
            .language_region
            .get_copied(key)
            .or_else(|| {
                self.likely_subtags_ext
                    .and_then(|ext| ext.language_region.get_copied(key))
            })
    }

    fn get_s(&self, s: Script) -> Option<(Language, Region)> {
        let key = &s.into();
        self.likely_subtags_sr.script.get_copied(key).or_else(|| {
            self.likely_subtags_ext
                .and_then(|ext| ext.script.get_copied(key))
        })
    }

    fn get_sr(&self, s: Script, r: Region) -> Option<Language> {
        let key = &(s.into(), r.into());
        self.likely_subtags_sr
            .script_region
            .get_copied(key)
            .or_else(|| {
                self.likely_subtags_ext
                    .and_then(|ext| ext.script_region.get_copied(key))
            })
    }

    fn get_r(&self, r: Region) -> Option<(Language, Script)> {
        let key = &r.into();
        self.likely_subtags_sr.region.get_copied(key).or_else(|| {
            self.likely_subtags_ext
                .and_then(|ext| ext.region.get_copied(key))
        })
    }

    fn get_und(&self) -> (Language, Script, Region) {
        self.likely_subtags_l.und
    }
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
    /// Creates a [`LocaleExpander`] with data for CLDR locales with Basic or higher coverage.
    ///
    /// Use this contructor if you are using likely subtags for data lookup.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_unstable<P>(provider: &P) -> Result<LocaleExpander, LocaleTransformError>
    where
        P: DataProvider<LikelySubtagsForLanguageV1Marker>
            + DataProvider<LikelySubtagsForScriptRegionV1Marker>
            + ?Sized,
    {
        let likely_subtags_l = provider.load(Default::default())?.take_payload()?;
        let likely_subtags_sr = provider.load(Default::default())?.take_payload()?;

        Ok(LocaleExpander {
            likely_subtags_l,
            likely_subtags_sr,
            likely_subtags_ext: None,
        })
    }

    /// Creates a [`LocaleExpander`] with data for all locales.
    ///
    /// Use this constructor if you are using likely subtags for comprehensive support of all
    /// languages and regions, including ones that may not have CLDR data.
    ///
    /// [📚 Help choosing a constructor](icu_provider::constructors)
    /// <div class="stab unstable">
    /// ⚠️ The bounds on this function may change over time, including in SemVer minor releases.
    /// </div>
    pub fn try_new_extended_unstable<P>(
        provider: &P,
    ) -> Result<LocaleExpander, LocaleTransformError>
    where
        P: DataProvider<LikelySubtagsForLanguageV1Marker>
            + DataProvider<LikelySubtagsForScriptRegionV1Marker>
            + DataProvider<LikelySubtagsExtendedV1Marker>
            + ?Sized,
    {
        let likely_subtags_l = provider.load(Default::default())?.take_payload()?;
        let likely_subtags_sr = provider.load(Default::default())?.take_payload()?;
        let likely_subtags_ext = Some(provider.load(Default::default())?.take_payload()?);

        Ok(LocaleExpander {
            likely_subtags_l,
            likely_subtags_sr,
            likely_subtags_ext,
        })
    }

    icu_provider::gen_any_buffer_constructors!(locale: skip, options: skip, error: LocaleTransformError, functions: [
        Self::try_new_extended_unstable,
        try_new_extended_with_any_provider,
        try_new_extended_with_buffer_provider
    ]);

    fn try_new_compat<P>(provider: &P) -> Result<LocaleExpander, LocaleTransformError>
    where
        P: DataProvider<LikelySubtagsForLanguageV1Marker>
            + DataProvider<LikelySubtagsForScriptRegionV1Marker>
            + DataProvider<LikelySubtagsExtendedV1Marker>
            + DataProvider<LikelySubtagsV1Marker>
            + ?Sized,
    {
        let payload_l = provider
            .load(Default::default())
            .and_then(DataResponse::take_payload);
        let payload_sr = provider
            .load(Default::default())
            .and_then(DataResponse::take_payload);
        let payload_ext = provider
            .load(Default::default())
            .and_then(DataResponse::take_payload);

        let (likely_subtags_l, likely_subtags_sr, likely_subtags_ext) =
            match (payload_l, payload_sr, payload_ext) {
                (Ok(l), Ok(sr), Ok(ext)) => (l, sr, Some(ext)),
                _ => {
                    let result: DataPayload<LikelySubtagsV1Marker> =
                        provider.load(Default::default())?.take_payload()?;
                    (
                        result.map_project_cloned(|st, _| {
                            LikelySubtagsForLanguageV1::clone_from_borrowed(st)
                        }),
                        result.map_project(|st, _| st.into()),
                        None,
                    )
                }
            };

        Ok(LocaleExpander {
            likely_subtags_l,
            likely_subtags_sr,
            likely_subtags_ext,
        })
    }

    #[doc = icu_provider::gen_any_buffer_docs!(ANY, icu_provider, Self::try_new_unstable)]
    pub fn try_new_with_any_provider(
        provider: &impl AnyProvider,
    ) -> Result<LocaleExpander, LocaleTransformError> {
        Self::try_new_compat(&provider.as_downcasting())
    }

    #[doc = icu_provider::gen_any_buffer_docs!(BUFFER, icu_provider, Self::try_new_unstable)]
    #[cfg(feature = "serde")]
    pub fn try_new_with_buffer_provider(
        provider: &impl BufferProvider,
    ) -> Result<LocaleExpander, LocaleTransformError> {
        Self::try_new_compat(&provider.as_deserializing())
    }

    fn as_borrowed(&self) -> LocaleExpanderBorrowed {
        LocaleExpanderBorrowed {
            likely_subtags_l: self.likely_subtags_l.get(),
            likely_subtags_sr: self.likely_subtags_sr.get(),
            likely_subtags_ext: self.likely_subtags_ext.as_ref().map(|p| p.get()),
        }
    }

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
        let data = self.as_borrowed();

        if !langid.language.is_empty() && langid.script.is_some() && langid.region.is_some() {
            return TransformResult::Unmodified;
        }

        if !langid.language.is_empty() {
            if let Some(region) = langid.region {
                if let Some(script) = data.get_lr(langid.language, region) {
                    return update_langid(Language::UND, Some(script), None, langid);
                }
            }
            if let Some(script) = langid.script {
                if let Some(region) = data.get_ls(langid.language, script) {
                    return update_langid(Language::UND, None, Some(region), langid);
                }
            }
            if let Some((script, region)) = data.get_l(langid.language) {
                return update_langid(Language::UND, Some(script), Some(region), langid);
            }
        }
        if let Some(script) = langid.script {
            if let Some(region) = langid.region {
                if let Some(language) = data.get_sr(script, region) {
                    return update_langid(language, None, None, langid);
                }
            }
            if let Some((language, region)) = data.get_s(script) {
                return update_langid(language, None, Some(region), langid);
            }
        }
        if let Some(region) = langid.region {
            if let Some((language, script)) = data.get_r(region) {
                return update_langid(language, Some(script), None, langid);
            }
        }

        update_langid(
            data.get_und().0,
            Some(data.get_und().1),
            Some(data.get_und().2),
            langid,
        )
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

#[cfg(feature = "serde")]
#[cfg(test)]
mod tests {
    use super::*;
    use icu_locid::locale;

    struct RejectByKeyProvider<P> {
        keys: Vec<DataKey>,
        inner: P,
    }

    impl<P: BufferProvider> BufferProvider for RejectByKeyProvider<P> {
        fn load_buffer(
            &self,
            key: DataKey,
            req: DataRequest,
        ) -> Result<DataResponse<BufferMarker>, DataError> {
            if self.keys.contains(&key) {
                Err(DataErrorKind::MissingDataKey.with_str_context("rejected"))
            } else {
                self.inner.load_buffer(key, req)
            }
        }
    }

    #[test]
    fn test_old_keys() {
        let provider = RejectByKeyProvider {
            keys: vec![
                LikelySubtagsForLanguageV1Marker::KEY,
                LikelySubtagsForScriptRegionV1Marker::KEY,
            ],
            inner: icu_testdata::buffer(),
        };
        let lc = LocaleExpander::try_new_with_buffer_provider(&provider)
            .expect("should create with old keys");
        let mut locale = locale!("zh-CN");
        assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
        assert_eq!(locale, locale!("zh-Hans-CN"));
    }

    #[test]
    fn test_new_keys() {
        let provider = RejectByKeyProvider {
            keys: vec![LikelySubtagsV1Marker::KEY],
            inner: icu_testdata::buffer(),
        };
        let lc = LocaleExpander::try_new_with_buffer_provider(&provider)
            .expect("should create with new keys");
        let mut locale = locale!("zh-CN");
        assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
        assert_eq!(locale, locale!("zh-Hans-CN"));
    }

    #[test]
    fn test_mixed_keys() {
        // Include the old key and one of the new keys but not both new keys.
        // Not sure if this is a useful test.
        let provider = RejectByKeyProvider {
            keys: vec![LikelySubtagsForScriptRegionV1Marker::KEY],
            inner: icu_testdata::buffer(),
        };
        let lc = LocaleExpander::try_new_with_buffer_provider(&provider)
            .expect("should create with mixed keys");
        let mut locale = locale!("zh-CN");
        assert_eq!(lc.maximize(&mut locale), TransformResult::Modified);
        assert_eq!(locale, locale!("zh-Hans-CN"));
    }

    #[test]
    fn test_no_keys() {
        let provider = RejectByKeyProvider {
            keys: vec![
                LikelySubtagsForLanguageV1Marker::KEY,
                LikelySubtagsForScriptRegionV1Marker::KEY,
                LikelySubtagsV1Marker::KEY,
            ],
            inner: icu_testdata::buffer(),
        };
        if LocaleExpander::try_new_with_buffer_provider(&provider).is_ok() {
            panic!("should not create: no data present")
        };
    }
}
