#ifndef LocaleFallbackPriority_HPP
#define LocaleFallbackPriority_HPP

#include "LocaleFallbackPriority.d.hpp"

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


inline capi::LocaleFallbackPriority LocaleFallbackPriority::AsFFI() const {
  return static_cast<capi::LocaleFallbackPriority>(value);
}

inline LocaleFallbackPriority LocaleFallbackPriority::FromFFI(capi::LocaleFallbackPriority c_enum) {
  switch (c_enum) {
    case capi::LocaleFallbackPriority_Language:
    case capi::LocaleFallbackPriority_Region:
    case capi::LocaleFallbackPriority_Collation:
      return static_cast<LocaleFallbackPriority::Value>(c_enum);
    default:
      abort();
  }
}
#endif // LocaleFallbackPriority_HPP
