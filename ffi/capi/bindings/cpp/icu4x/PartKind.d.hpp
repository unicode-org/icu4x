#ifndef ICU4X_PartKind_D_HPP
#define ICU4X_PartKind_D_HPP

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
    enum PartKind {
      PartKind_Unknown = 0,
      PartKind_Error = 1,
      PartKind_ListElement = 2,
      PartKind_ListLiteral = 3,
      PartKind_DecimalPlusSign = 4,
      PartKind_DecimalMinusSign = 5,
      PartKind_DecimalInteger = 6,
      PartKind_DecimalFraction = 7,
      PartKind_DecimalGroup = 8,
      PartKind_DecimalDecimal = 9,
      PartKind_DateTimeEra = 10,
      PartKind_DateTimeYear = 11,
      PartKind_DateTimeRelatedYear = 12,
      PartKind_DateTimeYearName = 13,
      PartKind_DateTimeMonth = 14,
      PartKind_DateTimeDay = 15,
      PartKind_DateTimeExtendedYear = 16,
      PartKind_DateTimeJulianDay = 17,
      PartKind_DateTimeWeekday = 18,
      PartKind_DateTimeDayPeriod = 19,
      PartKind_DateTimeHour = 20,
      PartKind_DateTimeMinute = 21,
      PartKind_DateTimeSecond = 22,
      PartKind_DateTimeTimeZoneName = 23,
      PartKind_DateTimeStartRange = 24,
      PartKind_DateTimeEndRange = 25,
      PartKind_RelativeTimeLiteral = 26,
      PartKind_DurationLiteral = 27,
      PartKind_UnitYear = 28,
      PartKind_UnitMonth = 29,
      PartKind_UnitWeek = 30,
      PartKind_UnitDay = 31,
      PartKind_UnitHour = 32,
      PartKind_UnitMinute = 33,
      PartKind_UnitSecond = 34,
      PartKind_UnitMillisecond = 35,
      PartKind_UnitMicrosecond = 36,
      PartKind_UnitNanosecond = 37,
    };

    typedef struct PartKind_option {union { PartKind ok; }; bool is_ok; } PartKind_option;
} // namespace capi
} // namespace

namespace icu4x {
/**
 * The kind of a {@link PartSpan}, corresponding to `writeable::Part`.
 *
 * See the [Rust documentation for `Part`](https://docs.rs/writeable/0.6.4/writeable/struct.Part.html) for more information.
 *
 * 🚧 This API is unstable and may experience breaking changes outside major releases.
 */
class PartKind {
public:
    enum Value {
        Unknown = 0,
        Error = 1,
        ListElement = 2,
        ListLiteral = 3,
        DecimalPlusSign = 4,
        DecimalMinusSign = 5,
        DecimalInteger = 6,
        DecimalFraction = 7,
        DecimalGroup = 8,
        DecimalDecimal = 9,
        DateTimeEra = 10,
        DateTimeYear = 11,
        DateTimeRelatedYear = 12,
        DateTimeYearName = 13,
        DateTimeMonth = 14,
        DateTimeDay = 15,
        DateTimeExtendedYear = 16,
        DateTimeJulianDay = 17,
        DateTimeWeekday = 18,
        DateTimeDayPeriod = 19,
        DateTimeHour = 20,
        DateTimeMinute = 21,
        DateTimeSecond = 22,
        DateTimeTimeZoneName = 23,
        DateTimeStartRange = 24,
        DateTimeEndRange = 25,
        RelativeTimeLiteral = 26,
        DurationLiteral = 27,
        UnitYear = 28,
        UnitMonth = 29,
        UnitWeek = 30,
        UnitDay = 31,
        UnitHour = 32,
        UnitMinute = 33,
        UnitSecond = 34,
        UnitMillisecond = 35,
        UnitMicrosecond = 36,
        UnitNanosecond = 37,
    };

    PartKind(): value(Value::Unknown) {}

    // Implicit conversions between enum and ::Value
    constexpr PartKind(Value v) : value(v) {}
    constexpr operator Value() const { return value; }
    // Prevent usage as boolean value
    explicit operator bool() const = delete;

    inline icu4x::capi::PartKind AsFFI() const;
    inline static icu4x::PartKind FromFFI(icu4x::capi::PartKind c_enum);
private:
    Value value;
};

} // namespace
#endif // ICU4X_PartKind_D_HPP
