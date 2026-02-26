#ifndef ICU4X_DateDuration_HPP
#define ICU4X_DateDuration_HPP

#include "DateDuration.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <functional>
#include <optional>
#include <cstdlib>
#include "diplomat_runtime.hpp"


namespace icu4x {
namespace capi {

} // namespace capi
} // namespace


inline icu4x::capi::DateDuration icu4x::DateDuration::AsFFI() const {
    return icu4x::capi::DateDuration {
        /* .is_negative = */ is_negative,
        /* .years = */ years,
        /* .months = */ months,
        /* .weeks = */ weeks,
        /* .days = */ days,
    };
}

inline icu4x::DateDuration icu4x::DateDuration::FromFFI(icu4x::capi::DateDuration c_struct) {
    return icu4x::DateDuration {
        /* .is_negative = */ c_struct.is_negative,
        /* .years = */ c_struct.years,
        /* .months = */ c_struct.months,
        /* .weeks = */ c_struct.weeks,
        /* .days = */ c_struct.days,
    };
}


#endif // ICU4X_DateDuration_HPP
