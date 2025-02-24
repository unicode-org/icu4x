use crate::posix::parse::PosixLocale;
use icu_locale::Locale;

fn expect_success(src: &str, expected: &str) {
    let posix_locale =
        PosixLocale::try_from_str(src).expect(&format!("Unable to parse POSIX locale: `{src}`"));
    let converted_locale = posix_locale
        .try_convert_lossy()
        .expect(&format!("Unable to convert to BCP-47 identifier: `{src}`"));

    let expected_locale = Locale::try_from_str(expected)
        .expect(&format!("Unable to parse BCP-47 identifier: `{expected}`"));
    assert_eq!(converted_locale, expected_locale, "POSIX locale: `{src}`");
}

#[test]
fn default_locale() {
    expect_success("C", "und-posix");
    expect_success("POSIX", "und-posix");
}

#[test]
fn region() {
    expect_success("en_US", "en-US-posix");
    expect_success("ne_NP", "ne-NP-posix");
    expect_success("zh_TW", "zh-TW-posix");
}

#[test]
fn codeset_ignored() {
    expect_success("lv_LV.iso885913", "lv-LV-posix");
    expect_success("hy_AM.armscii8", "hy-AM-posix");
}

#[test]
fn modifier() {
    // Currency
    expect_success("it_IT@euro", "it-IT-posix-u-cu-eur");

    // Script
    expect_success("uz_UZ@cyrillic", "uz-Cyrl-UZ-posix");
    expect_success("sd_IN@devanagari", "sd-Deva-IN-posix");
    expect_success("sr_RS@latin", "sr-Latn-RS-posix");

    // Language
    expect_success("aa_ER@saaho", "ssy-ER-posix");

    // Variant
    expect_success("ca_ES@valencia", "ca-ES-posix-valencia");
}

mod error {
    mod parse {
        use crate::posix::parse::{ParseError, PosixLocale};

        fn expect_error(src: &str, expected: ParseError) {
            let result = PosixLocale::try_from_str(src);
            match result {
                Ok(invalid_locale) => {
                    panic!("Expected the error `{expected:?}`, got the locale `{invalid_locale:?}` from input of `{src}`")
                }
                Err(error) => {
                    assert_eq!(error, expected, "Comparing expected output of `{src}`")
                }
            }
        }

        #[test]
        fn empty_locale() {
            expect_error("", ParseError::EmptyLocale);
        }

        #[test]
        fn empty_section() {
            // Single, empty optional section
            expect_error("en_", ParseError::EmptySection { offset: 2 });
            expect_error("en.", ParseError::EmptySection { offset: 2 });
            expect_error("en@", ParseError::EmptySection { offset: 2 });

            // Multiple optional sections, one empty
            expect_error("en_.utf8@euro", ParseError::EmptySection { offset: 2 });
            expect_error("en_US.@euro", ParseError::EmptySection { offset: 5 });
            expect_error("en_US.utf8@", ParseError::EmptySection { offset: 10 });

            // Single delimiter (excluding "." as that should return `ParseError::InvalidLocale` instead)
            expect_error("_", ParseError::EmptySection { offset: 0 });
            expect_error("@", ParseError::EmptySection { offset: 0 });

            // All delimiters
            expect_error("_.@", ParseError::EmptySection { offset: 0 });
        }

        #[test]
        fn invalid_character() {
            const SAMPLE_LOCALES: [&str; 2] = [
                "en",              // No optional fields
                "en_US.utf8@euro", // All optional fields
            ];

            for locale in SAMPLE_LOCALES {
                // Insert an invalid character ('/') at every position along the sample locale
                for offset in 0..=locale.len() {
                    let (left, right) = locale.split_at(offset);
                    let invalid_locale = format!("{left}/{right}");
                    expect_error(&invalid_locale, ParseError::InvalidCharacter { offset });
                }
            }

            // Test a single '/' character
            expect_error("/", ParseError::InvalidCharacter { offset: 0 });
        }

        #[test]
        fn invalid_locale() {
            expect_error(".", ParseError::InvalidLocale);
            expect_error("..", ParseError::InvalidLocale);
        }

