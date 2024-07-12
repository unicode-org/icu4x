#ifndef TimeZoneInvalidIdError_HPP
#define TimeZoneInvalidIdError_HPP

#include "TimeZoneInvalidIdError.d.hpp"

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

inline diplomat::capi::TimeZoneInvalidIdError TimeZoneInvalidIdError::AsFFI() const {
  return static_cast<diplomat::capi::TimeZoneInvalidIdError>(value);
}

inline TimeZoneInvalidIdError TimeZoneInvalidIdError::FromFFI(diplomat::capi::TimeZoneInvalidIdError c_enum) {
  switch (c_enum) {
    case diplomat::capi::TimeZoneInvalidIdError_TodoZst:
      return static_cast<TimeZoneInvalidIdError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TimeZoneInvalidIdError_HPP
