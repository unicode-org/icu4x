#ifndef ICU4XLocaleFallbackPriority_HPP
#define ICU4XLocaleFallbackPriority_HPP

#include "ICU4XLocaleFallbackPriority.d.hpp"

#include <stdio.h>
#include <stdint.h>
#include <stddef.h>
#include <stdbool.h>
#include <memory>
#include <optional>
#include "diplomat_runtime.hpp"
#include "ICU4XLocaleFallbackPriority.h"


inline capi::ICU4XLocaleFallbackPriority ICU4XLocaleFallbackPriority::AsFFI() const {
  return static_cast<capi::ICU4XLocaleFallbackPriority>(value);
}

inline ICU4XLocaleFallbackPriority ICU4XLocaleFallbackPriority::FromFFI(capi::ICU4XLocaleFallbackPriority c_enum) {
  switch (c_enum) {
    case capi::ICU4XLocaleFallbackPriority_Language:
    case capi::ICU4XLocaleFallbackPriority_Region:
    case capi::ICU4XLocaleFallbackPriority_Collation:
      return static_cast<ICU4XLocaleFallbackPriority::Value>(c_enum);
    default:
      abort();
  }
}
#endif // ICU4XLocaleFallbackPriority_HPP
