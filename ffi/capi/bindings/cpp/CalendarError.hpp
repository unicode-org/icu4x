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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::CalendarError CalendarError::AsFFI() const {
  return static_cast<capi::CalendarError>(value);
}

inline CalendarError CalendarError::FromFFI(capi::CalendarError c_enum) {
  switch (c_enum) {
    case capi::CalendarError_Unknown:
    case capi::CalendarError_OutOfRange:
    case capi::CalendarError_UnknownEra:
    case capi::CalendarError_UnknownMonthCode:
      return static_cast<CalendarError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // CalendarError_HPP
