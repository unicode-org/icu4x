// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;
use icu_locid::locale;
use icu_relativetime::{options::Numeric, RelativeTimeFormatter, RelativeTimeFormatterOptions};
use writeable::assert_writeable_eq;

macro_rules! generate_test {
    ($test_name: ident, $constructor: ident, $options: expr,
     [$(($en_time: literal, $en_expected: literal)),+ $(,)?],
     [$(($ar_time: literal, $ar_expected: literal)),+ $(,)?]) => {
        #[test]
        fn $test_name(){
            let relative_time_formatter = RelativeTimeFormatter::$constructor(
                &icu_testdata::unstable(),
                &locale!("en").into(),
                $options
            )
            .expect("Data should load successfully.");

            $(
                assert_writeable_eq!(
                    relative_time_formatter.format(FixedDecimal::from($en_time)),
                    $en_expected
                );
            )+

            let relative_time_formatter = RelativeTimeFormatter::$constructor(
                &icu_testdata::unstable(),
                &locale!("ar").into(),
                $options
            )
            .expect("Data should load successfully.");

            $(
                assert_writeable_eq!(
                    relative_time_formatter.format(FixedDecimal::from($ar_time)),
                    $ar_expected
                );
            )+
        }
    };
}
generate_test!(
    test_long_second_always,
    try_new_long_second_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 seconds ago"),
        (-2, "2 seconds ago"),
        (-1, "1 second ago"),
        (0, "in 0 seconds"),
        (1, "in 1 second"),
        (2, "in 2 seconds"),
        (-10, "10 seconds ago")
    ],
    [
        (-10, "قبل ١٠ ثوانِ"),
        (-2, "قبل ثانيتين"),
        (-1, "قبل ثانية واحدة"),
        (0, "خلال ٠ ثانية"),
        (1, "خلال ثانية واحدة"),
        (2, "خلال ثانيتين"),
        (-10, "قبل ١٠ ثوانِ")
    ]
);
generate_test!(
    test_long_second_auto,
    try_new_long_second_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 seconds ago"),
        (-2, "2 seconds ago"),
        (-1, "1 second ago"),
        (0, "now"),
        (1, "in 1 second"),
        (2, "in 2 seconds"),
        (-10, "10 seconds ago")
    ],
    [
        (-10, "قبل ١٠ ثوانِ"),
        (-2, "قبل ثانيتين"),
        (-1, "قبل ثانية واحدة"),
        (0, "الآن"),
        (1, "خلال ثانية واحدة"),
        (2, "خلال ثانيتين"),
        (-10, "قبل ١٠ ثوانِ")
    ]
);
generate_test!(
    test_long_minute_always,
    try_new_long_minute_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 minutes ago"),
        (-2, "2 minutes ago"),
        (-1, "1 minute ago"),
        (0, "in 0 minutes"),
        (1, "in 1 minute"),
        (2, "in 2 minutes"),
        (-10, "10 minutes ago")
    ],
    [
        (-10, "قبل ١٠ دقائق"),
        (-2, "قبل دقيقتين"),
        (-1, "قبل دقيقة واحدة"),
        (0, "خلال ٠ دقيقة"),
        (1, "خلال دقيقة واحدة"),
        (2, "خلال دقيقتين"),
        (-10, "قبل ١٠ دقائق")
    ]
);
generate_test!(
    test_long_minute_auto,
    try_new_long_minute_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 minutes ago"),
        (-2, "2 minutes ago"),
        (-1, "1 minute ago"),
        (0, "this minute"),
        (1, "in 1 minute"),
        (2, "in 2 minutes"),
        (-10, "10 minutes ago")
    ],
    [
        (-10, "قبل ١٠ دقائق"),
        (-2, "قبل دقيقتين"),
        (-1, "قبل دقيقة واحدة"),
        (0, "هذه الدقيقة"),
        (1, "خلال دقيقة واحدة"),
        (2, "خلال دقيقتين"),
        (-10, "قبل ١٠ دقائق")
    ]
);
generate_test!(
    test_long_hour_always,
    try_new_long_hour_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 hours ago"),
        (-2, "2 hours ago"),
        (-1, "1 hour ago"),
        (0, "in 0 hours"),
        (1, "in 1 hour"),
        (2, "in 2 hours"),
        (-10, "10 hours ago")
    ],
    [
        (-10, "قبل ١٠ ساعات"),
        (-2, "قبل ساعتين"),
        (-1, "قبل ساعة واحدة"),
        (0, "خلال ٠ ساعة"),
        (1, "خلال ساعة واحدة"),
        (2, "خلال ساعتين"),
        (-10, "قبل ١٠ ساعات")
    ]
);
generate_test!(
    test_long_hour_auto,
    try_new_long_hour_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 hours ago"),
        (-2, "2 hours ago"),
        (-1, "1 hour ago"),
        (0, "this hour"),
        (1, "in 1 hour"),
        (2, "in 2 hours"),
        (-10, "10 hours ago")
    ],
    [
        (-10, "قبل ١٠ ساعات"),
        (-2, "قبل ساعتين"),
        (-1, "قبل ساعة واحدة"),
        (0, "الساعة الحالية"),
        (1, "خلال ساعة واحدة"),
        (2, "خلال ساعتين"),
        (-10, "قبل ١٠ ساعات")
    ]
);
generate_test!(
    test_long_day_always,
    try_new_long_day_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 days ago"),
        (-2, "2 days ago"),
        (-1, "1 day ago"),
        (0, "in 0 days"),
        (1, "in 1 day"),
        (2, "in 2 days"),
        (-10, "10 days ago")
    ],
    [
        (-10, "قبل ١٠ أيام"),
        (-2, "قبل يومين"),
        (-1, "قبل يوم واحد"),
        (0, "خلال ٠ يوم"),
        (1, "خلال يوم واحد"),
        (2, "خلال يومين"),
        (-10, "قبل ١٠ أيام")
    ]
);
generate_test!(
    test_long_day_auto,
    try_new_long_day_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 days ago"),
        (-2, "2 days ago"),
        (-1, "yesterday"),
        (0, "today"),
        (1, "tomorrow"),
        (2, "in 2 days"),
        (-10, "10 days ago")
    ],
    [
        (-10, "قبل ١٠ أيام"),
        (-2, "أول أمس"),
        (-1, "أمس"),
        (0, "اليوم"),
        (1, "غدًا"),
        (2, "بعد الغد"),
        (-10, "قبل ١٠ أيام")
    ]
);
generate_test!(
    test_long_week_always,
    try_new_long_week_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 weeks ago"),
        (-2, "2 weeks ago"),
        (-1, "1 week ago"),
        (0, "in 0 weeks"),
        (1, "in 1 week"),
        (2, "in 2 weeks"),
        (-10, "10 weeks ago")
    ],
    [
        (-10, "قبل ١٠ أسابيع"),
        (-2, "قبل أسبوعين"),
        (-1, "قبل أسبوع واحد"),
        (0, "خلال ٠ أسبوع"),
        (1, "خلال أسبوع واحد"),
        (2, "خلال أسبوعين"),
        (-10, "قبل ١٠ أسابيع")
    ]
);
generate_test!(
    test_long_week_auto,
    try_new_long_week_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 weeks ago"),
        (-2, "2 weeks ago"),
        (-1, "last week"),
        (0, "this week"),
        (1, "next week"),
        (2, "in 2 weeks"),
        (-10, "10 weeks ago")
    ],
    [
        (-10, "قبل ١٠ أسابيع"),
        (-2, "قبل أسبوعين"),
        (-1, "الأسبوع الماضي"),
        (0, "هذا الأسبوع"),
        (1, "الأسبوع القادم"),
        (2, "خلال أسبوعين"),
        (-10, "قبل ١٠ أسابيع")
    ]
);
generate_test!(
    test_long_month_always,
    try_new_long_month_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 months ago"),
        (-2, "2 months ago"),
        (-1, "1 month ago"),
        (0, "in 0 months"),
        (1, "in 1 month"),
        (2, "in 2 months"),
        (-10, "10 months ago")
    ],
    [
        (-10, "قبل ١٠ أشهر"),
        (-2, "قبل شهرين"),
        (-1, "قبل شهر واحد"),
        (0, "خلال ٠ شهر"),
        (1, "خلال شهر واحد"),
        (2, "خلال شهرين"),
        (-10, "قبل ١٠ أشهر")
    ]
);
generate_test!(
    test_long_month_auto,
    try_new_long_month_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 months ago"),
        (-2, "2 months ago"),
        (-1, "last month"),
        (0, "this month"),
        (1, "next month"),
        (2, "in 2 months"),
        (-10, "10 months ago")
    ],
    [
        (-10, "قبل ١٠ أشهر"),
        (-2, "قبل شهرين"),
        (-1, "الشهر الماضي"),
        (0, "هذا الشهر"),
        (1, "الشهر القادم"),
        (2, "خلال شهرين"),
        (-10, "قبل ١٠ أشهر")
    ]
);
generate_test!(
    test_long_quarter_always,
    try_new_long_quarter_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 quarters ago"),
        (-2, "2 quarters ago"),
        (-1, "1 quarter ago"),
        (0, "in 0 quarters"),
        (1, "in 1 quarter"),
        (2, "in 2 quarters"),
        (-10, "10 quarters ago")
    ],
    [
        (-10, "قبل ١٠ أرباع سنة"),
        (-2, "قبل ربعي سنة"),
        (-1, "قبل ربع سنة واحد"),
        (0, "خلال ٠ ربع سنة"),
        (1, "خلال ربع سنة واحد"),
        (2, "خلال ربعي سنة"),
        (-10, "قبل ١٠ أرباع سنة")
    ]
);
generate_test!(
    test_long_quarter_auto,
    try_new_long_quarter_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 quarters ago"),
        (-2, "2 quarters ago"),
        (-1, "last quarter"),
        (0, "this quarter"),
        (1, "next quarter"),
        (2, "in 2 quarters"),
        (-10, "10 quarters ago")
    ],
    [
        (-10, "قبل ١٠ أرباع سنة"),
        (-2, "قبل ربعي سنة"),
        (-1, "الربع الأخير"),
        (0, "هذا الربع"),
        (1, "الربع القادم"),
        (2, "خلال ربعي سنة"),
        (-10, "قبل ١٠ أرباع سنة")
    ]
);
generate_test!(
    test_long_year_always,
    try_new_long_year_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 years ago"),
        (-2, "2 years ago"),
        (-1, "1 year ago"),
        (0, "in 0 years"),
        (1, "in 1 year"),
        (2, "in 2 years"),
        (-10, "10 years ago")
    ],
    [
        (-10, "قبل ١٠ سنوات"),
        (-2, "قبل سنتين"),
        (-1, "قبل سنة واحدة"),
        (0, "خلال ٠ سنة"),
        (1, "خلال سنة واحدة"),
        (2, "خلال سنتين"),
        (-10, "قبل ١٠ سنوات")
    ]
);
generate_test!(
    test_long_year_auto,
    try_new_long_year_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 years ago"),
        (-2, "2 years ago"),
        (-1, "last year"),
        (0, "this year"),
        (1, "next year"),
        (2, "in 2 years"),
        (-10, "10 years ago")
    ],
    [
        (-10, "قبل ١٠ سنوات"),
        (-2, "قبل سنتين"),
        (-1, "السنة الماضية"),
        (0, "السنة الحالية"),
        (1, "السنة القادمة"),
        (2, "خلال سنتين"),
        (-10, "قبل ١٠ سنوات")
    ]
);
generate_test!(
    test_short_second_always,
    try_new_short_second_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 sec. ago"),
        (-2, "2 sec. ago"),
        (-1, "1 sec. ago"),
        (0, "in 0 sec."),
        (1, "in 1 sec."),
        (2, "in 2 sec."),
        (-10, "10 sec. ago")
    ],
    [
        (-10, "قبل ١٠ ثوانٍ"),
        (-2, "قبل ثانيتين"),
        (-1, "قبل ثانية واحدة"),
        (0, "خلال ٠ ثانية"),
        (1, "خلال ثانية واحدة"),
        (2, "خلال ثانيتين"),
        (-10, "قبل ١٠ ثوانٍ")
    ]
);
generate_test!(
    test_short_second_auto,
    try_new_short_second_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 sec. ago"),
        (-2, "2 sec. ago"),
        (-1, "1 sec. ago"),
        (0, "now"),
        (1, "in 1 sec."),
        (2, "in 2 sec."),
        (-10, "10 sec. ago")
    ],
    [
        (-10, "قبل ١٠ ثوانٍ"),
        (-2, "قبل ثانيتين"),
        (-1, "قبل ثانية واحدة"),
        (0, "الآن"),
        (1, "خلال ثانية واحدة"),
        (2, "خلال ثانيتين"),
        (-10, "قبل ١٠ ثوانٍ")
    ]
);
generate_test!(
    test_short_minute_always,
    try_new_short_minute_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 min. ago"),
        (-2, "2 min. ago"),
        (-1, "1 min. ago"),
        (0, "in 0 min."),
        (1, "in 1 min."),
        (2, "in 2 min."),
        (-10, "10 min. ago")
    ],
    [
        (-10, "قبل ١٠ دقائق"),
        (-2, "قبل دقيقتين"),
        (-1, "قبل دقيقة واحدة"),
        (0, "خلال ٠ دقيقة"),
        (1, "خلال دقيقة واحدة"),
        (2, "خلال دقيقتين"),
        (-10, "قبل ١٠ دقائق")
    ]
);
generate_test!(
    test_short_minute_auto,
    try_new_short_minute_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 min. ago"),
        (-2, "2 min. ago"),
        (-1, "1 min. ago"),
        (0, "this minute"),
        (1, "in 1 min."),
        (2, "in 2 min."),
        (-10, "10 min. ago")
    ],
    [
        (-10, "قبل ١٠ دقائق"),
        (-2, "قبل دقيقتين"),
        (-1, "قبل دقيقة واحدة"),
        (0, "هذه الدقيقة"),
        (1, "خلال دقيقة واحدة"),
        (2, "خلال دقيقتين"),
        (-10, "قبل ١٠ دقائق")
    ]
);
generate_test!(
    test_short_hour_always,
    try_new_short_hour_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 hr. ago"),
        (-2, "2 hr. ago"),
        (-1, "1 hr. ago"),
        (0, "in 0 hr."),
        (1, "in 1 hr."),
        (2, "in 2 hr."),
        (-10, "10 hr. ago")
    ],
    [
        (-10, "قبل ١٠ ساعات"),
        (-2, "قبل ساعتين"),
        (-1, "قبل ساعة واحدة"),
        (0, "خلال ٠ ساعة"),
        (1, "خلال ساعة واحدة"),
        (2, "خلال ساعتين"),
        (-10, "قبل ١٠ ساعات")
    ]
);
generate_test!(
    test_short_hour_auto,
    try_new_short_hour_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 hr. ago"),
        (-2, "2 hr. ago"),
        (-1, "1 hr. ago"),
        (0, "this hour"),
        (1, "in 1 hr."),
        (2, "in 2 hr."),
        (-10, "10 hr. ago")
    ],
    [
        (-10, "قبل ١٠ ساعات"),
        (-2, "قبل ساعتين"),
        (-1, "قبل ساعة واحدة"),
        (0, "الساعة الحالية"),
        (1, "خلال ساعة واحدة"),
        (2, "خلال ساعتين"),
        (-10, "قبل ١٠ ساعات")
    ]
);
generate_test!(
    test_short_day_always,
    try_new_short_day_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 days ago"),
        (-2, "2 days ago"),
        (-1, "1 day ago"),
        (0, "in 0 days"),
        (1, "in 1 day"),
        (2, "in 2 days"),
        (-10, "10 days ago")
    ],
    [
        (-10, "قبل ١٠ أيام"),
        (-2, "قبل يومين"),
        (-1, "قبل يوم واحد"),
        (0, "خلال ٠ يوم"),
        (1, "خلال يوم واحد"),
        (2, "خلال يومين"),
        (-10, "قبل ١٠ أيام")
    ]
);
generate_test!(
    test_short_day_auto,
    try_new_short_day_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 days ago"),
        (-2, "2 days ago"),
        (-1, "yesterday"),
        (0, "today"),
        (1, "tomorrow"),
        (2, "in 2 days"),
        (-10, "10 days ago")
    ],
    [
        (-10, "قبل ١٠ أيام"),
        (-2, "أول أمس"),
        (-1, "أمس"),
        (0, "اليوم"),
        (1, "غدًا"),
        (2, "بعد الغد"),
        (-10, "قبل ١٠ أيام")
    ]
);
generate_test!(
    test_short_week_always,
    try_new_short_week_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 wk. ago"),
        (-2, "2 wk. ago"),
        (-1, "1 wk. ago"),
        (0, "in 0 wk."),
        (1, "in 1 wk."),
        (2, "in 2 wk."),
        (-10, "10 wk. ago")
    ],
    [
        (-10, "قبل ١٠ أسابيع"),
        (-2, "قبل أسبوعين"),
        (-1, "قبل أسبوع واحد"),
        (0, "خلال ٠ أسبوع"),
        (1, "خلال أسبوع واحد"),
        (2, "خلال ٢ أسبوعين"),
        (-10, "قبل ١٠ أسابيع")
    ]
);
generate_test!(
    test_short_week_auto,
    try_new_short_week_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 wk. ago"),
        (-2, "2 wk. ago"),
        (-1, "last wk."),
        (0, "this wk."),
        (1, "next wk."),
        (2, "in 2 wk."),
        (-10, "10 wk. ago")
    ],
    [
        (-10, "قبل ١٠ أسابيع"),
        (-2, "قبل أسبوعين"),
        (-1, "الأسبوع الماضي"),
        (0, "هذا الأسبوع"),
        (1, "الأسبوع القادم"),
        (2, "خلال ٢ أسبوعين"),
        (-10, "قبل ١٠ أسابيع")
    ]
);
generate_test!(
    test_short_month_always,
    try_new_short_month_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 mo. ago"),
        (-2, "2 mo. ago"),
        (-1, "1 mo. ago"),
        (0, "in 0 mo."),
        (1, "in 1 mo."),
        (2, "in 2 mo."),
        (-10, "10 mo. ago")
    ],
    [
        (-10, "خلال ١٠ أشهر"),
        (-2, "قبل شهرين"),
        (-1, "قبل شهر واحد"),
        (0, "خلال ٠ شهر"),
        (1, "خلال شهر واحد"),
        (2, "خلال شهرين"),
        (-10, "خلال ١٠ أشهر")
    ]
);
generate_test!(
    test_short_month_auto,
    try_new_short_month_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 mo. ago"),
        (-2, "2 mo. ago"),
        (-1, "last mo."),
        (0, "this mo."),
        (1, "next mo."),
        (2, "in 2 mo."),
        (-10, "10 mo. ago")
    ],
    [
        (-10, "خلال ١٠ أشهر"),
        (-2, "قبل شهرين"),
        (-1, "الشهر الماضي"),
        (0, "هذا الشهر"),
        (1, "الشهر القادم"),
        (2, "خلال شهرين"),
        (-10, "خلال ١٠ أشهر")
    ]
);
generate_test!(
    test_short_quarter_always,
    try_new_short_quarter_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 qtrs. ago"),
        (-2, "2 qtrs. ago"),
        (-1, "1 qtr. ago"),
        (0, "in 0 qtrs."),
        (1, "in 1 qtr."),
        (2, "in 2 qtrs."),
        (-10, "10 qtrs. ago")
    ],
    [
        (-10, "قبل ١٠ أرباع سنة"),
        (-2, "قبل ربعي سنة"),
        (-1, "قبل ربع سنة واحد"),
        (0, "خلال ٠ ربع سنة"),
        (1, "خلال ربع سنة واحد"),
        (2, "خلال ربعي سنة"),
        (-10, "قبل ١٠ أرباع سنة")
    ]
);
generate_test!(
    test_short_quarter_auto,
    try_new_short_quarter_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 qtrs. ago"),
        (-2, "2 qtrs. ago"),
        (-1, "last qtr."),
        (0, "this qtr."),
        (1, "next qtr."),
        (2, "in 2 qtrs."),
        (-10, "10 qtrs. ago")
    ],
    [
        (-10, "قبل ١٠ أرباع سنة"),
        (-2, "قبل ربعي سنة"),
        (-1, "الربع الأخير"),
        (0, "هذا الربع"),
        (1, "الربع القادم"),
        (2, "خلال ربعي سنة"),
        (-10, "قبل ١٠ أرباع سنة")
    ]
);
generate_test!(
    test_short_year_always,
    try_new_short_year_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 yr. ago"),
        (-2, "2 yr. ago"),
        (-1, "1 yr. ago"),
        (0, "in 0 yr."),
        (1, "in 1 yr."),
        (2, "in 2 yr."),
        (-10, "10 yr. ago")
    ],
    [
        (-10, "قبل ١٠ سنوات"),
        (-2, "قبل سنتين"),
        (-1, "قبل سنة واحدة"),
        (0, "خلال ٠ سنة"),
        (1, "خلال سنة واحدة"),
        (2, "خلال سنتين"),
        (-10, "قبل ١٠ سنوات")
    ]
);
generate_test!(
    test_short_year_auto,
    try_new_short_year_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 yr. ago"),
        (-2, "2 yr. ago"),
        (-1, "last yr."),
        (0, "this yr."),
        (1, "next yr."),
        (2, "in 2 yr."),
        (-10, "10 yr. ago")
    ],
    [
        (-10, "قبل ١٠ سنوات"),
        (-2, "قبل سنتين"),
        (-1, "السنة الماضية"),
        (0, "السنة الحالية"),
        (1, "السنة القادمة"),
        (2, "خلال سنتين"),
        (-10, "قبل ١٠ سنوات")
    ]
);
generate_test!(
    test_narrow_second_always,
    try_new_narrow_second_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 sec. ago"),
        (-2, "2 sec. ago"),
        (-1, "1 sec. ago"),
        (0, "in 0 sec."),
        (1, "in 1 sec."),
        (2, "in 2 sec."),
        (-10, "10 sec. ago")
    ],
    [
        (-10, "قبل ١٠ ثوانٍ"),
        (-2, "قبل ثانيتين"),
        (-1, "قبل ثانية واحدة"),
        (0, "خلال ٠ ثانية"),
        (1, "خلال ثانية واحدة"),
        (2, "خلال ثانيتين"),
        (-10, "قبل ١٠ ثوانٍ")
    ]
);
generate_test!(
    test_narrow_second_auto,
    try_new_narrow_second_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 sec. ago"),
        (-2, "2 sec. ago"),
        (-1, "1 sec. ago"),
        (0, "now"),
        (1, "in 1 sec."),
        (2, "in 2 sec."),
        (-10, "10 sec. ago")
    ],
    [
        (-10, "قبل ١٠ ثوانٍ"),
        (-2, "قبل ثانيتين"),
        (-1, "قبل ثانية واحدة"),
        (0, "الآن"),
        (1, "خلال ثانية واحدة"),
        (2, "خلال ثانيتين"),
        (-10, "قبل ١٠ ثوانٍ")
    ]
);
generate_test!(
    test_narrow_minute_always,
    try_new_narrow_minute_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 min. ago"),
        (-2, "2 min. ago"),
        (-1, "1 min. ago"),
        (0, "in 0 min."),
        (1, "in 1 min."),
        (2, "in 2 min."),
        (-10, "10 min. ago")
    ],
    [
        (-10, "قبل ١٠ دقائق"),
        (-2, "قبل دقيقتين"),
        (-1, "قبل دقيقة واحدة"),
        (0, "خلال ٠ دقيقة"),
        (1, "خلال دقيقة واحدة"),
        (2, "خلال دقيقتين"),
        (-10, "قبل ١٠ دقائق")
    ]
);
generate_test!(
    test_narrow_minute_auto,
    try_new_narrow_minute_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 min. ago"),
        (-2, "2 min. ago"),
        (-1, "1 min. ago"),
        (0, "this minute"),
        (1, "in 1 min."),
        (2, "in 2 min."),
        (-10, "10 min. ago")
    ],
    [
        (-10, "قبل ١٠ دقائق"),
        (-2, "قبل دقيقتين"),
        (-1, "قبل دقيقة واحدة"),
        (0, "هذه الدقيقة"),
        (1, "خلال دقيقة واحدة"),
        (2, "خلال دقيقتين"),
        (-10, "قبل ١٠ دقائق")
    ]
);
generate_test!(
    test_narrow_hour_always,
    try_new_narrow_hour_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 hr. ago"),
        (-2, "2 hr. ago"),
        (-1, "1 hr. ago"),
        (0, "in 0 hr."),
        (1, "in 1 hr."),
        (2, "in 2 hr."),
        (-10, "10 hr. ago")
    ],
    [
        (-10, "قبل ١٠ ساعات"),
        (-2, "قبل ساعتين"),
        (-1, "قبل ساعة واحدة"),
        (0, "خلال ٠ ساعة"),
        (1, "خلال ساعة واحدة"),
        (2, "خلال ساعتين"),
        (-10, "قبل ١٠ ساعات")
    ]
);
generate_test!(
    test_narrow_hour_auto,
    try_new_narrow_hour_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 hr. ago"),
        (-2, "2 hr. ago"),
        (-1, "1 hr. ago"),
        (0, "this hour"),
        (1, "in 1 hr."),
        (2, "in 2 hr."),
        (-10, "10 hr. ago")
    ],
    [
        (-10, "قبل ١٠ ساعات"),
        (-2, "قبل ساعتين"),
        (-1, "قبل ساعة واحدة"),
        (0, "الساعة الحالية"),
        (1, "خلال ساعة واحدة"),
        (2, "خلال ساعتين"),
        (-10, "قبل ١٠ ساعات")
    ]
);
generate_test!(
    test_narrow_day_always,
    try_new_narrow_day_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 days ago"),
        (-2, "2 days ago"),
        (-1, "1 day ago"),
        (0, "in 0 days"),
        (1, "in 1 day"),
        (2, "in 2 days"),
        (-10, "10 days ago")
    ],
    [
        (-10, "قبل ١٠ أيام"),
        (-2, "قبل يومين"),
        (-1, "قبل يوم واحد"),
        (0, "خلال ٠ يوم"),
        (1, "خلال يوم واحد"),
        (2, "خلال يومين"),
        (-10, "قبل ١٠ أيام")
    ]
);
generate_test!(
    test_narrow_day_auto,
    try_new_narrow_day_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 days ago"),
        (-2, "2 days ago"),
        (-1, "yesterday"),
        (0, "today"),
        (1, "tomorrow"),
        (2, "in 2 days"),
        (-10, "10 days ago")
    ],
    [
        (-10, "قبل ١٠ أيام"),
        (-2, "أول أمس"),
        (-1, "أمس"),
        (0, "اليوم"),
        (1, "غدًا"),
        (2, "بعد الغد"),
        (-10, "قبل ١٠ أيام")
    ]
);
generate_test!(
    test_narrow_week_always,
    try_new_narrow_week_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 wk. ago"),
        (-2, "2 wk. ago"),
        (-1, "1 wk. ago"),
        (0, "in 0 wk."),
        (1, "in 1 wk."),
        (2, "in 2 wk."),
        (-10, "10 wk. ago")
    ],
    [
        (-10, "قبل ١٠ أسابيع"),
        (-2, "قبل أسبوعين"),
        (-1, "قبل أسبوع واحد"),
        (0, "خلال ٠ أسبوع"),
        (1, "خلال أسبوع واحد"),
        (2, "خلال أسبوعين"),
        (-10, "قبل ١٠ أسابيع")
    ]
);
generate_test!(
    test_narrow_week_auto,
    try_new_narrow_week_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 wk. ago"),
        (-2, "2 wk. ago"),
        (-1, "last wk."),
        (0, "this wk."),
        (1, "next wk."),
        (2, "in 2 wk."),
        (-10, "10 wk. ago")
    ],
    [
        (-10, "قبل ١٠ أسابيع"),
        (-2, "قبل أسبوعين"),
        (-1, "الأسبوع الماضي"),
        (0, "هذا الأسبوع"),
        (1, "الأسبوع القادم"),
        (2, "خلال أسبوعين"),
        (-10, "قبل ١٠ أسابيع")
    ]
);
generate_test!(
    test_narrow_month_always,
    try_new_narrow_month_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 mo. ago"),
        (-2, "2 mo. ago"),
        (-1, "1 mo. ago"),
        (0, "in 0 mo."),
        (1, "in 1 mo."),
        (2, "in 2 mo."),
        (-10, "10 mo. ago")
    ],
    [
        (-10, "قبل ١٠ أشهر"),
        (-2, "قبل شهرين"),
        (-1, "قبل شهر واحد"),
        (0, "خلال ٠ شهر"),
        (1, "خلال شهر واحد"),
        (2, "خلال شهرين"),
        (-10, "قبل ١٠ أشهر")
    ]
);
generate_test!(
    test_narrow_month_auto,
    try_new_narrow_month_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 mo. ago"),
        (-2, "2 mo. ago"),
        (-1, "last mo."),
        (0, "this mo."),
        (1, "next mo."),
        (2, "in 2 mo."),
        (-10, "10 mo. ago")
    ],
    [
        (-10, "قبل ١٠ أشهر"),
        (-2, "قبل شهرين"),
        (-1, "الشهر الماضي"),
        (0, "هذا الشهر"),
        (1, "الشهر القادم"),
        (2, "خلال شهرين"),
        (-10, "قبل ١٠ أشهر")
    ]
);
generate_test!(
    test_narrow_quarter_always,
    try_new_narrow_quarter_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 qtrs. ago"),
        (-2, "2 qtrs. ago"),
        (-1, "1 qtr. ago"),
        (0, "in 0 qtrs."),
        (1, "in 1 qtr."),
        (2, "in 2 qtrs."),
        (-10, "10 qtrs. ago")
    ],
    [
        (-10, "قبل ١٠ أرباع سنة"),
        (-2, "قبل ربعي سنة"),
        (-1, "قبل ربع سنة واحد"),
        (0, "خلال ٠ ربع سنة"),
        (1, "خلال ربع سنة واحد"),
        (2, "خلال ربعي سنة"),
        (-10, "قبل ١٠ أرباع سنة")
    ]
);
generate_test!(
    test_narrow_quarter_auto,
    try_new_narrow_quarter_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 qtrs. ago"),
        (-2, "2 qtrs. ago"),
        (-1, "last qtr."),
        (0, "this qtr."),
        (1, "next qtr."),
        (2, "in 2 qtrs."),
        (-10, "10 qtrs. ago")
    ],
    [
        (-10, "قبل ١٠ أرباع سنة"),
        (-2, "قبل ربعي سنة"),
        (-1, "الربع الأخير"),
        (0, "هذا الربع"),
        (1, "الربع القادم"),
        (2, "خلال ربعي سنة"),
        (-10, "قبل ١٠ أرباع سنة")
    ]
);
generate_test!(
    test_narrow_year_always,
    try_new_narrow_year_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Always
    },
    [
        (-10, "10 yr. ago"),
        (-2, "2 yr. ago"),
        (-1, "1 yr. ago"),
        (0, "in 0 yr."),
        (1, "in 1 yr."),
        (2, "in 2 yr."),
        (-10, "10 yr. ago")
    ],
    [
        (-10, "قبل ١٠ سنوات"),
        (-2, "قبل سنتين"),
        (-1, "قبل سنة واحدة"),
        (0, "خلال ٠ سنة"),
        (1, "خلال سنة واحدة"),
        (2, "خلال سنتين"),
        (-10, "قبل ١٠ سنوات")
    ]
);
generate_test!(
    test_narrow_year_auto,
    try_new_narrow_year_unstable,
    RelativeTimeFormatterOptions {
        numeric: Numeric::Auto
    },
    [
        (-10, "10 yr. ago"),
        (-2, "2 yr. ago"),
        (-1, "last yr."),
        (0, "this yr."),
        (1, "next yr."),
        (2, "in 2 yr."),
        (-10, "10 yr. ago")
    ],
    [
        (-10, "قبل ١٠ سنوات"),
        (-2, "قبل سنتين"),
        (-1, "السنة الماضية"),
        (0, "السنة الحالية"),
        (1, "السنة القادمة"),
        (2, "خلال سنتين"),
        (-10, "قبل ١٠ سنوات")
    ]
);
