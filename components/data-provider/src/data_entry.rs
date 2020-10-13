// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/master/LICENSE ).
use icu_locale::LanguageIdentifier;
use std::borrow::Borrow;
use std::borrow::Cow;
use std::fmt;
use std::fmt::Write;

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
        write!(f, "DataEntry{{{}}}", self)
    }
}

impl fmt::Display for DataEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let components = self.get_components();
        let mut it = components.iter();
        if let Some(s) = it.next() {
            f.write_str(s)?;
            for s in it {
                f.write_char('/')?;
                f.write_str(s)?;
            }
        }
        Ok(())
    }
}

impl DataEntry {
    /// Gets the standard path components of this DataEntry. These components should be used when
    /// persisting the DataEntry on the filesystem or in structured data.
    ///
    /// # Example
    ///
    /// ```
    /// use std::borrow::Cow;
    /// use icu_data_provider::prelude::*;
    /// use icu_locale_macros::langid;
    ///
    /// let data_entry = DataEntry {
    ///     variant: Some(Cow::Borrowed("GBP")),
    ///     langid: langid!("pt_BR"),
    /// };
    /// let components = data_entry.get_components();
    ///
    /// assert_eq!(
    ///     ["GBP", "pt-BR"],
    ///     components.iter().collect::<Vec<&str>>()[..]
    /// );
    /// ```
    pub fn get_components(&self) -> DataEntryComponents {
        self.into()
    }
}

/// The standard components of a DataEntry path.
pub struct DataEntryComponents {
    components: [Option<Cow<'static, str>>; 2],
}

impl DataEntryComponents {
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        self.components
            .iter()
            .filter_map(|option| option.as_ref().map(|cow| cow.borrow()))
    }
}

impl From<&DataEntry> for DataEntryComponents {
    fn from(data_entry: &DataEntry) -> Self {
        DataEntryComponents {
            components: [
                if let Some(variant) = &data_entry.variant {
                    // Does not actually clone if the variant is borrowed
                    Some(variant.clone())
                } else {
                    None
                },
                Some(Cow::Owned(data_entry.langid.to_string())),
            ],
        }
    }
}

#[test]
fn test_to_string() {
    use icu_locale_macros::langid;

    struct TestCase {
        pub data_entry: DataEntry,
        pub expected: &'static str,
    }
    let cases = [
        TestCase {
            data_entry: DataEntry {
                variant: None,
                langid: LanguageIdentifier::default(),
            },
            expected: "und",
        },
        TestCase {
            data_entry: DataEntry {
                variant: Some(Cow::Borrowed("GBP")),
                langid: LanguageIdentifier::default(),
            },
            expected: "GBP/und",
        },
        TestCase {
            data_entry: DataEntry {
                variant: Some(Cow::Borrowed("GBP")),
                langid: langid!("en-ZA"),
            },
            expected: "GBP/en-ZA",
        },
    ];
    for cas in cases.iter() {
        assert_eq!(cas.expected, cas.data_entry.to_string());
        assert_eq!(
            cas.expected,
            cas.data_entry
                .get_components()
                .iter()
                .collect::<Vec<&str>>()
                .join("/")
        );
    }
}
