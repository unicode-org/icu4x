#ifndef ICU4XTimeZoneInvalidOffsetError_HPP
#define ICU4XTimeZoneInvalidOffsetError_HPP

#include "ICU4XTimeZoneInvalidOffsetError.d.hpp"

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


inline capi::ICU4XTimeZoneInvalidOffsetError ICU4XTimeZoneInvalidOffsetError::AsFFI() const {
  return static_cast<capi::ICU4XTimeZoneInvalidOffsetError>(value);
}

inline ICU4XTimeZoneInvalidOffsetError ICU4XTimeZoneInvalidOffsetError::FromFFI(capi::ICU4XTimeZoneInvalidOffsetError c_enum) {
  switch (c_enum) {
    case capi::ICU4XTimeZoneInvalidOffsetError_TodoZst:
      return static_cast<ICU4XTimeZoneInvalidOffsetError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XTimeZoneInvalidOffsetError_HPP
