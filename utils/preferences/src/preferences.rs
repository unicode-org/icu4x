use icu_locid::subtags::{Language, Region, Script};
use icu_locid::Locale;

pub trait Preferences {
    fn language(&self) -> &Language {
        &Language::UND
    }
    fn script(&self) -> Option<&Script> {
        None
    }
    fn region(&self) -> Option<&Region> {
        None
    }
}

impl Preferences for Locale {
    fn language(&self) -> &Language {
        &self.id.language
    }

    fn script(&self) -> Option<&Script> {
        self.id.script.as_ref()
    }

    fn region(&self) -> Option<&Region> {
        self.id.region.as_ref()
    }
}
