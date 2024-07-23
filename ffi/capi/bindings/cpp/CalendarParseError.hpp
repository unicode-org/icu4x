#ifndef CalendarParseError_HPP
#define CalendarParseError_HPP

#include "CalendarParseError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::CalendarParseError CalendarParseError::AsFFI() const {
  return static_cast<diplomat::capi::CalendarParseError>(value);
}

inline CalendarParseError CalendarParseError::FromFFI(diplomat::capi::CalendarParseError c_enum) {
  switch (c_enum) {
    case diplomat::capi::CalendarParseError_Unknown:
    case diplomat::capi::CalendarParseError_InvalidSyntax:
    case diplomat::capi::CalendarParseError_OutOfRange:
    case diplomat::capi::CalendarParseError_MissingFields:
    case diplomat::capi::CalendarParseError_UnknownCalendar:
      return static_cast<CalendarParseError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CalendarParseError_HPP
