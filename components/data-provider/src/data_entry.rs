use icu_locale::LanguageIdentifier;
use std::borrow::Cow;
use std::fmt;

/// A variant and language identifier, used for requesting data from a DataProvider.
///
/// All of the fields in a DataEntry should be resolved at runtime.
#[derive(PartialEq, Clone)]
pub struct DataEntry {
    // TODO: Consider making this a list of variants
    pub variant: Option<Cow<'static, str>>,
    pub langid: LanguageIdentifier,
}

impl fmt::Debug for DataEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.variant {
            Some(variant) => write!(f, "DataEntry({}/{})", variant, self.langid),
            None => write!(f, "DataEntry({})", self.langid),
        }
    }
}

impl fmt::Display for DataEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.variant {
            Some(variant) => write!(f, "{}/{}", variant, self.langid),
            None => write!(f, "{}", self.langid),
        }
    }
}
