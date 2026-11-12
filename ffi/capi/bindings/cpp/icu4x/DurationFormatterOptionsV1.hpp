#ifndef ICU4X_DurationFormatterOptionsV1_HPP
#define ICU4X_DurationFormatterOptionsV1_HPP

#include "DurationFormatterOptionsV1.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DurationBaseStyle.hpp"
#include "DurationFieldDisplay.hpp"
#include "DurationUnitStyle.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {

} // namespace capi
} // namespace


inline icu4x::capi::DurationFormatterOptionsV1 icu4x::DurationFormatterOptionsV1::AsFFI() const {
    return icu4x::capi::DurationFormatterOptionsV1 {
        /* .base = */ base.AsFFI(),
        /* .year = */ year.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { year.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .year_visibility = */ year_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { year_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .month = */ month.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { month.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .month_visibility = */ month_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { month_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .week = */ week.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { week.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .week_visibility = */ week_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { week_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .day = */ day.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { day.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .day_visibility = */ day_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { day_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .hour = */ hour.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { hour.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .hour_visibility = */ hour_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { hour_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .minute = */ minute.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { minute.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .minute_visibility = */ minute_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { minute_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .second = */ second.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { second.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .second_visibility = */ second_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { second_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .millisecond = */ millisecond.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { millisecond.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .millisecond_visibility = */ millisecond_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { millisecond_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .microsecond = */ microsecond.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { microsecond.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .microsecond_visibility = */ microsecond_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { microsecond_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .nanosecond = */ nanosecond.has_value() ? (icu4x::capi::DurationUnitStyle_option{ { nanosecond.value().AsFFI() }, true }) : (icu4x::capi::DurationUnitStyle_option{ {}, false }),
        /* .nanosecond_visibility = */ nanosecond_visibility.has_value() ? (icu4x::capi::DurationFieldDisplay_option{ { nanosecond_visibility.value().AsFFI() }, true }) : (icu4x::capi::DurationFieldDisplay_option{ {}, false }),
        /* .fractional_digits = */ fractional_digits.has_value() ? (icu4x::diplomat::capi::OptionU8{ { fractional_digits.value() }, true }) : (icu4x::diplomat::capi::OptionU8{ {}, false }),
    };
}

inline icu4x::DurationFormatterOptionsV1 icu4x::DurationFormatterOptionsV1::FromFFI(icu4x::capi::DurationFormatterOptionsV1 c_struct) {
    return icu4x::DurationFormatterOptionsV1 {
        /* .base = */ icu4x::DurationBaseStyle::FromFFI(c_struct.base),
        /* .year = */ c_struct.year.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.year.ok)) : std::nullopt,
        /* .year_visibility = */ c_struct.year_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.year_visibility.ok)) : std::nullopt,
        /* .month = */ c_struct.month.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.month.ok)) : std::nullopt,
        /* .month_visibility = */ c_struct.month_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.month_visibility.ok)) : std::nullopt,
        /* .week = */ c_struct.week.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.week.ok)) : std::nullopt,
        /* .week_visibility = */ c_struct.week_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.week_visibility.ok)) : std::nullopt,
        /* .day = */ c_struct.day.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.day.ok)) : std::nullopt,
        /* .day_visibility = */ c_struct.day_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.day_visibility.ok)) : std::nullopt,
        /* .hour = */ c_struct.hour.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.hour.ok)) : std::nullopt,
        /* .hour_visibility = */ c_struct.hour_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.hour_visibility.ok)) : std::nullopt,
        /* .minute = */ c_struct.minute.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.minute.ok)) : std::nullopt,
        /* .minute_visibility = */ c_struct.minute_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.minute_visibility.ok)) : std::nullopt,
        /* .second = */ c_struct.second.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.second.ok)) : std::nullopt,
        /* .second_visibility = */ c_struct.second_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.second_visibility.ok)) : std::nullopt,
        /* .millisecond = */ c_struct.millisecond.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.millisecond.ok)) : std::nullopt,
        /* .millisecond_visibility = */ c_struct.millisecond_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.millisecond_visibility.ok)) : std::nullopt,
        /* .microsecond = */ c_struct.microsecond.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.microsecond.ok)) : std::nullopt,
        /* .microsecond_visibility = */ c_struct.microsecond_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.microsecond_visibility.ok)) : std::nullopt,
        /* .nanosecond = */ c_struct.nanosecond.is_ok ? std::optional(icu4x::DurationUnitStyle::FromFFI(c_struct.nanosecond.ok)) : std::nullopt,
        /* .nanosecond_visibility = */ c_struct.nanosecond_visibility.is_ok ? std::optional(icu4x::DurationFieldDisplay::FromFFI(c_struct.nanosecond_visibility.ok)) : std::nullopt,
        /* .fractional_digits = */ c_struct.fractional_digits.is_ok ? std::optional(c_struct.fractional_digits.ok) : std::nullopt,
    };
}


#endif // ICU4X_DurationFormatterOptionsV1_HPP
