#ifndef ICU4X_Duration_HPP
#define ICU4X_Duration_HPP

#include "Duration.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "DurationSign.hpp"
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {

} // namespace capi
} // namespace


inline icu4x::capi::Duration icu4x::Duration::AsFFI() const {
    return icu4x::capi::Duration {
        /* .sign = */ sign.AsFFI(),
        /* .years = */ years,
        /* .months = */ months,
        /* .weeks = */ weeks,
        /* .days = */ days,
        /* .hours = */ hours,
        /* .minutes = */ minutes,
        /* .seconds = */ seconds,
        /* .milliseconds = */ milliseconds,
        /* .microseconds = */ microseconds,
        /* .nanoseconds = */ nanoseconds,
    };
}

inline icu4x::Duration icu4x::Duration::FromFFI(icu4x::capi::Duration c_struct) {
    return icu4x::Duration {
        /* .sign = */ icu4x::DurationSign::FromFFI(c_struct.sign),
        /* .years = */ c_struct.years,
        /* .months = */ c_struct.months,
        /* .weeks = */ c_struct.weeks,
        /* .days = */ c_struct.days,
        /* .hours = */ c_struct.hours,
        /* .minutes = */ c_struct.minutes,
        /* .seconds = */ c_struct.seconds,
        /* .milliseconds = */ c_struct.milliseconds,
        /* .microseconds = */ c_struct.microseconds,
        /* .nanoseconds = */ c_struct.nanoseconds,
    };
}


#endif // ICU4X_Duration_HPP
