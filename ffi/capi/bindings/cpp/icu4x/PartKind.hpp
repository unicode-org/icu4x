#ifndef ICU4X_PartKind_HPP
#define ICU4X_PartKind_HPP

#include "PartKind.d.hpp"

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

inline icu4x::capi::PartKind icu4x::PartKind::AsFFI() const {
    return static_cast<icu4x::capi::PartKind>(value);
}

inline icu4x::PartKind icu4x::PartKind::FromFFI(icu4x::capi::PartKind c_enum) {
    switch (c_enum) {
        case icu4x::capi::PartKind_Unknown:
        case icu4x::capi::PartKind_Error:
        case icu4x::capi::PartKind_ListElement:
        case icu4x::capi::PartKind_ListLiteral:
        case icu4x::capi::PartKind_DecimalPlusSign:
        case icu4x::capi::PartKind_DecimalMinusSign:
        case icu4x::capi::PartKind_DecimalInteger:
        case icu4x::capi::PartKind_DecimalFraction:
        case icu4x::capi::PartKind_DecimalGroup:
        case icu4x::capi::PartKind_DecimalDecimal:
        case icu4x::capi::PartKind_DateTimeEra:
        case icu4x::capi::PartKind_DateTimeYear:
        case icu4x::capi::PartKind_DateTimeRelatedYear:
        case icu4x::capi::PartKind_DateTimeYearName:
        case icu4x::capi::PartKind_DateTimeMonth:
        case icu4x::capi::PartKind_DateTimeDay:
        case icu4x::capi::PartKind_DateTimeExtendedYear:
        case icu4x::capi::PartKind_DateTimeJulianDay:
        case icu4x::capi::PartKind_DateTimeWeekday:
        case icu4x::capi::PartKind_DateTimeDayPeriod:
        case icu4x::capi::PartKind_DateTimeHour:
        case icu4x::capi::PartKind_DateTimeMinute:
        case icu4x::capi::PartKind_DateTimeSecond:
        case icu4x::capi::PartKind_DateTimeTimeZoneName:
        case icu4x::capi::PartKind_DateTimeStartRange:
        case icu4x::capi::PartKind_DateTimeEndRange:
        case icu4x::capi::PartKind_RelativeTimeLiteral:
        case icu4x::capi::PartKind_DurationLiteral:
        case icu4x::capi::PartKind_UnitYear:
        case icu4x::capi::PartKind_UnitMonth:
        case icu4x::capi::PartKind_UnitWeek:
        case icu4x::capi::PartKind_UnitDay:
        case icu4x::capi::PartKind_UnitHour:
        case icu4x::capi::PartKind_UnitMinute:
        case icu4x::capi::PartKind_UnitSecond:
        case icu4x::capi::PartKind_UnitMillisecond:
        case icu4x::capi::PartKind_UnitMicrosecond:
        case icu4x::capi::PartKind_UnitNanosecond:
            return static_cast<icu4x::PartKind::Value>(c_enum);
        default:
            std::abort();
    }
}
#endif // ICU4X_PartKind_HPP
