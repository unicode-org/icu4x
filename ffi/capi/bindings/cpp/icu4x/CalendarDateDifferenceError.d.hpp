#ifndef ICU4X_CalendarDateDifferenceError_D_HPP
#define ICU4X_CalendarDateDifferenceError_D_HPP

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
    enum CalendarDateDifferenceError {
      CalendarDateDifferenceError_Unknown = 0,
      CalendarDateDifferenceError_MismatchedCalendars = 1,
    };

    typedef struct CalendarDateDifferenceError_option {union { CalendarDateDifferenceError ok; }; bool is_ok; } CalendarDateDifferenceError_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/cal/enum.AnyCalendarDifferenceError.html)
 */
class CalendarDateDifferenceError {
public:
    enum Value {
        Unknown = 0,
        MismatchedCalendars = 1,
    };

    CalendarDateDifferenceError(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr CalendarDateDifferenceError(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::CalendarDateDifferenceError AsFFI() const;
    inline static icu4x::CalendarDateDifferenceError FromFFI(icu4x::capi::CalendarDateDifferenceError c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_CalendarDateDifferenceError_D_HPP
