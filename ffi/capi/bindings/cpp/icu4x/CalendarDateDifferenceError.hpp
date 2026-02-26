#ifndef ICU4X_CalendarDateDifferenceError_HPP
#define ICU4X_CalendarDateDifferenceError_HPP

#include "CalendarDateDifferenceError.d.hpp"

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

inline icu4x::capi::CalendarDateDifferenceError icu4x::CalendarDateDifferenceError::AsFFI() const {
    return static_cast<icu4x::capi::CalendarDateDifferenceError>(value);
}

inline icu4x::CalendarDateDifferenceError icu4x::CalendarDateDifferenceError::FromFFI(icu4x::capi::CalendarDateDifferenceError c_enum) {
    switch (c_enum) {
        case icu4x::capi::CalendarDateDifferenceError_Unknown:
        case icu4x::capi::CalendarDateDifferenceError_MismatchedCalendars:
            return static_cast<icu4x::CalendarDateDifferenceError::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_CalendarDateDifferenceError_HPP