        #[test]
        fn repeated_delimiter() {
            // Repeated delimiter at the end of locale
            expect_error(
                "en_US.utf8@euro_US",
                ParseError::RepeatedDelimiter {
                    first_offset: 2,
                    second_offset: 15,
                },
            );
            expect_error(
                "en_US.utf8@euro.utf8",
                ParseError::RepeatedDelimiter {
                    first_offset: 5,
                    second_offset: 15,
                },
            );
            expect_error(
                "en_US.utf8@euro@euro",
                ParseError::RepeatedDelimiter {
                    first_offset: 10,
                    second_offset: 15,
                },
            );

            // Multiple repeated delimiters
            expect_error(
                "en.utf8.utf8.utf8",
                ParseError::RepeatedDelimiter {
                    first_offset: 2,
                    second_offset: 7,
                },
            );

            // Consecutive repeated delimiters
            expect_error(
                "en__US.utf8@euro",
                ParseError::RepeatedDelimiter {
                    first_offset: 2,
                    second_offset: 3,
                },
            );
            expect_error(
                "en_US..utf8@euro",
                ParseError::RepeatedDelimiter {
                    first_offset: 5,
                    second_offset: 6,
                },
            );
            expect_error(
                "en_US.utf8@@euro",
                ParseError::RepeatedDelimiter {
                    first_offset: 10,
                    second_offset: 11,
                },
            );
        }

        #[test]
        fn unordered_delimiter() {
            expect_error(
                "en_US@euro.utf8",
                ParseError::UnorderedDelimiter {
                    first_offset: 5,
                    second_offset: 10,
                },
            );
            expect_error(
                "en.utf8_US@euro",
                ParseError::UnorderedDelimiter {
                    first_offset: 2,
                    second_offset: 7,
                },
            );
            expect_error(
                "en.utf8@euro_US",
                ParseError::UnorderedDelimiter {
                    first_offset: 7,
                    second_offset: 12,
                },
            );
            expect_error(
                "en@euro_US.utf8",
                ParseError::UnorderedDelimiter {
                    first_offset: 2,
                    second_offset: 7,
                },
            );
            expect_error(
                "en@euro.utf8_US",
                ParseError::UnorderedDelimiter {
                    first_offset: 2,
                    second_offset: 7,
                },
            );
        }

        #[test]
        fn offset() {
            // Empty section
            let src = "en_.utf8@euro";
            match PosixLocale::try_from_str(src) {
                Err(ParseError::EmptySection { offset }) => {
                    assert_eq!(&src[offset..offset + 1], "_");
                }
                _ => unreachable!(),
            }

            // Invalid character
            let src = "en_U/S";
            match PosixLocale::try_from_str(src) {
                Err(ParseError::InvalidCharacter { offset }) => {
                    assert_eq!(&src[offset..offset + 1], "/");
                }
                _ => unreachable!(),
            }

            // Repeated delimiter
            let src = "en_US.utf8@euro_US";
            match PosixLocale::try_from_str(src) {
                Err(ParseError::RepeatedDelimiter {
                    first_offset,
                    second_offset,
                }) => {
                    assert_eq!(&src[first_offset..first_offset + 1], "_");
                    assert_eq!(&src[second_offset..second_offset + 1], "_");
                }
                _ => unreachable!(),
            }

            // Unordered delimiter
            let src = "en_US@euro.utf8";
            match PosixLocale::try_from_str(src) {
                Err(ParseError::UnorderedDelimiter {
                    first_offset,
                    second_offset,
                }) => {
                    assert_eq!(&src[first_offset..first_offset + 1], "@");
                    assert_eq!(&src[second_offset..second_offset + 1], ".");
                }
                _ => unreachable!(),
            }
        }
    }

    mod conversion {
        use crate::posix::parse::{ConversionError, PosixLocale};

        fn expect_error(src: &str, expected: ConversionError) {
            let result = PosixLocale::try_from_str(src)
                .expect(&format!("Unable to parse POSIX locale: `{src}`"))
                .try_convert_lossy();
            match result {
                Ok(invalid_locale) => {
                    panic!("Expected the error `{expected:?}`, got the locale `{invalid_locale:?}` from input of `{src}`")
                }
                Err(error) => {
                    assert_eq!(error, expected, "Comparing expected output of `{src}`")
                }
            }
        }

        #[test]
        fn invalid_language() {
            expect_error(
                "invalid",
                ConversionError::InvalidLanguage {
                    start_offset: 0,
                    end_offset: 7,
                },
            );
        }

        #[test]
        fn invalid_region() {
            expect_error(
                "en_invalid",
                ConversionError::InvalidRegion {
                    start_offset: 3,
                    end_offset: 10,
                },
            );
        }
    }
}
