// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locale::ParseError;
use icu_provider::DataLocale;
use std::hash::Hash;
use std::str::FromStr;
use writeable::Writeable;

/// A family of locales to export.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DataLocaleFamily {
    pub(crate) locale: Option<DataLocale>,
    pub(crate) annotations: DataLocaleFamilyAnnotations,
}

impl DataLocaleFamily {
    /// The family containing all ancestors and descendants of the selected locale.
    ///
    /// This is the recommended family type since it reflects regional preferences.
    ///
    /// For example, the family `::with_descendants("en-001")` contains:
    ///
    /// - Self: "en-001"
    /// - Ancestors: "und", "en"
    /// - Descendants: "en-GB", "en-ZA", ...
    ///
    /// Stylized on the CLI as: "en-US"
    ///
    /// The `und` locale is treated specially and behaves like `::single("und")`.
    pub fn with_descendants(locale: DataLocale) -> Self {
        let annotations = if locale.is_und() {
            DataLocaleFamilyAnnotations::single()
        } else {
            DataLocaleFamilyAnnotations::with_descendants()
        };

        Self {
            locale: Some(locale),
            annotations,
        }
    }

    /// The family containing all ancestors of the selected locale.
    ///
    /// This family type does not include regional variants unless the selected locale is itself
    /// a regional variant.
    ///
    /// For example, the family `::without_descendants("en-001")` contains:
    ///
    /// - Self: "en-001"
    /// - Ancestors: "und", "en"
    ///
    /// Stylized on the CLI as: "^en-US"
    ///
    /// The `und` locale is treated specially and behaves like `::single("und")`.
    pub fn without_descendants(locale: DataLocale) -> Self {
        let annotations = if locale.is_und() {
            DataLocaleFamilyAnnotations::single()
        } else {
            DataLocaleFamilyAnnotations::without_descendants()
        };
        Self {
            locale: Some(locale),
            annotations,
        }
    }

    /// The family containing all descendants of the selected locale.
    ///
    /// This family may be useful if the root locale is not desired.
    ///
    /// For example, the family `::without_ancestors("en-001")` contains:
    ///
    /// - Self: "en-001"
    /// - Descendants: "en-GB", "en-ZA", ...
    ///
    /// but it does _not_ contain the ancestors "en" and "und".
    ///
    /// Stylized on the CLI as: "%en-US"
    ///
    /// The `und` locale is treated specially and behaves like `::single("und")`.
    pub fn without_ancestors(locale: DataLocale) -> Self {
        let annotations = if locale.is_und() {
            DataLocaleFamilyAnnotations::single()
        } else {
            DataLocaleFamilyAnnotations::without_ancestors()
        };
        Self {
            locale: Some(locale),
            annotations,
        }
    }

    /// The family containing only the selected locale.
    ///
    /// For example, the family `::single("en-001")` contains only "en-001".
    ///
    /// Stylized on the CLI as: "@en-US"
    pub const fn single(locale: DataLocale) -> Self {
        Self {
            locale: Some(locale),
            annotations: DataLocaleFamilyAnnotations::single(),
        }
    }

    /// The family containing all locales.
    ///
    /// Stylized on the CLI as: "full"
    pub const FULL: Self = Self {
        locale: None,
        annotations: DataLocaleFamilyAnnotations {
            include_ancestors: false,
            include_descendants: true,
        },
    };
}

impl Writeable for DataLocaleFamily {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        if let Some(locale) = self.locale.as_ref() {
            self.annotations.write_to(sink)?;
            locale.write_to(sink)
        } else {
            sink.write_str("full")
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        if let Some(locale) = self.locale.as_ref() {
            self.annotations.writeable_length_hint() + locale.writeable_length_hint()
        } else {
            writeable::LengthHint::exact(4)
        }
    }
}

writeable::impl_display_with_writeable!(DataLocaleFamily);

/// Inner fields of a [`DataLocaleFamily`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct DataLocaleFamilyAnnotations {
    pub(crate) include_ancestors: bool,
    pub(crate) include_descendants: bool,
}

impl DataLocaleFamilyAnnotations {
    #[inline]
    pub(crate) const fn with_descendants() -> Self {
        Self {
            include_ancestors: true,
            include_descendants: true,
        }
    }

    #[inline]
    pub(crate) const fn without_descendants() -> Self {
        Self {
            include_ancestors: true,
            include_descendants: false,
        }
    }

    #[inline]
    pub(crate) const fn without_ancestors() -> Self {
        Self {
            include_ancestors: false,
            include_descendants: true,
        }
    }

    #[inline]
    pub(crate) const fn single() -> Self {
        Self {
            include_ancestors: false,
            include_descendants: false,
        }
    }
}

impl Writeable for DataLocaleFamilyAnnotations {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        match (self.include_ancestors, self.include_descendants) {
            (true, true) => Ok(()),
            (true, false) => sink.write_char('^'),
            (false, true) => sink.write_char('%'),
            (false, false) => sink.write_char('@'),
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        writeable::LengthHint::exact(match (self.include_ancestors, self.include_descendants) {
            (true, true) => 0,
            _ => 1,
        })
    }
}

/// An error while parsing a [`DataLocaleFamily`].
#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum DataLocaleFamilyParseError {
    /// An error bubbled up from parsing a [`DataLocale`].
    #[displaydoc("{0}")]
    Locale(ParseError),
    /// Some other error specific to parsing the family, such as an invalid lead byte.
    #[displaydoc("Invalid locale family")]
    InvalidFamily,
}

impl From<ParseError> for DataLocaleFamilyParseError {
    fn from(err: ParseError) -> Self {
        Self::Locale(err)
    }
}

impl std::error::Error for DataLocaleFamilyParseError {}

impl FromStr for DataLocaleFamily {
    type Err = DataLocaleFamilyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "full" {
            return Ok(Self::FULL);
        }
        let mut iter = s.chars();
        let first = iter
            .next()
            .ok_or(DataLocaleFamilyParseError::InvalidFamily)?;
        match first {
            '^' => Ok(Self {
                locale: Some(iter.as_str().parse()?),
                annotations: DataLocaleFamilyAnnotations::without_descendants(),
            }),
            '%' => Ok(Self {
                locale: Some(iter.as_str().parse()?),
                annotations: DataLocaleFamilyAnnotations::without_ancestors(),
            }),
            '@' => Ok(Self {
                locale: Some(iter.as_str().parse()?),
                annotations: DataLocaleFamilyAnnotations::single(),
            }),
            b if b.is_ascii_alphanumeric() => Ok(Self {
                locale: Some(s.parse()?),
                annotations: DataLocaleFamilyAnnotations::with_descendants(),
            }),
            _ => Err(DataLocaleFamilyParseError::InvalidFamily),
        }
    }
}

#[test]
fn test_locale_family_parsing() {
    let valid_families = ["und", "de-CH", "^es", "@pt-BR", "%en-001", "full"];
    let invalid_families = ["invalid", "@invalid", "-foo", "@full", "full-001"];
    for family_str in valid_families {
        let family = family_str.parse::<DataLocaleFamily>().unwrap();
        let family_to_str = family.to_string();
        assert_eq!(family_str, family_to_str);
    }
    for family_str in invalid_families {
        assert!(family_str.parse::<DataLocaleFamily>().is_err());
    }
}
