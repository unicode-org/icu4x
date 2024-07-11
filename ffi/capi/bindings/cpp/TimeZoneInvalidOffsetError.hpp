#ifndef TimeZoneInvalidOffsetError_HPP
#define TimeZoneInvalidOffsetError_HPP

#include "TimeZoneInvalidOffsetError.d.hpp"

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


inline capi::TimeZoneInvalidOffsetError TimeZoneInvalidOffsetError::AsFFI() const {
  return static_cast<capi::TimeZoneInvalidOffsetError>(value);
}

inline TimeZoneInvalidOffsetError TimeZoneInvalidOffsetError::FromFFI(capi::TimeZoneInvalidOffsetError c_enum) {
  switch (c_enum) {
    case capi::TimeZoneInvalidOffsetError_TodoZst:
      return static_cast<TimeZoneInvalidOffsetError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TimeZoneInvalidOffsetError_HPP
