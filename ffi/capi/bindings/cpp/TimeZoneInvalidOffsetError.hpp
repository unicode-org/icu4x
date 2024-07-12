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


namespace diplomat {
namespace capi {
    extern "C" {
    
    
    } // extern "C"
} // namespace capi
} // namespace

inline diplomat::capi::TimeZoneInvalidOffsetError TimeZoneInvalidOffsetError::AsFFI() const {
  return static_cast<diplomat::capi::TimeZoneInvalidOffsetError>(value);
}

inline TimeZoneInvalidOffsetError TimeZoneInvalidOffsetError::FromFFI(diplomat::capi::TimeZoneInvalidOffsetError c_enum) {
  switch (c_enum) {
    case diplomat::capi::TimeZoneInvalidOffsetError_TodoZst:
      return static_cast<TimeZoneInvalidOffsetError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TimeZoneInvalidOffsetError_HPP
