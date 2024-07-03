// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use displaydoc::Display;
use icu_locale::LanguageIdentifier;
use icu_locale::ParseError;
use std::hash::Hash;
use std::str::FromStr;
use writeable::Writeable;

/// A family of locales to export.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LocaleFamily {
    pub(crate) langid: Option<LanguageIdentifier>,
    pub(crate) annotations: LocaleFamilyAnnotations,
}

impl LocaleFamily {
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
    pub const fn with_descendants(langid: LanguageIdentifier) -> Self {
        let annotations = if langid.is_empty() {
            LocaleFamilyAnnotations::single()
        } else {
            LocaleFamilyAnnotations::with_descendants()
        };

        Self {
            langid: Some(langid),
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
    pub const fn without_descendants(langid: LanguageIdentifier) -> Self {
        let annotations = if langid.is_empty() {
            LocaleFamilyAnnotations::single()
        } else {
            LocaleFamilyAnnotations::without_descendants()
        };
        Self {
            langid: Some(langid),
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
    pub const fn without_ancestors(langid: LanguageIdentifier) -> Self {
        let annotations = if langid.is_empty() {
            LocaleFamilyAnnotations::single()
        } else {
            LocaleFamilyAnnotations::without_ancestors()
        };
        Self {
            langid: Some(langid),
            annotations,
        }
    }

    /// The family containing only the selected locale.
    ///
    /// For example, the family `::single("en-001")` contains only "en-001".
    ///
    /// Stylized on the CLI as: "@en-US"
    pub const fn single(langid: LanguageIdentifier) -> Self {
        Self {
            langid: Some(langid),
            annotations: LocaleFamilyAnnotations::single(),
        }
    }

    /// The family containing all locales.
    ///
    /// Stylized on the CLI as: "full"
    pub const FULL: Self = Self {
        langid: None,
        annotations: LocaleFamilyAnnotations {
            include_ancestors: false,
            include_descendants: true,
        },
    };
}

impl Writeable for LocaleFamily {
    fn write_to<W: core::fmt::Write + ?Sized>(&self, sink: &mut W) -> core::fmt::Result {
        if let Some(langid) = self.langid.as_ref() {
            self.annotations.write_to(sink)?;
            langid.write_to(sink)
        } else {
            sink.write_str("full")
        }
    }

    fn writeable_length_hint(&self) -> writeable::LengthHint {
        if let Some(langid) = self.langid.as_ref() {
            self.annotations.writeable_length_hint() + langid.writeable_length_hint()
        } else {
            writeable::LengthHint::exact(4)
        }
    }
}

writeable::impl_display_with_writeable!(LocaleFamily);

/// Inner fields of a [`LocaleFamily`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(crate) struct LocaleFamilyAnnotations {
    pub(crate) include_ancestors: bool,
    pub(crate) include_descendants: bool,
}

impl LocaleFamilyAnnotations {
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

impl Writeable for LocaleFamilyAnnotations {
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

/// An error while parsing a [`LocaleFamily`].
#[derive(Debug, Copy, Clone, PartialEq, Display)]
#[non_exhaustive]
pub enum LocaleFamilyParseError {
    /// An error bubbled up from parsing a [`LanguageIdentifier`].
    #[displaydoc("{0}")]
    LanguageIdentifier(ParseError),
    /// Some other error specific to parsing the family, such as an invalid lead byte.
    #[displaydoc("Invalid locale family")]
    InvalidFamily,
}

impl From<ParseError> for LocaleFamilyParseError {
    fn from(err: ParseError) -> Self {
        Self::LanguageIdentifier(err)
    }
}

impl std::error::Error for LocaleFamilyParseError {}

impl FromStr for LocaleFamily {
    type Err = LocaleFamilyParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "full" {
            return Ok(Self::FULL);
        }
        let (first, remainder) = s
            .as_bytes()
            .split_first()
            .ok_or(LocaleFamilyParseError::InvalidFamily)?;
        match first {
            b'^' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_utf8(remainder)?),
                annotations: LocaleFamilyAnnotations::without_descendants(),
            }),
            b'%' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_utf8(remainder)?),
                annotations: LocaleFamilyAnnotations::without_ancestors(),
            }),
            b'@' => Ok(Self {
                langid: Some(LanguageIdentifier::try_from_utf8(remainder)?),
                annotations: LocaleFamilyAnnotations::single(),
            }),
            b if b.is_ascii_alphanumeric() => Ok(Self {
                langid: Some(s.parse()?),
                annotations: LocaleFamilyAnnotations::with_descendants(),
            }),
            _ => Err(LocaleFamilyParseError::InvalidFamily),
        }
    }
}

#[test]
fn test_locale_family_parsing() {
    let valid_families = ["und", "de-CH", "^es", "@pt-BR", "%en-001", "full"];
    let invalid_families = ["invalid", "@invalid", "-foo", "@full", "full-001"];
    for family_str in valid_families {
        let family = family_str.parse::<LocaleFamily>().unwrap();
        let family_to_str = family.to_string();
        assert_eq!(family_str, family_to_str);
    }
    for family_str in invalid_families {
        assert!(family_str.parse::<LocaleFamily>().is_err());
    }
}
