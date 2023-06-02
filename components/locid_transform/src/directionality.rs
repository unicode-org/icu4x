use crate::provider::*;
use crate::{LocaleExpander, LocaleTransformError};
use icu_locid::subtags::Script;
use icu_locid::{subtags_script, Locale};
use icu_provider::{DataPayload, DataProvider};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    LeftToRight,
    RightToLeft,
    Unknown,
}

#[derive(Debug)]
pub struct LocaleDirectionality {
    rtl: DataPayload<DirectionalityV1Marker>,
    expander: LocaleExpander,
}

impl LocaleDirectionality {
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

    pub fn try_new_with_expander_unstable<P>(
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

    pub fn get(&self, locale: &Locale) -> Direction {
        let mut locale = locale.clone();

        // 1. Maximize the locale to utilize the likely subtags data
        let _transform_result = self.expander.maximize(&mut locale);
        // 2. Get script subtag of the locale
        let script = script_of_locale(&locale);

        // 3. Get the directionality of the script
        let directionality = *self
            .rtl
            .get()
            .rtl
            .get(&script.into_tinystr().to_unvalidated())
            // TODO: Do we make `LocaleDirectionality::get` fallible despite the DataPayload
            // being defined to contain all scripts?
            .unwrap_or_else(|| panic!("Script {script} not found in directionality data"));
        let directionality = <Option<_> as zerovec::ule::AsULE>::from_unaligned(directionality);
        match directionality {
            Some(true) => Direction::RightToLeft,
            Some(false) => Direction::LeftToRight,
            None => Direction::Unknown,
        }
    }

    pub fn is_right_to_left(&self, locale: &Locale) -> bool {
        self.get(locale) == Direction::RightToLeft
    }

    pub fn is_left_to_right(&self, locale: &Locale) -> bool {
        self.get(locale) == Direction::LeftToRight
    }
}

fn script_of_locale(locale: &Locale) -> Script {
    match locale.id.script {
        Some(script) => script,
        None => subtags_script!("Zzzz"),
    }
}
