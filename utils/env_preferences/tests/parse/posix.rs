// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use env_preferences::parse::posix::PosixLocale;
use icu_locale_core::Locale;

fn expect_success(src: &str, expected: &str) {
    let posix_locale = PosixLocale::try_from_str(src).expect(src);
    let converted_locale = posix_locale.try_convert_lossy().expect(src);

    let expected_locale = Locale::try_from_str(expected).expect(src);
    assert_eq!(converted_locale, expected_locale, "POSIX locale: `{src}`");
}

#[test]
fn default_locale() {
    expect_success("C", "en-US-posix");
    expect_success("POSIX", "en-US-posix");
}

#[test]
fn region() {
    expect_success("en_US", "en-US");
    expect_success("ne_NP", "ne-NP");
    expect_success("zh_TW", "zh-TW");
}

#[test]
fn codeset_ignored() {
    expect_success("lv_LV.iso885913", "lv-LV");
    expect_success("hy_AM.armscii8", "hy-AM");
}

#[test]
fn modifier() {
    // Currency
    expect_success("it_IT@euro", "it-IT-u-cu-eur");

    // Script
    expect_success("uz_UZ@cyrillic", "uz-Cyrl-UZ");
    expect_success("sd_IN@devanagari", "sd-Deva-IN");
    expect_success("sr_RS@latin", "sr-Latn-RS");

    // Language
    expect_success("aa_ER@saaho", "ssy-ER");

    // Variant
    expect_success("ca_ES@valencia", "ca-ES-valencia");
}

mod error {
    mod parse {
        use env_preferences::parse::posix::{PosixLocale, PosixParseError};
        use env_preferences::ParseError;

        fn expect_error(src: &str, posix_error: PosixParseError) {
            let result = PosixLocale::try_from_str(src);
            let expected = ParseError::Posix(posix_error);

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
            expect_error("", PosixParseError::EmptyLocale);
        }

        #[test]
        fn empty_section() {
            // Single, empty optional section
            expect_error("en_", PosixParseError::EmptySection { offset: 2 });
            expect_error("en.", PosixParseError::EmptySection { offset: 2 });
            expect_error("en@", PosixParseError::EmptySection { offset: 2 });

            // Multiple optional sections, one empty
            expect_error("en_.utf8@euro", PosixParseError::EmptySection { offset: 2 });
            expect_error("en_US.@euro", PosixParseError::EmptySection { offset: 5 });
            expect_error("en_US.utf8@", PosixParseError::EmptySection { offset: 10 });

            // Single delimiter (excluding "." as that should return `PosixParseError::InvalidLocale` instead)
            expect_error("_", PosixParseError::EmptySection { offset: 0 });
            expect_error("@", PosixParseError::EmptySection { offset: 0 });

            // All delimiters
            expect_error("_.@", PosixParseError::EmptySection { offset: 0 });
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
                    expect_error(
                        &invalid_locale,
                        PosixParseError::InvalidCharacter { offset },
                    );
                }
            }

