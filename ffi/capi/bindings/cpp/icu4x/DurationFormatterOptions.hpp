#ifndef ICU4X_DurationFormatterOptions_HPP
#define ICU4X_DurationFormatterOptions_HPP

#include "DurationFormatterOptions.d.hpp"

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


inline icu4x::capi::DurationFormatterOptions icu4x::DurationFormatterOptions::AsFFI() const {
    return icu4x::capi::DurationFormatterOptions {
        /* .base = */ base.AsFFI(),
        /* .year = */ year.AsFFI(),
        /* .year_visibility = */ year_visibility.AsFFI(),
        /* .month = */ month.AsFFI(),
        /* .month_visibility = */ month_visibility.AsFFI(),
        /* .week = */ week.AsFFI(),
        /* .week_visibility = */ week_visibility.AsFFI(),
        /* .day = */ day.AsFFI(),
        /* .day_visibility = */ day_visibility.AsFFI(),
        /* .hour = */ hour.AsFFI(),
        /* .hour_visibility = */ hour_visibility.AsFFI(),
        /* .minute = */ minute.AsFFI(),
        /* .minute_visibility = */ minute_visibility.AsFFI(),
        /* .second = */ second.AsFFI(),
        /* .second_visibility = */ second_visibility.AsFFI(),
        /* .millisecond = */ millisecond.AsFFI(),
        /* .millisecond_visibility = */ millisecond_visibility.AsFFI(),
        /* .microsecond = */ microsecond.AsFFI(),
        /* .microsecond_visibility = */ microsecond_visibility.AsFFI(),
        /* .nanosecond = */ nanosecond.AsFFI(),
        /* .nanosecond_visibility = */ nanosecond_visibility.AsFFI(),
        /* .fractional_digits = */ fractional_digits,
    };
}

inline icu4x::DurationFormatterOptions icu4x::DurationFormatterOptions::FromFFI(icu4x::capi::DurationFormatterOptions c_struct) {
    return icu4x::DurationFormatterOptions {
        /* .base = */ icu4x::DurationBaseStyle::FromFFI(c_struct.base),
        /* .year = */ icu4x::DurationUnitStyle::FromFFI(c_struct.year),
        /* .year_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.year_visibility),
        /* .month = */ icu4x::DurationUnitStyle::FromFFI(c_struct.month),
        /* .month_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.month_visibility),
        /* .week = */ icu4x::DurationUnitStyle::FromFFI(c_struct.week),
        /* .week_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.week_visibility),
        /* .day = */ icu4x::DurationUnitStyle::FromFFI(c_struct.day),
        /* .day_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.day_visibility),
        /* .hour = */ icu4x::DurationUnitStyle::FromFFI(c_struct.hour),
        /* .hour_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.hour_visibility),
        /* .minute = */ icu4x::DurationUnitStyle::FromFFI(c_struct.minute),
        /* .minute_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.minute_visibility),
        /* .second = */ icu4x::DurationUnitStyle::FromFFI(c_struct.second),
        /* .second_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.second_visibility),
        /* .millisecond = */ icu4x::DurationUnitStyle::FromFFI(c_struct.millisecond),
        /* .millisecond_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.millisecond_visibility),
        /* .microsecond = */ icu4x::DurationUnitStyle::FromFFI(c_struct.microsecond),
        /* .microsecond_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.microsecond_visibility),
        /* .nanosecond = */ icu4x::DurationUnitStyle::FromFFI(c_struct.nanosecond),
        /* .nanosecond_visibility = */ icu4x::DurationFieldDisplay::FromFFI(c_struct.nanosecond_visibility),
        /* .fractional_digits = */ c_struct.fractional_digits,
    };
}


#endif // ICU4X_DurationFormatterOptions_HPP
