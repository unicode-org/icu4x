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


namespace capi {
    extern "C" {
    
    
    } // extern "C"
}


inline capi::TimeZoneInvalidIdError TimeZoneInvalidIdError::AsFFI() const {
  return static_cast<capi::TimeZoneInvalidIdError>(value);
}

inline TimeZoneInvalidIdError TimeZoneInvalidIdError::FromFFI(capi::TimeZoneInvalidIdError c_enum) {
  switch (c_enum) {
    case capi::TimeZoneInvalidIdError_TodoZst:
      return static_cast<TimeZoneInvalidIdError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // TimeZoneInvalidIdError_HPP
