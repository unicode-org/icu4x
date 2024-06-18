#ifndef ICU4XTimeZoneInvalidIdError_HPP
#define ICU4XTimeZoneInvalidIdError_HPP

#include "ICU4XTimeZoneInvalidIdError.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XTimeZoneInvalidIdError.h"


inline capi::ICU4XTimeZoneInvalidIdError ICU4XTimeZoneInvalidIdError::AsFFI() const {
  return static_cast<capi::ICU4XTimeZoneInvalidIdError>(value);
}

inline ICU4XTimeZoneInvalidIdError ICU4XTimeZoneInvalidIdError::FromFFI(capi::ICU4XTimeZoneInvalidIdError c_enum) {
  switch (c_enum) {
    case capi::ICU4XTimeZoneInvalidIdError_TodoZst:
      return static_cast<ICU4XTimeZoneInvalidIdError::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XTimeZoneInvalidIdError_HPP
