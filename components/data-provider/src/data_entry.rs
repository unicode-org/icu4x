use icu_locale::LanguageIdentifier;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;
use std::path::PathBuf;

/// A variant and language identifier, used for requesting data from a DataProvider.
///
/// The fields in a DataEntry are not generally known until runtime.
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
        if let Some(variant) = &self.variant {
            f.write_str(&variant)?;
        }
        f.write_char('/')?;
        self.langid.fmt(f)?;
        Ok(())
    }
}

impl DataEntry {
    /// Append standard path components for this data entry to a PathBuf.
    ///
    /// # Example
    ///
    /// ```
    /// use std::path::PathBuf;
    /// use icu_data_provider::prelude::*;
    ///
    /// let data_entry = DataEntry {
    ///     variant: None,
    ///     langid: "pt_BR".parse().unwrap(),
    /// };
    /// let mut path_buf = PathBuf::new();
    /// data_entry.append_path_to(&mut path_buf);
    ///
    /// assert_eq!(Some("pt-BR"), path_buf.to_str());
    /// ```
    pub fn append_path_to(&self, path_buf: &mut PathBuf) {
        if let Some(variant) = &self.variant {
            path_buf.push(&variant as &str);
        }
        // TODO: Is there a alloc-less way to append the langid to the PathBuf?
        path_buf.push(self.langid.to_string());
    }
}
