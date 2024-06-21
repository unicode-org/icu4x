#ifndef ICU4XCalendarError_HPP
#define ICU4XCalendarError_HPP

#include "ICU4XCalendarError.d.hpp"

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


inline capi::ICU4XCalendarError ICU4XCalendarError::AsFFI() const {
  return static_cast<capi::ICU4XCalendarError>(value);
}

inline ICU4XCalendarError ICU4XCalendarError::FromFFI(capi::ICU4XCalendarError c_enum) {
  switch (c_enum) {
    case capi::ICU4XCalendarError_Unknown:
    case capi::ICU4XCalendarError_OutOfRange:
    case capi::ICU4XCalendarError_UnknownEra:
    case capi::ICU4XCalendarError_UnknownMonthCode:
      return static_cast<ICU4XCalendarError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XCalendarError_HPP
