// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Algorithms to determine where to position grouping separators.

use crate::options::GroupingStrategy;
use crate::provider::GroupingSizes;
use core::cmp;

/// Returns whether to display a grouping separator at the given magnitude.
///
/// `upper_magnitude` is the magnitude of the highest-power digit, used for resolving minimum
/// grouping digits.
pub fn check(
    upper_magnitude: i16,
    magnitude: i16,
    strategy: GroupingStrategy,
    sizes: GroupingSizes,
) -> bool {
    let primary = if sizes.primary == 0 {
        return false;
    } else {
        sizes.primary as i16
    };
    if magnitude < primary {
        return false;
    }
    let min_grouping = {
        use GroupingStrategy::*;
        match strategy {
            Never => return false,
            // Note: Auto and Always are the same for DecimalFormatter.
            // When currencies are implemented, this will change.
            Auto | Always => cmp::max(1, sizes.min_grouping) as i16,
            Min2 => cmp::max(2, sizes.min_grouping) as i16,
        }
    };
    if upper_magnitude < primary + min_grouping - 1 {
        return false;
    }
    let secondary = if sizes.secondary == 0 {
        primary
    } else {
        sizes.secondary as i16
    };
    let magnitude_prime = magnitude - primary;
    if magnitude_prime % secondary == 0 {
        return true;
    }
    false
}

/// Returns whether to display a grouping separator at the given magnitude.
///
/// `lower_magnitude` is the magnitude of the lowest-power digit, used for resolving minimum
/// grouping digits.
/// `magnitude` is the magnitude of the digit, which is negative for fraction digits.
pub fn check_fraction(
    lower_magnitude: i16,
    magnitude: i16,
    strategy: GroupingStrategy,
    sizes: GroupingSizes,
) -> bool {
    if magnitude >= 0 {
        return false;
    }
    if sizes.fraction == 0 {
        return false;
    }
    let min_grouping = {
        use GroupingStrategy::*;
        match strategy {
            Never => return false,
            // Note: Auto and Always are the same for DecimalFormatter.
            // When currencies are implemented, this will change.
            Auto | Always => cmp::max(1, sizes.min_grouping) as i16,
            Min2 => cmp::max(2, sizes.min_grouping) as i16,
        }
    };

    let fraction = sizes.fraction as i16;

    // If there are fewer than `fraction + min_grouping` fraction digits,
    // we do not group.
    // e.g. fraction=3, min2=2. Total digits must be 5+.
    if -lower_magnitude < fraction + min_grouping {
        return false;
    }

    // Magnitude is negative: -1, -2, -3...
    // We want to group at -fraction, -2*fraction, etc.
    // e.g. fraction=3. Group at -3, -6...
    (-magnitude) % fraction == 0
}

