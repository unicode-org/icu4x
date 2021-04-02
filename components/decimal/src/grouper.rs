// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

//! Algorithms to determine where to position grouping separators.

use crate::options::GroupingStrategy;
use crate::provider::GroupingSizesV1;

/// Returns whether to display a grouping separator at the given magnitude.
///
/// `upper_magnitude` is the magnitude of the highest-power digit, used for resolving minimum
/// grouping digits.
pub fn check(
    upper_magnitude: i16,
    magnitude: i16,
    strategy: GroupingStrategy,
    sizes: &GroupingSizesV1,
) -> bool {
    if magnitude < (sizes.primary as i16) {
        return false;
    }
    let min_grouping = {
        use GroupingStrategy::*;
        match strategy {
            Never => return false,
            // Note: Auto and Always are the same for FixedDecimalFormat.
            // When currencies are implemented, this will change.
            Auto | Always => sizes.min_grouping as i16,
            Min2 => i16::max(2, sizes.min_grouping as i16),
        }
    };
    if upper_magnitude < (sizes.primary as i16) + min_grouping - 1 {
        return false;
    }
    let magnitude_prime = magnitude - (sizes.primary as i16);
    if magnitude_prime % (sizes.secondary as i16) == 0 {
        return true;
    }
    false
}

#[test]
fn test_grouper() {
    use crate::options;
    use crate::FixedDecimalFormat;
    use fixed_decimal::FixedDecimal;
    use icu_locid::LanguageIdentifier;
    use writeable::Writeable;

    let western_sizes = GroupingSizesV1 {
        min_grouping: 1,
        primary: 3,
        secondary: 3,
    };
    let indic_sizes = GroupingSizesV1 {
        min_grouping: 1,
        primary: 3,
        secondary: 2,
    };
    let western_sizes_min3 = GroupingSizesV1 {
        min_grouping: 3,
        primary: 3,
        secondary: 3,
    };

    #[derive(Debug)]
    struct TestCase {
        strategy: GroupingStrategy,
        sizes: GroupingSizesV1,
        // Expected results for numbers with magnitude 3, 4, 5, and 6
        expected: [&'static str; 4],
    }
    #[allow(clippy::redundant_clone)]
    let cases = [
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: western_sizes.clone(),
            expected: ["1,000", "10,000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: western_sizes.clone(),
            expected: ["1000", "10,000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: indic_sizes.clone(),
            expected: ["1,000", "10,000", "1,00,000", "10,00,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: indic_sizes.clone(),
            expected: ["1000", "10,000", "1,00,000", "10,00,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Auto,
            sizes: western_sizes_min3.clone(),
            expected: ["1000", "10000", "100,000", "1,000,000"],
        },
        TestCase {
            strategy: GroupingStrategy::Min2,
            sizes: western_sizes_min3.clone(),
            expected: ["1000", "10000", "100,000", "1,000,000"],
        },
    ];
    for cas in &cases {
        for i in 0..4 {
            let dec = FixedDecimal::from(1)
                .multiplied_pow10((i as i16) + 3)
                .unwrap();
            let mut data_struct: crate::provider::DecimalSymbolsV1 = Default::default();
            data_struct.grouping_sizes = cas.sizes.clone();
            let provider = icu_provider::struct_provider::StructProvider {
                key: crate::provider::key::SYMBOLS_V1,
                data: &data_struct,
            };
            let options = options::FixedDecimalFormatOptions {
                grouping_strategy: cas.strategy,
                ..Default::default()
            };
            let fdf =
                FixedDecimalFormat::try_new(LanguageIdentifier::und(), &provider, options).unwrap();
            let actual = fdf.format(&dec).writeable_to_string();
            assert_eq!(cas.expected[i], actual, "{:?}", cas);
        }
    }
}
