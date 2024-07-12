#ifndef CalendarError_HPP
#define CalendarError_HPP

#include "CalendarError.d.hpp"

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

inline diplomat::capi::CalendarError CalendarError::AsFFI() const {
  return static_cast<diplomat::capi::CalendarError>(value);
}

inline CalendarError CalendarError::FromFFI(diplomat::capi::CalendarError c_enum) {
  switch (c_enum) {
    case diplomat::capi::CalendarError_Unknown:
    case diplomat::capi::CalendarError_OutOfRange:
    case diplomat::capi::CalendarError_UnknownEra:
    case diplomat::capi::CalendarError_UnknownMonthCode:
      return static_cast<CalendarError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CalendarError_HPP