            // Test a single '/' character
            expect_error("/", PosixParseError::InvalidCharacter { offset: 0 });
        }

        #[test]
        fn invalid_locale() {
            expect_error(".", PosixParseError::InvalidLocale);
            expect_error("..", PosixParseError::InvalidLocale);
        }

        #[test]
        fn repeated_delimiter() {
            // Repeated delimiter at the end of locale
            expect_error(
                "en_US.utf8@euro_US",
                PosixParseError::RepeatedDelimiter {
                    first_offset: 2,
                    second_offset: 15,
                },
            );
            expect_error(
                "en_US.utf8@euro.utf8",
                PosixParseError::RepeatedDelimiter {
                    first_offset: 5,
                    second_offset: 15,
                },
            );
            expect_error(
                "en_US.utf8@euro@euro",
                PosixParseError::RepeatedDelimiter {
                    first_offset: 10,
                    second_offset: 15,
                },
            );

            // Multiple repeated delimiters
            expect_error(
                "en.utf8.utf8.utf8",
                PosixParseError::RepeatedDelimiter {
                    first_offset: 2,
                    second_offset: 7,
                },
            );

            // Consecutive repeated delimiters
            expect_error(
                "en__US.utf8@euro",
                PosixParseError::RepeatedDelimiter {
                    first_offset: 2,
                    second_offset: 3,
                },
            );
            expect_error(
                "en_US..utf8@euro",
                PosixParseError::RepeatedDelimiter {
                    first_offset: 5,
                    second_offset: 6,
                },
            );
            expect_error(
                "en_US.utf8@@euro",
                PosixParseError::RepeatedDelimiter {
                    first_offset: 10,
                    second_offset: 11,
                },
            );
        }

        #[test]
        fn unordered_delimiter() {
            expect_error(
                "en_US@euro.utf8",
                PosixParseError::UnorderedDelimiter {
                    first_offset: 5,
                    second_offset: 10,
                },
            );
            expect_error(
                "en.utf8_US@euro",
                PosixParseError::UnorderedDelimiter {
                    first_offset: 2,
                    second_offset: 7,
                },
            );
            expect_error(
                "en.utf8@euro_US",
                PosixParseError::UnorderedDelimiter {
                    first_offset: 7,
                    second_offset: 12,
                },
            );
            expect_error(
                "en@euro_US.utf8",
                PosixParseError::UnorderedDelimiter {
                    first_offset: 2,
                    second_offset: 7,
                },
            );
            expect_error(
                "en@euro.utf8_US",
                PosixParseError::UnorderedDelimiter {
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
                Err(ParseError::Posix(PosixParseError::EmptySection { offset })) => {
                    assert_eq!(&src[offset..offset + 1], "_");
                }
                _ => unreachable!(),
            }

            // Invalid character
            let src = "en_U/S";
            match PosixLocale::try_from_str(src) {
                Err(ParseError::Posix(PosixParseError::InvalidCharacter { offset })) => {
                    assert_eq!(&src[offset..offset + 1], "/");
                }
                _ => unreachable!(),
            }

            // Repeated delimiter
            let src = "en_US.utf8@euro_US";
            match PosixLocale::try_from_str(src) {
                Err(ParseError::Posix(PosixParseError::RepeatedDelimiter {
                    first_offset,
                    second_offset,
                })) => {
                    assert_eq!(&src[first_offset..first_offset + 1], "_");
                    assert_eq!(&src[second_offset..second_offset + 1], "_");
                }
                _ => unreachable!(),
            }

            // Unordered delimiter
            let src = "en_US@euro.utf8";
            match PosixLocale::try_from_str(src) {
                Err(ParseError::Posix(PosixParseError::UnorderedDelimiter {
                    first_offset,
                    second_offset,
                })) => {
                    assert_eq!(&src[first_offset..first_offset + 1], "@");
                    assert_eq!(&src[second_offset..second_offset + 1], ".");
                }
                _ => unreachable!(),
            }
        }
    }

    mod conversion {
        use env_preferences::parse::posix::PosixLocale;

        fn expect_error(src: &str, icu_error: icu_locale_core::ParseError) {
            let result = PosixLocale::try_from_str(src)
                .expect(src)
                .try_convert_lossy();
            let expected = env_preferences::ParseError::Icu(icu_error);
            match result {
                Ok(invalid_locale) => {
                    panic!("Expected the error `{icu_error:?}`, got the locale `{invalid_locale:?}` from input of `{src}`")
                }
                Err(error) => {
                    assert_eq!(error, expected, "Comparing expected output of `{src}`")
                }
            }
        }

        #[test]
        fn invalid_language() {
            expect_error("invalid", icu_locale_core::ParseError::InvalidLanguage);
        }

        #[test]
        fn invalid_region() {
            expect_error("en_invalid", icu_locale_core::ParseError::InvalidSubtag);
        }
    }
}
