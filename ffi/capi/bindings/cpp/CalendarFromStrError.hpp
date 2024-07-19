#ifndef CalendarFromStrError_HPP
#define CalendarFromStrError_HPP

#include "CalendarFromStrError.d.hpp"

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

inline diplomat::capi::CalendarFromStrError CalendarFromStrError::AsFFI() const {
  return static_cast<diplomat::capi::CalendarFromStrError>(value);
}

inline CalendarFromStrError CalendarFromStrError::FromFFI(diplomat::capi::CalendarFromStrError c_enum) {
  switch (c_enum) {
    case diplomat::capi::CalendarFromStrError_Unknown:
    case diplomat::capi::CalendarFromStrError_InvalidSyntax:
    case diplomat::capi::CalendarFromStrError_OutOfRange:
    case diplomat::capi::CalendarFromStrError_MissingFields:
    case diplomat::capi::CalendarFromStrError_UnknownCalendar:
      return static_cast<CalendarFromStrError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CalendarFromStrError_HPP
