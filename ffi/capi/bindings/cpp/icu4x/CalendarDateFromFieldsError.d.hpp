#ifndef ICU4X_CalendarDateFromFieldsError_D_HPP
#define ICU4X_CalendarDateFromFieldsError_D_HPP

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
    enum CalendarDateFromFieldsError {
      CalendarDateFromFieldsError_Unknown = 0,
      CalendarDateFromFieldsError_OutOfRange = 1,
      CalendarDateFromFieldsError_UnknownEra = 2,
      CalendarDateFromFieldsError_MonthCodeInvalidSyntax = 3,
      CalendarDateFromFieldsError_MonthCodeNotInCalendar = 4,
      CalendarDateFromFieldsError_MonthCodeNotInYear = 5,
      CalendarDateFromFieldsError_InconsistentYear = 6,
      CalendarDateFromFieldsError_InconsistentMonth = 7,
      CalendarDateFromFieldsError_NotEnoughFields = 8,
    };

    typedef struct CalendarDateFromFieldsError_option {union { CalendarDateFromFieldsError ok; }; bool is_ok; } CalendarDateFromFieldsError_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * Additional information: [1](https://docs.rs/icu/2.1.1/icu/calendar/error/enum.DateFromFieldsError.html)
 */
class CalendarDateFromFieldsError {
public:
    enum Value {
        Unknown = 0,
        OutOfRange = 1,
        UnknownEra = 2,
        MonthCodeInvalidSyntax = 3,
        MonthCodeNotInCalendar = 4,
        MonthCodeNotInYear = 5,
        InconsistentYear = 6,
        InconsistentMonth = 7,
        NotEnoughFields = 8,
    };

    CalendarDateFromFieldsError(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr CalendarDateFromFieldsError(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::CalendarDateFromFieldsError AsFFI() const;
    inline static icu4x::CalendarDateFromFieldsError FromFFI(icu4x::capi::CalendarDateFromFieldsError c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_CalendarDateFromFieldsError_D_HPP