#[test]
fn test_grouper() {
    use crate::input::Decimal;
    use crate::options;
    use crate::provider::*;
    use crate::DecimalFormatter;
    use icu_provider::prelude::*;
    use std::cell::RefCell;
    use writeable::assert_writeable_eq;

    let western_sizes = GroupingSizes {
        min_grouping: 1,
        primary: 3,
        secondary: 3,
        fraction: 0,
    };
    let indic_sizes = GroupingSizes {
        min_grouping: 1,
        primary: 3,
        secondary: 2,
        fraction: 0,
    };
    let western_sizes_min3 = GroupingSizes {
        min_grouping: 3,
        primary: 3,
        secondary: 3,
        fraction: 0,
    };

    // primary=0 implies no grouping; the other fields are ignored
    let zero_test = GroupingSizes {
        min_grouping: 0,
        primary: 0,
        secondary: 0,
        fraction: 0,
    };

    // secondary=0 implies that it inherits from primary
    let blank_secondary = GroupingSizes {
        min_grouping: 0,
        primary: 3,
        secondary: 0,
        fraction: 0,
    };

    #[derive(Debug)]
    struct TestCase {
        strategy: GroupingStrategy,
        sizes: GroupingSizes,
        // Expected results for numbers with magnitude 3, 4, 5, and 6
        expected: [&'static str; 4],
    }
    let cases = [
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: western_sizes,
            expected: ["1,000", "10,000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: western_sizes,
            expected: ["1000", "10,000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: indic_sizes,
            expected: ["1,000", "10,000", "1,00,000", "10,00,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: indic_sizes,
            expected: ["1000", "10,000", "1,00,000", "10,00,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: western_sizes_min3,
            expected: ["1000", "10000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: western_sizes_min3,
            expected: ["1000", "10000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: zero_test,
            expected: ["1000", "10000", "100000", "1000000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: zero_test,
            expected: ["1000", "10000", "100000", "1000000"],
        },
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: blank_secondary,
            expected: ["1,000", "10,000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: blank_secondary,
            expected: ["1000", "10,000", "100,000", "1,000,000"],
        },
    ];
    for cas in &cases {
        for i in 0..4 {
            let dec = {
                let mut dec = Decimal::from(1);
                dec.multiply_pow10((i as i16) + 3);
                dec
            };
            let symbols = DecimalSymbols {
                grouping_sizes: cas.sizes,
                ..DecimalSymbols::new_en_for_testing()
            };
            let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
            struct Provider(RefCell<Option<DecimalSymbols<'static>>>, [char; 10]);
            impl DataProvider<DecimalSymbolsV1> for Provider {
                fn load(
                    &self,
                    _req: DataRequest,
                ) -> Result<DataResponse<DecimalSymbolsV1>, DataError> {
                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(
                            self.0
                                .borrow_mut()
                                .take()
                                // We only have one payload
                                .ok_or(DataErrorKind::Custom.into_error())?,
                        ),
                    })
                }
            }
            impl DataProvider<DecimalDigitsV1> for Provider {
                fn load(
                    &self,
                    _req: DataRequest,
                ) -> Result<DataResponse<DecimalDigitsV1>, DataError> {
                    Ok(DataResponse {
                        metadata: Default::default(),
                        payload: DataPayload::from_owned(self.1),
                    })
                }
            }
            let provider = Provider(RefCell::new(Some(symbols)), digits);
            let options = options::DecimalFormatterOptions {
                grouping_strategy: Some(cas.strategy),
                ..Default::default()
            };
            let formatter =
                DecimalFormatter::try_new_unstable(&provider, Default::default(), options).unwrap();
            let actual = formatter.format(&dec);
            assert_writeable_eq!(actual, cas.expected[i], "{:?}", cas);
        }
    }
}

#[test]
fn test_fraction_grouper() {
    use crate::input::Decimal;
    use crate::options;
    use crate::provider::*;
    use crate::DecimalFormatter;
    use icu_provider::prelude::*;
    use std::cell::RefCell;
    use writeable::assert_writeable_eq;
    use zerovec::VarZeroCow;

    // Use thin space (U+202F) as grouping separator to produce valid
    // NIST/SI-style output. Fraction grouping with comma separators
    // would produce bogus output like "12,345.123,45".
    fn make_symbols(sizes: GroupingSizes) -> DecimalSymbols<'static> {
        let strings = DecimalSymbolStrsBuilder {
            minus_sign_prefix: VarZeroCow::new_borrowed("-"),
            minus_sign_suffix: VarZeroCow::new_borrowed(""),
            plus_sign_prefix: VarZeroCow::new_borrowed("+"),
            plus_sign_suffix: VarZeroCow::new_borrowed(""),
            decimal_separator: VarZeroCow::new_borrowed("."),
            grouping_separator: VarZeroCow::new_borrowed("\u{202F}"),
            numsys: VarZeroCow::new_borrowed("latn"),
        };
        DecimalSymbols {
            strings: VarZeroCow::from_encodeable(&strings),
            grouping_sizes: sizes,
        }
    }

    #[derive(Debug)]
    struct TestCase {
        strategy: GroupingStrategy,
        sizes: GroupingSizes,
        input: &'static str,
        expected: &'static str,
    }
    let cases = [
        // Basic fraction grouping by 3 with thin space
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: GroupingSizes {
                min_grouping: 1,
                primary: 3,
                secondary: 3,
                fraction: 3,
            },
            input: "0.1234567",
            expected: "0.123\u{202F}456\u{202F}7",
        },
        // Fraction grouping by 2
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: GroupingSizes {
                min_grouping: 1,
                primary: 2,
                secondary: 2,
                fraction: 2,
            },
            input: "0.12345",
            expected: "0.12\u{202F}34\u{202F}5",
        },
        // Never strategy disables everything
        TestCase {
            strategy: GroupingStrategy::Never,
            sizes: GroupingSizes {
                min_grouping: 1,
                primary: 3,
                secondary: 3,
                fraction: 3,
            },
            input: "0.1234567",
            expected: "0.1234567",
        },
        // Integer + fraction grouping with thin space
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: GroupingSizes {
                min_grouping: 1,
                primary: 3,
                secondary: 3,
                fraction: 3,
            },
            input: "12345.123456",
            expected: "12\u{202F}345.123\u{202F}456",
        },
        // NIST: Min2 suppresses 4-digit groups (3 + min2 = 5 digits needed)
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: GroupingSizes {
                min_grouping: 1,
                primary: 3,
                secondary: 3,
                fraction: 3,
            },
            input: "0.1234",
            expected: "0.1234",
        },
        // NIST: 5 fraction digits → grouping appears
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: GroupingSizes {
                min_grouping: 1,
                primary: 3,
                secondary: 3,
                fraction: 3,
            },
            input: "0.12345",
            expected: "0.123\u{202F}45",
        },
        // fraction: 0 means no fraction grouping
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: GroupingSizes {
                min_grouping: 1,
                primary: 3,
                secondary: 3,
                fraction: 0,
            },
            input: "0.1234567",
            expected: "0.1234567",
        },
    ];

    for cas in &cases {
        let dec: Decimal = cas.input.parse().unwrap();
        let symbols = make_symbols(cas.sizes);
        let digits = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
        struct Provider(RefCell<Option<DecimalSymbols<'static>>>, [char; 10]);
        impl DataProvider<DecimalSymbolsV1> for Provider {
            fn load(&self, _req: DataRequest) -> Result<DataResponse<DecimalSymbolsV1>, DataError> {
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(
                        self.0
                            .borrow_mut()
                            .take()
                            // We only have one payload
                            .ok_or(DataErrorKind::Custom.into_error())?,
                    ),
                })
            }
        }
        impl DataProvider<DecimalDigitsV1> for Provider {
            fn load(&self, _req: DataRequest) -> Result<DataResponse<DecimalDigitsV1>, DataError> {
                Ok(DataResponse {
                    metadata: Default::default(),
                    payload: DataPayload::from_owned(self.1),
                })
            }
        }
        let provider = Provider(RefCell::new(Some(symbols)), digits);
        let options = options::DecimalFormatterOptions {
            grouping_strategy: Some(cas.strategy),
        };
        let formatter =
            DecimalFormatter::try_new_unstable(&provider, Default::default(), options).unwrap();
        let actual = formatter.format(&dec);
        assert_writeable_eq!(actual, cas.expected, "{:?}", cas);
    }
}
