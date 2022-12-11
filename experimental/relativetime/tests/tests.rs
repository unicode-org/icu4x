// This file is part of ICU4X. For terms of use, please see the file
// called LICENSE at the top level of the ICU4X source tree
// (online at: https://github.com/unicode-org/icu4x/blob/main/LICENSE ).

use fixed_decimal::FixedDecimal;
use icu_locid::locale;
use icu_relativetime::{options::Numeric, RelativeTimeFormatter, RelativeTimeFormatterOptions};
use writeable::assert_writeable_eq;

macro_rules! generate_test {
    ($test_name: ident, $constructor: ident, $options: expr, [$(($time: literal, $expected: literal)),+ $(,)?]) => {
        #[test]
        fn $test_name(){
            let relative_time_formatter = RelativeTimeFormatter::$constructor(
                &icu_testdata::unstable(),
                &locale!("ar").into(),
                $options
            )
            .expect("Data should load successfully.");

            $(
                assert_writeable_eq!(
                    relative_time_formatter.format(FixedDecimal::from($time)),
                    $expected
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
        (-10, "قبل ١٠ سنوات"),
        (-2, "قبل سنتين"),
        (-1, "السنة الماضية"),
        (0, "السنة الحالية"),
        (1, "السنة القادمة"),
        (2, "خلال سنتين"),
        (-10, "قبل ١٠ سنوات")
    ]
);
